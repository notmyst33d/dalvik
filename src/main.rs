use std::{env, fs::File, io::Read, process::exit};

use dalvikdex::dex::Dex;
use dalvikvm::dalvik_vm::DalvikVm;

const CHUNK_SIZE: usize = 4096;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dex_file_path = match args.get(1) {
        Some(result) => result,
        None => {
            println!("DEX file not passed");
            exit(1);
        },
    };

    let mut dex_file = match File::open(dex_file_path) {
        Ok(result) => result,
        Err(error) => {
            println!("Cannot open file: {error}");
            exit(1);
        },
    };

    let mut dex_buffer: Vec<u8> = vec![];
    let mut dex_chunk: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

    loop {
        let length = match dex_file.read(&mut dex_chunk) {
            Ok(result) => result,
            Err(error) => {
                println!("Cannot read file: {error}");
                exit(1);
            },
        };

        if length != CHUNK_SIZE {
            dex_buffer.extend(&dex_chunk[..length]);
            break;
        }

        dex_buffer.extend(&dex_chunk);
    }

    let dex = match Dex::new(dex_buffer) {
        Ok(result) => result,
        Err(error) => panic!("Cannot create Dex: {error}"),
    };

    let vm = DalvikVm::new(dex);
    match vm.run() {
        Ok(_) => {},
        Err(error) => panic!("VM returned an error: {error}"),
    };
}
