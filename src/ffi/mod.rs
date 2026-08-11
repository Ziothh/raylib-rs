//! This module imports all of the generated C FFI raylib code

// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// This module contains auto-generated Rust representations of raylib's enums.
pub mod enums {
    include!(concat!(env!("OUT_DIR"), "/enums.rs"));
}

/// This module contains auto-generated Rust representations of raylib's colors.
pub mod colors {
    include!(concat!(env!("OUT_DIR"), "/colors.rs"));
}
