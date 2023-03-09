use crate::dalvik_instruction::DalvikInstruction;

pub const OPCODE: u8 = 0x0E;

pub fn decode(_: Vec<u8>) -> DalvikInstruction {
    DalvikInstruction {
        opcode: OPCODE,
        parameters: vec![],
    }
}
