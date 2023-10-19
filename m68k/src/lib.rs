//! Low level access to m68k processors
//! 
//! I don't know what I'm doing so I stole most of the code from `cortex-m`

#![no_std]

#![deny(clippy::missing_inline_in_public_items)]

#![feature(asm_experimental_arch)]

#[macro_use]
mod macros;

pub mod asm;

pub mod interrupt;

#[cfg(feature = "critical-section-single-core")]
mod critical_section;

/// Used to reexport items for use in macros. Do not use directly.
#[doc(hidden)]
pub mod _export {
    pub use critical_section;
}