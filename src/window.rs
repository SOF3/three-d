//!
//! Default windows for easy setup and event handling.
//! Currently [glutin](https://crates.io/crates/glutin/main.rs) for cross-platform desktop
//! and canvas using [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) for web, but
//! can be replaced by any other window with similar functionality.
//!

mod settings;
#[doc(inline)]
pub use settings::*;

#[cfg(all(feature = "glutin-window", not(target_arch = "wasm32")))]
mod glutin_window;
#[doc(inline)]
#[cfg(all(feature = "glutin-window", not(target_arch = "wasm32")))]
pub use glutin_window::*;

#[cfg(all(feature = "canvas", target_arch = "wasm32"))]
mod canvas;
#[doc(inline)]
#[cfg(all(feature = "canvas", target_arch = "wasm32"))]
pub use canvas::*;
