#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pof {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Pof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pof {
    #[inline(always)]
    fn from(val: u8) -> Pof {
        Pof::from_bits(val)
    }
}
impl From<Pof> for u8 {
    #[inline(always)]
    fn from(val: Pof) -> u8 {
        Pof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Systemoff {
    _RESERVED_0 = 0x0,
    #[doc = "Enable System OFF mode"]
    ENTER = 0x01,
}
impl Systemoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systemoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systemoff {
    #[inline(always)]
    fn from(val: u8) -> Systemoff {
        Systemoff::from_bits(val)
    }
}
impl From<Systemoff> for u8 {
    #[inline(always)]
    fn from(val: Systemoff) -> u8 {
        Systemoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Threshold {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Set threshold to 1.9 V"]
    V19 = 0x06,
    #[doc = "Set threshold to 2.0 V"]
    V20 = 0x07,
    #[doc = "Set threshold to 2.1 V"]
    V21 = 0x08,
    #[doc = "Set threshold to 2.2 V"]
    V22 = 0x09,
    #[doc = "Set threshold to 2.3 V"]
    V23 = 0x0a,
    #[doc = "Set threshold to 2.4 V"]
    V24 = 0x0b,
    #[doc = "Set threshold to 2.5 V"]
    V25 = 0x0c,
    #[doc = "Set threshold to 2.6 V"]
    V26 = 0x0d,
    #[doc = "Set threshold to 2.7 V"]
    V27 = 0x0e,
    #[doc = "Set threshold to 2.8 V"]
    V28 = 0x0f,
}
impl Threshold {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Threshold {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Threshold {
    #[inline(always)]
    fn from(val: u8) -> Threshold {
        Threshold::from_bits(val)
    }
}
impl From<Threshold> for u8 {
    #[inline(always)]
    fn from(val: Threshold) -> u8 {
        Threshold::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Thresholdvddh {
    #[doc = "Set threshold to 2.7 V"]
    V27 = 0x0,
    #[doc = "Set threshold to 2.8 V"]
    V28 = 0x01,
    #[doc = "Set threshold to 2.9 V"]
    V29 = 0x02,
    #[doc = "Set threshold to 3.0 V"]
    V30 = 0x03,
    #[doc = "Set threshold to 3.1 V"]
    V31 = 0x04,
    #[doc = "Set threshold to 3.2 V"]
    V32 = 0x05,
    #[doc = "Set threshold to 3.3 V"]
    V33 = 0x06,
    #[doc = "Set threshold to 3.4 V"]
    V34 = 0x07,
    #[doc = "Set threshold to 3.5 V"]
    V35 = 0x08,
    #[doc = "Set threshold to 3.6 V"]
    V36 = 0x09,
    #[doc = "Set threshold to 3.7 V"]
    V37 = 0x0a,
    #[doc = "Set threshold to 3.8 V"]
    V38 = 0x0b,
    #[doc = "Set threshold to 3.9 V"]
    V39 = 0x0c,
    #[doc = "Set threshold to 4.0 V"]
    V40 = 0x0d,
    #[doc = "Set threshold to 4.1 V"]
    V41 = 0x0e,
    #[doc = "Set threshold to 4.2 V"]
    V42 = 0x0f,
}
impl Thresholdvddh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Thresholdvddh {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Thresholdvddh {
    #[inline(always)]
    fn from(val: u8) -> Thresholdvddh {
        Thresholdvddh::from_bits(val)
    }
}
impl From<Thresholdvddh> for u8 {
    #[inline(always)]
    fn from(val: Thresholdvddh) -> u8 {
        Thresholdvddh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vregh {
    #[doc = "Normal voltage mode. Voltage supplied on VDD and VDDH."]
    INACTIVE = 0x0,
    #[doc = "High voltage mode. Voltage supplied on VDDH."]
    ACTIVE = 0x01,
}
impl Vregh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vregh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vregh {
    #[inline(always)]
    fn from(val: u8) -> Vregh {
        Vregh::from_bits(val)
    }
}
impl From<Vregh> for u8 {
    #[inline(always)]
    fn from(val: Vregh) -> u8 {
        Vregh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum VreghDcdcenDcdcen {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl VreghDcdcenDcdcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VreghDcdcenDcdcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VreghDcdcenDcdcen {
    #[inline(always)]
    fn from(val: u8) -> VreghDcdcenDcdcen {
        VreghDcdcenDcdcen::from_bits(val)
    }
}
impl From<VreghDcdcenDcdcen> for u8 {
    #[inline(always)]
    fn from(val: VreghDcdcenDcdcen) -> u8 {
        VreghDcdcenDcdcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum VregmainDcdcenDcdcen {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl VregmainDcdcenDcdcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VregmainDcdcenDcdcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VregmainDcdcenDcdcen {
    #[inline(always)]
    fn from(val: u8) -> VregmainDcdcenDcdcen {
        VregmainDcdcenDcdcen::from_bits(val)
    }
}
impl From<VregmainDcdcenDcdcen> for u8 {
    #[inline(always)]
    fn from(val: VregmainDcdcenDcdcen) -> u8 {
        VregmainDcdcenDcdcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum VregradioDcdcenDcdcen {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl VregradioDcdcenDcdcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VregradioDcdcenDcdcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VregradioDcdcenDcdcen {
    #[inline(always)]
    fn from(val: u8) -> VregradioDcdcenDcdcen {
        VregradioDcdcenDcdcen::from_bits(val)
    }
}
impl From<VregradioDcdcenDcdcen> for u8 {
    #[inline(always)]
    fn from(val: VregradioDcdcenDcdcen) -> u8 {
        VregradioDcdcenDcdcen::to_bits(val)
    }
}
