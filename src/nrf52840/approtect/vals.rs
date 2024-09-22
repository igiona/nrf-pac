#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Disable(pub u8);
impl Disable {
    #[doc = "Software disable APPROTECT mechanism"]
    pub const SWDISABLE: Self = Self(0x5a);
}
impl Disable {
    pub const fn from_bits(val: u8) -> Disable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Disable {
    #[inline(always)]
    fn from(val: u8) -> Disable {
        Disable::from_bits(val)
    }
}
impl From<Disable> for u8 {
    #[inline(always)]
    fn from(val: Disable) -> u8 {
        Disable::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Forceprotect(pub u8);
impl Forceprotect {
    #[doc = "Software force enable APPROTECT mechanism"]
    pub const FORCE: Self = Self(0x0);
}
impl Forceprotect {
    pub const fn from_bits(val: u8) -> Forceprotect {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Forceprotect {
    #[inline(always)]
    fn from(val: u8) -> Forceprotect {
        Forceprotect::from_bits(val)
    }
}
impl From<Forceprotect> for u8 {
    #[inline(always)]
    fn from(val: Forceprotect) -> u8 {
        Forceprotect::to_bits(val)
    }
}
