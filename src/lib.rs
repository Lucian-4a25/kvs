#![deny(missing_docs)]
//! A simple key/value store.

pub use engine::KvsEngine;
pub use error::{KvsError, Result};
pub use kv::KvStore;
pub use util::*;

mod engine;
mod error;
mod kv;
mod util;
