use built;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    built::write_built_file().expect("Failed to acquire build-time information");

    let target_name = match std::env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_TARGET_"))
        .get_one()
    {
        Ok(x) => x,
        Err(GetOneError::None) => panic!("No BSP feature defined"),
        Err(GetOneError::Multiple) => panic!("Multiple BSP features enabled"),
    }
    .trim_start_matches("CARGO_FEATURE_TARGET_")
    .to_lowercase();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dst = out_dir.join("memory.x");
    fs::copy(format!("ld/{}.x", target_name), &dst).unwrap();

    println!("cargo:rustc-link-search={}", out_dir.display());
}

// Source: embassy-stm32 builds script

enum GetOneError {
    None,
    Multiple,
}

trait IteratorExt: Iterator {
    fn get_one(self) -> Result<Self::Item, GetOneError>;
}

impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self) -> Result<Self::Item, GetOneError> {
        match self.next() {
            None => Err(GetOneError::None),
            Some(res) => match self.next() {
                Some(_) => Err(GetOneError::Multiple),
                None => Ok(res),
            },
        }
    }
}
