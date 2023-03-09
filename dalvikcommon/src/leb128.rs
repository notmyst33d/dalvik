use std::{io::Read, io::Result};

pub struct LEB128;

impl LEB128 {
    pub fn read_uleb128<R: Read>(reader: &mut R) -> Result<u128> {
        let mut byte: [u8; 1] = [0; 1];
        let mut value: u128 = 0;

        for increment in 0..16 {
            reader.read(&mut byte)?;
            value |= ((byte[0] & 0x7F) as u128) << increment * 7;
            if byte[0] < 0x7F {
                break;
            }
        }

        Ok(value)
    }
}
