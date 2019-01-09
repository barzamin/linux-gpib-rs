extern crate bindgen;

fn main() {
    println!("cargo:rustc-link-lib=gpib");
}
