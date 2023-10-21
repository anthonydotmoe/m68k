#![no_main]
#![no_std]

use m68k_rt::entry;
use m68k::asm::{Trap, trap};
extern crate panic_abort;

struct RegisterBlock {
    pub reg: u8,
}

struct Device {
    pub p: &'static mut RegisterBlock,
}

impl Device {
    fn new() -> Device {
        Device {
            p: unsafe { &mut *(0x0080_0000 as *mut RegisterBlock) }
        }
    }
}

pub fn putc() {
    let mut uart = Device::new();
    
    uart.p.reg = 5;
}

#[entry]
unsafe fn main() -> ! {
    trap(Trap::Trap0);
    putc();
    loop {}
}