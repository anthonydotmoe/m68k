//! Startup code and minimal runtime for m68k processors.

#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

extern crate m68k_rt_macros as macros;

use core::arch::asm;
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

/// Attribute to declare an exception handler
///
/// # Syntax
///
/// ```
/// # use m68k_rt::exception;
/// #[exception]
/// fn SysTick() {
///     // ..
/// }
///
/// # fn main() {}
/// ```
///
/// where the name of the function must be one of:
///
/// - `DefaultHandler`
/// - `NonMaskableInt`
/// - `HardFault`
/// - `MemoryManagement` (a)
/// - `BusFault` (a)
/// - `UsageFault` (a)
/// - `SecureFault` (b)
/// - `SVCall`
/// - `DebugMonitor` (a)
/// - `PendSV`
/// - `SysTick`
///
/// (a) Not available on Cortex-M0 variants (`thumbv6m-none-eabi`)
///
/// (b) Only available on ARMv8-M
///
/// # Usage
///
/// ## HardFault handler
///
/// `#[exception(trampoline = true)] unsafe fn HardFault(..` sets the hard fault handler.
/// If the trampoline parameter is set to true, the handler must have signature `unsafe fn(&ExceptionFrame) -> !`.
/// If set to false, the handler must have signature `unsafe fn() -> !`.
///
/// This handler is not allowed to return as that can cause undefined behavior.
///
/// To maintain backwards compatibility the attribute can be used without trampoline parameter (`#[exception]`),
/// which sets the trampoline to true.
///
/// ## Default handler
///
/// `#[exception] unsafe fn DefaultHandler(..` sets the *default* handler. All exceptions which have
/// not been assigned a handler will be serviced by this handler. This handler must have signature
/// `unsafe fn(irqn: i16) [-> !]`. `irqn` is the IRQ number (See CMSIS); `irqn` will be a negative
/// number when the handler is servicing a core exception; `irqn` will be a positive number when the
/// handler is servicing a device specific exception (interrupt).
///
/// ## Other handlers
///
/// `#[exception] fn Name(..` overrides the default handler for the exception with the given `Name`.
/// These handlers must have signature `[unsafe] fn() [-> !]`. When overriding these other exception
/// it's possible to add state to them by declaring `static mut` variables at the beginning of the
/// body of the function. These variables will be safe to access from the function body.
///
/// # Properties
///
/// Exception handlers can only be called by the hardware. Other parts of the program can't refer to
/// the exception handlers, much less invoke them as if they were functions.
///
/// `static mut` variables declared within an exception handler are safe to access and can be used
/// to preserve state across invocations of the handler. The compiler can't prove this is safe so
/// the attribute will help by making a transformation to the source code: for this reason a
/// variable like `static mut FOO: u32` will become `let FOO: &mut u32;`.
///
/// # Safety
///
/// It is not generally safe to register handlers for non-maskable interrupts. On Cortex-M,
/// `HardFault` is non-maskable (at least in general), and there is an explicitly non-maskable
/// interrupt `NonMaskableInt`.
///
/// The reason for that is that non-maskable interrupts will preempt any currently running function,
/// even if that function executes within a critical section. Thus, if it was safe to define NMI
/// handlers, critical sections wouldn't work safely anymore.
///
/// This also means that defining a `DefaultHandler` must be unsafe, as that will catch
/// `NonMaskableInt` and `HardFault` if no handlers for those are defined.
///
/// The safety requirements on those handlers is as follows: The handler must not access any data
/// that is protected via a critical section and shared with other interrupts that may be preempted
/// by the NMI while holding the critical section. As long as this requirement is fulfilled, it is
/// safe to handle NMIs.
///
/// # Examples
///
/// - Setting the default handler
///
/// ```
/// use m68k_rt::exception;
///
/// #[exception]
/// unsafe fn DefaultHandler(irqn: i16) {
///     println!("IRQn = {}", irqn);
/// }
///
/// # fn main() {}
/// ```
///
/// - Overriding the `SysTick` handler
///
/// ```
/// use m68k_rt::exception;
///
/// #[exception]
/// fn SysTick() {
///     static mut COUNT: i32 = 0;
///
///     // `COUNT` is safe to access and has type `&mut i32`
///     *COUNT += 1;
///
///     println!("{}", COUNT);
/// }
///
/// # fn main() {}
/// ```
// pub use macros::exception;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LowerExceptionFrame {
    sr: u16,
    pc: u32,
}

#[derive(Clone, Copy)]
pub struct AccessInformation {
    bits: u16,
}

impl fmt::Debug for AccessInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fc = (self.bits & 0b111) as u8;
        let ins = self.bits & (1 << 3) == 0;
        let rw = self.bits & (1 << 4) != 0;
        
        f.debug_struct("Access Information")
            .field("read", &rw)
            .field("instruction", &ins)
            .field("fc", &fc)
            .finish()
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExceptionFrame {
    
    /// Information about access: R/W, I/N, FC
    ai: AccessInformation,
    
    /// Access Address
    aa: u32,

    ir: u16,
    sr: u16,
    pc: u32,
}

impl fmt::Debug for ExceptionFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u32);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:08x}", self.0)
            }
        }
        f.debug_struct("ExceptionFrame")
            .field("Access Information", &self.ai)
            .field("Access Address", &Hex(self.aa))
            .field("Instruction Register", &Hex(self.ir as u32))
            // TODO: Impl Debug for Sr
            .field("Status Register", &Hex(self.sr as u32))
            .field("Program Counter", &Hex(self.pc))
            .finish()
    }
}

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