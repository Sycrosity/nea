use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Only re-run the build script when build.rs is changed - aka never
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(not(any(feature = "esp32", feature = "esp32c3")))]
    compile_error!("This crate only supports ESP32 and ESP32-C3 targets.");

    //Required to obtain backtraces (e.g. when using the "esp-backtrace" crate.) on
    // riscv32imc targets. NOTE: May negatively impact performance of produced
    // code
    #[cfg(target_arch = "riscv32")]
    println!("cargo:force-frame-pointers");

    // esp-hal requires this
    println!("cargo::rustc-link-arg=-Tlinkall.x");

    Ok(())
}
