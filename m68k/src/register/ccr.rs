//! Condition Code Register

use core::arch::asm;

/// Condition Code Register
#[derive(Clone, Copy, Debug)]
pub struct Ccr {
    bits: u8,
}

impl Ccr {
    /// Creates a `Ccr` value from raw bits.
    #[inline]
    pub fn from_bits(bits: u8) -> Self {
        Self { bits }
    }
    
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(self) -> u8 {
        self.bits
    }

    /// Read the Carry flag
    #[inline]
    pub fn c(self) -> bool {
        self.bits & (1 << 0) != 0
    }
    
    /// Read the Overflow flag
    #[inline]
    pub fn v(self) -> bool {
        self.bits & (1 << 1) != 0
    }

    /// Read the Zero flag
    #[inline]
    pub fn z(self) -> bool {
        self.bits & (1 << 2) != 0
    }

    /// Read the Negative flag
    #[inline]
    pub fn n(self) -> bool {
        self.bits & (1 << 3) != 0
    }

    /// Read the Extend flag
    #[inline]
    pub fn x(self) -> bool {
        self.bits & (1 << 4) != 0
    }
}
/*
/// Read the CCR register
#[inline]
pub fn read() -> Ccr {
    let r;
    unsafe { asm!("move.b %ccr,{}", out(reg) r, options(nomem, nostack, preserves_flags)) };
    Ccr::from_bits(r)
}

/// Set the value of the CCR register
#[inline]
pub unsafe fn write(ccr: Ccr) {
    let ccr = ccr.bits();
    asm!("move {},%ccr", in(reg) ccr, options(nomem, nostack));
}
*/