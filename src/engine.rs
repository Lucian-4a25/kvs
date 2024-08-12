use crate::Result;

///
/// kvs engine definition
pub trait KvsEngine {
    /// set key
     fn set(&mut self, key: String, value: String) -> Result<()>;

    /// get key
     fn get(&mut self, key: String) -> Result<Option<String>>;

    /// remove key
     fn remove(&mut self, key: String) -> Result<()>;
}