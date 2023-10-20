//! Processor core registers
//! 
//! The following registers can only be accessed in PRIVILEGED mode:
//! 
//! - SSP
//! - SR
//! 
//! The rest of registers can be accessed in either PRIVILEGED or UNPRIVILEGED
//! mode:
//! 
//! - CCR

pub mod ssp;

pub mod sr;

pub mod ccr;