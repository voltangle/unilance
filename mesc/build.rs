#![feature(string_remove_matches)]

use std::env;
use std::path::PathBuf;
use std::process::Command;

// TODO: Add some feature flags for USE_* defines in the hardware config and use that

fn main() {
    println!("cargo:rerun-if-changed=c_src");

    let target_port = std::env::var("TARGET_PORT")
        .expect("TARGET_PORT is not defined. Are you using task build-<target-name>?");
    let target_name = std::env::var("TARGET_NAME")
        .expect("TARGET_NAME is not defined. Are you using task build-<target-name>?");

    let mut build = cc::Build::new();

    let arm_gcc_toolchain_includes: Vec<String> = {
        let output = Command::new("bash")
            .arg("-c")
            .arg("arm-none-eabi-gcc -Wp,-v -E - < /dev/null 2>&1 | sed -n 's/^ //p'")
            .output()
            .expect("failed to run bash command");

        if !output.status.success() {
            panic!("arm-none-eabi-gcc failed");
        }

        let stdout = String::from_utf8(output.stdout)
            .expect("arm-none-eabi-gcc output was not valid UTF-8");

        stdout
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect()
    };

    for include in &arm_gcc_toolchain_includes {
        build.include(include);
    }

    for var in std::env::vars() {
        // assumes the fact that any features declared in the [features] section
        // of Cargo.toml are MESC defines in disguise
        if var.0.starts_with("CARGO_FEATURE_") {
            let mut name = var.0.clone();
            name.remove_matches("CARGO_FEATURE_");

            build.define(&name, None);
        }
    }

    build
        .include("c_src/")
        .include("c_src/hardware_conf")
        .include(format!(
            "c_src/hardware_conf/{}_{}",
            target_port, target_name
        ))
        .define("LOGLENGTH", Some("10"))
        // MESC sources
        .include("c_src/MESC_Common/Inc")
        .file("c_src/MESC_Common/Src/MESCerror.c")
        .file("c_src/MESC_Common/Src/MESCfluxobs.c")
        .file("c_src/MESC_Common/Src/MESCfoc.c")
        .file("c_src/MESC_Common/Src/MESChfi.c")
        .file("c_src/MESC_Common/Src/MESClrobs.c")
        .file("c_src/MESC_Common/Src/MESCmeasure.c")
        .file("c_src/MESC_Common/Src/MESCmotor.c")
        .file("c_src/MESC_Common/Src/MESCmotor_state.c")
        .file("c_src/MESC_Common/Src/MESCposition.c")
        .file("c_src/MESC_Common/Src/MESCpwm.c")
        .file("c_src/MESC_Common/Src/MESCsin_lut.c")
        .file("c_src/MESC_Common/Src/MESCtemp.c")
        .flag("-Wno-unused-parameter")
        .compile("MESC");

    // Bindings to MESC
    let mut bindgen = bindgen::Builder::default().clang_arg("-I./c_src");

    for include in &arm_gcc_toolchain_includes {
        bindgen = bindgen.clang_arg(format!("-I{}", include));
    }
    let bindings = bindgen
        .clang_arg("-I./c_src/")
        .clang_arg("-I./c_src/MESC_Common/Inc")
        .clang_arg(format!(
            "-I./c_src/hardware_conf/{}_{}",
            target_port, target_name
        ))
        .clang_arg("-I./c_src/hardware_conf")
        .header("c_src/mesc_wrap.h")
        .use_core()
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    return;
}
