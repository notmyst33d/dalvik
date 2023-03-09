# Dalvik
This repository contains several modules that implement different functionality

## dalvikdex
This module is used for parsing data from `.dex` files

- [X] Adler32 verification
- [X] SHA-1 verification
- [X] String parsing
- [X] Type ID parsing
- [X] Prototype ID parsing
- [X] Method ID parsing
- [X] Field ID parsing
- [X] Method parsing
- [X] Field parsing

## dalvikinst
This module is used for decoding Dalvik bytecode

- [X] const-string
- [X] invoke-virtual
- [X] return-void
- [X] return

Note: `dalvikinst` might support other instructions, but if they are not on this list that means they have not been tested yet

## dalvikvm
This module is used for interpreting decoded Dalvik bytecode

- [ ] Bytecode interpretation
- [ ] Standard library

## Performance notes
* `VecReader` and its convenience functions(`read_u16`, `read_u32`, etc) probably add a lot of overhead, but its probably fine as long as its only used when parsing something
* `opt-level` in release profile probably needs to be changed to 2(or maybe even 3) for better performance

## General code notes
* `Result<(), Box<dyn Error>>` <- That doesnt look like a good idea
* The entirety of `dalvikinst` is horrible
* I have no idea how to cleanly make instruction invocation in `DalvikVm`(maybe clean up `dalvikinst` and put something like an `execute` function in each of the instruction implementations in `dalvikinst`?)
