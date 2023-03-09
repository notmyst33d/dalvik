use crate::dalvik_instruction::DalvikInstruction;

pub const OPCODE: u8 = 0x0F;

pub fn decode(data: Vec<u8>) -> DalvikInstruction {
    let return_value_regiser = data[0];

    DalvikInstruction {
        opcode: OPCODE,
        parameters: vec![return_value_regiser as u32],
    }
}
