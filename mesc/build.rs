use std::env;
use std::path::PathBuf;

// TODO: Add some feature flags for USE_* defines in the hardware config and use that

fn main() {
    println!("cargo:rerun-if-changed=c_src");

    let target_port = std::env::var("TARGET_PORT")
        .expect("TARGET_PORT is not defined. Are you using task build-<target-name>?");
    let target_name = std::env::var("TARGET_NAME")
        .expect("TARGET_NAME is not defined. Are you using task build-<target-name>?");

    // FIXME: all these fixed paths to arm-none-eabi are stupid
    cc::Build::new()
        .include("/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin/../lib/gcc/arm-none-eabi/14.2.1/include")
        .include("/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin/../lib/gcc/arm-none-eabi/14.2.1/include-fixed")
        .include("/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin/../lib/gcc/arm-none-eabi/14.2.1/../../../../arm-none-eabi/include")
        .include("c_src/")
        .include("c_src/hardware_conf")
        .include(format!("c_src/hardware_conf/{}_{}", target_port, target_name))
        // MESC sources
        .include("c_src/MESC_Common/Inc")
        .file("c_src/MESC_Common/Src/MESChfi.c")
        .file("c_src/MESC_Common/Src/MESCpwm.c")
        .file("c_src/MESC_Common/Src/MESCmotor_state.c")
        .file("c_src/MESC_Common/Src/MESCfoc.c")
        .file("c_src/MESC_Common/Src/MESCmotor.c")
        .file("c_src/MESC_Common/Src/MESCposition.c")
        .file("c_src/MESC_Common/Src/MESCsin_lut.c")
        .file("c_src/MESC_Common/Src/MESClrobs.c")
        .file("c_src/MESC_Common/Src/MESCfluxobs.c")
        .file("c_src/MESC_Common/Src/MESCerror.c")
        .file("c_src/MESC_Common/Src/MESCtemp.c")
        .file("c_src/MESC_Common/Src/MESCmeasure.c")
        .flag("-Wno-unused-parameter")
        .compile("MESC");

    // Bindings to MESC
    let bindings = bindgen::Builder::default()
        .clang_arg("-I./c_src")
        .clang_arg("-I/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin/../lib/gcc/arm-none-eabi/14.2.1/include")
        .clang_arg("-I/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin/../lib/gcc/arm-none-eabi/14.2.1/include-fixed")
        .clang_arg("-I/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin/../lib/gcc/arm-none-eabi/14.2.1/../../../../arm-none-eabi/include")
        .clang_arg("-I./c_src/")
        .clang_arg("-I./c_src/MESC_Common/Inc")
        .clang_arg(format!("-I./c_src/hardware_conf/{}_{}", target_port, target_name))
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
