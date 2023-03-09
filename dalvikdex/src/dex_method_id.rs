use crate::dex_proto_id::DexProtoId;

#[derive(Debug, Clone)]
pub struct DexMethodId {
    pub class_id: String,
    pub proto_id: DexProtoId,
    pub name_id: String,
}
