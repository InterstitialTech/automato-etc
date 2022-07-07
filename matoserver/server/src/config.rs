use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub static_path: Option<PathBuf>,
    pub automato_ids: Vec<u8>,
}
