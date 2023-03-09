use crate::dalvik_instruction::DalvikInstruction;

pub const OPCODE: u8 = 0x14;

pub fn decode(data: Vec<u8>) -> DalvikInstruction {
    let destination_register = data[0];
    let value =
        (data[4] as u32) << 24 | (data[3] as u32) << 16 | (data[2] as u32) << 8 | (data[1] as u32);

    DalvikInstruction {
        opcode: OPCODE,
        parameters: vec![destination_register as u32, value],
    }
}
