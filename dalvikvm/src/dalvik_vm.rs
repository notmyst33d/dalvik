use std::collections::HashMap;

use dalvikdex::{Dex, DexMethod};
use dalvikinst::DalvikInstruction;

pub struct DalvikVm {
    dex: Dex,
}

impl DalvikVm {
    pub fn new(dex: Dex) -> Self {
        Self {
            dex,
        }
    }

    pub fn run_method(&mut self, method: &DexMethod) {
        method.instructions.iter().for_each(|instruction| {
            match instruction {
                DalvikInstruction::SgetObject { register, field_index } => {
                    let field_id = &self.dex.field_ids[*field_index as usize];

                    #[cfg(debug_assertions)]
                    println!(
                        "sget-object v{}, {}->{}:{}",
                        register, field_id.class_id, field_id.name_id, field_id.type_id,
                    );
                }
                DalvikInstruction::ConstString { register, string_index } => {
                    let string = &self.dex.strings[*string_index as usize];

                    #[cfg(debug_assertions)]
                    println!("const-string v{}, \"{}\"", register, string);
                }
                DalvikInstruction::InvokeVirtual(data) => {
                    let method_id = &self.dex.method_ids[data.method_index as usize];

                    #[cfg(debug_assertions)]
                    println!(
                        "invoke-virtual {{{:?}}}, {}->{}({})",
                        data.parameters,
                        method_id.class_id,
                        method_id.name_id,
                        method_id.proto_id.shorty_id,
                    );
                }
                DalvikInstruction::ReturnVoid => {
                    #[cfg(debug_assertions)]
                    println!("return-void");
                }
                _ => panic!(
                    "Unimplemented instruction {:#?} can cause undefined behaviour, aborting...",
                    instruction
                ),
            };

            #[cfg(debug_assertions)] {
                println!("=== VM REGISTERS DUMP ===");
                /*self.registers.iter().for_each(|(register, value)| {
                    match value {
                        DalvikObject::DalvikString(string) => {
                            println!("v{}: \"{}\"", register, string.value);
                        }
                        DalvikObject::DalvikClass(_) => todo!(),
                    };
                });*/
                println!("=========================");
            }
        });
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
