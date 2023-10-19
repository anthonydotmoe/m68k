// Miscellaneous assembly instructions

use core::arch::asm;

// NOTE: This is a `pure` asm block, but applying that option allows the compiler to eliminate
// the nop entirely (or to collapse multiple subsequent ones). Since the user probably wants N
// nops when they call `nop` N times, let's not add that option.
#[inline(always)]
pub fn nop() {
    unsafe {
        asm!("nop", options(nomem, nostack, preserves_flags))
    };
}

/// Generate an illegal instruction exception
#[inline(always)]
pub fn illegal() -> ! {
    unsafe {
        asm!("illegal", options(noreturn, nomem, nostack, preserves_flags))
    }
}