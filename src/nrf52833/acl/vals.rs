#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Read {
    #[doc = "Allow read instructions to region n."]
    ENABLE = 0x0,
    #[doc = "Block read instructions to region n."]
    DISABLE = 0x01,
}
impl Read {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Read {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Read {
    #[inline(always)]
    fn from(val: u8) -> Read {
        Read::from_bits(val)
    }
}
impl From<Read> for u8 {
    #[inline(always)]
    fn from(val: Read) -> u8 {
        Read::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Write {
    #[doc = "Allow write and erase instructions to region n."]
    ENABLE = 0x0,
    #[doc = "Block write and erase instructions to region n."]
    DISABLE = 0x01,
}
impl Write {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Write {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Write {
    #[inline(always)]
    fn from(val: u8) -> Write {
        Write::from_bits(val)
    }
}
impl From<Write> for u8 {
    #[inline(always)]
    fn from(val: Write) -> u8 {
        Write::to_bits(val)
    }
}
