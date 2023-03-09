use crate::dalvik_instruction::DalvikInstruction;

pub const OPCODE: u8 = 0x22;

pub fn decode(data: Vec<u8>) -> DalvikInstruction {
    let destination_register = data[0];
    let type_idx = (data[2] as u16) << 8 | (data[1] as u16);

    DalvikInstruction {
        opcode: OPCODE,
        parameters: vec![destination_register as u32, type_idx as u32],
    }
}
