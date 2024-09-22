#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConfigWen {
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
impl ConfigWen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfigWen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfigWen {
    #[inline(always)]
    fn from(val: u8) -> ConfigWen {
        ConfigWen::from_bits(val)
    }
}
impl From<ConfigWen> for u8 {
    #[inline(always)]
    fn from(val: ConfigWen) -> u8 {
        ConfigWen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConfignsWen {
    #[doc = "Read only access"]
    REN = 0x0,
    #[doc = "Write enabled"]
    WEN = 0x01,
    #[doc = "Erase enabled"]
    EEN = 0x02,
    _RESERVED_3 = 0x03,
}
impl ConfignsWen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfignsWen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfignsWen {
    #[inline(always)]
    fn from(val: u8) -> ConfignsWen {
        ConfignsWen::from_bits(val)
    }
}
impl From<ConfignsWen> for u8 {
    #[inline(always)]
    fn from(val: ConfignsWen) -> u8 {
        ConfignsWen::to_bits(val)
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key(pub u32);
impl Key {
    #[doc = "Key value"]
    pub const KEYVALID: Self = Self(0x0afb_e5a7);
}
impl Key {
    pub const fn from_bits(val: u32) -> Key {
        Self(val & 0x0fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Key {
    #[inline(always)]
    fn from(val: u32) -> Key {
        Key::from_bits(val)
    }
}
impl From<Key> for u32 {
    #[inline(always)]
    fn from(val: Key) -> u32 {
        Key::to_bits(val)
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
pub enum Set {
    _RESERVED_0 = 0x0,
    #[doc = "Set value"]
    SET = 0x01,
}
impl Set {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Set {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Set {
    #[inline(always)]
    fn from(val: u8) -> Set {
        Set::from_bits(val)
    }
}
impl From<Set> for u8 {
    #[inline(always)]
    fn from(val: Set) -> u8 {
        Set::to_bits(val)
    }
}
