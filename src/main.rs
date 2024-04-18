use std::{env, fs};

use dalvikdex::Dex;
use dalvikvm::DalvikVm;

const CHUNK_SIZE: usize = 4096;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dex_file = fs::read(&args[1]).unwrap();
    let dex = match Dex::new(dex_file) {
        Ok(result) => result,
        Err(error) => panic!("Cannot create Dex: {error}"),
    };

    println!("{:#?}", dex);
    let mut vm = DalvikVm::new(dex);
    match vm.run() {
        Ok(_) => {},
        Err(error) => panic!("VM returned an error: {error}"),
    };
}
