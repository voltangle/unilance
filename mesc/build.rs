use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=c_src");
    let target = std::env::var("TARGET").unwrap();

    let mut build = cc::Build::new();

    // Bindings to MESC
    let mut bindgen = bindgen::Builder::default().clang_arg("-I./c_src");

    let arm_gcc_toolchain_includes: Vec<String> = {
        let output = Command::new("bash")
            .arg("-c")
            .arg("arm-none-eabi-gcc -Wp,-v -E - < /dev/null 2>&1 | sed -n 's/^ //p'")
            .output()
            .expect("failed to run arm-none-eabi-gcc");

        if !output.status.success() {
            panic!(
                "arm-none-eabi-gcc failed: {:?}",
                String::from_utf8(output.stderr)
            );
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
        bindgen = bindgen.clang_arg(format!("-I{}", include));
    }

    for var in std::env::vars() {
        // assumes the fact that any features declared in the [features] section
        // of Cargo.toml are MESC defines in disguise
        if var.0.starts_with("CARGO_FEATURE_") {
            let mut name = var.0.clone();
            name.replace_range(0.."CARGO_FEATURE_".len(), "");

            build.define(&name, None);
            bindgen = bindgen.clang_arg(format!("-D{}", &name));
        }
    }

    build
        .define("LOGLENGTH", Some("10"))
        .compiler("arm-none-eabi-gcc")
        .include("c_src/")
        // MESC sources
        .include("c_src/MESC_Common/Inc")
        .file("c_src/MESC_Common/Src/MESCerror.c")
        .file("c_src/MESC_Common/Src/MESCfluxobs.c")
        .file("c_src/MESC_Common/Src/MESCfoc.c")
        .file("c_src/MESC_Common/Src/MESChfi.c")
        .file("c_src/MESC_Common/Src/MESClrobs.c")
        .file("c_src/MESC_Common/Src/MESCmeasure.c")
        .file("c_src/MESC_Common/Src/MESCmotor_state.c")
        .file("c_src/MESC_Common/Src/MESCposition.c")
        .file("c_src/MESC_Common/Src/MESCpwm.c")
        .file("c_src/MESC_Common/Src/MESCsin_lut.c")
        .file("c_src/MESC_Common/Src/MESCtemp.c")
        .flag("-Wno-unused-parameter")
        .compile("libmesc");

    // FIXME: make this adapt itself to the selected architecture, only applicable
    // to thumbv7em-none-eabihf
    let bindings = bindgen
        .header("c_src/mesc_wrap.h")
        .clang_arg("-I./c_src/")
        .clang_arg("-I./c_src/MESC_Common/Inc")
        .clang_arg("-DLOGLENGTH=10")
        .clang_arg(format!("--target={}", target))
        .clang_arg("-mthumb")
        .clang_arg("-mcpu=cortex-m4")
        .clang_arg("-mfpu=fpv4-sp-d16")
        .clang_arg("-mfloat-abi=hard")
        .clang_arg("-fshort-enums")
        .clang_arg("--gcc-toolchain=/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin/../arm-none-eabi")
        .derive_default(true)
        .use_core()
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
