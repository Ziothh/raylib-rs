use crate::wrap::{raylib_api::RayLibApiDefinition, enums::wrap_exposed_enums, colors::wrap_default_colors};

mod bind;
mod wrap;

pub fn main() {
    // Files to watch that should trigger a rebuild
    println!("cargo:rerun-if-changed=src/wrapper.h");

    // Compile raylib
    bind::compile_raylib("third_party/raylib");

    // Link libraries
    bind::link_libs();

    // Generate bindings
    bind::generate_bindings("src/wrapper.h");

    // Load the API definitions
    let api_path = "third_party/raylib/tools/rlparser/output/raylib_api.json";
    println!("cargo:rerun-if-changed={api_path}");
    let api_defs = RayLibApiDefinition::load(api_path).unwrap();

    // Generate safe wrappers
    wrap_exposed_enums(api_defs.clone());
    wrap_default_colors(api_defs);
}
