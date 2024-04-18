use dalvikinst::dalvik_instruction::DalvikInstruction;

#[derive(Clone, Debug)]
pub struct DexMethod {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: String,
    pub class: String,
    pub access_flags: u32,
    pub access_type: DexMethodType,
    pub instructions: Vec<DalvikInstruction>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DexMethodType {
    Direct,
    Virtual,
}
