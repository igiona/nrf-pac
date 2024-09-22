#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApprotectPall(pub u32);
impl ApprotectPall {
    #[doc = "Protected"]
    pub const PROTECTED: Self = Self(0x0);
    #[doc = "Unprotected"]
    pub const UNPROTECTED: Self = Self(0x50fa_50fa);
}
impl ApprotectPall {
    pub const fn from_bits(val: u32) -> ApprotectPall {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for ApprotectPall {
    #[inline(always)]
    fn from(val: u32) -> ApprotectPall {
        ApprotectPall::from_bits(val)
    }
}
impl From<ApprotectPall> for u32 {
    #[inline(always)]
    fn from(val: ApprotectPall) -> u32 {
        ApprotectPall::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EraseprotectPall(pub u32);
impl EraseprotectPall {
    #[doc = "Protected"]
    pub const PROTECTED: Self = Self(0x0);
    #[doc = "Unprotected"]
    pub const UNPROTECTED: Self = Self(0xffff_ffff);
}
impl EraseprotectPall {
    pub const fn from_bits(val: u32) -> EraseprotectPall {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for EraseprotectPall {
    #[inline(always)]
    fn from(val: u32) -> EraseprotectPall {
        EraseprotectPall::from_bits(val)
    }
}
impl From<EraseprotectPall> for u32 {
    #[inline(always)]
    fn from(val: EraseprotectPall) -> u32 {
        EraseprotectPall::to_bits(val)
    }
}
