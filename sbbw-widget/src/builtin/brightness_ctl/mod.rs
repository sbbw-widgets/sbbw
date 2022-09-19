use serde::{Deserialize, Serialize};

#[cfg(not(target_os = "macos"))]
mod win_tux;

#[derive(Serialize, Deserialize)]
pub struct SbbwBrightnessDevice {
    pub name: String,
    pub value: u32,
}

pub mod prelude {
    #[cfg(not(target_os = "macos"))]
    pub use super::win_tux::*;
}
