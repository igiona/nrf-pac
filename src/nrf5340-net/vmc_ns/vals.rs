#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS0power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS0power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS0power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS0power {
    #[inline(always)]
    fn from(val: u8) -> PowerS0power {
        PowerS0power::from_bits(val)
    }
}
impl From<PowerS0power> for u8 {
    #[inline(always)]
    fn from(val: PowerS0power) -> u8 {
        PowerS0power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS0retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS0retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS0retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS0retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS0retention {
        PowerS0retention::from_bits(val)
    }
}
impl From<PowerS0retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS0retention) -> u8 {
        PowerS0retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS1power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS1power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS1power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS1power {
    #[inline(always)]
    fn from(val: u8) -> PowerS1power {
        PowerS1power::from_bits(val)
    }
}
impl From<PowerS1power> for u8 {
    #[inline(always)]
    fn from(val: PowerS1power) -> u8 {
        PowerS1power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS1retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS1retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS1retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS1retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS1retention {
        PowerS1retention::from_bits(val)
    }
}
impl From<PowerS1retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS1retention) -> u8 {
        PowerS1retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS2power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS2power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS2power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS2power {
    #[inline(always)]
    fn from(val: u8) -> PowerS2power {
        PowerS2power::from_bits(val)
    }
}
impl From<PowerS2power> for u8 {
    #[inline(always)]
    fn from(val: PowerS2power) -> u8 {
        PowerS2power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS2retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS2retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS2retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS2retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS2retention {
        PowerS2retention::from_bits(val)
    }
}
impl From<PowerS2retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS2retention) -> u8 {
        PowerS2retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS3power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS3power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS3power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS3power {
    #[inline(always)]
    fn from(val: u8) -> PowerS3power {
        PowerS3power::from_bits(val)
    }
}
impl From<PowerS3power> for u8 {
    #[inline(always)]
    fn from(val: PowerS3power) -> u8 {
        PowerS3power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS3retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS3retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS3retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS3retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS3retention {
        PowerS3retention::from_bits(val)
    }
}
impl From<PowerS3retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS3retention) -> u8 {
        PowerS3retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS0power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS0power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS0power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS0power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS0power {
        PowerclrS0power::from_bits(val)
    }
}
impl From<PowerclrS0power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS0power) -> u8 {
        PowerclrS0power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS0retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS0retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS0retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS0retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS0retention {
        PowerclrS0retention::from_bits(val)
    }
}
impl From<PowerclrS0retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS0retention) -> u8 {
        PowerclrS0retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS1power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS1power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS1power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS1power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS1power {
        PowerclrS1power::from_bits(val)
    }
}
impl From<PowerclrS1power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS1power) -> u8 {
        PowerclrS1power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS1retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS1retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS1retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS1retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS1retention {
        PowerclrS1retention::from_bits(val)
    }
}
impl From<PowerclrS1retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS1retention) -> u8 {
        PowerclrS1retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS2power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS2power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS2power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS2power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS2power {
        PowerclrS2power::from_bits(val)
    }
}
impl From<PowerclrS2power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS2power) -> u8 {
        PowerclrS2power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS2retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS2retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS2retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS2retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS2retention {
        PowerclrS2retention::from_bits(val)
    }
}
impl From<PowerclrS2retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS2retention) -> u8 {
        PowerclrS2retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS3power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS3power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS3power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS3power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS3power {
        PowerclrS3power::from_bits(val)
    }
}
impl From<PowerclrS3power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS3power) -> u8 {
        PowerclrS3power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS3retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS3retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS3retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS3retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS3retention {
        PowerclrS3retention::from_bits(val)
    }
}
impl From<PowerclrS3retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS3retention) -> u8 {
        PowerclrS3retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS0power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS0power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS0power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS0power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS0power {
        PowersetS0power::from_bits(val)
    }
}
impl From<PowersetS0power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS0power) -> u8 {
        PowersetS0power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS0retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS0retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS0retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS0retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS0retention {
        PowersetS0retention::from_bits(val)
    }
}
impl From<PowersetS0retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS0retention) -> u8 {
        PowersetS0retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS1power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS1power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS1power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS1power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS1power {
        PowersetS1power::from_bits(val)
    }
}
impl From<PowersetS1power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS1power) -> u8 {
        PowersetS1power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS1retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS1retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS1retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS1retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS1retention {
        PowersetS1retention::from_bits(val)
    }
}
impl From<PowersetS1retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS1retention) -> u8 {
        PowersetS1retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS2power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS2power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS2power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS2power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS2power {
        PowersetS2power::from_bits(val)
    }
}
impl From<PowersetS2power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS2power) -> u8 {
        PowersetS2power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS2retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS2retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS2retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS2retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS2retention {
        PowersetS2retention::from_bits(val)
    }
}
impl From<PowersetS2retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS2retention) -> u8 {
        PowersetS2retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS3power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS3power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS3power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS3power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS3power {
        PowersetS3power::from_bits(val)
    }
}
impl From<PowersetS3power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS3power) -> u8 {
        PowersetS3power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS3retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS3retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS3retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS3retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS3retention {
        PowersetS3retention::from_bits(val)
    }
}
impl From<PowersetS3retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS3retention) -> u8 {
        PowersetS3retention::to_bits(val)
    }
}
