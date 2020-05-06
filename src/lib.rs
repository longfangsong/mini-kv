#![deny(missing_docs)]
//! A simple key/value store.
#[macro_use]
extern crate log;

pub use server::KvServer;

/// client
pub mod client;
/// common
pub mod common;
/// Error type for this crate
pub mod error;
/// server
pub mod server;

/// result type
pub type Result<T> = std::result::Result<T, error::Error>;
