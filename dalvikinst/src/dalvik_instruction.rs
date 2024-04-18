use crate::DalvikInstError;
use dalvikcommon::BytesBuffer;

pub fn decode_invoke_common(data: &[u8]) -> InvokeCommon {
    let mut parameters: Vec<u8> = vec![];

    let argument_word_count = (data[0] & 0xF0) >> 4;
    let method_index = (data[2] as u16) << 8 | (data[1] as u16);
    let fedc = (data[4] as u16) << 8 | (data[3] as u16);

    for increment in 0..argument_word_count {
        if increment == 4 {
            let g = (data[0] & 0x0F) as u32;
            parameters.push(g as u8);
            break;
        }
        let value = (fedc & (0x000F << 4 * increment)) >> 4 * increment;
        parameters.push(value as u8);
    }

    InvokeCommon { method_index, parameters }
}

#[derive(Debug, Clone)]
pub struct InvokeCommon {
    pub method_index: u16,
    pub parameters: Vec<u8>,
}

static DECODERS: &'static [(u32, u8, fn(&[u8]) -> Result<DalvikInstruction, DalvikInstError>)] = &[
    (0x0e, 2, |_| Ok(DalvikInstruction::ReturnVoid)),
    (0x1a, 4, |data| Ok(DalvikInstruction::ConstString { register: data[0], string_index: (data[2] as u16) << 8 | (data[1] as u16) })),
    (0x62, 4, |data| Ok(DalvikInstruction::SgetObject { register: data[0], field_index: (data[2] as u16) << 8 | (data[1] as u16) })),
    (0x6e, 6, |data| Ok(DalvikInstruction::InvokeVirtual(decode_invoke_common(data)))),
    (0x70, 6, |data| Ok(DalvikInstruction::InvokeDirect(decode_invoke_common(data)))),
];

#[derive(Debug, Clone)]
pub enum DalvikInstruction {
    ReturnVoid,
    ConstString { register: u8, string_index: u16 },
    SgetObject { register: u8, field_index: u16 },
    InvokeVirtual(InvokeCommon),
    InvokeDirect(InvokeCommon),
}

pub fn decode_inst(buffer: &mut BytesBuffer) -> Result<DalvikInstruction, DalvikInstError> {
    let opcode = buffer.read_u8()? as u32;

    for decoder in DECODERS {
        if decoder.0 == opcode {
            return decoder.2(buffer.read(decoder.1 as usize - 1)?);
        }
    }

    Err(DalvikInstError::NoDecoder(opcode))
}

/*
#[derive(Clone, Debug)]
pub struct DalvikInstruction {
    pub opcode: u8,
    pub parameters: Vec<u32>,
}

impl DalvikInstruction {
    pub fn new() {}

    pub fn from_vec(data: Vec<u8>) -> io::Result<Vec<Self>> {
        let decoder = DalvikInstructionDecoder::new();
        let mut buffer = VecReader::new(data);
        let mut instructions: Vec<Self> = vec![];

        while !buffer.eof() {
            instructions.push(decoder.decode(&mut buffer)?);
        }

        Ok(instructions)
    }
}
*/
