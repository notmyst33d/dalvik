use crate::dalvik_instruction::DalvikInstruction;

use super::invoke_common;

pub const OPCODE: u8 = 0x70;

pub fn decode(data: Vec<u8>) -> DalvikInstruction {
    DalvikInstruction {
        opcode: OPCODE,
        parameters: invoke_common::decode_parameters(data),
    }
}
