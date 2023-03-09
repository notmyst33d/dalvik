use std::io;

use dalvikcommon::vec_reader::VecReader;

use crate::dalvik_instruction_decoder::DalvikInstructionDecoder;

#[derive(Debug)]
pub struct DalvikInstruction {
    pub opcode: u8,
    pub parameters: Vec<u32>,
}

impl DalvikInstruction {
    pub fn new() {}

    pub fn from_vec(data: Vec<u8>) -> io::Result<Vec<Self>> {
        let decoder = DalvikInstructionDecoder::new();
        let mut reader = VecReader::new(data);
        let mut instructions: Vec<Self> = vec![];

        while !reader.eof() {
            instructions.push(decoder.decode(&mut reader)?);
        }

        Ok(instructions)
    }
}
