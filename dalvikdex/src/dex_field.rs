#[derive(Debug)]
pub struct DexField {
    pub name: String,
    pub field_type: String,
    pub class: String,
    pub access_flags: u32,
    pub access_type: DexFieldType,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DexFieldType {
    Static,
    Instance,
}
