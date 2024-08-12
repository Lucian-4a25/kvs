#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use kv::KvStore;
pub use engine::KvsEngine;

mod engine;
mod error;
mod kv;
