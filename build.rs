use std::env;
use std::path::PathBuf;

fn main() {
    let libpq = pkg_config::Config::new().probe("libpq").unwrap();
    println!("cargo::rerun-if-changed=build.rs");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(
            libpq
                .include_paths
                .iter()
                .map(|include_path| format!("-I{}", include_path.display())),
        )
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .generate()
        .unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap()
}
