// This is auto-generated, do not edit!
// Edit lib.rs.in instead

#![no_std]
#![allow(non_camel_case_types)]
#![doc = include_str!("../README.md")]

#[cfg(all(
    feature = "nrf52833",
    feature = "nrf52840",
    feature = "nrf5340-app-ns",
    feature = "nrf5340-app-s",
    feature = "nrf5340-net"
))]
compile_error!("You must not enable multiple Cargo features: `nrf52833`, `nrf52840`, `nrf5340-app-ns`, `nrf5340-app-s`, `nrf5340-net`.");

#[cfg(not(any(
    feature = "nrf52833",
    feature = "nrf52840",
    feature = "nrf5340-app-ns",
    feature = "nrf5340-app-s",
    feature = "nrf5340-net"
)))]
compile_error!("You must enable at least one Cargo feature: `nrf52833`, `nrf52840`, `nrf5340-app-ns`, `nrf5340-app-s`, `nrf5340-net`.");

#[cfg_attr(feature = "nrf52833", path = "./nrf52833/mod.rs")]
#[cfg_attr(feature = "nrf52840", path = "./nrf52840/mod.rs")]
#[cfg_attr(feature = "nrf5340-app-ns", path = "./nrf5340-app-ns/mod.rs")]
#[cfg_attr(feature = "nrf5340-app-s", path = "./nrf5340-app-s/mod.rs")]
#[cfg_attr(feature = "nrf5340-net", path = "./nrf5340-net/mod.rs")]
mod inner;
pub use inner::*;
