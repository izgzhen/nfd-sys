use std::env;
use std::process::Command;

fn main() {
    let mut p = env::current_dir().unwrap();

    p.push("nativefiledialog");
    p.push("src");

    Command::new("cmake").arg("CMakeLists.txt").current_dir(&p.as_path()).status().unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    Command::new("make").current_dir(&p.as_path()).status().unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    println!("cargo:rustc-link-search=native={}", p.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=nfd");
}
