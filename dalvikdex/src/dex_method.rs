use dalvikinst::dalvik_instruction::DalvikInstruction;

#[derive(Debug)]
pub struct DexMethod {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: String,
    pub class: String,
    pub access_flags: u32,
    pub instructions: Vec<DalvikInstruction>,
}
