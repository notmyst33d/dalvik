use crate::{DalvikCommonError, LEB128};

macro_rules! impl_primitive_read {
    ($n:tt, $s:expr, $t:ty) => {
        #[inline]
        pub fn $n(&mut self) -> Result<$t, DalvikCommonError> {
            if self.position + $s > self.data.len() {
                return Err(DalvikCommonError::NotEnoughData);
            }
            self.position += $s;
            Ok(unsafe {
                (self.data.as_ptr().add(self.position - $s) as *const $t).read_unaligned()
            })
        }
    };
}

#[derive(Debug)]
pub struct BytesBuffer {
    pub data: Vec<u8>,
    pub position: usize,
}

impl BytesBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }

    impl_primitive_read!(read_u8, 1, u8);
    impl_primitive_read!(read_u16, 2, u16);
    impl_primitive_read!(read_u32, 4, u32);

    pub fn read(&mut self, length: usize) -> Result<&[u8], DalvikCommonError> {
        if self.position + length > self.data.len() {
            return Err(DalvikCommonError::NotEnoughData)
        }
        self.position += length;
        Ok(&self.data[self.position - length..self.position])
    }

    pub fn read_uleb128(&mut self) -> Result<u128, DalvikCommonError> {
        LEB128::read_uleb128(self)
    }

    pub fn read_string(&mut self) -> Result<String, DalvikCommonError> {
        let mut buffer: Vec<u8> = vec![];
        let string_length = LEB128::read_uleb128(self)?;
        loop {
            let byte = self.read_u8()?;
            if byte == 0 {
                break;
            }

            buffer.push(byte);
        }
        let string = match String::from_utf8(buffer) {
            Ok(result) => result,
            Err(_) => return Err(DalvikCommonError::InvalidString),
        };
        if string.len() != string_length as usize {
            return Err(DalvikCommonError::StringSizeDoesNotMatch);
        };
        Ok(string)
    }
}
