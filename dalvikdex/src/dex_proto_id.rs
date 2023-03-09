#[derive(Debug, Clone)]
pub struct DexProtoId {
    pub shorty_id: String,
    pub return_type_id: String,
    pub parameter_ids: Vec<String>,
}
