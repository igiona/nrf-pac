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
pub enum PowerS10power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS10power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS10power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS10power {
    #[inline(always)]
    fn from(val: u8) -> PowerS10power {
        PowerS10power::from_bits(val)
    }
}
impl From<PowerS10power> for u8 {
    #[inline(always)]
    fn from(val: PowerS10power) -> u8 {
        PowerS10power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS10retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS10retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS10retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS10retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS10retention {
        PowerS10retention::from_bits(val)
    }
}
impl From<PowerS10retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS10retention) -> u8 {
        PowerS10retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS11power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS11power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS11power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS11power {
    #[inline(always)]
    fn from(val: u8) -> PowerS11power {
        PowerS11power::from_bits(val)
    }
}
impl From<PowerS11power> for u8 {
    #[inline(always)]
    fn from(val: PowerS11power) -> u8 {
        PowerS11power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS11retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS11retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS11retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS11retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS11retention {
        PowerS11retention::from_bits(val)
    }
}
impl From<PowerS11retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS11retention) -> u8 {
        PowerS11retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS12power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS12power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS12power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS12power {
    #[inline(always)]
    fn from(val: u8) -> PowerS12power {
        PowerS12power::from_bits(val)
    }
}
impl From<PowerS12power> for u8 {
    #[inline(always)]
    fn from(val: PowerS12power) -> u8 {
        PowerS12power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS12retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS12retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS12retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS12retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS12retention {
        PowerS12retention::from_bits(val)
    }
}
impl From<PowerS12retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS12retention) -> u8 {
        PowerS12retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS13power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS13power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS13power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS13power {
    #[inline(always)]
    fn from(val: u8) -> PowerS13power {
        PowerS13power::from_bits(val)
    }
}
impl From<PowerS13power> for u8 {
    #[inline(always)]
    fn from(val: PowerS13power) -> u8 {
        PowerS13power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS13retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS13retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS13retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS13retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS13retention {
        PowerS13retention::from_bits(val)
    }
}
impl From<PowerS13retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS13retention) -> u8 {
        PowerS13retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS14power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS14power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS14power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS14power {
    #[inline(always)]
    fn from(val: u8) -> PowerS14power {
        PowerS14power::from_bits(val)
    }
}
impl From<PowerS14power> for u8 {
    #[inline(always)]
    fn from(val: PowerS14power) -> u8 {
        PowerS14power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS14retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS14retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS14retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS14retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS14retention {
        PowerS14retention::from_bits(val)
    }
}
impl From<PowerS14retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS14retention) -> u8 {
        PowerS14retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS15power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS15power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS15power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS15power {
    #[inline(always)]
    fn from(val: u8) -> PowerS15power {
        PowerS15power::from_bits(val)
    }
}
impl From<PowerS15power> for u8 {
    #[inline(always)]
    fn from(val: PowerS15power) -> u8 {
        PowerS15power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS15retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS15retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS15retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS15retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS15retention {
        PowerS15retention::from_bits(val)
    }
}
impl From<PowerS15retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS15retention) -> u8 {
        PowerS15retention::to_bits(val)
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
pub enum PowerS4power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS4power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS4power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS4power {
    #[inline(always)]
    fn from(val: u8) -> PowerS4power {
        PowerS4power::from_bits(val)
    }
}
impl From<PowerS4power> for u8 {
    #[inline(always)]
    fn from(val: PowerS4power) -> u8 {
        PowerS4power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS4retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS4retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS4retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS4retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS4retention {
        PowerS4retention::from_bits(val)
    }
}
impl From<PowerS4retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS4retention) -> u8 {
        PowerS4retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS5power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS5power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS5power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS5power {
    #[inline(always)]
    fn from(val: u8) -> PowerS5power {
        PowerS5power::from_bits(val)
    }
}
impl From<PowerS5power> for u8 {
    #[inline(always)]
    fn from(val: PowerS5power) -> u8 {
        PowerS5power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS5retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS5retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS5retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS5retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS5retention {
        PowerS5retention::from_bits(val)
    }
}
impl From<PowerS5retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS5retention) -> u8 {
        PowerS5retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS6power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS6power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS6power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS6power {
    #[inline(always)]
    fn from(val: u8) -> PowerS6power {
        PowerS6power::from_bits(val)
    }
}
impl From<PowerS6power> for u8 {
    #[inline(always)]
    fn from(val: PowerS6power) -> u8 {
        PowerS6power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS6retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS6retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS6retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS6retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS6retention {
        PowerS6retention::from_bits(val)
    }
}
impl From<PowerS6retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS6retention) -> u8 {
        PowerS6retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS7power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS7power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS7power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS7power {
    #[inline(always)]
    fn from(val: u8) -> PowerS7power {
        PowerS7power::from_bits(val)
    }
}
impl From<PowerS7power> for u8 {
    #[inline(always)]
    fn from(val: PowerS7power) -> u8 {
        PowerS7power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS7retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS7retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS7retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS7retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS7retention {
        PowerS7retention::from_bits(val)
    }
}
impl From<PowerS7retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS7retention) -> u8 {
        PowerS7retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS8power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS8power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS8power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS8power {
    #[inline(always)]
    fn from(val: u8) -> PowerS8power {
        PowerS8power::from_bits(val)
    }
}
impl From<PowerS8power> for u8 {
    #[inline(always)]
    fn from(val: PowerS8power) -> u8 {
        PowerS8power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS8retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS8retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS8retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS8retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS8retention {
        PowerS8retention::from_bits(val)
    }
}
impl From<PowerS8retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS8retention) -> u8 {
        PowerS8retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS9power {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS9power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS9power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS9power {
    #[inline(always)]
    fn from(val: u8) -> PowerS9power {
        PowerS9power::from_bits(val)
    }
}
impl From<PowerS9power> for u8 {
    #[inline(always)]
    fn from(val: PowerS9power) -> u8 {
        PowerS9power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerS9retention {
    #[doc = "Off"]
    OFF = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowerS9retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerS9retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerS9retention {
    #[inline(always)]
    fn from(val: u8) -> PowerS9retention {
        PowerS9retention::from_bits(val)
    }
}
impl From<PowerS9retention> for u8 {
    #[inline(always)]
    fn from(val: PowerS9retention) -> u8 {
        PowerS9retention::to_bits(val)
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
pub enum PowerclrS10power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS10power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS10power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS10power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS10power {
        PowerclrS10power::from_bits(val)
    }
}
impl From<PowerclrS10power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS10power) -> u8 {
        PowerclrS10power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS10retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS10retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS10retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS10retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS10retention {
        PowerclrS10retention::from_bits(val)
    }
}
impl From<PowerclrS10retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS10retention) -> u8 {
        PowerclrS10retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS11power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS11power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS11power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS11power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS11power {
        PowerclrS11power::from_bits(val)
    }
}
impl From<PowerclrS11power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS11power) -> u8 {
        PowerclrS11power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS11retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS11retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS11retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS11retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS11retention {
        PowerclrS11retention::from_bits(val)
    }
}
impl From<PowerclrS11retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS11retention) -> u8 {
        PowerclrS11retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS12power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS12power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS12power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS12power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS12power {
        PowerclrS12power::from_bits(val)
    }
}
impl From<PowerclrS12power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS12power) -> u8 {
        PowerclrS12power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS12retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS12retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS12retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS12retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS12retention {
        PowerclrS12retention::from_bits(val)
    }
}
impl From<PowerclrS12retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS12retention) -> u8 {
        PowerclrS12retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS13power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS13power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS13power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS13power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS13power {
        PowerclrS13power::from_bits(val)
    }
}
impl From<PowerclrS13power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS13power) -> u8 {
        PowerclrS13power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS13retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS13retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS13retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS13retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS13retention {
        PowerclrS13retention::from_bits(val)
    }
}
impl From<PowerclrS13retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS13retention) -> u8 {
        PowerclrS13retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS14power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS14power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS14power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS14power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS14power {
        PowerclrS14power::from_bits(val)
    }
}
impl From<PowerclrS14power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS14power) -> u8 {
        PowerclrS14power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS14retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS14retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS14retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS14retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS14retention {
        PowerclrS14retention::from_bits(val)
    }
}
impl From<PowerclrS14retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS14retention) -> u8 {
        PowerclrS14retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS15power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS15power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS15power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS15power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS15power {
        PowerclrS15power::from_bits(val)
    }
}
impl From<PowerclrS15power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS15power) -> u8 {
        PowerclrS15power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS15retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS15retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS15retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS15retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS15retention {
        PowerclrS15retention::from_bits(val)
    }
}
impl From<PowerclrS15retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS15retention) -> u8 {
        PowerclrS15retention::to_bits(val)
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
pub enum PowerclrS4power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS4power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS4power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS4power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS4power {
        PowerclrS4power::from_bits(val)
    }
}
impl From<PowerclrS4power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS4power) -> u8 {
        PowerclrS4power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS4retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS4retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS4retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS4retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS4retention {
        PowerclrS4retention::from_bits(val)
    }
}
impl From<PowerclrS4retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS4retention) -> u8 {
        PowerclrS4retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS5power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS5power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS5power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS5power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS5power {
        PowerclrS5power::from_bits(val)
    }
}
impl From<PowerclrS5power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS5power) -> u8 {
        PowerclrS5power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS5retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS5retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS5retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS5retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS5retention {
        PowerclrS5retention::from_bits(val)
    }
}
impl From<PowerclrS5retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS5retention) -> u8 {
        PowerclrS5retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS6power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS6power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS6power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS6power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS6power {
        PowerclrS6power::from_bits(val)
    }
}
impl From<PowerclrS6power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS6power) -> u8 {
        PowerclrS6power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS6retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS6retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS6retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS6retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS6retention {
        PowerclrS6retention::from_bits(val)
    }
}
impl From<PowerclrS6retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS6retention) -> u8 {
        PowerclrS6retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS7power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS7power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS7power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS7power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS7power {
        PowerclrS7power::from_bits(val)
    }
}
impl From<PowerclrS7power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS7power) -> u8 {
        PowerclrS7power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS7retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS7retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS7retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS7retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS7retention {
        PowerclrS7retention::from_bits(val)
    }
}
impl From<PowerclrS7retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS7retention) -> u8 {
        PowerclrS7retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS8power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS8power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS8power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS8power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS8power {
        PowerclrS8power::from_bits(val)
    }
}
impl From<PowerclrS8power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS8power) -> u8 {
        PowerclrS8power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS8retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS8retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS8retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS8retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS8retention {
        PowerclrS8retention::from_bits(val)
    }
}
impl From<PowerclrS8retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS8retention) -> u8 {
        PowerclrS8retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS9power {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS9power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS9power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS9power {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS9power {
        PowerclrS9power::from_bits(val)
    }
}
impl From<PowerclrS9power> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS9power) -> u8 {
        PowerclrS9power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerclrS9retention {
    _RESERVED_0 = 0x0,
    #[doc = "Off"]
    OFF = 0x01,
}
impl PowerclrS9retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerclrS9retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerclrS9retention {
    #[inline(always)]
    fn from(val: u8) -> PowerclrS9retention {
        PowerclrS9retention::from_bits(val)
    }
}
impl From<PowerclrS9retention> for u8 {
    #[inline(always)]
    fn from(val: PowerclrS9retention) -> u8 {
        PowerclrS9retention::to_bits(val)
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
pub enum PowersetS10power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS10power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS10power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS10power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS10power {
        PowersetS10power::from_bits(val)
    }
}
impl From<PowersetS10power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS10power) -> u8 {
        PowersetS10power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS10retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS10retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS10retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS10retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS10retention {
        PowersetS10retention::from_bits(val)
    }
}
impl From<PowersetS10retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS10retention) -> u8 {
        PowersetS10retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS11power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS11power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS11power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS11power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS11power {
        PowersetS11power::from_bits(val)
    }
}
impl From<PowersetS11power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS11power) -> u8 {
        PowersetS11power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS11retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS11retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS11retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS11retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS11retention {
        PowersetS11retention::from_bits(val)
    }
}
impl From<PowersetS11retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS11retention) -> u8 {
        PowersetS11retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS12power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS12power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS12power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS12power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS12power {
        PowersetS12power::from_bits(val)
    }
}
impl From<PowersetS12power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS12power) -> u8 {
        PowersetS12power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS12retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS12retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS12retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS12retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS12retention {
        PowersetS12retention::from_bits(val)
    }
}
impl From<PowersetS12retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS12retention) -> u8 {
        PowersetS12retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS13power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS13power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS13power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS13power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS13power {
        PowersetS13power::from_bits(val)
    }
}
impl From<PowersetS13power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS13power) -> u8 {
        PowersetS13power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS13retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS13retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS13retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS13retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS13retention {
        PowersetS13retention::from_bits(val)
    }
}
impl From<PowersetS13retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS13retention) -> u8 {
        PowersetS13retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS14power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS14power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS14power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS14power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS14power {
        PowersetS14power::from_bits(val)
    }
}
impl From<PowersetS14power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS14power) -> u8 {
        PowersetS14power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS14retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS14retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS14retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS14retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS14retention {
        PowersetS14retention::from_bits(val)
    }
}
impl From<PowersetS14retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS14retention) -> u8 {
        PowersetS14retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS15power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS15power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS15power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS15power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS15power {
        PowersetS15power::from_bits(val)
    }
}
impl From<PowersetS15power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS15power) -> u8 {
        PowersetS15power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS15retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS15retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS15retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS15retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS15retention {
        PowersetS15retention::from_bits(val)
    }
}
impl From<PowersetS15retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS15retention) -> u8 {
        PowersetS15retention::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS4power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS4power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS4power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS4power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS4power {
        PowersetS4power::from_bits(val)
    }
}
impl From<PowersetS4power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS4power) -> u8 {
        PowersetS4power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS4retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS4retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS4retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS4retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS4retention {
        PowersetS4retention::from_bits(val)
    }
}
impl From<PowersetS4retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS4retention) -> u8 {
        PowersetS4retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS5power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS5power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS5power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS5power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS5power {
        PowersetS5power::from_bits(val)
    }
}
impl From<PowersetS5power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS5power) -> u8 {
        PowersetS5power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS5retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS5retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS5retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS5retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS5retention {
        PowersetS5retention::from_bits(val)
    }
}
impl From<PowersetS5retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS5retention) -> u8 {
        PowersetS5retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS6power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS6power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS6power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS6power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS6power {
        PowersetS6power::from_bits(val)
    }
}
impl From<PowersetS6power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS6power) -> u8 {
        PowersetS6power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS6retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS6retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS6retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS6retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS6retention {
        PowersetS6retention::from_bits(val)
    }
}
impl From<PowersetS6retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS6retention) -> u8 {
        PowersetS6retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS7power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS7power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS7power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS7power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS7power {
        PowersetS7power::from_bits(val)
    }
}
impl From<PowersetS7power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS7power) -> u8 {
        PowersetS7power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS7retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS7retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS7retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS7retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS7retention {
        PowersetS7retention::from_bits(val)
    }
}
impl From<PowersetS7retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS7retention) -> u8 {
        PowersetS7retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS8power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS8power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS8power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS8power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS8power {
        PowersetS8power::from_bits(val)
    }
}
impl From<PowersetS8power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS8power) -> u8 {
        PowersetS8power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS8retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS8retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS8retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS8retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS8retention {
        PowersetS8retention::from_bits(val)
    }
}
impl From<PowersetS8retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS8retention) -> u8 {
        PowersetS8retention::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS9power {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS9power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS9power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS9power {
    #[inline(always)]
    fn from(val: u8) -> PowersetS9power {
        PowersetS9power::from_bits(val)
    }
}
impl From<PowersetS9power> for u8 {
    #[inline(always)]
    fn from(val: PowersetS9power) -> u8 {
        PowersetS9power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowersetS9retention {
    _RESERVED_0 = 0x0,
    #[doc = "On"]
    ON = 0x01,
}
impl PowersetS9retention {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowersetS9retention {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowersetS9retention {
    #[inline(always)]
    fn from(val: u8) -> PowersetS9retention {
        PowersetS9retention::from_bits(val)
    }
}
impl From<PowersetS9retention> for u8 {
    #[inline(always)]
    fn from(val: PowersetS9retention) -> u8 {
        PowersetS9retention::to_bits(val)
    }
}
