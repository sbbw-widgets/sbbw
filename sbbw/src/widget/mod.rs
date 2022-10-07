use std::{
    collections::HashMap,
    process::Child,
    sync::{Arc, Mutex},
};

mod internal;
pub mod rpc;

lazy_static::lazy_static! {
    pub static ref WIDGETS: Arc<Mutex<HashMap<String, Child>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub mod prelude {
    pub use super::internal::*;
}
