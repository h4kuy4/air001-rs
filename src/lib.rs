#![no_std]

extern crate air001_rs_macros as macros;

use core::arch::global_asm;

pub mod vectors;
pub mod register;
pub mod gpio;
pub mod panic;

pub use macros::entry;

global_asm!(include_str!("startup.S"));
