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
pub enum Trap {
    Trap0,
    Trap1,
    Trap2,
    Trap3,
    Trap4,
    Trap5,
    Trap6,
    Trap7,
    Trap8,
    Trap9,
    Trap10,
    Trap11,
    Trap12,
    Trap13,
    Trap14,
    Trap15,
}

#[inline]
pub fn trap(t: Trap) {
    unsafe {
        match t {
            Trap::Trap0 => asm!("trap #0"),
            Trap::Trap1 => asm!("trap #1"),
            Trap::Trap2 => asm!("trap #2"),
            Trap::Trap3 => asm!("trap #3"),
            Trap::Trap4 => asm!("trap #4"),
            Trap::Trap5 => asm!("trap #5"),
            Trap::Trap6 => asm!("trap #6"),
            Trap::Trap7 => asm!("trap #7"),
            Trap::Trap8 => asm!("trap #8"),
            Trap::Trap9 => asm!("trap #9"),
            Trap::Trap10 => asm!("trap #10"),
            Trap::Trap11 => asm!("trap #11"),
            Trap::Trap12 => asm!("trap #12"),
            Trap::Trap13 => asm!("trap #13"),
            Trap::Trap14 => asm!("trap #14"),
            Trap::Trap15 => asm!("trap #15"),
        }
    }
}