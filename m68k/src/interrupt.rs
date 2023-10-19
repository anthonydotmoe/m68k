use core::arch::asm;
use core::sync::atomic::{compiler_fence, Ordering};

/// Trait for enums of external interrupt numbers.
/// 
/// This trait should be implemented by a peripheral access crate
/// on its enum of available external interrupts for a specific device.
/// Each variant must convert to a u16 of its interrupt number, which
/// is exception number - 16.
/// 
/// *The above is probably not true for m68k*
/// 
/// # Safety
/// 
/// This trait must only be implemented on enums of device interrupts. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
/// 
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait InterruptNumber: Copy {
    /// Return the interrupt number associated with this variant.
    /// 
    /// See trait documentation for safety requirements.
    fn number(self) -> u16;
}

/// Disables all interrupts
#[inline]
pub fn disable() {
    unsafe {
        asm!("ori.w #0x0700,sr", options(nomem, nostack, preserves_flags));
    }
    
    // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
    compiler_fence(Ordering::SeqCst);
}

/// Set the interrupt mask
/// 
/// # Safety
/// 
/// - Do not call this function inside a critical section.
#[inline]
pub unsafe fn set(mask: u8) {
    let mask_shifted: u16 = (mask as u16) << 8;
    asm!("and.i #$F8FF,sr");
    asm!("ori.w {},sr", in(reg) mask_shifted);
}

/// Get the interrupt mask
pub unsafe fn get() -> u8 {
    let sr: u16;
    asm!("move.w sr,{}", out(reg) sr);
    ((sr >> 8) & 0x07) as u8
}