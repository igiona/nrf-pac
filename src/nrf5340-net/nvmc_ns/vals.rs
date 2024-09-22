#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cacheen {
    #[doc = "Disable cache. Invalidates all cache entries."]
    DISABLED = 0x0,
    #[doc = "Enable cache"]
    ENABLED = 0x01,
}
impl Cacheen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cacheen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cacheen {
    #[inline(always)]
    fn from(val: u8) -> Cacheen {
        Cacheen::from_bits(val)
    }
}
impl From<Cacheen> for u8 {
    #[inline(always)]
    fn from(val: Cacheen) -> u8 {
        Cacheen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cacheprofen {
    #[doc = "Disable cache profiling"]
    DISABLED = 0x0,
    #[doc = "Enable cache profiling"]
    ENABLED = 0x01,
}
impl Cacheprofen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cacheprofen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cacheprofen {
    #[inline(always)]
    fn from(val: u8) -> Cacheprofen {
        Cacheprofen::from_bits(val)
    }
}
impl From<Cacheprofen> for u8 {
    #[inline(always)]
    fn from(val: Cacheprofen) -> u8 {
        Cacheprofen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Eraseall {
    #[doc = "No operation"]
    NOOPERATION = 0x0,
    #[doc = "Start chip erase"]
    ERASE = 0x01,
}
impl Eraseall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eraseall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eraseall {
    #[inline(always)]
    fn from(val: u8) -> Eraseall {
        Eraseall::from_bits(val)
    }
}
impl From<Eraseall> for u8 {
    #[inline(always)]
    fn from(val: Eraseall) -> u8 {
        Eraseall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ready {
    #[doc = "NVMC is busy (ongoing write or erase operation)"]
    BUSY = 0x0,
    #[doc = "NVMC is ready"]
    READY = 0x01,
}
impl Ready {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ready {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ready {
    #[inline(always)]
    fn from(val: u8) -> Ready {
        Ready::from_bits(val)
    }
}
impl From<Ready> for u8 {
    #[inline(always)]
    fn from(val: Ready) -> u8 {
        Ready::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Readynext {
    #[doc = "NVMC cannot accept any write operation"]
    BUSY = 0x0,
    #[doc = "NVMC is ready"]
    READY = 0x01,
}
impl Readynext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Readynext {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Readynext {
    #[inline(always)]
    fn from(val: u8) -> Readynext {
        Readynext::from_bits(val)
    }
}
impl From<Readynext> for u8 {
    #[inline(always)]
    fn from(val: Readynext) -> u8 {
        Readynext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wen {
    #[doc = "Read only access"]
    REN = 0x0,
    #[doc = "Write enabled"]
    WEN = 0x01,
    #[doc = "Erase enabled"]
    EEN = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Partial erase enabled"]
    PEEN = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Wen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wen {
    #[inline(always)]
    fn from(val: u8) -> Wen {
        Wen::from_bits(val)
    }
}
impl From<Wen> for u8 {
    #[inline(always)]
    fn from(val: Wen) -> u8 {
        Wen::to_bits(val)
    }
}
