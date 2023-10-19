//! Minimal `m68k-rt` based program

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate m68k_rt as rt;
extern crate panic_abort;

use rt::entry;

#[repr(C)]
struct MockPeripheral {
    pub reg: u32,
}

// the program entry point
#[entry]
fn main() -> ! {
    let periph = unsafe { &mut *(0xDEAD_BEEF as *mut MockPeripheral) };
    let i = 420;
    
    unsafe { core::ptr::write_volatile(&mut periph.reg, i)};
    loop {}
}