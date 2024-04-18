use std::sync::Arc;
use dalvikvm::{DalvikClass, DalvikObject, DalvikVm};

#[derive(Clone)]
pub struct JavaIoPrintStream;

impl JavaIoPrintStream {
    pub fn new() -> Self {
        Self {}
    }

    fn println(&self, string: &DalvikObject) {
        match string {
            DalvikObject::String(string) => println!("{}", string),
            _ => panic!("expected string, got something else idk"),
        }
    }
}

impl DalvikClass for JavaIoPrintStream {
    fn sget_object(&self, name: &str) -> Option<DalvikObject> {
        todo!()
    }

    fn invoke_virtual(&self, vm: &DalvikVm, name: &str, parameters: &[u8]) -> Option<DalvikObject> {
        match name {
            "println" => self.println(&vm.registers[&parameters[0]]),
            _ => (),
        };
        None
    }
}
