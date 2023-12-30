#![no_std]

extern crate air001_rs_macros as macros;

use core::arch::global_asm;

mod utils;

pub mod vectors;
pub mod gpio;
pub mod panic;
pub mod rcc;
pub mod timer;

pub use macros::entry;
pub use macros::exception;

global_asm!(include_str!("startup.S"));
