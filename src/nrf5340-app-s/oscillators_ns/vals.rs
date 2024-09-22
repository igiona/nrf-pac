#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bypass {
    #[doc = "Disable (use with crystal or low-swing external source)"]
    DISABLED = 0x0,
    #[doc = "Enable (use with rail-to-rail external source)"]
    ENABLED = 0x01,
}
impl Bypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bypass {
    #[inline(always)]
    fn from(val: u8) -> Bypass {
        Bypass::from_bits(val)
    }
}
impl From<Bypass> for u8 {
    #[inline(always)]
    fn from(val: Bypass) -> u8 {
        Bypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Capacitor disabled (use external caps)"]
    DISABLED = 0x0,
    #[doc = "Capacitor enabled"]
    ENABLED = 0x01,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Intcap {
    #[doc = "Use external load capacitors"]
    EXTERNAL = 0x0,
    #[doc = "6 pF internal load capacitance"]
    C6PF = 0x01,
    #[doc = "7 pF internal load capacitance"]
    C7PF = 0x02,
    #[doc = "9 pF internal load capacitance"]
    C9PF = 0x03,
}
impl Intcap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intcap {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intcap {
    #[inline(always)]
    fn from(val: u8) -> Intcap {
        Intcap::from_bits(val)
    }
}
impl From<Intcap> for u8 {
    #[inline(always)]
    fn from(val: Intcap) -> u8 {
        Intcap::to_bits(val)
    }
}
