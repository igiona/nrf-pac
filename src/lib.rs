// This is auto-generated, do not edit!
// Edit lib.rs.in instead

#![no_std]
#![allow(non_camel_case_types)]
#![doc = include_str!("../README.md")]

// Manually commented out, since for now we have only one chip
// #[cfg(all(feature = "nrf52840"))]
// compile_error!("You must not enable multiple Cargo features: `nrf52840`.");

// Manually commented out, since for now we have only one chip
// #[cfg(not(any(feature = "nrf52840")))]
// compile_error!("You must enable at least one Cargo feature: `nrf52840`.");

#[cfg_attr(feature = "nrf52840", path = "./nrf52840/mod.rs")]
mod inner;
pub use inner::*;
