use std::sync::Arc;
use std::collections::HashMap;
use dalvikdex::{Dex, DexMethod};
use dalvikinst::DalvikInstruction;
use crate::DalvikObject;

pub struct DalvikVm {
    dex: Dex,
    classes: HashMap<String, DalvikObject>,
    pub registers: HashMap<u8, DalvikObject>,
}

impl DalvikVm {
    pub fn new(dex: Dex) -> Self {
        Self {
            dex,
            classes: HashMap::new(),
            registers: HashMap::new(),
        }
    }

    pub fn run_method(&mut self, method: &DexMethod) {
        method.instructions.iter().for_each(|instruction| {
            match instruction {
                DalvikInstruction::SgetObject { register, field_index } => {
                    let field_id = &self.dex.field_ids[*field_index as usize];
                    #[cfg(debug_assertions)]
                    println!(
                        "trace: v{} = {}.{} (with type: {})",
                        register, &field_id.class_id[1..field_id.class_id.len() - 1].replace("/", "."), field_id.name_id, &field_id.type_id[1..field_id.type_id.len() - 1].replace("/", "."),
                    );

                    let class = match self.classes.get(&field_id.class_id) {
                        Some(result) => match result {
                            DalvikObject::Class(result) => result,
                            _ => panic!("expected class, got something else idk"),
                        },
                        None => panic!("class not registered: {}", field_id.class_id),
                    };

                    self.registers.insert(*register, match class.sget_object(&field_id.name_id) {
                        Some(result) => result,
                        None => panic!("nonexistent field \"{}\" on class {}", field_id.name_id, &field_id.class_id[1..field_id.class_id.len() - 1].replace("/", ".")),
                    });
                }
                DalvikInstruction::ConstString { register, string_index } => {
                    let string = &self.dex.strings[*string_index as usize];
                    #[cfg(debug_assertions)]
                    println!("trace: v{} = \"{}\"", register, string);

                    self.registers.insert(*register, DalvikObject::String(string.to_string()));
                }
                DalvikInstruction::InvokeVirtual(data) => {
                    let method_id = &self.dex.method_ids[data.method_index as usize];
                    #[cfg(debug_assertions)]
                    println!(
                        "trace: invoke-virtual {{{:?}}}, {}->{}({})",
                        data.parameters,
                        method_id.class_id,
                        method_id.name_id,
                        method_id.proto_id.shorty_id,
                    );

                    let class = match &self.registers[&data.parameters[0]] {
                        DalvikObject::Class(result) => result,
                        _ => panic!("expected class, got something else idk"),
                    };
                    class.invoke_virtual(&self, &method_id.name_id, &data.parameters[1..]);
                }
                DalvikInstruction::ReturnVoid => {
                    #[cfg(debug_assertions)]
                    println!("trace: return");
                }
                _ => panic!(
                    "Unimplemented instruction {:#?} can cause undefined behaviour, aborting...",
                    instruction
                ),
            };
        });
    }

    pub fn add_class(&mut self, name: &str, class: DalvikObject) {
        self.classes.insert(name.to_string(), class);
    }

    pub fn run(&mut self) -> Result<(), String> {
        let main = match self.dex.methods.iter().find(|method| method.name == "main") {
            Some(result) => result.clone(),
            None => return Err(String::from("Cannot find main method")),
        };

        self.run_method(&main);

        Ok(())
    }
}
