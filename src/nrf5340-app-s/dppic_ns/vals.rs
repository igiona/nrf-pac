#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh0 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh0 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh0 {
        ChenCh0::from_bits(val)
    }
}
impl From<ChenCh0> for u8 {
    #[inline(always)]
    fn from(val: ChenCh0) -> u8 {
        ChenCh0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh1 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh1 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh1 {
        ChenCh1::from_bits(val)
    }
}
impl From<ChenCh1> for u8 {
    #[inline(always)]
    fn from(val: ChenCh1) -> u8 {
        ChenCh1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh10 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh10 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh10 {
        ChenCh10::from_bits(val)
    }
}
impl From<ChenCh10> for u8 {
    #[inline(always)]
    fn from(val: ChenCh10) -> u8 {
        ChenCh10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh11 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh11 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh11 {
        ChenCh11::from_bits(val)
    }
}
impl From<ChenCh11> for u8 {
    #[inline(always)]
    fn from(val: ChenCh11) -> u8 {
        ChenCh11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh12 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh12 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh12 {
        ChenCh12::from_bits(val)
    }
}
impl From<ChenCh12> for u8 {
    #[inline(always)]
    fn from(val: ChenCh12) -> u8 {
        ChenCh12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh13 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh13 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh13 {
        ChenCh13::from_bits(val)
    }
}
impl From<ChenCh13> for u8 {
    #[inline(always)]
    fn from(val: ChenCh13) -> u8 {
        ChenCh13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh14 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh14 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh14 {
        ChenCh14::from_bits(val)
    }
}
impl From<ChenCh14> for u8 {
    #[inline(always)]
    fn from(val: ChenCh14) -> u8 {
        ChenCh14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh15 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh15 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh15 {
        ChenCh15::from_bits(val)
    }
}
impl From<ChenCh15> for u8 {
    #[inline(always)]
    fn from(val: ChenCh15) -> u8 {
        ChenCh15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh16 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh16 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh16 {
        ChenCh16::from_bits(val)
    }
}
impl From<ChenCh16> for u8 {
    #[inline(always)]
    fn from(val: ChenCh16) -> u8 {
        ChenCh16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh17 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh17 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh17 {
        ChenCh17::from_bits(val)
    }
}
impl From<ChenCh17> for u8 {
    #[inline(always)]
    fn from(val: ChenCh17) -> u8 {
        ChenCh17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh18 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh18 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh18 {
        ChenCh18::from_bits(val)
    }
}
impl From<ChenCh18> for u8 {
    #[inline(always)]
    fn from(val: ChenCh18) -> u8 {
        ChenCh18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh19 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh19 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh19 {
        ChenCh19::from_bits(val)
    }
}
impl From<ChenCh19> for u8 {
    #[inline(always)]
    fn from(val: ChenCh19) -> u8 {
        ChenCh19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh2 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh2 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh2 {
        ChenCh2::from_bits(val)
    }
}
impl From<ChenCh2> for u8 {
    #[inline(always)]
    fn from(val: ChenCh2) -> u8 {
        ChenCh2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh20 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh20 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh20 {
        ChenCh20::from_bits(val)
    }
}
impl From<ChenCh20> for u8 {
    #[inline(always)]
    fn from(val: ChenCh20) -> u8 {
        ChenCh20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh21 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh21 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh21 {
        ChenCh21::from_bits(val)
    }
}
impl From<ChenCh21> for u8 {
    #[inline(always)]
    fn from(val: ChenCh21) -> u8 {
        ChenCh21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh22 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh22 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh22 {
        ChenCh22::from_bits(val)
    }
}
impl From<ChenCh22> for u8 {
    #[inline(always)]
    fn from(val: ChenCh22) -> u8 {
        ChenCh22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh23 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh23 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh23 {
        ChenCh23::from_bits(val)
    }
}
impl From<ChenCh23> for u8 {
    #[inline(always)]
    fn from(val: ChenCh23) -> u8 {
        ChenCh23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh24 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh24 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh24 {
        ChenCh24::from_bits(val)
    }
}
impl From<ChenCh24> for u8 {
    #[inline(always)]
    fn from(val: ChenCh24) -> u8 {
        ChenCh24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh25 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh25 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh25 {
        ChenCh25::from_bits(val)
    }
}
impl From<ChenCh25> for u8 {
    #[inline(always)]
    fn from(val: ChenCh25) -> u8 {
        ChenCh25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh26 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh26 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh26 {
        ChenCh26::from_bits(val)
    }
}
impl From<ChenCh26> for u8 {
    #[inline(always)]
    fn from(val: ChenCh26) -> u8 {
        ChenCh26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh27 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh27 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh27 {
        ChenCh27::from_bits(val)
    }
}
impl From<ChenCh27> for u8 {
    #[inline(always)]
    fn from(val: ChenCh27) -> u8 {
        ChenCh27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh28 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh28 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh28 {
        ChenCh28::from_bits(val)
    }
}
impl From<ChenCh28> for u8 {
    #[inline(always)]
    fn from(val: ChenCh28) -> u8 {
        ChenCh28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh29 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh29 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh29 {
        ChenCh29::from_bits(val)
    }
}
impl From<ChenCh29> for u8 {
    #[inline(always)]
    fn from(val: ChenCh29) -> u8 {
        ChenCh29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh3 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh3 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh3 {
        ChenCh3::from_bits(val)
    }
}
impl From<ChenCh3> for u8 {
    #[inline(always)]
    fn from(val: ChenCh3) -> u8 {
        ChenCh3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh30 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh30 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh30 {
        ChenCh30::from_bits(val)
    }
}
impl From<ChenCh30> for u8 {
    #[inline(always)]
    fn from(val: ChenCh30) -> u8 {
        ChenCh30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh31 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh31 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh31 {
        ChenCh31::from_bits(val)
    }
}
impl From<ChenCh31> for u8 {
    #[inline(always)]
    fn from(val: ChenCh31) -> u8 {
        ChenCh31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh4 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh4 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh4 {
        ChenCh4::from_bits(val)
    }
}
impl From<ChenCh4> for u8 {
    #[inline(always)]
    fn from(val: ChenCh4) -> u8 {
        ChenCh4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh5 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh5 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh5 {
        ChenCh5::from_bits(val)
    }
}
impl From<ChenCh5> for u8 {
    #[inline(always)]
    fn from(val: ChenCh5) -> u8 {
        ChenCh5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh6 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh6 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh6 {
        ChenCh6::from_bits(val)
    }
}
impl From<ChenCh6> for u8 {
    #[inline(always)]
    fn from(val: ChenCh6) -> u8 {
        ChenCh6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh7 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh7 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh7 {
        ChenCh7::from_bits(val)
    }
}
impl From<ChenCh7> for u8 {
    #[inline(always)]
    fn from(val: ChenCh7) -> u8 {
        ChenCh7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh8 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh8 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh8 {
        ChenCh8::from_bits(val)
    }
}
impl From<ChenCh8> for u8 {
    #[inline(always)]
    fn from(val: ChenCh8) -> u8 {
        ChenCh8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenCh9 {
    #[doc = "Disable channel"]
    DISABLED = 0x0,
    #[doc = "Enable channel"]
    ENABLED = 0x01,
}
impl ChenCh9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenCh9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenCh9 {
    #[inline(always)]
    fn from(val: u8) -> ChenCh9 {
        ChenCh9::from_bits(val)
    }
}
impl From<ChenCh9> for u8 {
    #[inline(always)]
    fn from(val: ChenCh9) -> u8 {
        ChenCh9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh0 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh0 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh0 {
        ChenclrCh0::from_bits(val)
    }
}
impl From<ChenclrCh0> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh0) -> u8 {
        ChenclrCh0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh1 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh1 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh1 {
        ChenclrCh1::from_bits(val)
    }
}
impl From<ChenclrCh1> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh1) -> u8 {
        ChenclrCh1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh10 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh10 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh10 {
        ChenclrCh10::from_bits(val)
    }
}
impl From<ChenclrCh10> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh10) -> u8 {
        ChenclrCh10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh11 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh11 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh11 {
        ChenclrCh11::from_bits(val)
    }
}
impl From<ChenclrCh11> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh11) -> u8 {
        ChenclrCh11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh12 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh12 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh12 {
        ChenclrCh12::from_bits(val)
    }
}
impl From<ChenclrCh12> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh12) -> u8 {
        ChenclrCh12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh13 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh13 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh13 {
        ChenclrCh13::from_bits(val)
    }
}
impl From<ChenclrCh13> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh13) -> u8 {
        ChenclrCh13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh14 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh14 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh14 {
        ChenclrCh14::from_bits(val)
    }
}
impl From<ChenclrCh14> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh14) -> u8 {
        ChenclrCh14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh15 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh15 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh15 {
        ChenclrCh15::from_bits(val)
    }
}
impl From<ChenclrCh15> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh15) -> u8 {
        ChenclrCh15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh16 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh16 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh16 {
        ChenclrCh16::from_bits(val)
    }
}
impl From<ChenclrCh16> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh16) -> u8 {
        ChenclrCh16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh17 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh17 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh17 {
        ChenclrCh17::from_bits(val)
    }
}
impl From<ChenclrCh17> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh17) -> u8 {
        ChenclrCh17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh18 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh18 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh18 {
        ChenclrCh18::from_bits(val)
    }
}
impl From<ChenclrCh18> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh18) -> u8 {
        ChenclrCh18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh19 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh19 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh19 {
        ChenclrCh19::from_bits(val)
    }
}
impl From<ChenclrCh19> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh19) -> u8 {
        ChenclrCh19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh2 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh2 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh2 {
        ChenclrCh2::from_bits(val)
    }
}
impl From<ChenclrCh2> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh2) -> u8 {
        ChenclrCh2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh20 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh20 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh20 {
        ChenclrCh20::from_bits(val)
    }
}
impl From<ChenclrCh20> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh20) -> u8 {
        ChenclrCh20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh21 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh21 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh21 {
        ChenclrCh21::from_bits(val)
    }
}
impl From<ChenclrCh21> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh21) -> u8 {
        ChenclrCh21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh22 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh22 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh22 {
        ChenclrCh22::from_bits(val)
    }
}
impl From<ChenclrCh22> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh22) -> u8 {
        ChenclrCh22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh23 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh23 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh23 {
        ChenclrCh23::from_bits(val)
    }
}
impl From<ChenclrCh23> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh23) -> u8 {
        ChenclrCh23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh24 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh24 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh24 {
        ChenclrCh24::from_bits(val)
    }
}
impl From<ChenclrCh24> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh24) -> u8 {
        ChenclrCh24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh25 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh25 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh25 {
        ChenclrCh25::from_bits(val)
    }
}
impl From<ChenclrCh25> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh25) -> u8 {
        ChenclrCh25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh26 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh26 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh26 {
        ChenclrCh26::from_bits(val)
    }
}
impl From<ChenclrCh26> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh26) -> u8 {
        ChenclrCh26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh27 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh27 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh27 {
        ChenclrCh27::from_bits(val)
    }
}
impl From<ChenclrCh27> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh27) -> u8 {
        ChenclrCh27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh28 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh28 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh28 {
        ChenclrCh28::from_bits(val)
    }
}
impl From<ChenclrCh28> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh28) -> u8 {
        ChenclrCh28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh29 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh29 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh29 {
        ChenclrCh29::from_bits(val)
    }
}
impl From<ChenclrCh29> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh29) -> u8 {
        ChenclrCh29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh3 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh3 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh3 {
        ChenclrCh3::from_bits(val)
    }
}
impl From<ChenclrCh3> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh3) -> u8 {
        ChenclrCh3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh30 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh30 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh30 {
        ChenclrCh30::from_bits(val)
    }
}
impl From<ChenclrCh30> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh30) -> u8 {
        ChenclrCh30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh31 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh31 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh31 {
        ChenclrCh31::from_bits(val)
    }
}
impl From<ChenclrCh31> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh31) -> u8 {
        ChenclrCh31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh4 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh4 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh4 {
        ChenclrCh4::from_bits(val)
    }
}
impl From<ChenclrCh4> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh4) -> u8 {
        ChenclrCh4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh5 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh5 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh5 {
        ChenclrCh5::from_bits(val)
    }
}
impl From<ChenclrCh5> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh5) -> u8 {
        ChenclrCh5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh6 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh6 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh6 {
        ChenclrCh6::from_bits(val)
    }
}
impl From<ChenclrCh6> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh6) -> u8 {
        ChenclrCh6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh7 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh7 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh7 {
        ChenclrCh7::from_bits(val)
    }
}
impl From<ChenclrCh7> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh7) -> u8 {
        ChenclrCh7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh8 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh8 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh8 {
        ChenclrCh8::from_bits(val)
    }
}
impl From<ChenclrCh8> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh8) -> u8 {
        ChenclrCh8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChenclrCh9 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl ChenclrCh9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChenclrCh9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChenclrCh9 {
    #[inline(always)]
    fn from(val: u8) -> ChenclrCh9 {
        ChenclrCh9::from_bits(val)
    }
}
impl From<ChenclrCh9> for u8 {
    #[inline(always)]
    fn from(val: ChenclrCh9) -> u8 {
        ChenclrCh9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh0 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh0 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh0 {
        ChensetCh0::from_bits(val)
    }
}
impl From<ChensetCh0> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh0) -> u8 {
        ChensetCh0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh1 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh1 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh1 {
        ChensetCh1::from_bits(val)
    }
}
impl From<ChensetCh1> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh1) -> u8 {
        ChensetCh1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh10 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh10 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh10 {
        ChensetCh10::from_bits(val)
    }
}
impl From<ChensetCh10> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh10) -> u8 {
        ChensetCh10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh11 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh11 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh11 {
        ChensetCh11::from_bits(val)
    }
}
impl From<ChensetCh11> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh11) -> u8 {
        ChensetCh11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh12 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh12 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh12 {
        ChensetCh12::from_bits(val)
    }
}
impl From<ChensetCh12> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh12) -> u8 {
        ChensetCh12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh13 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh13 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh13 {
        ChensetCh13::from_bits(val)
    }
}
impl From<ChensetCh13> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh13) -> u8 {
        ChensetCh13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh14 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh14 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh14 {
        ChensetCh14::from_bits(val)
    }
}
impl From<ChensetCh14> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh14) -> u8 {
        ChensetCh14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh15 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh15 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh15 {
        ChensetCh15::from_bits(val)
    }
}
impl From<ChensetCh15> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh15) -> u8 {
        ChensetCh15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh16 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh16 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh16 {
        ChensetCh16::from_bits(val)
    }
}
impl From<ChensetCh16> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh16) -> u8 {
        ChensetCh16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh17 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh17 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh17 {
        ChensetCh17::from_bits(val)
    }
}
impl From<ChensetCh17> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh17) -> u8 {
        ChensetCh17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh18 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh18 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh18 {
        ChensetCh18::from_bits(val)
    }
}
impl From<ChensetCh18> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh18) -> u8 {
        ChensetCh18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh19 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh19 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh19 {
        ChensetCh19::from_bits(val)
    }
}
impl From<ChensetCh19> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh19) -> u8 {
        ChensetCh19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh2 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh2 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh2 {
        ChensetCh2::from_bits(val)
    }
}
impl From<ChensetCh2> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh2) -> u8 {
        ChensetCh2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh20 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh20 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh20 {
        ChensetCh20::from_bits(val)
    }
}
impl From<ChensetCh20> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh20) -> u8 {
        ChensetCh20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh21 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh21 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh21 {
        ChensetCh21::from_bits(val)
    }
}
impl From<ChensetCh21> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh21) -> u8 {
        ChensetCh21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh22 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh22 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh22 {
        ChensetCh22::from_bits(val)
    }
}
impl From<ChensetCh22> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh22) -> u8 {
        ChensetCh22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh23 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh23 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh23 {
        ChensetCh23::from_bits(val)
    }
}
impl From<ChensetCh23> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh23) -> u8 {
        ChensetCh23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh24 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh24 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh24 {
        ChensetCh24::from_bits(val)
    }
}
impl From<ChensetCh24> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh24) -> u8 {
        ChensetCh24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh25 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh25 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh25 {
        ChensetCh25::from_bits(val)
    }
}
impl From<ChensetCh25> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh25) -> u8 {
        ChensetCh25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh26 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh26 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh26 {
        ChensetCh26::from_bits(val)
    }
}
impl From<ChensetCh26> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh26) -> u8 {
        ChensetCh26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh27 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh27 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh27 {
        ChensetCh27::from_bits(val)
    }
}
impl From<ChensetCh27> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh27) -> u8 {
        ChensetCh27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh28 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh28 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh28 {
        ChensetCh28::from_bits(val)
    }
}
impl From<ChensetCh28> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh28) -> u8 {
        ChensetCh28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh29 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh29 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh29 {
        ChensetCh29::from_bits(val)
    }
}
impl From<ChensetCh29> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh29) -> u8 {
        ChensetCh29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh3 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh3 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh3 {
        ChensetCh3::from_bits(val)
    }
}
impl From<ChensetCh3> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh3) -> u8 {
        ChensetCh3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh30 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh30 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh30 {
        ChensetCh30::from_bits(val)
    }
}
impl From<ChensetCh30> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh30) -> u8 {
        ChensetCh30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh31 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh31 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh31 {
        ChensetCh31::from_bits(val)
    }
}
impl From<ChensetCh31> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh31) -> u8 {
        ChensetCh31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh4 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh4 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh4 {
        ChensetCh4::from_bits(val)
    }
}
impl From<ChensetCh4> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh4) -> u8 {
        ChensetCh4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh5 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh5 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh5 {
        ChensetCh5::from_bits(val)
    }
}
impl From<ChensetCh5> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh5) -> u8 {
        ChensetCh5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh6 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh6 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh6 {
        ChensetCh6::from_bits(val)
    }
}
impl From<ChensetCh6> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh6) -> u8 {
        ChensetCh6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh7 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh7 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh7 {
        ChensetCh7::from_bits(val)
    }
}
impl From<ChensetCh7> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh7) -> u8 {
        ChensetCh7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh8 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh8 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh8 {
        ChensetCh8::from_bits(val)
    }
}
impl From<ChensetCh8> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh8) -> u8 {
        ChensetCh8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChensetCh9 {
    #[doc = "Read: Channel disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Channel enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl ChensetCh9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChensetCh9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChensetCh9 {
    #[inline(always)]
    fn from(val: u8) -> ChensetCh9 {
        ChensetCh9::from_bits(val)
    }
}
impl From<ChensetCh9> for u8 {
    #[inline(always)]
    fn from(val: ChensetCh9) -> u8 {
        ChensetCh9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh0 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh0 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh0 {
        ChgCh0::from_bits(val)
    }
}
impl From<ChgCh0> for u8 {
    #[inline(always)]
    fn from(val: ChgCh0) -> u8 {
        ChgCh0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh1 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh1 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh1 {
        ChgCh1::from_bits(val)
    }
}
impl From<ChgCh1> for u8 {
    #[inline(always)]
    fn from(val: ChgCh1) -> u8 {
        ChgCh1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh10 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh10 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh10 {
        ChgCh10::from_bits(val)
    }
}
impl From<ChgCh10> for u8 {
    #[inline(always)]
    fn from(val: ChgCh10) -> u8 {
        ChgCh10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh11 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh11 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh11 {
        ChgCh11::from_bits(val)
    }
}
impl From<ChgCh11> for u8 {
    #[inline(always)]
    fn from(val: ChgCh11) -> u8 {
        ChgCh11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh12 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh12 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh12 {
        ChgCh12::from_bits(val)
    }
}
impl From<ChgCh12> for u8 {
    #[inline(always)]
    fn from(val: ChgCh12) -> u8 {
        ChgCh12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh13 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh13 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh13 {
        ChgCh13::from_bits(val)
    }
}
impl From<ChgCh13> for u8 {
    #[inline(always)]
    fn from(val: ChgCh13) -> u8 {
        ChgCh13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh14 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh14 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh14 {
        ChgCh14::from_bits(val)
    }
}
impl From<ChgCh14> for u8 {
    #[inline(always)]
    fn from(val: ChgCh14) -> u8 {
        ChgCh14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh15 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh15 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh15 {
        ChgCh15::from_bits(val)
    }
}
impl From<ChgCh15> for u8 {
    #[inline(always)]
    fn from(val: ChgCh15) -> u8 {
        ChgCh15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh16 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh16 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh16 {
        ChgCh16::from_bits(val)
    }
}
impl From<ChgCh16> for u8 {
    #[inline(always)]
    fn from(val: ChgCh16) -> u8 {
        ChgCh16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh17 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh17 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh17 {
        ChgCh17::from_bits(val)
    }
}
impl From<ChgCh17> for u8 {
    #[inline(always)]
    fn from(val: ChgCh17) -> u8 {
        ChgCh17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh18 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh18 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh18 {
        ChgCh18::from_bits(val)
    }
}
impl From<ChgCh18> for u8 {
    #[inline(always)]
    fn from(val: ChgCh18) -> u8 {
        ChgCh18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh19 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh19 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh19 {
        ChgCh19::from_bits(val)
    }
}
impl From<ChgCh19> for u8 {
    #[inline(always)]
    fn from(val: ChgCh19) -> u8 {
        ChgCh19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh2 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh2 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh2 {
        ChgCh2::from_bits(val)
    }
}
impl From<ChgCh2> for u8 {
    #[inline(always)]
    fn from(val: ChgCh2) -> u8 {
        ChgCh2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh20 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh20 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh20 {
        ChgCh20::from_bits(val)
    }
}
impl From<ChgCh20> for u8 {
    #[inline(always)]
    fn from(val: ChgCh20) -> u8 {
        ChgCh20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh21 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh21 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh21 {
        ChgCh21::from_bits(val)
    }
}
impl From<ChgCh21> for u8 {
    #[inline(always)]
    fn from(val: ChgCh21) -> u8 {
        ChgCh21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh22 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh22 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh22 {
        ChgCh22::from_bits(val)
    }
}
impl From<ChgCh22> for u8 {
    #[inline(always)]
    fn from(val: ChgCh22) -> u8 {
        ChgCh22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh23 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh23 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh23 {
        ChgCh23::from_bits(val)
    }
}
impl From<ChgCh23> for u8 {
    #[inline(always)]
    fn from(val: ChgCh23) -> u8 {
        ChgCh23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh24 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh24 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh24 {
        ChgCh24::from_bits(val)
    }
}
impl From<ChgCh24> for u8 {
    #[inline(always)]
    fn from(val: ChgCh24) -> u8 {
        ChgCh24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh25 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh25 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh25 {
        ChgCh25::from_bits(val)
    }
}
impl From<ChgCh25> for u8 {
    #[inline(always)]
    fn from(val: ChgCh25) -> u8 {
        ChgCh25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh26 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh26 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh26 {
        ChgCh26::from_bits(val)
    }
}
impl From<ChgCh26> for u8 {
    #[inline(always)]
    fn from(val: ChgCh26) -> u8 {
        ChgCh26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh27 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh27 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh27 {
        ChgCh27::from_bits(val)
    }
}
impl From<ChgCh27> for u8 {
    #[inline(always)]
    fn from(val: ChgCh27) -> u8 {
        ChgCh27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh28 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh28 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh28 {
        ChgCh28::from_bits(val)
    }
}
impl From<ChgCh28> for u8 {
    #[inline(always)]
    fn from(val: ChgCh28) -> u8 {
        ChgCh28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh29 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh29 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh29 {
        ChgCh29::from_bits(val)
    }
}
impl From<ChgCh29> for u8 {
    #[inline(always)]
    fn from(val: ChgCh29) -> u8 {
        ChgCh29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh3 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh3 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh3 {
        ChgCh3::from_bits(val)
    }
}
impl From<ChgCh3> for u8 {
    #[inline(always)]
    fn from(val: ChgCh3) -> u8 {
        ChgCh3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh30 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh30 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh30 {
        ChgCh30::from_bits(val)
    }
}
impl From<ChgCh30> for u8 {
    #[inline(always)]
    fn from(val: ChgCh30) -> u8 {
        ChgCh30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh31 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh31 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh31 {
        ChgCh31::from_bits(val)
    }
}
impl From<ChgCh31> for u8 {
    #[inline(always)]
    fn from(val: ChgCh31) -> u8 {
        ChgCh31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh4 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh4 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh4 {
        ChgCh4::from_bits(val)
    }
}
impl From<ChgCh4> for u8 {
    #[inline(always)]
    fn from(val: ChgCh4) -> u8 {
        ChgCh4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh5 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh5 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh5 {
        ChgCh5::from_bits(val)
    }
}
impl From<ChgCh5> for u8 {
    #[inline(always)]
    fn from(val: ChgCh5) -> u8 {
        ChgCh5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh6 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh6 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh6 {
        ChgCh6::from_bits(val)
    }
}
impl From<ChgCh6> for u8 {
    #[inline(always)]
    fn from(val: ChgCh6) -> u8 {
        ChgCh6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh7 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh7 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh7 {
        ChgCh7::from_bits(val)
    }
}
impl From<ChgCh7> for u8 {
    #[inline(always)]
    fn from(val: ChgCh7) -> u8 {
        ChgCh7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh8 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh8 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh8 {
        ChgCh8::from_bits(val)
    }
}
impl From<ChgCh8> for u8 {
    #[inline(always)]
    fn from(val: ChgCh8) -> u8 {
        ChgCh8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChgCh9 {
    #[doc = "Exclude"]
    EXCLUDED = 0x0,
    #[doc = "Include"]
    INCLUDED = 0x01,
}
impl ChgCh9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChgCh9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChgCh9 {
    #[inline(always)]
    fn from(val: u8) -> ChgCh9 {
        ChgCh9::from_bits(val)
    }
}
impl From<ChgCh9> for u8 {
    #[inline(always)]
    fn from(val: ChgCh9) -> u8 {
        ChgCh9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dis {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl Dis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis {
    #[inline(always)]
    fn from(val: u8) -> Dis {
        Dis::from_bits(val)
    }
}
impl From<Dis> for u8 {
    #[inline(always)]
    fn from(val: Dis) -> u8 {
        Dis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DisEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl DisEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisEn {
    #[inline(always)]
    fn from(val: u8) -> DisEn {
        DisEn::from_bits(val)
    }
}
impl From<DisEn> for u8 {
    #[inline(always)]
    fn from(val: DisEn) -> u8 {
        DisEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeChgEnEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeChgEnEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeChgEnEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeChgEnEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeChgEnEn {
        SubscribeChgEnEn::from_bits(val)
    }
}
impl From<SubscribeChgEnEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeChgEnEn) -> u8 {
        SubscribeChgEnEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksChgEnEn {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksChgEnEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksChgEnEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksChgEnEn {
    #[inline(always)]
    fn from(val: u8) -> TasksChgEnEn {
        TasksChgEnEn::from_bits(val)
    }
}
impl From<TasksChgEnEn> for u8 {
    #[inline(always)]
    fn from(val: TasksChgEnEn) -> u8 {
        TasksChgEnEn::to_bits(val)
    }
}
