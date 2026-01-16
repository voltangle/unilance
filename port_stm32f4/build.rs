use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    let target_name = std::env::var("TARGET_NAME")
        .expect("TARGET_NAME is not defined. Are you using task build-<target-name>?");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dst = out_dir.join("memory.x");
    fs::copy(format!("ld/{}.x", target_name), &dst).unwrap();

    println!("cargo:rustc-link-search={}", out_dir.display());
}
