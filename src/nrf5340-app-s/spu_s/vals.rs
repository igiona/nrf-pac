#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel0 {
    #[doc = "Channel 0 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 0 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel0 {
    #[inline(always)]
    fn from(val: u8) -> Channel0 {
        Channel0::from_bits(val)
    }
}
impl From<Channel0> for u8 {
    #[inline(always)]
    fn from(val: Channel0) -> u8 {
        Channel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel1 {
    #[doc = "Channel 1 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 1 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel1 {
    #[inline(always)]
    fn from(val: u8) -> Channel1 {
        Channel1::from_bits(val)
    }
}
impl From<Channel1> for u8 {
    #[inline(always)]
    fn from(val: Channel1) -> u8 {
        Channel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel10 {
    #[doc = "Channel 10 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 10 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel10 {
    #[inline(always)]
    fn from(val: u8) -> Channel10 {
        Channel10::from_bits(val)
    }
}
impl From<Channel10> for u8 {
    #[inline(always)]
    fn from(val: Channel10) -> u8 {
        Channel10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel11 {
    #[doc = "Channel 11 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 11 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel11 {
    #[inline(always)]
    fn from(val: u8) -> Channel11 {
        Channel11::from_bits(val)
    }
}
impl From<Channel11> for u8 {
    #[inline(always)]
    fn from(val: Channel11) -> u8 {
        Channel11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel12 {
    #[doc = "Channel 12 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 12 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel12 {
    #[inline(always)]
    fn from(val: u8) -> Channel12 {
        Channel12::from_bits(val)
    }
}
impl From<Channel12> for u8 {
    #[inline(always)]
    fn from(val: Channel12) -> u8 {
        Channel12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel13 {
    #[doc = "Channel 13 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 13 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel13 {
    #[inline(always)]
    fn from(val: u8) -> Channel13 {
        Channel13::from_bits(val)
    }
}
impl From<Channel13> for u8 {
    #[inline(always)]
    fn from(val: Channel13) -> u8 {
        Channel13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel14 {
    #[doc = "Channel 14 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 14 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel14 {
    #[inline(always)]
    fn from(val: u8) -> Channel14 {
        Channel14::from_bits(val)
    }
}
impl From<Channel14> for u8 {
    #[inline(always)]
    fn from(val: Channel14) -> u8 {
        Channel14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel15 {
    #[doc = "Channel 15 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 15 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel15 {
    #[inline(always)]
    fn from(val: u8) -> Channel15 {
        Channel15::from_bits(val)
    }
}
impl From<Channel15> for u8 {
    #[inline(always)]
    fn from(val: Channel15) -> u8 {
        Channel15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel16 {
    #[doc = "Channel 16 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 16 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel16 {
    #[inline(always)]
    fn from(val: u8) -> Channel16 {
        Channel16::from_bits(val)
    }
}
impl From<Channel16> for u8 {
    #[inline(always)]
    fn from(val: Channel16) -> u8 {
        Channel16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel17 {
    #[doc = "Channel 17 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 17 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel17 {
    #[inline(always)]
    fn from(val: u8) -> Channel17 {
        Channel17::from_bits(val)
    }
}
impl From<Channel17> for u8 {
    #[inline(always)]
    fn from(val: Channel17) -> u8 {
        Channel17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel18 {
    #[doc = "Channel 18 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 18 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel18 {
    #[inline(always)]
    fn from(val: u8) -> Channel18 {
        Channel18::from_bits(val)
    }
}
impl From<Channel18> for u8 {
    #[inline(always)]
    fn from(val: Channel18) -> u8 {
        Channel18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel19 {
    #[doc = "Channel 19 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 19 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel19 {
    #[inline(always)]
    fn from(val: u8) -> Channel19 {
        Channel19::from_bits(val)
    }
}
impl From<Channel19> for u8 {
    #[inline(always)]
    fn from(val: Channel19) -> u8 {
        Channel19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel2 {
    #[doc = "Channel 2 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 2 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel2 {
    #[inline(always)]
    fn from(val: u8) -> Channel2 {
        Channel2::from_bits(val)
    }
}
impl From<Channel2> for u8 {
    #[inline(always)]
    fn from(val: Channel2) -> u8 {
        Channel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel20 {
    #[doc = "Channel 20 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 20 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel20 {
    #[inline(always)]
    fn from(val: u8) -> Channel20 {
        Channel20::from_bits(val)
    }
}
impl From<Channel20> for u8 {
    #[inline(always)]
    fn from(val: Channel20) -> u8 {
        Channel20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel21 {
    #[doc = "Channel 21 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 21 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel21 {
    #[inline(always)]
    fn from(val: u8) -> Channel21 {
        Channel21::from_bits(val)
    }
}
impl From<Channel21> for u8 {
    #[inline(always)]
    fn from(val: Channel21) -> u8 {
        Channel21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel22 {
    #[doc = "Channel 22 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 22 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel22 {
    #[inline(always)]
    fn from(val: u8) -> Channel22 {
        Channel22::from_bits(val)
    }
}
impl From<Channel22> for u8 {
    #[inline(always)]
    fn from(val: Channel22) -> u8 {
        Channel22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel23 {
    #[doc = "Channel 23 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 23 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel23 {
    #[inline(always)]
    fn from(val: u8) -> Channel23 {
        Channel23::from_bits(val)
    }
}
impl From<Channel23> for u8 {
    #[inline(always)]
    fn from(val: Channel23) -> u8 {
        Channel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel24 {
    #[doc = "Channel 24 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 24 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel24 {
    #[inline(always)]
    fn from(val: u8) -> Channel24 {
        Channel24::from_bits(val)
    }
}
impl From<Channel24> for u8 {
    #[inline(always)]
    fn from(val: Channel24) -> u8 {
        Channel24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel25 {
    #[doc = "Channel 25 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 25 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel25 {
    #[inline(always)]
    fn from(val: u8) -> Channel25 {
        Channel25::from_bits(val)
    }
}
impl From<Channel25> for u8 {
    #[inline(always)]
    fn from(val: Channel25) -> u8 {
        Channel25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel26 {
    #[doc = "Channel 26 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 26 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel26 {
    #[inline(always)]
    fn from(val: u8) -> Channel26 {
        Channel26::from_bits(val)
    }
}
impl From<Channel26> for u8 {
    #[inline(always)]
    fn from(val: Channel26) -> u8 {
        Channel26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel27 {
    #[doc = "Channel 27 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 27 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel27 {
    #[inline(always)]
    fn from(val: u8) -> Channel27 {
        Channel27::from_bits(val)
    }
}
impl From<Channel27> for u8 {
    #[inline(always)]
    fn from(val: Channel27) -> u8 {
        Channel27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel28 {
    #[doc = "Channel 28 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 28 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel28 {
    #[inline(always)]
    fn from(val: u8) -> Channel28 {
        Channel28::from_bits(val)
    }
}
impl From<Channel28> for u8 {
    #[inline(always)]
    fn from(val: Channel28) -> u8 {
        Channel28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel29 {
    #[doc = "Channel 29 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 29 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel29 {
    #[inline(always)]
    fn from(val: u8) -> Channel29 {
        Channel29::from_bits(val)
    }
}
impl From<Channel29> for u8 {
    #[inline(always)]
    fn from(val: Channel29) -> u8 {
        Channel29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel3 {
    #[doc = "Channel 3 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 3 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel3 {
    #[inline(always)]
    fn from(val: u8) -> Channel3 {
        Channel3::from_bits(val)
    }
}
impl From<Channel3> for u8 {
    #[inline(always)]
    fn from(val: Channel3) -> u8 {
        Channel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel30 {
    #[doc = "Channel 30 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 30 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel30 {
    #[inline(always)]
    fn from(val: u8) -> Channel30 {
        Channel30::from_bits(val)
    }
}
impl From<Channel30> for u8 {
    #[inline(always)]
    fn from(val: Channel30) -> u8 {
        Channel30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel31 {
    #[doc = "Channel 31 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 31 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel31 {
    #[inline(always)]
    fn from(val: u8) -> Channel31 {
        Channel31::from_bits(val)
    }
}
impl From<Channel31> for u8 {
    #[inline(always)]
    fn from(val: Channel31) -> u8 {
        Channel31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel4 {
    #[doc = "Channel 4 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 4 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel4 {
    #[inline(always)]
    fn from(val: u8) -> Channel4 {
        Channel4::from_bits(val)
    }
}
impl From<Channel4> for u8 {
    #[inline(always)]
    fn from(val: Channel4) -> u8 {
        Channel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel5 {
    #[doc = "Channel 5 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 5 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel5 {
    #[inline(always)]
    fn from(val: u8) -> Channel5 {
        Channel5::from_bits(val)
    }
}
impl From<Channel5> for u8 {
    #[inline(always)]
    fn from(val: Channel5) -> u8 {
        Channel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel6 {
    #[doc = "Channel 6 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 6 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel6 {
    #[inline(always)]
    fn from(val: u8) -> Channel6 {
        Channel6::from_bits(val)
    }
}
impl From<Channel6> for u8 {
    #[inline(always)]
    fn from(val: Channel6) -> u8 {
        Channel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel7 {
    #[doc = "Channel 7 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 7 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel7 {
    #[inline(always)]
    fn from(val: u8) -> Channel7 {
        Channel7::from_bits(val)
    }
}
impl From<Channel7> for u8 {
    #[inline(always)]
    fn from(val: Channel7) -> u8 {
        Channel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel8 {
    #[doc = "Channel 8 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 8 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel8 {
    #[inline(always)]
    fn from(val: u8) -> Channel8 {
        Channel8::from_bits(val)
    }
}
impl From<Channel8> for u8 {
    #[inline(always)]
    fn from(val: Channel8) -> u8 {
        Channel8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel9 {
    #[doc = "Channel 9 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Channel 9 has its secure attribute set"]
    SECURE = 0x01,
}
impl Channel9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channel9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channel9 {
    #[inline(always)]
    fn from(val: u8) -> Channel9 {
        Channel9::from_bits(val)
    }
}
impl From<Channel9> for u8 {
    #[inline(always)]
    fn from(val: Channel9) -> u8 {
        Channel9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dma {
    #[doc = "Peripheral has no DMA capability"]
    NODMA = 0x0,
    #[doc = "Peripheral has DMA and DMA transfers always have the same security attribute as assigned to the peripheral"]
    NOSEPARATEATTRIBUTE = 0x01,
    #[doc = "Peripheral has DMA and DMA transfers can have a different security attribute than the one assigned to the peripheral"]
    SEPARATEATTRIBUTE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma {
    #[inline(always)]
    fn from(val: u8) -> Dma {
        Dma::from_bits(val)
    }
}
impl From<Dma> for u8 {
    #[inline(always)]
    fn from(val: Dma) -> u8 {
        Dma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dmasec {
    #[doc = "DMA transfers initiated by this peripheral have the non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "DMA transfers initiated by this peripheral have the secure attribute set"]
    SECURE = 0x01,
}
impl Dmasec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmasec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmasec {
    #[inline(always)]
    fn from(val: u8) -> Dmasec {
        Dmasec::from_bits(val)
    }
}
impl From<Dmasec> for u8 {
    #[inline(always)]
    fn from(val: Dmasec) -> u8 {
        Dmasec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DppiLockLock {
    #[doc = "DPPI\\[n\\].PERM register content can be changed"]
    UNLOCKED = 0x0,
    #[doc = "DPPI\\[n\\].PERM register can't be changed until next reset"]
    LOCKED = 0x01,
}
impl DppiLockLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DppiLockLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DppiLockLock {
    #[inline(always)]
    fn from(val: u8) -> DppiLockLock {
        DppiLockLock::from_bits(val)
    }
}
impl From<DppiLockLock> for u8 {
    #[inline(always)]
    fn from(val: DppiLockLock) -> u8 {
        DppiLockLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsFlashaccerr {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsFlashaccerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsFlashaccerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsFlashaccerr {
    #[inline(always)]
    fn from(val: u8) -> EventsFlashaccerr {
        EventsFlashaccerr::from_bits(val)
    }
}
impl From<EventsFlashaccerr> for u8 {
    #[inline(always)]
    fn from(val: EventsFlashaccerr) -> u8 {
        EventsFlashaccerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsPeriphaccerr {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsPeriphaccerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsPeriphaccerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsPeriphaccerr {
    #[inline(always)]
    fn from(val: u8) -> EventsPeriphaccerr {
        EventsPeriphaccerr::from_bits(val)
    }
}
impl From<EventsPeriphaccerr> for u8 {
    #[inline(always)]
    fn from(val: EventsPeriphaccerr) -> u8 {
        EventsPeriphaccerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsRamaccerr {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRamaccerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRamaccerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRamaccerr {
    #[inline(always)]
    fn from(val: u8) -> EventsRamaccerr {
        EventsRamaccerr::from_bits(val)
    }
}
impl From<EventsRamaccerr> for u8 {
    #[inline(always)]
    fn from(val: EventsRamaccerr) -> u8 {
        EventsRamaccerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExtdomainPermLock {
    #[doc = "This register can be updated"]
    UNLOCKED = 0x0,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED = 0x01,
}
impl ExtdomainPermLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtdomainPermLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtdomainPermLock {
    #[inline(always)]
    fn from(val: u8) -> ExtdomainPermLock {
        ExtdomainPermLock::from_bits(val)
    }
}
impl From<ExtdomainPermLock> for u8 {
    #[inline(always)]
    fn from(val: ExtdomainPermLock) -> u8 {
        ExtdomainPermLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExtdomainPermSecattr {
    #[doc = "Bus accesses from this domain have the non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Bus accesses from this domain have secure attribute set"]
    SECURE = 0x01,
}
impl ExtdomainPermSecattr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtdomainPermSecattr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtdomainPermSecattr {
    #[inline(always)]
    fn from(val: u8) -> ExtdomainPermSecattr {
        ExtdomainPermSecattr::from_bits(val)
    }
}
impl From<ExtdomainPermSecattr> for u8 {
    #[inline(always)]
    fn from(val: ExtdomainPermSecattr) -> u8 {
        ExtdomainPermSecattr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExtdomainPermSecuremapping {
    #[doc = "The bus access from this external domain always have the non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "The bus access from this external domain always have the secure attribute set"]
    SECURE = 0x01,
    #[doc = "Non-secure or secure attribute for bus access from this domain is defined by the EXTDOMAIN\\[n\\].PERM register"]
    USERSELECTABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl ExtdomainPermSecuremapping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtdomainPermSecuremapping {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtdomainPermSecuremapping {
    #[inline(always)]
    fn from(val: u8) -> ExtdomainPermSecuremapping {
        ExtdomainPermSecuremapping::from_bits(val)
    }
}
impl From<ExtdomainPermSecuremapping> for u8 {
    #[inline(always)]
    fn from(val: ExtdomainPermSecuremapping) -> u8 {
        ExtdomainPermSecuremapping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FlashnscRegionLock {
    #[doc = "This register can be updated"]
    UNLOCKED = 0x0,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED = 0x01,
}
impl FlashnscRegionLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashnscRegionLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashnscRegionLock {
    #[inline(always)]
    fn from(val: u8) -> FlashnscRegionLock {
        FlashnscRegionLock::from_bits(val)
    }
}
impl From<FlashnscRegionLock> for u8 {
    #[inline(always)]
    fn from(val: FlashnscRegionLock) -> u8 {
        FlashnscRegionLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FlashnscSizeLock {
    #[doc = "This register can be updated"]
    UNLOCKED = 0x0,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED = 0x01,
}
impl FlashnscSizeLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashnscSizeLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashnscSizeLock {
    #[inline(always)]
    fn from(val: u8) -> FlashnscSizeLock {
        FlashnscSizeLock::from_bits(val)
    }
}
impl From<FlashnscSizeLock> for u8 {
    #[inline(always)]
    fn from(val: FlashnscSizeLock) -> u8 {
        FlashnscSizeLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FlashnscSizeSize {
    #[doc = "The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    DISABLED = 0x0,
    #[doc = "The region n is defined as non-secure callable with size 32 bytes"]
    _32 = 0x01,
    #[doc = "The region n is defined as non-secure callable with size 64 bytes"]
    _64 = 0x02,
    #[doc = "The region n is defined as non-secure callable with size 128 bytes"]
    _128 = 0x03,
    #[doc = "The region n is defined as non-secure callable with size 256 bytes"]
    _256 = 0x04,
    #[doc = "The region n is defined as non-secure callable with size 512 bytes"]
    _512 = 0x05,
    #[doc = "The region n is defined as non-secure callable with size 1024 bytes"]
    _1024 = 0x06,
    #[doc = "The region n is defined as non-secure callable with size 2048 bytes"]
    _2048 = 0x07,
    #[doc = "The region n is defined as non-secure callable with size 4096 bytes"]
    _4096 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FlashnscSizeSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashnscSizeSize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashnscSizeSize {
    #[inline(always)]
    fn from(val: u8) -> FlashnscSizeSize {
        FlashnscSizeSize::from_bits(val)
    }
}
impl From<FlashnscSizeSize> for u8 {
    #[inline(always)]
    fn from(val: FlashnscSizeSize) -> u8 {
        FlashnscSizeSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FlashregionPermExecute {
    #[doc = "Block instruction fetches from flash region n"]
    DISABLE = 0x0,
    #[doc = "Allow instruction fetches from flash region n"]
    ENABLE = 0x01,
}
impl FlashregionPermExecute {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashregionPermExecute {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashregionPermExecute {
    #[inline(always)]
    fn from(val: u8) -> FlashregionPermExecute {
        FlashregionPermExecute::from_bits(val)
    }
}
impl From<FlashregionPermExecute> for u8 {
    #[inline(always)]
    fn from(val: FlashregionPermExecute) -> u8 {
        FlashregionPermExecute::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FlashregionPermLock {
    #[doc = "This register can be updated"]
    UNLOCKED = 0x0,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED = 0x01,
}
impl FlashregionPermLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashregionPermLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashregionPermLock {
    #[inline(always)]
    fn from(val: u8) -> FlashregionPermLock {
        FlashregionPermLock::from_bits(val)
    }
}
impl From<FlashregionPermLock> for u8 {
    #[inline(always)]
    fn from(val: FlashregionPermLock) -> u8 {
        FlashregionPermLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FlashregionPermRead {
    #[doc = "Block read operation from flash region n"]
    DISABLE = 0x0,
    #[doc = "Allow read operation from flash region n"]
    ENABLE = 0x01,
}
impl FlashregionPermRead {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashregionPermRead {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashregionPermRead {
    #[inline(always)]
    fn from(val: u8) -> FlashregionPermRead {
        FlashregionPermRead::from_bits(val)
    }
}
impl From<FlashregionPermRead> for u8 {
    #[inline(always)]
    fn from(val: FlashregionPermRead) -> u8 {
        FlashregionPermRead::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FlashregionPermSecattr {
    #[doc = "Flash region n security attribute is non-secure"]
    NON_SECURE = 0x0,
    #[doc = "Flash region n security attribute is secure"]
    SECURE = 0x01,
}
impl FlashregionPermSecattr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashregionPermSecattr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashregionPermSecattr {
    #[inline(always)]
    fn from(val: u8) -> FlashregionPermSecattr {
        FlashregionPermSecattr::from_bits(val)
    }
}
impl From<FlashregionPermSecattr> for u8 {
    #[inline(always)]
    fn from(val: FlashregionPermSecattr) -> u8 {
        FlashregionPermSecattr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FlashregionPermWrite {
    #[doc = "Block write operation to region n"]
    DISABLE = 0x0,
    #[doc = "Allow write operation to region n"]
    ENABLE = 0x01,
}
impl FlashregionPermWrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashregionPermWrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashregionPermWrite {
    #[inline(always)]
    fn from(val: u8) -> FlashregionPermWrite {
        FlashregionPermWrite::from_bits(val)
    }
}
impl From<FlashregionPermWrite> for u8 {
    #[inline(always)]
    fn from(val: FlashregionPermWrite) -> u8 {
        FlashregionPermWrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioportLockLock {
    #[doc = "GPIOPORT\\[n\\].PERM register content can be changed"]
    UNLOCKED = 0x0,
    #[doc = "GPIOPORT\\[n\\].PERM register can't be changed until next reset"]
    LOCKED = 0x01,
}
impl GpioportLockLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioportLockLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioportLockLock {
    #[inline(always)]
    fn from(val: u8) -> GpioportLockLock {
        GpioportLockLock::from_bits(val)
    }
}
impl From<GpioportLockLock> for u8 {
    #[inline(always)]
    fn from(val: GpioportLockLock) -> u8 {
        GpioportLockLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Locknsmpu {
    #[doc = "These registers can be updated"]
    UNLOCKED = 0x0,
    #[doc = "Disables writes to the MPU_CTRL_NS, MPU_RNR_NS, MPU_RBAR_NS, MPU_RLAR_NS, MPU_RBAR_A_NSn and MPU_RLAR_A_NSn from software or from a debug agent connected to the processor"]
    LOCKED = 0x01,
}
impl Locknsmpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Locknsmpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Locknsmpu {
    #[inline(always)]
    fn from(val: u8) -> Locknsmpu {
        Locknsmpu::from_bits(val)
    }
}
impl From<Locknsmpu> for u8 {
    #[inline(always)]
    fn from(val: Locknsmpu) -> u8 {
        Locknsmpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Locknsvtor {
    #[doc = "The address of the non-secure vector table can be updated"]
    UNLOCKED = 0x0,
    #[doc = "The address of the non-secure vector table is locked"]
    LOCKED = 0x01,
}
impl Locknsvtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Locknsvtor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Locknsvtor {
    #[inline(always)]
    fn from(val: u8) -> Locknsvtor {
        Locknsvtor::from_bits(val)
    }
}
impl From<Locknsvtor> for u8 {
    #[inline(always)]
    fn from(val: Locknsvtor) -> u8 {
        Locknsvtor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Locksau {
    #[doc = "These registers can be updated"]
    UNLOCKED = 0x0,
    #[doc = "Disables writes to the SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers from software or from a debug agent connected to the processor"]
    LOCKED = 0x01,
}
impl Locksau {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Locksau {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Locksau {
    #[inline(always)]
    fn from(val: u8) -> Locksau {
        Locksau::from_bits(val)
    }
}
impl From<Locksau> for u8 {
    #[inline(always)]
    fn from(val: Locksau) -> u8 {
        Locksau::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Locksmpu {
    #[doc = "These registers can be updated"]
    UNLOCKED = 0x0,
    #[doc = "Disables writes to the MPU_CTRL, MPU_RNR, MPU_RBAR, MPU_RLAR, MPU_RBAR_An and MPU_RLAR_An from software or from a debug agent connected to the processor in Secure state"]
    LOCKED = 0x01,
}
impl Locksmpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Locksmpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Locksmpu {
    #[inline(always)]
    fn from(val: u8) -> Locksmpu {
        Locksmpu::from_bits(val)
    }
}
impl From<Locksmpu> for u8 {
    #[inline(always)]
    fn from(val: Locksmpu) -> u8 {
        Locksmpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Locksvtaircr {
    #[doc = "These registers can be updated"]
    UNLOCKED = 0x0,
    #[doc = "Disables writes to the VTOR_S, AIRCR.PRIS, and AIRCR.BFHFNMINS registers"]
    LOCKED = 0x01,
}
impl Locksvtaircr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Locksvtaircr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Locksvtaircr {
    #[inline(always)]
    fn from(val: u8) -> Locksvtaircr {
        Locksvtaircr::from_bits(val)
    }
}
impl From<Locksvtaircr> for u8 {
    #[inline(always)]
    fn from(val: Locksvtaircr) -> u8 {
        Locksvtaircr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PeriphidPermLock {
    #[doc = "This register can be updated"]
    UNLOCKED = 0x0,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED = 0x01,
}
impl PeriphidPermLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphidPermLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphidPermLock {
    #[inline(always)]
    fn from(val: u8) -> PeriphidPermLock {
        PeriphidPermLock::from_bits(val)
    }
}
impl From<PeriphidPermLock> for u8 {
    #[inline(always)]
    fn from(val: PeriphidPermLock) -> u8 {
        PeriphidPermLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PeriphidPermSecattr {
    #[doc = "If SECUREMAPPING == UserSelectable: Peripheral is mapped in non-secure peripheral address space. If SECUREMAPPING == Split: Peripheral is mapped in non-secure and secure peripheral address space."]
    NONSECURE = 0x0,
    #[doc = "Peripheral is mapped in secure peripheral address space"]
    SECURE = 0x01,
}
impl PeriphidPermSecattr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphidPermSecattr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphidPermSecattr {
    #[inline(always)]
    fn from(val: u8) -> PeriphidPermSecattr {
        PeriphidPermSecattr::from_bits(val)
    }
}
impl From<PeriphidPermSecattr> for u8 {
    #[inline(always)]
    fn from(val: PeriphidPermSecattr) -> u8 {
        PeriphidPermSecattr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PeriphidPermSecuremapping {
    #[doc = "This peripheral is always accessible as a non-secure peripheral"]
    NONSECURE = 0x0,
    #[doc = "This peripheral is always accessible as a secure peripheral"]
    SECURE = 0x01,
    #[doc = "Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register"]
    USERSELECTABLE = 0x02,
    #[doc = "This peripheral implements the split security mechanism. Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register."]
    SPLIT = 0x03,
}
impl PeriphidPermSecuremapping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphidPermSecuremapping {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphidPermSecuremapping {
    #[inline(always)]
    fn from(val: u8) -> PeriphidPermSecuremapping {
        PeriphidPermSecuremapping::from_bits(val)
    }
}
impl From<PeriphidPermSecuremapping> for u8 {
    #[inline(always)]
    fn from(val: PeriphidPermSecuremapping) -> u8 {
        PeriphidPermSecuremapping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin0 {
    #[doc = "Pin 0 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 0 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin0 {
    #[inline(always)]
    fn from(val: u8) -> Pin0 {
        Pin0::from_bits(val)
    }
}
impl From<Pin0> for u8 {
    #[inline(always)]
    fn from(val: Pin0) -> u8 {
        Pin0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin1 {
    #[doc = "Pin 1 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 1 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin1 {
    #[inline(always)]
    fn from(val: u8) -> Pin1 {
        Pin1::from_bits(val)
    }
}
impl From<Pin1> for u8 {
    #[inline(always)]
    fn from(val: Pin1) -> u8 {
        Pin1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin10 {
    #[doc = "Pin 10 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 10 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin10 {
    #[inline(always)]
    fn from(val: u8) -> Pin10 {
        Pin10::from_bits(val)
    }
}
impl From<Pin10> for u8 {
    #[inline(always)]
    fn from(val: Pin10) -> u8 {
        Pin10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin11 {
    #[doc = "Pin 11 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 11 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin11 {
    #[inline(always)]
    fn from(val: u8) -> Pin11 {
        Pin11::from_bits(val)
    }
}
impl From<Pin11> for u8 {
    #[inline(always)]
    fn from(val: Pin11) -> u8 {
        Pin11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin12 {
    #[doc = "Pin 12 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 12 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin12 {
    #[inline(always)]
    fn from(val: u8) -> Pin12 {
        Pin12::from_bits(val)
    }
}
impl From<Pin12> for u8 {
    #[inline(always)]
    fn from(val: Pin12) -> u8 {
        Pin12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin13 {
    #[doc = "Pin 13 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 13 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin13 {
    #[inline(always)]
    fn from(val: u8) -> Pin13 {
        Pin13::from_bits(val)
    }
}
impl From<Pin13> for u8 {
    #[inline(always)]
    fn from(val: Pin13) -> u8 {
        Pin13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin14 {
    #[doc = "Pin 14 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 14 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin14 {
    #[inline(always)]
    fn from(val: u8) -> Pin14 {
        Pin14::from_bits(val)
    }
}
impl From<Pin14> for u8 {
    #[inline(always)]
    fn from(val: Pin14) -> u8 {
        Pin14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin15 {
    #[doc = "Pin 15 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 15 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin15 {
    #[inline(always)]
    fn from(val: u8) -> Pin15 {
        Pin15::from_bits(val)
    }
}
impl From<Pin15> for u8 {
    #[inline(always)]
    fn from(val: Pin15) -> u8 {
        Pin15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin16 {
    #[doc = "Pin 16 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 16 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin16 {
    #[inline(always)]
    fn from(val: u8) -> Pin16 {
        Pin16::from_bits(val)
    }
}
impl From<Pin16> for u8 {
    #[inline(always)]
    fn from(val: Pin16) -> u8 {
        Pin16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin17 {
    #[doc = "Pin 17 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 17 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin17 {
    #[inline(always)]
    fn from(val: u8) -> Pin17 {
        Pin17::from_bits(val)
    }
}
impl From<Pin17> for u8 {
    #[inline(always)]
    fn from(val: Pin17) -> u8 {
        Pin17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin18 {
    #[doc = "Pin 18 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 18 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin18 {
    #[inline(always)]
    fn from(val: u8) -> Pin18 {
        Pin18::from_bits(val)
    }
}
impl From<Pin18> for u8 {
    #[inline(always)]
    fn from(val: Pin18) -> u8 {
        Pin18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin19 {
    #[doc = "Pin 19 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 19 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin19 {
    #[inline(always)]
    fn from(val: u8) -> Pin19 {
        Pin19::from_bits(val)
    }
}
impl From<Pin19> for u8 {
    #[inline(always)]
    fn from(val: Pin19) -> u8 {
        Pin19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin2 {
    #[doc = "Pin 2 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 2 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin2 {
    #[inline(always)]
    fn from(val: u8) -> Pin2 {
        Pin2::from_bits(val)
    }
}
impl From<Pin2> for u8 {
    #[inline(always)]
    fn from(val: Pin2) -> u8 {
        Pin2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin20 {
    #[doc = "Pin 20 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 20 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin20 {
    #[inline(always)]
    fn from(val: u8) -> Pin20 {
        Pin20::from_bits(val)
    }
}
impl From<Pin20> for u8 {
    #[inline(always)]
    fn from(val: Pin20) -> u8 {
        Pin20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin21 {
    #[doc = "Pin 21 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 21 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin21 {
    #[inline(always)]
    fn from(val: u8) -> Pin21 {
        Pin21::from_bits(val)
    }
}
impl From<Pin21> for u8 {
    #[inline(always)]
    fn from(val: Pin21) -> u8 {
        Pin21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin22 {
    #[doc = "Pin 22 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 22 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin22 {
    #[inline(always)]
    fn from(val: u8) -> Pin22 {
        Pin22::from_bits(val)
    }
}
impl From<Pin22> for u8 {
    #[inline(always)]
    fn from(val: Pin22) -> u8 {
        Pin22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin23 {
    #[doc = "Pin 23 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 23 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin23 {
    #[inline(always)]
    fn from(val: u8) -> Pin23 {
        Pin23::from_bits(val)
    }
}
impl From<Pin23> for u8 {
    #[inline(always)]
    fn from(val: Pin23) -> u8 {
        Pin23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin24 {
    #[doc = "Pin 24 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 24 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin24 {
    #[inline(always)]
    fn from(val: u8) -> Pin24 {
        Pin24::from_bits(val)
    }
}
impl From<Pin24> for u8 {
    #[inline(always)]
    fn from(val: Pin24) -> u8 {
        Pin24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin25 {
    #[doc = "Pin 25 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 25 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin25 {
    #[inline(always)]
    fn from(val: u8) -> Pin25 {
        Pin25::from_bits(val)
    }
}
impl From<Pin25> for u8 {
    #[inline(always)]
    fn from(val: Pin25) -> u8 {
        Pin25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin26 {
    #[doc = "Pin 26 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 26 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin26 {
    #[inline(always)]
    fn from(val: u8) -> Pin26 {
        Pin26::from_bits(val)
    }
}
impl From<Pin26> for u8 {
    #[inline(always)]
    fn from(val: Pin26) -> u8 {
        Pin26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin27 {
    #[doc = "Pin 27 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 27 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin27 {
    #[inline(always)]
    fn from(val: u8) -> Pin27 {
        Pin27::from_bits(val)
    }
}
impl From<Pin27> for u8 {
    #[inline(always)]
    fn from(val: Pin27) -> u8 {
        Pin27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin28 {
    #[doc = "Pin 28 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 28 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin28 {
    #[inline(always)]
    fn from(val: u8) -> Pin28 {
        Pin28::from_bits(val)
    }
}
impl From<Pin28> for u8 {
    #[inline(always)]
    fn from(val: Pin28) -> u8 {
        Pin28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin29 {
    #[doc = "Pin 29 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 29 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin29 {
    #[inline(always)]
    fn from(val: u8) -> Pin29 {
        Pin29::from_bits(val)
    }
}
impl From<Pin29> for u8 {
    #[inline(always)]
    fn from(val: Pin29) -> u8 {
        Pin29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin3 {
    #[doc = "Pin 3 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 3 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin3 {
    #[inline(always)]
    fn from(val: u8) -> Pin3 {
        Pin3::from_bits(val)
    }
}
impl From<Pin3> for u8 {
    #[inline(always)]
    fn from(val: Pin3) -> u8 {
        Pin3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin30 {
    #[doc = "Pin 30 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 30 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin30 {
    #[inline(always)]
    fn from(val: u8) -> Pin30 {
        Pin30::from_bits(val)
    }
}
impl From<Pin30> for u8 {
    #[inline(always)]
    fn from(val: Pin30) -> u8 {
        Pin30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin31 {
    #[doc = "Pin 31 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 31 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin31 {
    #[inline(always)]
    fn from(val: u8) -> Pin31 {
        Pin31::from_bits(val)
    }
}
impl From<Pin31> for u8 {
    #[inline(always)]
    fn from(val: Pin31) -> u8 {
        Pin31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin4 {
    #[doc = "Pin 4 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 4 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin4 {
    #[inline(always)]
    fn from(val: u8) -> Pin4 {
        Pin4::from_bits(val)
    }
}
impl From<Pin4> for u8 {
    #[inline(always)]
    fn from(val: Pin4) -> u8 {
        Pin4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin5 {
    #[doc = "Pin 5 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 5 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin5 {
    #[inline(always)]
    fn from(val: u8) -> Pin5 {
        Pin5::from_bits(val)
    }
}
impl From<Pin5> for u8 {
    #[inline(always)]
    fn from(val: Pin5) -> u8 {
        Pin5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin6 {
    #[doc = "Pin 6 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 6 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin6 {
    #[inline(always)]
    fn from(val: u8) -> Pin6 {
        Pin6::from_bits(val)
    }
}
impl From<Pin6> for u8 {
    #[inline(always)]
    fn from(val: Pin6) -> u8 {
        Pin6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin7 {
    #[doc = "Pin 7 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 7 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin7 {
    #[inline(always)]
    fn from(val: u8) -> Pin7 {
        Pin7::from_bits(val)
    }
}
impl From<Pin7> for u8 {
    #[inline(always)]
    fn from(val: Pin7) -> u8 {
        Pin7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin8 {
    #[doc = "Pin 8 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 8 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin8 {
    #[inline(always)]
    fn from(val: u8) -> Pin8 {
        Pin8::from_bits(val)
    }
}
impl From<Pin8> for u8 {
    #[inline(always)]
    fn from(val: Pin8) -> u8 {
        Pin8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin9 {
    #[doc = "Pin 9 has its non-secure attribute set"]
    NONSECURE = 0x0,
    #[doc = "Pin 9 has its secure attribute set"]
    SECURE = 0x01,
}
impl Pin9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pin9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pin9 {
    #[inline(always)]
    fn from(val: u8) -> Pin9 {
        Pin9::from_bits(val)
    }
}
impl From<Pin9> for u8 {
    #[inline(always)]
    fn from(val: Pin9) -> u8 {
        Pin9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Present {
    #[doc = "Peripheral is not present"]
    NOTPRESENT = 0x0,
    #[doc = "Peripheral is present"]
    ISPRESENT = 0x01,
}
impl Present {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Present {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Present {
    #[inline(always)]
    fn from(val: u8) -> Present {
        Present::from_bits(val)
    }
}
impl From<Present> for u8 {
    #[inline(always)]
    fn from(val: Present) -> u8 {
        Present::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishFlashaccerrEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishFlashaccerrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishFlashaccerrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishFlashaccerrEn {
    #[inline(always)]
    fn from(val: u8) -> PublishFlashaccerrEn {
        PublishFlashaccerrEn::from_bits(val)
    }
}
impl From<PublishFlashaccerrEn> for u8 {
    #[inline(always)]
    fn from(val: PublishFlashaccerrEn) -> u8 {
        PublishFlashaccerrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishPeriphaccerrEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishPeriphaccerrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishPeriphaccerrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishPeriphaccerrEn {
    #[inline(always)]
    fn from(val: u8) -> PublishPeriphaccerrEn {
        PublishPeriphaccerrEn::from_bits(val)
    }
}
impl From<PublishPeriphaccerrEn> for u8 {
    #[inline(always)]
    fn from(val: PublishPeriphaccerrEn) -> u8 {
        PublishPeriphaccerrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishRamaccerrEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishRamaccerrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishRamaccerrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishRamaccerrEn {
    #[inline(always)]
    fn from(val: u8) -> PublishRamaccerrEn {
        PublishRamaccerrEn::from_bits(val)
    }
}
impl From<PublishRamaccerrEn> for u8 {
    #[inline(always)]
    fn from(val: PublishRamaccerrEn) -> u8 {
        PublishRamaccerrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RamnscRegionLock {
    #[doc = "This register can be updated"]
    UNLOCKED = 0x0,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED = 0x01,
}
impl RamnscRegionLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamnscRegionLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamnscRegionLock {
    #[inline(always)]
    fn from(val: u8) -> RamnscRegionLock {
        RamnscRegionLock::from_bits(val)
    }
}
impl From<RamnscRegionLock> for u8 {
    #[inline(always)]
    fn from(val: RamnscRegionLock) -> u8 {
        RamnscRegionLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RamnscSizeLock {
    #[doc = "This register can be updated"]
    UNLOCKED = 0x0,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED = 0x01,
}
impl RamnscSizeLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamnscSizeLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamnscSizeLock {
    #[inline(always)]
    fn from(val: u8) -> RamnscSizeLock {
        RamnscSizeLock::from_bits(val)
    }
}
impl From<RamnscSizeLock> for u8 {
    #[inline(always)]
    fn from(val: RamnscSizeLock) -> u8 {
        RamnscSizeLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RamnscSizeSize {
    #[doc = "The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    DISABLED = 0x0,
    #[doc = "The region n is defined as non-secure callable with size 32 bytes"]
    _32 = 0x01,
    #[doc = "The region n is defined as non-secure callable with size 64 bytes"]
    _64 = 0x02,
    #[doc = "The region n is defined as non-secure callable with size 128 bytes"]
    _128 = 0x03,
    #[doc = "The region n is defined as non-secure callable with size 256 bytes"]
    _256 = 0x04,
    #[doc = "The region n is defined as non-secure callable with size 512 bytes"]
    _512 = 0x05,
    #[doc = "The region n is defined as non-secure callable with size 1024 bytes"]
    _1024 = 0x06,
    #[doc = "The region n is defined as non-secure callable with size 2048 bytes"]
    _2048 = 0x07,
    #[doc = "The region n is defined as non-secure callable with size 4096 bytes"]
    _4096 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl RamnscSizeSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamnscSizeSize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamnscSizeSize {
    #[inline(always)]
    fn from(val: u8) -> RamnscSizeSize {
        RamnscSizeSize::from_bits(val)
    }
}
impl From<RamnscSizeSize> for u8 {
    #[inline(always)]
    fn from(val: RamnscSizeSize) -> u8 {
        RamnscSizeSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RamregionPermExecute {
    #[doc = "Block instruction fetches from RAM region n"]
    DISABLE = 0x0,
    #[doc = "Allow instruction fetches from RAM region n"]
    ENABLE = 0x01,
}
impl RamregionPermExecute {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamregionPermExecute {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamregionPermExecute {
    #[inline(always)]
    fn from(val: u8) -> RamregionPermExecute {
        RamregionPermExecute::from_bits(val)
    }
}
impl From<RamregionPermExecute> for u8 {
    #[inline(always)]
    fn from(val: RamregionPermExecute) -> u8 {
        RamregionPermExecute::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RamregionPermLock {
    #[doc = "This register can be updated"]
    UNLOCKED = 0x0,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED = 0x01,
}
impl RamregionPermLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamregionPermLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamregionPermLock {
    #[inline(always)]
    fn from(val: u8) -> RamregionPermLock {
        RamregionPermLock::from_bits(val)
    }
}
impl From<RamregionPermLock> for u8 {
    #[inline(always)]
    fn from(val: RamregionPermLock) -> u8 {
        RamregionPermLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RamregionPermRead {
    #[doc = "Block read operation from RAM region n"]
    DISABLE = 0x0,
    #[doc = "Allow read operation from RAM region n"]
    ENABLE = 0x01,
}
impl RamregionPermRead {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamregionPermRead {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamregionPermRead {
    #[inline(always)]
    fn from(val: u8) -> RamregionPermRead {
        RamregionPermRead::from_bits(val)
    }
}
impl From<RamregionPermRead> for u8 {
    #[inline(always)]
    fn from(val: RamregionPermRead) -> u8 {
        RamregionPermRead::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RamregionPermSecattr {
    #[doc = "RAM region n security attribute is non-secure"]
    NON_SECURE = 0x0,
    #[doc = "RAM region n security attribute is secure"]
    SECURE = 0x01,
}
impl RamregionPermSecattr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamregionPermSecattr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamregionPermSecattr {
    #[inline(always)]
    fn from(val: u8) -> RamregionPermSecattr {
        RamregionPermSecattr::from_bits(val)
    }
}
impl From<RamregionPermSecattr> for u8 {
    #[inline(always)]
    fn from(val: RamregionPermSecattr) -> u8 {
        RamregionPermSecattr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RamregionPermWrite {
    #[doc = "Block write operation to RAM region n"]
    DISABLE = 0x0,
    #[doc = "Allow write operation to RAM region n"]
    ENABLE = 0x01,
}
impl RamregionPermWrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamregionPermWrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamregionPermWrite {
    #[inline(always)]
    fn from(val: u8) -> RamregionPermWrite {
        RamregionPermWrite::from_bits(val)
    }
}
impl From<RamregionPermWrite> for u8 {
    #[inline(always)]
    fn from(val: RamregionPermWrite) -> u8 {
        RamregionPermWrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tzm {
    #[doc = "Arm TrustZone support not available"]
    NOTAVAILABLE = 0x0,
    #[doc = "Arm TrustZone support is available"]
    ENABLED = 0x01,
}
impl Tzm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tzm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tzm {
    #[inline(always)]
    fn from(val: u8) -> Tzm {
        Tzm::from_bits(val)
    }
}
impl From<Tzm> for u8 {
    #[inline(always)]
    fn from(val: Tzm) -> u8 {
        Tzm::to_bits(val)
    }
}
