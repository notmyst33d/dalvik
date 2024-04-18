//use dalvik_instruction_decoder::DalvikInstructionDecoder;

//pub mod inst;
pub mod dalvik_instruction;
pub mod dalvik_inst_error;

pub use dalvik_instruction::{DalvikInstruction, decode_inst};
pub use dalvik_inst_error::DalvikInstError;
//pub mod dalvik_instruction_decoder;

/*pub fn register_default_decoders(decoder: &mut DalvikInstructionDecoder) {
    decoder.register(inst::const_string::OPCODE, const_string::decode, 4);
    decoder.register(inst::invoke_direct::OPCODE, invoke_direct::decode, 6);
    decoder.register(inst::invoke_virtual::OPCODE, invoke_virtual::decode, 6);
    decoder.register(return_void::OPCODE, return_void::decode, 2);
    decoder.register(sget_object::OPCODE, sget_object::decode, 4);
}*/
