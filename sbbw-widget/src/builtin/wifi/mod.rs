#[cfg(target_os = "linux")]
mod linux;

pub mod prelude {
    #[cfg(target_os = "linux")]
    pub use super::linux::*;
}
