use crate::dalvik_instruction::DalvikInstruction;

pub const OPCODE: u8 = 0x01;

pub fn decode(data: Vec<u8>) -> DalvikInstruction {
    let source_register = (data[0] & 0xF0) >> 4;
    let destination_register = data[0] & 0x0F;

    DalvikInstruction {
        opcode: OPCODE,
        parameters: vec![destination_register as u32, source_register as u32],
    }
}
