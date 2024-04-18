use crate::{DalvikCommonError, BytesBuffer};

pub struct LEB128;

impl LEB128 {
    pub fn read_uleb128(buffer: &mut BytesBuffer) -> Result<u128, DalvikCommonError> {
        let mut value: u128 = 0;

        for increment in 0..16 {
            let byte = buffer.read_u8()?;
            value |= ((byte & 0x7F) as u128) << increment * 7;
            if byte < 0x7F {
                break;
            }
        }

        Ok(value)
    }
}
