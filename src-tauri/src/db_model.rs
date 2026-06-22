use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbPackage {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author_id: Option<String>,
    pub author_name: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub active: bool,
    pub manifest_json: String,
    pub imported_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbEntry {
    pub id: String,
    pub package_id: String,
    pub entry_type: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub language: Option<String>,
    pub aliases_json: Option<String>,
    pub tags_json: Option<String>,
    pub data_json: String,
    pub source_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbRelation {
    pub id: String,
    pub package_id: String,
    pub from_entry_id: String,
    pub relation_type: String,
    pub to_entry_id: String,
    pub weight: f64,
    pub status: String,
    pub data_json: String,
}
