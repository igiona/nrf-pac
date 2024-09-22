#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HostCryptokeySel {
    #[doc = "Use device root key K_DR from CRYPTOCELL AO power domain"]
    K_DR = 0x0,
    #[doc = "Use hard-coded RTL key K_PRTL"]
    K_PRTL = 0x01,
    #[doc = "Use provided session key"]
    SESSION = 0x02,
    _RESERVED_3 = 0x03,
}
impl HostCryptokeySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HostCryptokeySel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HostCryptokeySel {
    #[inline(always)]
    fn from(val: u8) -> HostCryptokeySel {
        HostCryptokeySel::from_bits(val)
    }
}
impl From<HostCryptokeySel> for u8 {
    #[inline(always)]
    fn from(val: HostCryptokeySel) -> u8 {
        HostCryptokeySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HostIotKprtlLock {
    #[doc = "K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    DISABLED = 0x0,
    #[doc = "K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    ENABLED = 0x01,
}
impl HostIotKprtlLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HostIotKprtlLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HostIotKprtlLock {
    #[inline(always)]
    fn from(val: u8) -> HostIotKprtlLock {
        HostIotKprtlLock::from_bits(val)
    }
}
impl From<HostIotKprtlLock> for u8 {
    #[inline(always)]
    fn from(val: HostIotKprtlLock) -> u8 {
        HostIotKprtlLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lcs {
    #[doc = "CC310 operates in debug mode"]
    DEBUG = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "CC310 operates in secure mode"]
    SECURE = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcs {
    #[inline(always)]
    fn from(val: u8) -> Lcs {
        Lcs::from_bits(val)
    }
}
impl From<Lcs> for u8 {
    #[inline(always)]
    fn from(val: Lcs) -> u8 {
        Lcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LcsIsValid {
    #[doc = "Valid LCS not yet retained in the CRYPTOCELL AO power domain"]
    INVALID = 0x0,
    #[doc = "Valid LCS successfully retained in the CRYPTOCELL AO power domain"]
    VALID = 0x01,
}
impl LcsIsValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LcsIsValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LcsIsValid {
    #[inline(always)]
    fn from(val: u8) -> LcsIsValid {
        LcsIsValid::from_bits(val)
    }
}
impl From<LcsIsValid> for u8 {
    #[inline(always)]
    fn from(val: LcsIsValid) -> u8 {
        LcsIsValid::to_bits(val)
    }
}
