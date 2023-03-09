use crate::{dex_field::DexField, dex_method::DexMethod};

#[derive(Debug)]
pub struct DexClass {
    pub name: String,
    pub fields: Vec<DexField>,
    pub methods: Vec<DexMethod>,
    pub access_flags: u32,
}
