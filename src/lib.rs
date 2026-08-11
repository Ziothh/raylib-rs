// #![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

pub mod ffi;

mod vectors;
pub use vectors::*;

mod rectange;
pub use rectange::*;

mod colors;
pub use colors::*;

mod texture;
pub use texture::*;

mod window;
pub use window::*;

mod keyboard;
pub use keyboard::*;

#[cfg(feature = "macros")]
#[macro_use]
pub mod macros;
