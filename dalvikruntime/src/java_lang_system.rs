use std::sync::Arc;
use dalvikvm::{DalvikClass, DalvikObject, DalvikVm};
use crate::JavaIoPrintStream;

pub struct JavaLangSystem {
    out: Option<Arc<JavaIoPrintStream>>,
}

impl JavaLangSystem {
    pub fn new() -> Self {
        Self {
            out: Some(Arc::new(JavaIoPrintStream::new())),
        }
    }
}

impl DalvikClass for JavaLangSystem {
    fn sget_object(&self, name: &str) -> Option<DalvikObject> {
        match name {
            "out" => self.out.clone().map(|v| DalvikObject::Class(v)),
            _ => None,
        }
    }

    fn invoke_virtual(&self, vm: &DalvikVm, name: &str, parameters: &[u8]) -> Option<DalvikObject> {
        todo!()
    }
}
