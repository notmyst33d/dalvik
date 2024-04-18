use std::sync::Arc;
use dalvikvm::{DalvikVm, DalvikObject};

pub mod java_lang_system;
pub mod java_io_printstream;

pub use java_lang_system::JavaLangSystem;
pub use java_io_printstream::JavaIoPrintStream;

pub fn register_classes(vm: &mut DalvikVm) {
    vm.add_class("Ljava/lang/System;", DalvikObject::Class(Arc::new(JavaLangSystem::new())));
}
