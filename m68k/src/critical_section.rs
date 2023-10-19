use critical_section::{set_impl, Impl, RawRestoreState};

use crate::interrupt;

struct M68kCriticalSection;
set_impl!(M68kCriticalSection);

unsafe impl Impl for M68kCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let current_mask = interrupt::get();
        interrupt::disable();
        current_mask
    }
    
    unsafe fn release(mask: RawRestoreState) {
        interrupt::set(mask);
    }
}