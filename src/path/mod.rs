pub mod error;

#[cfg(not(target_family = "wasm"))]
mod desktop;
#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(not(target_family = "wasm"))]
pub use desktop::*;
#[cfg(target_family = "wasm")]
pub use wasm::*;
