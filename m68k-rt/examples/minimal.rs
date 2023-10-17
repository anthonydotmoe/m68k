//! Minimal `m68k-rt` based program

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate m68k_rt as rt;
extern crate panic_halt;

use rt::entry;

// the program entry point
#[entry]
fn main() -> ! {
    loop {}
}