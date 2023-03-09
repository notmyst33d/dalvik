use dalvikdex::{dex::Dex, dex_method::DexMethod};
use dalvikinst::{sget_object, const_string, invoke_virtual, return_void};

pub struct DalvikVm {
    dex: Dex,
}

impl DalvikVm {
    pub fn new(dex: Dex) -> Self {
        Self { dex }
    }

    pub fn interpret_method(&self, method: &DexMethod) {
        method.instructions.iter().for_each(|instruction| {
            match instruction.opcode {
                sget_object::OPCODE => {
                    let register = instruction.parameters[0];
                    let object = &self.dex.field_ids[instruction.parameters[1] as usize];
                    println!("[sget-object] Register: {}, Object: {:#?}", register, object);
                },
                const_string::OPCODE => {
                    let register = instruction.parameters[0];
                    let object = &self.dex.strings[instruction.parameters[1] as usize];
                    println!("[const-string] Register: {}, String: {}", register, object);
                },
                invoke_virtual::OPCODE => {
                    let argument_count = instruction.parameters[0];
                    let method = &self.dex.method_ids[instruction.parameters[1] as usize];
                    println!("[invoke-virtual] Argument count: {}, Method: {:#?}", argument_count, method);
                },
                return_void::OPCODE => {
                    println!("[return-void]");
                },
                _ => panic!("Unimplemented opcode 0x{:02X} can cause undefined behaviour, aborting...", instruction.opcode)
            };
        });
    }

    pub fn run(&self) -> Result<(), String> {
        let main = match self.dex.methods.iter().find(|method| method.name == "main") {
            Some(result) => result,
            None => return Err(String::from("Cannot find main method")),
        };

        self.interpret_method(main);

        Ok(())
    }
}
