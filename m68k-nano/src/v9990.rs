use volatile_register::{RO, RW, WO};

const V9990_BASE_ADDRESS: u32 = 0x0080_0000;

pub struct V9990 {
    p: &'static mut RegisterBlock
}

#[repr(C)]
struct RegisterBlock {

    /// VRAM Data
    pub vd: RW<u8>,

    /// Palette Data
    pub pd: RW<u8>,

    /// Command Data
    pub cd: RW<u8>,

    /// Register Data
    pub rd: RW<u8>,

    /// Register Select
    pub rs: WO<u8>,

    /// Status Register
    pub s:  RO<u8>,

    /// Interrupt Flag Register
    pub i:  RW<u8>,

    /// System Control Register
    pub sc: WO<u8>,    

    // TODO: Kanji ROM Registers follow
}

impl V9990 {
    pub fn new() -> V9990 {
        V9990 {
            p: unsafe { &mut *(V9990_BASE_ADDRESS as *mut RegisterBlock) }
        }
    }
}

/// Status: Command being executed
pub const S_CE: u8 = 1 << 0;

/// Status: In the second field period during interlace
pub const S_EO: u8 = 1 << 1;

/// Status: Currently selected master clock source
pub const S_MCS: u8 = 1 << 2;

/// Status: Border color detect at completion of SRCH command (becomes "1" when detected)
pub const S_BD: u8 = 1 << 4;

/// Status: Horizontal non-display period
pub const S_HR: u8 = 1 << 5;

/// Status: Vertical non-display period
pub const S_VR: u8 = 1 << 6;

/// Status: Command data transfer ready. Reset through command data port
pub const S_TR: u8 = 1 << 7;


/// Vertical display period completion flag interrupt bit
pub const I_VI_FLAG: u8 = 1 << 0;

/// Display position flag interrupt bit
pub const I_HI_FLAG: u8 = 1 << 1;

/// Command completion flag interrupt bit
pub const I_CE_FLAG: u8 = 1 << 2;

/// Master clock select system control bit
pub const SC_MCS: u8 = 1 << 0;