#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Detectmode {
    #[doc = "DETECT directly connected to PIN DETECT signals"]
    DEFAULT = 0x0,
    #[doc = "Use the latched LDETECT behavior"]
    LDETECT = 0x01,
}
impl Detectmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Detectmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Detectmode {
    #[inline(always)]
    fn from(val: u8) -> Detectmode {
        Detectmode::from_bits(val)
    }
}
impl From<Detectmode> for u8 {
    #[inline(always)]
    fn from(val: Detectmode) -> u8 {
        Detectmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dir {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl Dir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dir {
    #[inline(always)]
    fn from(val: u8) -> Dir {
        Dir::from_bits(val)
    }
}
impl From<Dir> for u8 {
    #[inline(always)]
    fn from(val: Dir) -> u8 {
        Dir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Drive {
    #[doc = "Standard '0', standard '1'"]
    S0S1 = 0x0,
    #[doc = "High drive '0', standard '1'"]
    H0S1 = 0x01,
    #[doc = "Standard '0', high drive '1'"]
    S0H1 = 0x02,
    #[doc = "High drive '0', high 'drive '1''"]
    H0H1 = 0x03,
    #[doc = "Disconnect '0' standard '1' (normally used for wired-or connections)"]
    D0S1 = 0x04,
    #[doc = "Disconnect '0', high drive '1' (normally used for wired-or connections)"]
    D0H1 = 0x05,
    #[doc = "Standard '0'. disconnect '1' (normally used for wired-and connections)"]
    S0D1 = 0x06,
    #[doc = "High drive '0', disconnect '1' (normally used for wired-and connections)"]
    H0D1 = 0x07,
}
impl Drive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Drive {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Drive {
    #[inline(always)]
    fn from(val: u8) -> Drive {
        Drive::from_bits(val)
    }
}
impl From<Drive> for u8 {
    #[inline(always)]
    fn from(val: Drive) -> u8 {
        Drive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pull {
    #[doc = "No pull"]
    DISABLED = 0x0,
    #[doc = "Pull down on pin"]
    PULLDOWN = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Pull up on pin"]
    PULLUP = 0x03,
}
impl Pull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pull {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pull {
    #[inline(always)]
    fn from(val: u8) -> Pull {
        Pull::from_bits(val)
    }
}
impl From<Pull> for u8 {
    #[inline(always)]
    fn from(val: Pull) -> u8 {
        Pull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sense {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Sense for high level"]
    HIGH = 0x02,
    #[doc = "Sense for low level"]
    LOW = 0x03,
}
impl Sense {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sense {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sense {
    #[inline(always)]
    fn from(val: u8) -> Sense {
        Sense::from_bits(val)
    }
}
impl From<Sense> for u8 {
    #[inline(always)]
    fn from(val: Sense) -> u8 {
        Sense::to_bits(val)
    }
}
