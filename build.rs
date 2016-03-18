use std::env;

fn main() {
    // Example: NFD_DIR=~/nativefiledialog/src cargo build
    let nfd_dir = env::var("NFD_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}", nfd_dir);
    println!("cargo:rustc-link-lib=static=nfd");
}
