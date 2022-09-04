mod impls;

use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: u64,
    pub format: String,
    pub variables: Option<HashMap<String, String>>,
    pub send_telemetry: bool,
    pub show_user: bool,
    pub url: Option<String>,
    pub url_label: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChecksumAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Checksum {
    pub algorithm: ChecksumAlgorithm,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceReference {
    Path(String),
    Reference(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourcePresentationHint {
    Normal,
    Emphasize,
    Deemphasize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub name: Option<String>,
    pub source_reference: Option<SourceReference>,
    pub presentation_hint: Option<SourcePresentationHint>,
    pub origin: Option<String>,
    pub sources: Vec<Source>,
    pub adapter_data: Option<Value>,
    pub checksums: Vec<Checksum>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Breakpoint {
    pub id: Option<u64>,
    pub verified: bool,
    pub message: Option<String>,
    pub source: Option<Source>,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub end_line: Option<u64>,
    pub end_column: Option<u64>,
    pub instruction_reference: Option<String>,
    pub offset: Option<i64>,
}
