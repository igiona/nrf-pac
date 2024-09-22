#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctrlap {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Ctrlap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrlap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrlap {
    #[inline(always)]
    fn from(val: u8) -> Ctrlap {
        Ctrlap::from_bits(val)
    }
}
impl From<Ctrlap> for u8 {
    #[inline(always)]
    fn from(val: Ctrlap) -> u8 {
        Ctrlap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dif {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Dif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dif {
    #[inline(always)]
    fn from(val: u8) -> Dif {
        Dif::from_bits(val)
    }
}
impl From<Dif> for u8 {
    #[inline(always)]
    fn from(val: Dif) -> u8 {
        Dif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dog0 {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Dog0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dog0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dog0 {
    #[inline(always)]
    fn from(val: u8) -> Dog0 {
        Dog0::from_bits(val)
    }
}
impl From<Dog0> for u8 {
    #[inline(always)]
    fn from(val: Dog0) -> u8 {
        Dog0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dog1 {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Dog1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dog1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dog1 {
    #[inline(always)]
    fn from(val: u8) -> Dog1 {
        Dog1::from_bits(val)
    }
}
impl From<Dog1> for u8 {
    #[inline(always)]
    fn from(val: Dog1) -> u8 {
        Dog1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Forceoff {
    #[doc = "Release Force-OFF"]
    RELEASE = 0x0,
    #[doc = "Hold Force-OFF"]
    HOLD = 0x01,
}
impl Forceoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Forceoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Forceoff {
    #[inline(always)]
    fn from(val: u8) -> Forceoff {
        Forceoff::from_bits(val)
    }
}
impl From<Forceoff> for u8 {
    #[inline(always)]
    fn from(val: Forceoff) -> u8 {
        Forceoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lockup {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lockup {
    #[inline(always)]
    fn from(val: u8) -> Lockup {
        Lockup::from_bits(val)
    }
}
impl From<Lockup> for u8 {
    #[inline(always)]
    fn from(val: Lockup) -> u8 {
        Lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpcomp {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Lpcomp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpcomp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpcomp {
    #[inline(always)]
    fn from(val: u8) -> Lpcomp {
        Lpcomp::from_bits(val)
    }
}
impl From<Lpcomp> for u8 {
    #[inline(always)]
    fn from(val: Lpcomp) -> u8 {
        Lpcomp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Nfc {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Nfc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfc {
    #[inline(always)]
    fn from(val: u8) -> Nfc {
        Nfc::from_bits(val)
    }
}
impl From<Nfc> for u8 {
    #[inline(always)]
    fn from(val: Nfc) -> u8 {
        Nfc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Off {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Off {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Off {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Off {
    #[inline(always)]
    fn from(val: u8) -> Off {
        Off::from_bits(val)
    }
}
impl From<Off> for u8 {
    #[inline(always)]
    fn from(val: Off) -> u8 {
        Off::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resetpin {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Resetpin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resetpin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resetpin {
    #[inline(always)]
    fn from(val: u8) -> Resetpin {
        Resetpin::from_bits(val)
    }
}
impl From<Resetpin> for u8 {
    #[inline(always)]
    fn from(val: Resetpin) -> u8 {
        Resetpin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sreq {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Sreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sreq {
    #[inline(always)]
    fn from(val: u8) -> Sreq {
        Sreq::from_bits(val)
    }
}
impl From<Sreq> for u8 {
    #[inline(always)]
    fn from(val: Sreq) -> u8 {
        Sreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vbus {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Vbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbus {
    #[inline(always)]
    fn from(val: u8) -> Vbus {
        Vbus::from_bits(val)
    }
}
impl From<Vbus> for u8 {
    #[inline(always)]
    fn from(val: Vbus) -> u8 {
        Vbus::to_bits(val)
    }
}
