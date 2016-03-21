use std::env;
use std::process::Command;


fn main() {
    let mut p = env::current_dir().unwrap();

    // Example: NFD_DIR=/PATH/TO/ cargo build
    p.push("nativefiledialog");
    p.push("src");

    Command::new("git").args(&["clone", "https://github.com/mlabbe/nativefiledialog"]).status().unwrap();
    Command::new("scons").current_dir(&p.as_path()).status().unwrap();


    println!("cargo:rustc-link-search=native={}", p.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=nfd");
}
