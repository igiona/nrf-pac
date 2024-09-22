#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mutex {
    #[doc = "Mutex n is in unlocked state"]
    UNLOCKED = 0x0,
    #[doc = "Mutex n is in locked state"]
    LOCKED = 0x01,
}
impl Mutex {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mutex {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mutex {
    #[inline(always)]
    fn from(val: u8) -> Mutex {
        Mutex::from_bits(val)
    }
}
impl From<Mutex> for u8 {
    #[inline(always)]
    fn from(val: Mutex) -> u8 {
        Mutex::to_bits(val)
    }
}
