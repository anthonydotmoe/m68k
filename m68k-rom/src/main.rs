#![no_main]
#![no_std]

use m68k_rt::entry;
extern crate panic_abort;

#[entry]
unsafe fn main() -> ! {
    loop {}
}