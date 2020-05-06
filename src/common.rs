use serde::{Deserialize, Serialize};

/// Operation request to server
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    /// Get the value for key
    Get {
        /// the key to get
        key: String,
    },
    /// Insert the key-value pair
    Set {
        /// the key to set
        key: String,
        /// the value to set
        value: String,
    },
    /// remove the key
    Remove {
        /// the key to remove
        key: String,
    },
}

/// Get result from server
#[derive(Debug, Serialize, Deserialize)]
pub enum GetResponse {
    /// successful result
    Ok(Option<String>),
    /// failed result
    Err(String),
}

/// Set result from server
#[derive(Debug, Serialize, Deserialize)]
pub enum SetResponse {
    /// successful result
    Ok(()),
    /// failed result
    Err(String),
}

/// Remove result from server
#[derive(Debug, Serialize, Deserialize)]
pub enum RemoveResponse {
    /// successful result
    Ok(()),
    /// failed result
    Err(String),
}
