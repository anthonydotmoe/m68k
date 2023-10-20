//! Status Register

use core::arch::asm;

use super::ccr::Ccr;

/// Status Register

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Sr {
    bits: u8,
    ccr: Ccr
}

impl Sr {
    /// Creates a `Sr` value from raw bits.
    #[inline]
    pub fn from_bits(bits: u16) -> Self {
        Self {
            bits: (bits >> 8) as u8,
            ccr:  Ccr::from_bits((bits & 0x00FF) as u8),
        }
    }

    /// Returns the Interrupt Mask value
    #[inline]
    pub fn i(self) -> u8 {
        self.bits & 0b00000111
    }

    /// Returns the Supervisor State flag
    #[inline]
    pub fn s(self) -> bool {
        self.bits & (1 << 5) != 0
    }

    /// Returns the Trace Mode flag
    #[inline]
    pub fn t(self) -> bool {
        self.bits & (1 << 7) != 0
    }
}

/// Read the Status Register
#[inline]
pub fn read() -> Sr {
    let r;
    unsafe { asm!("move.w %sr,{}", out(reg) r, options(nomem, nostack, preserves_flags)) };
    Sr::from_bits(r)
}