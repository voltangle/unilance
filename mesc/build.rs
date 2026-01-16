use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=c_src");

    let target_name = std::env::var("TARGET_NAME")
        .expect("TARGET_NAME is not defined. Are you using task build-<target-name>?");

    // FIXME: all these fixed paths to arm-none-eabi are stupid
    cc::Build::new()
        .define("STM32F405xx", None)
        .include("/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin/../lib/gcc/arm-none-eabi/14.2.1/include")
        .include("/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin/../lib/gcc/arm-none-eabi/14.2.1/include-fixed")
        .include("/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin/../lib/gcc/arm-none-eabi/14.2.1/../../../../arm-none-eabi/include")
        .include("c_src/")
        .include("c_src/STM32CubeF4/Drivers/CMSIS/Device/ST/STM32F4xx/Include")
        .include("c_src/STM32CubeF4/Drivers/CMSIS/Include")
        .include(format!("c_src/hardware_conf/{}", target_name))
        // MESC sources
        .include("c_src/MESC_Common/Inc")
        .file("c_src/MESC_Common/Src/MESChfi.c")
        .file("c_src/MESC_Common/Src/MESCpwm.c")
        .file("c_src/MESC_Common/Src/MESCmotor_state.c")
        .file("c_src/MESC_Common/Src/MESCfoc.c")
        .file("c_src/MESC_Common/Src/MESCmotor.c")
        .file("c_src/MESC_Common/Src/MESCBLDC.c")
        .file("c_src/MESC_Common/Src/MESCposition.c")
        .file("c_src/MESC_Common/Src/MESCsin_lut.c")
        .file("c_src/MESC_Common/Src/MESClrobs.c")
        .file("c_src/MESC_Common/Src/MESCfluxobs.c")
        .file("c_src/MESC_Common/Src/MESCerror.c")
        .file("c_src/MESC_Common/Src/MESCtemp.c")
        .file("c_src/MESC_Common/Src/MESCmeasure.c")
        .file("c_src/MESC_Common/Src/MESCinput.c")
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
        .clang_arg(format!("-I./c_src/hardware_conf/{}", target_name))
        .clang_arg("-I./c_src/STM32CubeF4/Drivers/CMSIS/Device/ST/STM32F4xx/Include")
        .clang_arg("-I./c_src/STM32CubeF4/Drivers/CMSIS/Include")
        .clang_arg("-DSTM32F405xx")
        .header("c_src/hal_types.h")
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
