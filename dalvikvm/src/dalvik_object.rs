use std::sync::Arc;
use crate::{DalvikVm, DalvikClass};

pub enum DalvikObject {
    String(String),
    Class(Arc<dyn DalvikClass>),
}

