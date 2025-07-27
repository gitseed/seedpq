fn main() {
    pkg_config::Config::new().probe("libpq").unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
