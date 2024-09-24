use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Only re-run the build script when build.rs is changed - aka never
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(not(any(feature = "esp32", feature = "esp32c3")))]
    compile_error!("This crate only supports ESP32 and ESP32-C3 targets.");

    // esp-hal requires this
    println!("cargo::rustc-link-arg=-Tlinkall.x");

    // esp-wifi requires this
    println!("cargo::rustc-link-arg=-Trom_functions.x");

    Ok(())
}
