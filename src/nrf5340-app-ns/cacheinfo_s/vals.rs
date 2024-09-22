#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mru {
    #[doc = "Way0 was most recently used"]
    WAY0 = 0x0,
    #[doc = "Way1 was most recently used"]
    WAY1 = 0x01,
}
impl Mru {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mru {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mru {
    #[inline(always)]
    fn from(val: u8) -> Mru {
        Mru::from_bits(val)
    }
}
impl From<Mru> for u8 {
    #[inline(always)]
    fn from(val: Mru) -> u8 {
        Mru::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum V {
    #[doc = "Invalid cache line"]
    INVALID = 0x0,
    #[doc = "Valid cache line"]
    VALID = 0x01,
}
impl V {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> V {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for V {
    #[inline(always)]
    fn from(val: u8) -> V {
        V::from_bits(val)
    }
}
impl From<V> for u8 {
    #[inline(always)]
    fn from(val: V) -> u8 {
        V::to_bits(val)
    }
}
