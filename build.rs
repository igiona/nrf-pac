/// This is auto-generated, do not edit!
/// Edit build.rs.in instead
///
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

macro_rules! convert_required_x_file {
    ($out:expr, $( ($feature:expr, $file_identifier:expr) ),* ) => {
        $(
            if std::env::var_os(concat!("CARGO_FEATURE_", $feature).to_uppercase()).is_some() {
                File::create($out.join("device.x"))
                    .unwrap()
                    .write_all(include_bytes!(concat!("device-", $file_identifier, ".x")))
                    .unwrap();
            }
        )*
    };
}

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        convert_required_x_file!(
            out,
            ("nrf52833", "nrf52833"),
            ("nrf52840", "nrf52840"),
            ("nrf5340-app-ns", "nrf5340-app"),
            ("nrf5340-app-s", "nrf5340-app"),
            ("nrf5340-net", "nrf5340-net")
        );
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=device.x");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
