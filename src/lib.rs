// This is auto-generated, do not edit!
// Edit lib.rs.in instead

#![no_std]
#![allow(non_camel_case_types)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "nrf52833", feature = "nrf52840"))]
compile_error!("You must not enable multiple Cargo features: `nrf52833`, `nrf52840`.");

#[cfg(not(any(feature = "nrf52833", feature = "nrf52840")))]
compile_error!("You must enable at least one Cargo feature: `nrf52833`, `nrf52840`.");

#[cfg_attr(feature = "nrf52833", path = "./nrf52833/mod.rs")]
#[cfg_attr(feature = "nrf52840", path = "./nrf52840/mod.rs")]
mod inner;
pub use inner::*;
