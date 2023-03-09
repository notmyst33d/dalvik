use crate::dalvik_instruction::DalvikInstruction;

pub const OPCODE: u8 = 0x1A;

pub fn decode(data: Vec<u8>) -> DalvikInstruction {
    let destination_register = data[0];
    let string_idx = (data[2] as u16) << 8 | (data[1] as u16);

    DalvikInstruction {
        opcode: OPCODE,
        parameters: vec![destination_register as u32, string_idx as u32],
    }
}
