#![deny(missing_docs)]
//! A simple key/value store.

pub use kvstore::KvStore;

/// Error type for this crate
pub mod error;
/// real store
pub mod kvstore;

/// result type
pub type Result<T> = std::result::Result<T, error::Error>;
