//! Startup code and minimal runtime for m68k processors.

#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

extern crate m68k_rt_macros as macros;

use core::arch::asm;
use core::arch::global_asm;

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


/// Attribute to declare the entry point of the program
///
/// The specified function will be called by the reset handler *after* RAM has been initialized.
///
/// The type of the specified function must be `[unsafe] fn() -> !` (never ending function)
///
/// # Properties
///
/// The entry point will be called by the reset handler. The program can't reference to the entry
/// point, much less invoke it.
///
/// `static mut` variables declared within the entry point are safe to access. The compiler can't
/// prove this is safe so the attribute will help by making a transformation to the source code: for
/// this reason a variable like `static mut FOO: u32` will become `let FOO: &'static mut u32;`. Note
/// that `&'static mut` references have move semantics.
///
/// # Examples
///
/// - Simple entry point
///
/// ``` no_run
/// # #![no_main]
/// # use m68k_rt::entry;
/// #[entry]
/// fn main() -> ! {
///     loop {
///         /* .. */
///     }
/// }
/// ```
///
/// - `static mut` variables local to the entry point are safe to modify.
///
/// ``` no_run
/// # #![no_main]
/// # use m68k_rt::entry;
/// #[entry]
/// fn main() -> ! {
///     static mut FOO: u32 = 0;
///
///     let foo: &'static mut u32 = FOO;
///     assert_eq!(*foo, 0);
///     *foo = 1;
///     assert_eq!(*foo, 1);
///
///     loop {
///         /* .. */
///     }
/// }
/// ```
pub use macros::entry;

/// Attribute to mark which function will be called at the beginning of the reset handler.
///
/// **IMPORTANT**: This attribute can appear at most *once* in the dependency graph.
///
/// The function must have the signature of `unsafe fn()`.
///
/// # Safety
///
/// The function will be called before memory is initialized, as soon as possible after reset. Any
/// access of memory, including any static variables, will result in undefined behavior.
///
/// **Warning**: Due to [rvalue static promotion][rfc1414] static variables may be accessed whenever
/// taking a reference to a constant. This means that even trivial expressions such as `&1` in the
/// `#[pre_init]` function *or any code called by it* will cause **immediate undefined behavior**.
///
/// Users are advised to only use the `#[pre_init]` feature when absolutely necessary as these
/// constraints make safe usage difficult.
///
/// # Examples
///
/// ```
/// # use m68k_rt::pre_init;
/// #[pre_init]
/// unsafe fn before_main() {
///     // do something here
/// }
///
/// # fn main() {}
/// ```
///
/// [rfc1414]: https://github.com/rust-lang/rfcs/blob/master/text/1414-rvalue_static_promotion.md
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