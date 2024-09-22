#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Connect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Connect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Connect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Connect {
    #[inline(always)]
    fn from(val: u8) -> Connect {
        Connect::from_bits(val)
    }
}
impl From<Connect> for u8 {
    #[inline(always)]
    fn from(val: Connect) -> u8 {
        Connect::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cpufpben(pub u8);
impl Cpufpben {
    #[doc = "Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    pub const DISABLED: Self = Self(0x0);
    #[doc = "Enable CPU FPB unit (default behavior)"]
    pub const ENABLED: Self = Self(0xff);
}
impl Cpufpben {
    pub const fn from_bits(val: u8) -> Cpufpben {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Cpufpben {
    #[inline(always)]
    fn from(val: u8) -> Cpufpben {
        Cpufpben::from_bits(val)
    }
}
impl From<Cpufpben> for u8 {
    #[inline(always)]
    fn from(val: Cpufpben) -> u8 {
        Cpufpben::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cpuniden(pub u8);
impl Cpuniden {
    #[doc = "Disable CPU ITM and ETM functionality"]
    pub const DISABLED: Self = Self(0x0);
    #[doc = "Enable CPU ITM and ETM functionality (default behavior)"]
    pub const ENABLED: Self = Self(0xff);
}
impl Cpuniden {
    pub const fn from_bits(val: u8) -> Cpuniden {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Cpuniden {
    #[inline(always)]
    fn from(val: u8) -> Cpuniden {
        Cpuniden::from_bits(val)
    }
}
impl From<Cpuniden> for u8 {
    #[inline(always)]
    fn from(val: Cpuniden) -> u8 {
        Cpuniden::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pall(pub u8);
impl Pall {
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x0);
    #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware and software"]
    pub const HWDISABLED: Self = Self(0x5a);
    #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware"]
    pub const DISABLED: Self = Self(0xff);
}
impl Pall {
    pub const fn from_bits(val: u8) -> Pall {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Pall {
    #[inline(always)]
    fn from(val: u8) -> Pall {
        Pall::from_bits(val)
    }
}
impl From<Pall> for u8 {
    #[inline(always)]
    fn from(val: Pall) -> u8 {
        Pall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Protect {
    #[doc = "Operation as GPIO pins. Same protection as normal GPIO pins."]
    DISABLED = 0x0,
    #[doc = "Operation as NFC antenna pins. Configures the protection for NFC operation."]
    NFC = 0x01,
}
impl Protect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Protect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Protect {
    #[inline(always)]
    fn from(val: u8) -> Protect {
        Protect::from_bits(val)
    }
}
impl From<Protect> for u8 {
    #[inline(always)]
    fn from(val: Protect) -> u8 {
        Protect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vout {
    #[doc = "1.8 V"]
    _1V8 = 0x0,
    #[doc = "2.1 V"]
    _2V1 = 0x01,
    #[doc = "2.4 V"]
    _2V4 = 0x02,
    #[doc = "2.7 V"]
    _2V7 = 0x03,
    #[doc = "3.0 V"]
    _3V0 = 0x04,
    #[doc = "3.3 V"]
    _3V3 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Default voltage: 1.8 V"]
    DEFAULT = 0x07,
}
impl Vout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vout {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vout {
    #[inline(always)]
    fn from(val: u8) -> Vout {
        Vout::from_bits(val)
    }
}
impl From<Vout> for u8 {
    #[inline(always)]
    fn from(val: Vout) -> u8 {
        Vout::to_bits(val)
    }
}
