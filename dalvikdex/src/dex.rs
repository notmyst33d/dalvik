// TODO: Temporary
#![allow(unused)]

use std::error::Error;

use adler32::adler32;
use crypto::{digest::Digest, sha1::Sha1};

use dalvikcommon::BytesBuffer;
use dalvikinst::{DalvikInstruction, decode_inst};

use crate::{
    dex_field::{DexField, DexFieldType}, dex_field_id::DexFieldId, dex_method::{DexMethod, DexMethodType},
    dex_method_id::DexMethodId, dex_proto_id::DexProtoId,
};

#[derive(Debug)]
pub enum DexEndianness {
    Little,
    Big,
}

#[derive(Debug)]
pub struct Dex {
    // Version of the DEX file, usually (0)35
    pub version: u8,

    // Endianness, currently its assumed to be little endian
    pub endianness: DexEndianness,

    // Strings
    pub strings: Vec<String>,

    // Methods
    pub methods: Vec<DexMethod>,

    // Fields
    pub fields: Vec<DexField>,

    // Various IDs
    pub type_ids: Vec<String>,
    pub proto_ids: Vec<DexProtoId>,
    pub field_ids: Vec<DexFieldId>,
    pub method_ids: Vec<DexMethodId>,
}

impl Dex {
    pub fn new(dex_buffer: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut buffer = BytesBuffer::new(dex_buffer.clone());
        if buffer.read(3)? != b"dex" {
            return Err("magic != \"dex\"".into());
        };

        buffer.position += 1;
        let version = String::from_utf8(buffer.read(3)?.to_vec())?.parse::<u8>()?;

        buffer.position += 1;
        let checksum = buffer.read_u32()?;
        match adler32(&dex_buffer.as_slice()[12..]) {
            Ok(result) => {
                if checksum != result {
                    return Err("Adler32 checksum doesnt match".into());
                }
            }
            Err(error) => return Err(format!("Adler32 checksum failed: {error}").into()),
        }

        let signature = buffer.read(20)?;
        let mut verify_signature: [u8; 20] = [0; 20];
        let mut sha = Sha1::new();
        sha.input(&dex_buffer.as_slice()[32..]);
        sha.result(&mut verify_signature);
        if signature != verify_signature {
            return Err("SHA1 signature verification failed".into());
        }

        let _file_size = buffer.read_u32()?;
        let _header_size = buffer.read_u32()?;

        let endianness = DexEndianness::Little;
        if buffer.read(4)? == &[0x12, 0x34, 0x56, 0x78] {
            return Err("Big endian is not supported (yet)".into());
        }

        let link_size = buffer.read_u32()?;
        let link_off = buffer.read_u32()?;
        let map_off = buffer.read_u32()?;
        let string_ids_size = buffer.read_u32()?;
        let string_ids_off = buffer.read_u32()?;
        let type_ids_size = buffer.read_u32()?;
        let type_ids_off = buffer.read_u32()?;
        let proto_ids_size = buffer.read_u32()?;
        let proto_ids_off = buffer.read_u32()?;
        let field_ids_size = buffer.read_u32()?;
        let field_ids_off = buffer.read_u32()?;
        let method_ids_size = buffer.read_u32()?;
        let method_ids_off = buffer.read_u32()?;
        let class_defs_size = buffer.read_u32()?;
        let class_defs_off = buffer.read_u32()?;
        let data_size = buffer.read_u32()?;
        let data_off = buffer.read_u32()?;

        let mut strings: Vec<String> = vec![];
        let mut methods: Vec<DexMethod> = vec![];
        let mut fields: Vec<DexField> = vec![];

        // IDs
        let mut type_ids: Vec<String> = vec![];
        let mut proto_ids: Vec<DexProtoId> = vec![];
        let mut field_ids: Vec<DexFieldId> = vec![];
        let mut method_ids: Vec<DexMethodId> = vec![];

        // Get strings
        buffer.position = string_ids_off as usize;
        for _ in 0..string_ids_size {
            let position = buffer.read_u32()?;
            let previous_position = buffer.position;
            buffer.position = position as usize;
            strings.push(buffer.read_string()?);
            buffer.position = previous_position;
        }

        // Get type IDs
        buffer.position = type_ids_off as usize;
        for _ in 0..type_ids_size {
            let descriptor_idx = buffer.read_u32()?;
            type_ids.push(strings[descriptor_idx as usize].clone());
        }

        // Get proto IDs
        buffer.position = proto_ids_off as usize;
        for _ in 0..proto_ids_size {
            let shorty_idx = buffer.read_u32()?;
            let return_type_idx = buffer.read_u32()?;
            let parameters_off = buffer.read_u32()?;

            let shorty_id = strings[shorty_idx as usize].clone();
            let return_type_id = type_ids[return_type_idx as usize].clone();
            let mut parameter_ids: Vec<String> = vec![];

            if parameters_off != 0 {
                let previous_position = buffer.position;
                buffer.position = parameters_off as usize;

                let type_list_size = buffer.read_u32()?;
                for _ in 0..type_list_size {
                    let type_idx = buffer.read_u16()?;
                    let type_id = type_ids[type_idx as usize].clone();
                    parameter_ids.push(type_id);
                }

                buffer.position = previous_position;
            };

            proto_ids.push(DexProtoId {
                shorty_id,
                return_type_id,
                parameter_ids,
            });
        }

        // Get field IDs
        buffer.position = field_ids_off as usize;
        for _ in 0..field_ids_size {
            let class_idx = buffer.read_u16()?;
            let type_idx = buffer.read_u16()?;
            let name_idx = buffer.read_u32()?;

            let class_id = type_ids[class_idx as usize].clone();
            let type_id = type_ids[type_idx as usize].clone();
            let name_id = strings[name_idx as usize].clone();

            field_ids.push(DexFieldId {
                class_id,
                type_id,
                name_id,
            });
        }

        // Get method IDs
        buffer.position = method_ids_off as usize;
        for _ in 0..method_ids_size {
            let class_idx = buffer.read_u16()?;
            let proto_idx = buffer.read_u16()?;
            let name_idx = buffer.read_u32()?;

            let class_id = type_ids[class_idx as usize].clone();
            let proto_id = proto_ids[proto_idx as usize].clone();
            let name_id = strings[name_idx as usize].clone();

            method_ids.push(DexMethodId {
                class_id,
                proto_id,
                name_id,
            });
        }

        buffer.position = class_defs_off as usize;
        for _ in 0..class_defs_size {
            let class_idx = buffer.read_u32()?;
            let access_flags = buffer.read_u32()?;
            let superclass_idx = buffer.read_u32()?;
            let interfaces_off = buffer.read_u32()?;
            let source_file_idx = buffer.read_u32()?;
            let annotations_off = buffer.read_u32()?;
            let class_data_off = buffer.read_u32()?;
            let static_values_off = buffer.read_u32()?;

            let class_id = type_ids[class_idx as usize].clone();

            let previous_position = buffer.position;
            buffer.position = class_data_off as usize;

            let static_fields_size = buffer.read_uleb128()?;
            let instance_fields_size = buffer.read_uleb128()?;
            let direct_methods_size = buffer.read_uleb128()?;
            let virtual_methods_size = buffer.read_uleb128()?;

            let mut read_fields = |buffer: &mut BytesBuffer,
                                   size: usize, access_type: DexFieldType|
             -> Result<(), Box<dyn Error>> {
                let mut previous_idx_diff = 0;
                for _ in 0..size {
                    let field_idx_diff = buffer.read_uleb128()?;
                    let access_flags = buffer.read_uleb128()?;

                    let field_id = field_ids[(field_idx_diff + previous_idx_diff) as usize].clone();

                    fields.push(DexField {
                        name: field_id.name_id,
                        field_type: field_id.type_id,
                        class: field_id.class_id,
                        access_flags: access_flags as u32,
                        access_type: access_type.clone(),
                    });

                    previous_idx_diff = field_idx_diff;
                }
                Ok(())
            };

            let mut read_methods = |buffer: &mut BytesBuffer,
                                    size: usize, access_type: DexMethodType|
             -> Result<(), Box<dyn Error>> {
                let mut previous_idx_diff = 0;
                for _ in 0..size {
                    let method_idx_diff = buffer.read_uleb128()?;
                    let access_flags = buffer.read_uleb128()?;
                    let code_off = buffer.read_uleb128()?;

                    let method_id =
                        method_ids[(method_idx_diff + previous_idx_diff) as usize].clone();

                    let previous_position = buffer.position;
                    buffer.position = code_off as usize;

                    let registers_size = buffer.read_u16()?;
                    let ins_size = buffer.read_u16()?;
                    let outs_size = buffer.read_u16()?;
                    let tries_size = buffer.read_u16()?;
                    let debug_info_off = buffer.read_u32()?;
                    let insns_size = buffer.read_u32()?;
                    let mut instructions_buffer = BytesBuffer::new(buffer.read((insns_size * 2) as usize)?.to_vec());
                    let mut instructions = vec![];

                    while instructions_buffer.position != instructions_buffer.data.len() {
                        instructions.push(decode_inst(&mut instructions_buffer)?);
                    }

                    buffer.position = previous_position;

                    methods.push(DexMethod {
                        name: method_id.name_id,
                        parameters: method_id.proto_id.parameter_ids,
                        return_type: method_id.proto_id.return_type_id,
                        class: method_id.class_id,
                        access_flags: access_flags as u32,
                        access_type: access_type.clone(),
                        instructions,
                    });

                    previous_idx_diff = method_idx_diff;
                }
                Ok(())
            };

            read_fields(&mut buffer, static_fields_size as usize, DexFieldType::Static)?;
            read_fields(&mut buffer, instance_fields_size as usize, DexFieldType::Instance)?;
            read_methods(&mut buffer, direct_methods_size as usize, DexMethodType::Direct)?;
            read_methods(&mut buffer, virtual_methods_size as usize, DexMethodType::Virtual)?;

            buffer.position = previous_position;
        }

        Ok(Self {
            version,
            endianness,
            strings,
            methods,
            fields,
            type_ids,
            proto_ids,
            field_ids,
            method_ids,
        })
    }
}
