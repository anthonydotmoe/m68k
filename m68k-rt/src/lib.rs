//! Startup code and minimal runtime for m68k processors.

#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

extern crate m68k_rt_macros as macros;

use core::arch::global_asm;
use core::fmt;

/// Parse cfg attributes inside a global_asm call.
macro_rules! cfg_global_asm {
    {@inner, [$($x:tt)*], } => {
        global_asm!{$($x)*}
    };
    (@inner, [$($x:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
        #[cfg(not($meta))]
        cfg_global_asm!{@inner, [$($x)*], $($rest)*}
    };
    {@inner, [$($x:tt)*], $asm:literal, $($rest:tt)*} => {
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
    };
    {$($asms:tt)*} => {
        cfg_global_asm!{@inner, [], $($asms)*}
    };
}

// This reset vector is the initial entry point after a system reset.
// Calls an optional user-provided __pre_init and then initialized RAM.
// If the target has an FPU, it is enabled.
// Finally jumps to the user main function.

cfg_global_asm! {
    /*
    "section Reset,code",
    "public Reset",
    "Reset:",
    */
   ".section .Reset, \"ax\"
    .global Reset
    .type Reset,%function
    Reset:",

    // Run user pre-init code which must be executed immediately after startup,
    // before the potentially time-consuming memory initiliazation takes place.
    "   bra     __pre_init",

    // If enabled, initialize RAM with zeros.
    #[cfg(feature = "zero-init-ram")]
    "   ; TODO: Init ram routine with _ram_start and _ram_end",
    
    // Initialize .bss memory. `__sbss` and `__ebss` come from the linker script.
    // Note: sbss = Pointer to the start of .bss, ebss = Pointer to the
    // inclusive end of the .bss section.
    #[cfg(not(feature = "zero-init-ram"))]
    "   ; TODO: Copy .bss section to some spot",

    // Jump to user main function. 
    "   jsr main
        illegal",
}

pub use macros::entry;

pub use macros::pre_init;

pub enum Exception {
    BusError,
    AddressError,
    IllegalInstruction,
    ZeroDivide,
    CHKInstruction,
    TRAPVInstruction,
    PrivilegeViolation,
    Trace,
    Line1010Emulator,
    Line1111Emulator,
    FormatError,
}

pub use self::Exception as exception;

extern "C" {
    fn Reset() -> !;

    fn BusError();

    fn AddressError();

    fn IllegalInstruction();

    fn ZeroDivide();

    fn CHKInstruction();

    fn TRAPVInstruction();

    fn PrivilegeViolation();

    fn Trace();

    fn Line1010Emulator();

    fn Line1111Emulator();
    
    fn FormatError();
}

pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Vector; 14] = [
    // Exception 2: Bus Error
    Vector {
        handler: BusError,
    },
    // Exception 3: Address Error
    Vector {
        handler: AddressError,
    },
    // Exception 4: 
    Vector {
        handler: IllegalInstruction,
    },
    // Exception 5: 
    Vector {
        handler: ZeroDivide,
    },
    // Exception 6: 
    Vector {
        handler: CHKInstruction,
    },
    // Exception 7: 
    Vector {
        handler: TRAPVInstruction,
    },
    // Exception 8: 
    Vector {
        handler: PrivilegeViolation,
    },
    // Exception 9: 
    Vector {
        handler: Trace,
    },
    // Exception 10: 
    Vector {
        handler: Line1010Emulator,
    },
    // Exception 11: 
    Vector {
        handler: Line1111Emulator,
    },
    // Exception 12: Reserved
    Vector { reserved: 0 },
    // Exception 13: Reserved
    Vector { reserved: 0 },
    // Exception 14: Format Error
    Vector {
        handler: FormatError,
    },
    // Exception 15: Uninitialized Interrupt Vector
    Vector { reserved: 0 },
];