use std::{collections::BTreeMap, io};

use dalvikcommon::vec_reader::VecReader;

use crate::{dalvik_instruction::DalvikInstruction, register_default_decoders};

#[derive(Debug)]
pub struct DalvikInstructionDecoder {
    decoders: BTreeMap<u8, (fn(Vec<u8>) -> DalvikInstruction, usize)>,
}

impl DalvikInstructionDecoder {
    pub fn new() -> Self {
        let mut s = Self {
            decoders: BTreeMap::new(),
        };

        register_default_decoders(&mut s);

        s
    }

    pub fn register(&mut self, opcode: u8, decoder: fn(Vec<u8>) -> DalvikInstruction, size: usize) {
        self.decoders.insert(opcode, (decoder, size));
    }

    pub fn decode(&self, reader: &mut VecReader) -> io::Result<DalvikInstruction> {
        let opcode = reader.read_u8()?;
        let decoder = match self.decoders.get(&opcode) {
            Some(result) => result,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Cannot find instruction decoder for 0x{opcode:02X}"),
                ))
            }
        };

        Ok(decoder.0(reader.read_vec(decoder.1 - 1)?))
    }
}
