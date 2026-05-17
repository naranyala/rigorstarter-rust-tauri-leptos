use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistryItem {
    pub name: String,
    pub id: String,
    pub category: String,
    pub status: String,
    pub line_count: usize,
}
