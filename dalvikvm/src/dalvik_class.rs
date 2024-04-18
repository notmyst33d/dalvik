use std::sync::Arc;
use crate::{DalvikVm, DalvikObject};

pub trait DalvikClass {
    fn sget_object(&self, name: &str) -> Option<DalvikObject>;
    fn invoke_virtual(&self, vm: &DalvikVm, name: &str, parameters: &[u8]) -> Option<DalvikObject>;
}
