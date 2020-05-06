use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum LogEntry {
    Set(String, String),
    Remove(String),
}
