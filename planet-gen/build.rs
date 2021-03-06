extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_autogen_warning("/**\n * Autogenerated file, please don't edit\n */")
      .with_include_guard("PLANET_GEN_H")
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("../target/debug/include/planet_gen/planet_gen.h");
}