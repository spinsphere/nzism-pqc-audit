pub mod csv;
pub mod text;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub entity_name: String,
    pub sector: String,
    pub sub_sector: String,
    pub domain: String,
    pub host: String,
    pub port: u16,
    pub notes: String,
}
