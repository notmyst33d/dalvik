use dalvik_instruction_decoder::DalvikInstructionDecoder;

pub mod r#const;
pub mod const_string;
pub mod invoke_common;
pub mod invoke_direct;
pub mod invoke_virtual;
pub mod r#move;
pub mod move_from_16;
pub mod new_instance;
pub mod r#return;
pub mod return_void;
pub mod sget_object;

pub mod dalvik_instruction;
pub mod dalvik_instruction_decoder;

pub fn register_default_decoders(decoder: &mut DalvikInstructionDecoder) {
    decoder.register(r#const::OPCODE, r#const::decode, 6);
    decoder.register(const_string::OPCODE, const_string::decode, 4);
    decoder.register(invoke_direct::OPCODE, invoke_direct::decode, 6);
    decoder.register(invoke_virtual::OPCODE, invoke_virtual::decode, 6);
    decoder.register(r#move::OPCODE, r#move::decode, 2);
    decoder.register(move_from_16::OPCODE, move_from_16::decode, 4);
    decoder.register(new_instance::OPCODE, new_instance::decode, 4);
    decoder.register(r#return::OPCODE, r#return::decode, 2);
    decoder.register(return_void::OPCODE, return_void::decode, 2);
    decoder.register(sget_object::OPCODE, sget_object::decode, 4);
}
