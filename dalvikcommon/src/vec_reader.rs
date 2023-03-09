use std::io::{BufWriter, Read, Write, self};

use crate::leb128::LEB128;

#[derive(Debug)]
pub struct VecReader {
    data: Vec<u8>,
    position: usize,
}

impl VecReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }

    pub fn tell(&self) -> usize {
        self.position
    }

    pub fn seek(&mut self, position: usize) {
        self.position = 0;
        self.forward(position);
    }

    pub fn forward(&mut self, length: usize) -> usize {
        if self.position + length > self.data.len() {
            let remainder = self.data.len() - self.position;
            self.position += remainder;
            return remainder;
        }

        self.position += length;
        length
    }

    pub fn backward(&mut self, length: usize) -> usize {
        if self.position.checked_sub(length).is_none() {
            let remainder = self.position;
            self.position -= remainder;
            return remainder;
        }

        self.position -= length;
        length
    }

    pub fn eof(&self) -> bool {
        self.position == self.data.len()
    }

    // Convenicence methods
    pub fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer: [u8; 1] = [0; 1];
        self.read(&mut buffer)?;
        Ok(u8::from_le_bytes(buffer))
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer: [u8; 2] = [0; 2];
        self.read(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer: [u8; 4] = [0; 4];
        self.read(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    pub fn read_uleb128(&mut self) -> io::Result<u128> {
        LEB128::read_uleb128(self)
    }

    pub fn read_string(&mut self) -> io::Result<String> {
        let mut buffer: Vec<u8> = vec![];
        let mut byte: [u8; 1] = [0; 1];
        let string_length = LEB128::read_uleb128(self)?;
        loop {
            self.read(&mut byte)?;
            if byte[0] == 0 {
                break;
            }

            buffer.extend(byte);
        }
        let string = match String::from_utf8(buffer) {
            Ok(result) => result,
            Err(error) => return Err(io::Error::new(io::ErrorKind::Other, error)),
        };
        if string.len() != string_length as usize {
            return Err(io::Error::new(io::ErrorKind::Other, "String length doesnt match"));
        };
        Ok(string)
    }

    pub fn read_vec(&mut self, size: usize) -> io::Result<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read(&mut buffer)?;
        Ok(buffer)
    }

    pub fn read_vec_reader(&mut self, size: usize) -> io::Result<Self> {
        Ok(VecReader::new(self.read_vec(size)?))
    }
}

impl io::Read for VecReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let old_position = self.position;
        let length = buffer.len();
        let mut writer = BufWriter::new(buffer);

        for _ in 0..length {
            let byte = match self.data.get(self.position) {
                Some(result) => result,
                None => break,
            };

            if writer.write(&[*byte])? != 1 {
                return Err(io::Error::new(io::ErrorKind::Other, "Buffer cannot be written to"));
            }

            self.position += 1;
        }

        Ok(self.position - old_position)
    }
}
