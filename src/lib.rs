#![deny(missing_docs)]
//! A simple key/value store.

pub use engine::KvsEngine;
pub use error::{KvsError, Result};
pub use kv::KvStore;
pub use logger::{init_logger, LOGGER};
pub use util::*;

mod engine;
mod error;
mod kv;
mod logger;
mod util;
