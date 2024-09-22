#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DetectmodeDetectmode {
    #[doc = "DETECT directly connected to PIN DETECT signals"]
    DEFAULT = 0x0,
    #[doc = "Use the latched LDETECT behavior"]
    LDETECT = 0x01,
}
impl DetectmodeDetectmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectmodeDetectmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectmodeDetectmode {
    #[inline(always)]
    fn from(val: u8) -> DetectmodeDetectmode {
        DetectmodeDetectmode::from_bits(val)
    }
}
impl From<DetectmodeDetectmode> for u8 {
    #[inline(always)]
    fn from(val: DetectmodeDetectmode) -> u8 {
        DetectmodeDetectmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DetectmodeSecDetectmode {
    #[doc = "DETECT directly connected to PIN DETECT signals"]
    DEFAULT = 0x0,
    #[doc = "Use the latched LDETECT behavior"]
    LDETECT = 0x01,
}
impl DetectmodeSecDetectmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectmodeSecDetectmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectmodeSecDetectmode {
    #[inline(always)]
    fn from(val: u8) -> DetectmodeSecDetectmode {
        DetectmodeSecDetectmode::from_bits(val)
    }
}
impl From<DetectmodeSecDetectmode> for u8 {
    #[inline(always)]
    fn from(val: DetectmodeSecDetectmode) -> u8 {
        DetectmodeSecDetectmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dir {
    #[doc = "Configure pin as an input pin"]
    INPUT = 0x0,
    #[doc = "Configure pin as an output pin"]
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
pub enum DirPin0 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin0 {
    #[inline(always)]
    fn from(val: u8) -> DirPin0 {
        DirPin0::from_bits(val)
    }
}
impl From<DirPin0> for u8 {
    #[inline(always)]
    fn from(val: DirPin0) -> u8 {
        DirPin0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin1 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin1 {
    #[inline(always)]
    fn from(val: u8) -> DirPin1 {
        DirPin1::from_bits(val)
    }
}
impl From<DirPin1> for u8 {
    #[inline(always)]
    fn from(val: DirPin1) -> u8 {
        DirPin1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin10 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin10 {
    #[inline(always)]
    fn from(val: u8) -> DirPin10 {
        DirPin10::from_bits(val)
    }
}
impl From<DirPin10> for u8 {
    #[inline(always)]
    fn from(val: DirPin10) -> u8 {
        DirPin10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin11 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin11 {
    #[inline(always)]
    fn from(val: u8) -> DirPin11 {
        DirPin11::from_bits(val)
    }
}
impl From<DirPin11> for u8 {
    #[inline(always)]
    fn from(val: DirPin11) -> u8 {
        DirPin11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin12 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin12 {
    #[inline(always)]
    fn from(val: u8) -> DirPin12 {
        DirPin12::from_bits(val)
    }
}
impl From<DirPin12> for u8 {
    #[inline(always)]
    fn from(val: DirPin12) -> u8 {
        DirPin12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin13 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin13 {
    #[inline(always)]
    fn from(val: u8) -> DirPin13 {
        DirPin13::from_bits(val)
    }
}
impl From<DirPin13> for u8 {
    #[inline(always)]
    fn from(val: DirPin13) -> u8 {
        DirPin13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin14 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin14 {
    #[inline(always)]
    fn from(val: u8) -> DirPin14 {
        DirPin14::from_bits(val)
    }
}
impl From<DirPin14> for u8 {
    #[inline(always)]
    fn from(val: DirPin14) -> u8 {
        DirPin14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin15 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin15 {
    #[inline(always)]
    fn from(val: u8) -> DirPin15 {
        DirPin15::from_bits(val)
    }
}
impl From<DirPin15> for u8 {
    #[inline(always)]
    fn from(val: DirPin15) -> u8 {
        DirPin15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin16 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin16 {
    #[inline(always)]
    fn from(val: u8) -> DirPin16 {
        DirPin16::from_bits(val)
    }
}
impl From<DirPin16> for u8 {
    #[inline(always)]
    fn from(val: DirPin16) -> u8 {
        DirPin16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin17 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin17 {
    #[inline(always)]
    fn from(val: u8) -> DirPin17 {
        DirPin17::from_bits(val)
    }
}
impl From<DirPin17> for u8 {
    #[inline(always)]
    fn from(val: DirPin17) -> u8 {
        DirPin17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin18 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin18 {
    #[inline(always)]
    fn from(val: u8) -> DirPin18 {
        DirPin18::from_bits(val)
    }
}
impl From<DirPin18> for u8 {
    #[inline(always)]
    fn from(val: DirPin18) -> u8 {
        DirPin18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin19 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin19 {
    #[inline(always)]
    fn from(val: u8) -> DirPin19 {
        DirPin19::from_bits(val)
    }
}
impl From<DirPin19> for u8 {
    #[inline(always)]
    fn from(val: DirPin19) -> u8 {
        DirPin19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin2 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin2 {
    #[inline(always)]
    fn from(val: u8) -> DirPin2 {
        DirPin2::from_bits(val)
    }
}
impl From<DirPin2> for u8 {
    #[inline(always)]
    fn from(val: DirPin2) -> u8 {
        DirPin2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin20 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin20 {
    #[inline(always)]
    fn from(val: u8) -> DirPin20 {
        DirPin20::from_bits(val)
    }
}
impl From<DirPin20> for u8 {
    #[inline(always)]
    fn from(val: DirPin20) -> u8 {
        DirPin20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin21 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin21 {
    #[inline(always)]
    fn from(val: u8) -> DirPin21 {
        DirPin21::from_bits(val)
    }
}
impl From<DirPin21> for u8 {
    #[inline(always)]
    fn from(val: DirPin21) -> u8 {
        DirPin21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin22 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin22 {
    #[inline(always)]
    fn from(val: u8) -> DirPin22 {
        DirPin22::from_bits(val)
    }
}
impl From<DirPin22> for u8 {
    #[inline(always)]
    fn from(val: DirPin22) -> u8 {
        DirPin22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin23 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin23 {
    #[inline(always)]
    fn from(val: u8) -> DirPin23 {
        DirPin23::from_bits(val)
    }
}
impl From<DirPin23> for u8 {
    #[inline(always)]
    fn from(val: DirPin23) -> u8 {
        DirPin23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin24 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin24 {
    #[inline(always)]
    fn from(val: u8) -> DirPin24 {
        DirPin24::from_bits(val)
    }
}
impl From<DirPin24> for u8 {
    #[inline(always)]
    fn from(val: DirPin24) -> u8 {
        DirPin24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin25 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin25 {
    #[inline(always)]
    fn from(val: u8) -> DirPin25 {
        DirPin25::from_bits(val)
    }
}
impl From<DirPin25> for u8 {
    #[inline(always)]
    fn from(val: DirPin25) -> u8 {
        DirPin25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin26 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin26 {
    #[inline(always)]
    fn from(val: u8) -> DirPin26 {
        DirPin26::from_bits(val)
    }
}
impl From<DirPin26> for u8 {
    #[inline(always)]
    fn from(val: DirPin26) -> u8 {
        DirPin26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin27 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin27 {
    #[inline(always)]
    fn from(val: u8) -> DirPin27 {
        DirPin27::from_bits(val)
    }
}
impl From<DirPin27> for u8 {
    #[inline(always)]
    fn from(val: DirPin27) -> u8 {
        DirPin27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin28 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin28 {
    #[inline(always)]
    fn from(val: u8) -> DirPin28 {
        DirPin28::from_bits(val)
    }
}
impl From<DirPin28> for u8 {
    #[inline(always)]
    fn from(val: DirPin28) -> u8 {
        DirPin28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin29 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin29 {
    #[inline(always)]
    fn from(val: u8) -> DirPin29 {
        DirPin29::from_bits(val)
    }
}
impl From<DirPin29> for u8 {
    #[inline(always)]
    fn from(val: DirPin29) -> u8 {
        DirPin29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin3 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin3 {
    #[inline(always)]
    fn from(val: u8) -> DirPin3 {
        DirPin3::from_bits(val)
    }
}
impl From<DirPin3> for u8 {
    #[inline(always)]
    fn from(val: DirPin3) -> u8 {
        DirPin3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin30 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin30 {
    #[inline(always)]
    fn from(val: u8) -> DirPin30 {
        DirPin30::from_bits(val)
    }
}
impl From<DirPin30> for u8 {
    #[inline(always)]
    fn from(val: DirPin30) -> u8 {
        DirPin30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin31 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin31 {
    #[inline(always)]
    fn from(val: u8) -> DirPin31 {
        DirPin31::from_bits(val)
    }
}
impl From<DirPin31> for u8 {
    #[inline(always)]
    fn from(val: DirPin31) -> u8 {
        DirPin31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin4 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin4 {
    #[inline(always)]
    fn from(val: u8) -> DirPin4 {
        DirPin4::from_bits(val)
    }
}
impl From<DirPin4> for u8 {
    #[inline(always)]
    fn from(val: DirPin4) -> u8 {
        DirPin4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin5 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin5 {
    #[inline(always)]
    fn from(val: u8) -> DirPin5 {
        DirPin5::from_bits(val)
    }
}
impl From<DirPin5> for u8 {
    #[inline(always)]
    fn from(val: DirPin5) -> u8 {
        DirPin5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin6 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin6 {
    #[inline(always)]
    fn from(val: u8) -> DirPin6 {
        DirPin6::from_bits(val)
    }
}
impl From<DirPin6> for u8 {
    #[inline(always)]
    fn from(val: DirPin6) -> u8 {
        DirPin6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin7 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin7 {
    #[inline(always)]
    fn from(val: u8) -> DirPin7 {
        DirPin7::from_bits(val)
    }
}
impl From<DirPin7> for u8 {
    #[inline(always)]
    fn from(val: DirPin7) -> u8 {
        DirPin7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin8 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin8 {
    #[inline(always)]
    fn from(val: u8) -> DirPin8 {
        DirPin8::from_bits(val)
    }
}
impl From<DirPin8> for u8 {
    #[inline(always)]
    fn from(val: DirPin8) -> u8 {
        DirPin8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirPin9 {
    #[doc = "Pin set as input"]
    INPUT = 0x0,
    #[doc = "Pin set as output"]
    OUTPUT = 0x01,
}
impl DirPin9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirPin9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirPin9 {
    #[inline(always)]
    fn from(val: u8) -> DirPin9 {
        DirPin9::from_bits(val)
    }
}
impl From<DirPin9> for u8 {
    #[inline(always)]
    fn from(val: DirPin9) -> u8 {
        DirPin9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin0 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin0 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin0 {
        DirclrPin0::from_bits(val)
    }
}
impl From<DirclrPin0> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin0) -> u8 {
        DirclrPin0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin1 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin1 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin1 {
        DirclrPin1::from_bits(val)
    }
}
impl From<DirclrPin1> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin1) -> u8 {
        DirclrPin1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin10 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin10 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin10 {
        DirclrPin10::from_bits(val)
    }
}
impl From<DirclrPin10> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin10) -> u8 {
        DirclrPin10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin11 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin11 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin11 {
        DirclrPin11::from_bits(val)
    }
}
impl From<DirclrPin11> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin11) -> u8 {
        DirclrPin11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin12 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin12 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin12 {
        DirclrPin12::from_bits(val)
    }
}
impl From<DirclrPin12> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin12) -> u8 {
        DirclrPin12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin13 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin13 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin13 {
        DirclrPin13::from_bits(val)
    }
}
impl From<DirclrPin13> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin13) -> u8 {
        DirclrPin13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin14 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin14 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin14 {
        DirclrPin14::from_bits(val)
    }
}
impl From<DirclrPin14> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin14) -> u8 {
        DirclrPin14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin15 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin15 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin15 {
        DirclrPin15::from_bits(val)
    }
}
impl From<DirclrPin15> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin15) -> u8 {
        DirclrPin15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin16 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin16 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin16 {
        DirclrPin16::from_bits(val)
    }
}
impl From<DirclrPin16> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin16) -> u8 {
        DirclrPin16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin17 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin17 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin17 {
        DirclrPin17::from_bits(val)
    }
}
impl From<DirclrPin17> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin17) -> u8 {
        DirclrPin17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin18 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin18 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin18 {
        DirclrPin18::from_bits(val)
    }
}
impl From<DirclrPin18> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin18) -> u8 {
        DirclrPin18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin19 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin19 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin19 {
        DirclrPin19::from_bits(val)
    }
}
impl From<DirclrPin19> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin19) -> u8 {
        DirclrPin19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin2 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin2 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin2 {
        DirclrPin2::from_bits(val)
    }
}
impl From<DirclrPin2> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin2) -> u8 {
        DirclrPin2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin20 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin20 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin20 {
        DirclrPin20::from_bits(val)
    }
}
impl From<DirclrPin20> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin20) -> u8 {
        DirclrPin20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin21 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin21 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin21 {
        DirclrPin21::from_bits(val)
    }
}
impl From<DirclrPin21> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin21) -> u8 {
        DirclrPin21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin22 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin22 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin22 {
        DirclrPin22::from_bits(val)
    }
}
impl From<DirclrPin22> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin22) -> u8 {
        DirclrPin22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin23 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin23 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin23 {
        DirclrPin23::from_bits(val)
    }
}
impl From<DirclrPin23> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin23) -> u8 {
        DirclrPin23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin24 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin24 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin24 {
        DirclrPin24::from_bits(val)
    }
}
impl From<DirclrPin24> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin24) -> u8 {
        DirclrPin24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin25 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin25 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin25 {
        DirclrPin25::from_bits(val)
    }
}
impl From<DirclrPin25> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin25) -> u8 {
        DirclrPin25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin26 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin26 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin26 {
        DirclrPin26::from_bits(val)
    }
}
impl From<DirclrPin26> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin26) -> u8 {
        DirclrPin26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin27 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin27 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin27 {
        DirclrPin27::from_bits(val)
    }
}
impl From<DirclrPin27> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin27) -> u8 {
        DirclrPin27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin28 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin28 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin28 {
        DirclrPin28::from_bits(val)
    }
}
impl From<DirclrPin28> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin28) -> u8 {
        DirclrPin28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin29 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin29 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin29 {
        DirclrPin29::from_bits(val)
    }
}
impl From<DirclrPin29> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin29) -> u8 {
        DirclrPin29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin3 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin3 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin3 {
        DirclrPin3::from_bits(val)
    }
}
impl From<DirclrPin3> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin3) -> u8 {
        DirclrPin3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin30 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin30 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin30 {
        DirclrPin30::from_bits(val)
    }
}
impl From<DirclrPin30> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin30) -> u8 {
        DirclrPin30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin31 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin31 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin31 {
        DirclrPin31::from_bits(val)
    }
}
impl From<DirclrPin31> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin31) -> u8 {
        DirclrPin31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin4 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin4 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin4 {
        DirclrPin4::from_bits(val)
    }
}
impl From<DirclrPin4> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin4) -> u8 {
        DirclrPin4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin5 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin5 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin5 {
        DirclrPin5::from_bits(val)
    }
}
impl From<DirclrPin5> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin5) -> u8 {
        DirclrPin5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin6 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin6 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin6 {
        DirclrPin6::from_bits(val)
    }
}
impl From<DirclrPin6> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin6) -> u8 {
        DirclrPin6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin7 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin7 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin7 {
        DirclrPin7::from_bits(val)
    }
}
impl From<DirclrPin7> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin7) -> u8 {
        DirclrPin7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin8 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin8 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin8 {
        DirclrPin8::from_bits(val)
    }
}
impl From<DirclrPin8> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin8) -> u8 {
        DirclrPin8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirclrPin9 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_CLEAR = 0x01,
}
impl DirclrPin9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirclrPin9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirclrPin9 {
    #[inline(always)]
    fn from(val: u8) -> DirclrPin9 {
        DirclrPin9::from_bits(val)
    }
}
impl From<DirclrPin9> for u8 {
    #[inline(always)]
    fn from(val: DirclrPin9) -> u8 {
        DirclrPin9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin0 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin0 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin0 {
        DirsetPin0::from_bits(val)
    }
}
impl From<DirsetPin0> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin0) -> u8 {
        DirsetPin0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin1 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin1 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin1 {
        DirsetPin1::from_bits(val)
    }
}
impl From<DirsetPin1> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin1) -> u8 {
        DirsetPin1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin10 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin10 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin10 {
        DirsetPin10::from_bits(val)
    }
}
impl From<DirsetPin10> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin10) -> u8 {
        DirsetPin10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin11 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin11 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin11 {
        DirsetPin11::from_bits(val)
    }
}
impl From<DirsetPin11> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin11) -> u8 {
        DirsetPin11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin12 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin12 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin12 {
        DirsetPin12::from_bits(val)
    }
}
impl From<DirsetPin12> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin12) -> u8 {
        DirsetPin12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin13 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin13 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin13 {
        DirsetPin13::from_bits(val)
    }
}
impl From<DirsetPin13> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin13) -> u8 {
        DirsetPin13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin14 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin14 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin14 {
        DirsetPin14::from_bits(val)
    }
}
impl From<DirsetPin14> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin14) -> u8 {
        DirsetPin14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin15 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin15 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin15 {
        DirsetPin15::from_bits(val)
    }
}
impl From<DirsetPin15> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin15) -> u8 {
        DirsetPin15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin16 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin16 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin16 {
        DirsetPin16::from_bits(val)
    }
}
impl From<DirsetPin16> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin16) -> u8 {
        DirsetPin16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin17 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin17 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin17 {
        DirsetPin17::from_bits(val)
    }
}
impl From<DirsetPin17> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin17) -> u8 {
        DirsetPin17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin18 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin18 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin18 {
        DirsetPin18::from_bits(val)
    }
}
impl From<DirsetPin18> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin18) -> u8 {
        DirsetPin18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin19 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin19 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin19 {
        DirsetPin19::from_bits(val)
    }
}
impl From<DirsetPin19> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin19) -> u8 {
        DirsetPin19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin2 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin2 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin2 {
        DirsetPin2::from_bits(val)
    }
}
impl From<DirsetPin2> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin2) -> u8 {
        DirsetPin2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin20 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin20 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin20 {
        DirsetPin20::from_bits(val)
    }
}
impl From<DirsetPin20> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin20) -> u8 {
        DirsetPin20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin21 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin21 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin21 {
        DirsetPin21::from_bits(val)
    }
}
impl From<DirsetPin21> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin21) -> u8 {
        DirsetPin21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin22 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin22 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin22 {
        DirsetPin22::from_bits(val)
    }
}
impl From<DirsetPin22> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin22) -> u8 {
        DirsetPin22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin23 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin23 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin23 {
        DirsetPin23::from_bits(val)
    }
}
impl From<DirsetPin23> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin23) -> u8 {
        DirsetPin23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin24 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin24 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin24 {
        DirsetPin24::from_bits(val)
    }
}
impl From<DirsetPin24> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin24) -> u8 {
        DirsetPin24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin25 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin25 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin25 {
        DirsetPin25::from_bits(val)
    }
}
impl From<DirsetPin25> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin25) -> u8 {
        DirsetPin25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin26 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin26 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin26 {
        DirsetPin26::from_bits(val)
    }
}
impl From<DirsetPin26> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin26) -> u8 {
        DirsetPin26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin27 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin27 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin27 {
        DirsetPin27::from_bits(val)
    }
}
impl From<DirsetPin27> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin27) -> u8 {
        DirsetPin27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin28 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin28 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin28 {
        DirsetPin28::from_bits(val)
    }
}
impl From<DirsetPin28> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin28) -> u8 {
        DirsetPin28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin29 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin29 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin29 {
        DirsetPin29::from_bits(val)
    }
}
impl From<DirsetPin29> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin29) -> u8 {
        DirsetPin29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin3 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin3 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin3 {
        DirsetPin3::from_bits(val)
    }
}
impl From<DirsetPin3> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin3) -> u8 {
        DirsetPin3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin30 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin30 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin30 {
        DirsetPin30::from_bits(val)
    }
}
impl From<DirsetPin30> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin30) -> u8 {
        DirsetPin30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin31 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin31 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin31 {
        DirsetPin31::from_bits(val)
    }
}
impl From<DirsetPin31> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin31) -> u8 {
        DirsetPin31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin4 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin4 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin4 {
        DirsetPin4::from_bits(val)
    }
}
impl From<DirsetPin4> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin4) -> u8 {
        DirsetPin4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin5 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin5 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin5 {
        DirsetPin5::from_bits(val)
    }
}
impl From<DirsetPin5> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin5) -> u8 {
        DirsetPin5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin6 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin6 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin6 {
        DirsetPin6::from_bits(val)
    }
}
impl From<DirsetPin6> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin6) -> u8 {
        DirsetPin6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin7 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin7 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin7 {
        DirsetPin7::from_bits(val)
    }
}
impl From<DirsetPin7> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin7) -> u8 {
        DirsetPin7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin8 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin8 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin8 {
        DirsetPin8::from_bits(val)
    }
}
impl From<DirsetPin8> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin8) -> u8 {
        DirsetPin8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DirsetPin9 {
    #[doc = "Read: pin set as input"]
    INPUT = 0x0,
    #[doc = "Read: pin set as output"]
    R_OUTPUT_W_SET = 0x01,
}
impl DirsetPin9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DirsetPin9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DirsetPin9 {
    #[inline(always)]
    fn from(val: u8) -> DirsetPin9 {
        DirsetPin9::from_bits(val)
    }
}
impl From<DirsetPin9> for u8 {
    #[inline(always)]
    fn from(val: DirsetPin9) -> u8 {
        DirsetPin9::to_bits(val)
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
    #[doc = "Disconnect '0', standard '1' (normally used for wired-or connections)"]
    D0S1 = 0x04,
    #[doc = "Disconnect '0', high drive '1' (normally used for wired-or connections)"]
    D0H1 = 0x05,
    #[doc = "Standard '0', disconnect '1' (normally used for wired-and connections)"]
    S0D1 = 0x06,
    #[doc = "High drive '0', disconnect '1' (normally used for wired-and connections)"]
    H0D1 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "Extra high drive '0', extra high drive '1'"]
    E0E1 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Drive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Drive {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum InPin0 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin0 {
    #[inline(always)]
    fn from(val: u8) -> InPin0 {
        InPin0::from_bits(val)
    }
}
impl From<InPin0> for u8 {
    #[inline(always)]
    fn from(val: InPin0) -> u8 {
        InPin0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin1 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin1 {
    #[inline(always)]
    fn from(val: u8) -> InPin1 {
        InPin1::from_bits(val)
    }
}
impl From<InPin1> for u8 {
    #[inline(always)]
    fn from(val: InPin1) -> u8 {
        InPin1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin10 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin10 {
    #[inline(always)]
    fn from(val: u8) -> InPin10 {
        InPin10::from_bits(val)
    }
}
impl From<InPin10> for u8 {
    #[inline(always)]
    fn from(val: InPin10) -> u8 {
        InPin10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin11 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin11 {
    #[inline(always)]
    fn from(val: u8) -> InPin11 {
        InPin11::from_bits(val)
    }
}
impl From<InPin11> for u8 {
    #[inline(always)]
    fn from(val: InPin11) -> u8 {
        InPin11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin12 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin12 {
    #[inline(always)]
    fn from(val: u8) -> InPin12 {
        InPin12::from_bits(val)
    }
}
impl From<InPin12> for u8 {
    #[inline(always)]
    fn from(val: InPin12) -> u8 {
        InPin12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin13 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin13 {
    #[inline(always)]
    fn from(val: u8) -> InPin13 {
        InPin13::from_bits(val)
    }
}
impl From<InPin13> for u8 {
    #[inline(always)]
    fn from(val: InPin13) -> u8 {
        InPin13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin14 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin14 {
    #[inline(always)]
    fn from(val: u8) -> InPin14 {
        InPin14::from_bits(val)
    }
}
impl From<InPin14> for u8 {
    #[inline(always)]
    fn from(val: InPin14) -> u8 {
        InPin14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin15 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin15 {
    #[inline(always)]
    fn from(val: u8) -> InPin15 {
        InPin15::from_bits(val)
    }
}
impl From<InPin15> for u8 {
    #[inline(always)]
    fn from(val: InPin15) -> u8 {
        InPin15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin16 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin16 {
    #[inline(always)]
    fn from(val: u8) -> InPin16 {
        InPin16::from_bits(val)
    }
}
impl From<InPin16> for u8 {
    #[inline(always)]
    fn from(val: InPin16) -> u8 {
        InPin16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin17 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin17 {
    #[inline(always)]
    fn from(val: u8) -> InPin17 {
        InPin17::from_bits(val)
    }
}
impl From<InPin17> for u8 {
    #[inline(always)]
    fn from(val: InPin17) -> u8 {
        InPin17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin18 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin18 {
    #[inline(always)]
    fn from(val: u8) -> InPin18 {
        InPin18::from_bits(val)
    }
}
impl From<InPin18> for u8 {
    #[inline(always)]
    fn from(val: InPin18) -> u8 {
        InPin18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin19 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin19 {
    #[inline(always)]
    fn from(val: u8) -> InPin19 {
        InPin19::from_bits(val)
    }
}
impl From<InPin19> for u8 {
    #[inline(always)]
    fn from(val: InPin19) -> u8 {
        InPin19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin2 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin2 {
    #[inline(always)]
    fn from(val: u8) -> InPin2 {
        InPin2::from_bits(val)
    }
}
impl From<InPin2> for u8 {
    #[inline(always)]
    fn from(val: InPin2) -> u8 {
        InPin2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin20 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin20 {
    #[inline(always)]
    fn from(val: u8) -> InPin20 {
        InPin20::from_bits(val)
    }
}
impl From<InPin20> for u8 {
    #[inline(always)]
    fn from(val: InPin20) -> u8 {
        InPin20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin21 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin21 {
    #[inline(always)]
    fn from(val: u8) -> InPin21 {
        InPin21::from_bits(val)
    }
}
impl From<InPin21> for u8 {
    #[inline(always)]
    fn from(val: InPin21) -> u8 {
        InPin21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin22 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin22 {
    #[inline(always)]
    fn from(val: u8) -> InPin22 {
        InPin22::from_bits(val)
    }
}
impl From<InPin22> for u8 {
    #[inline(always)]
    fn from(val: InPin22) -> u8 {
        InPin22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin23 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin23 {
    #[inline(always)]
    fn from(val: u8) -> InPin23 {
        InPin23::from_bits(val)
    }
}
impl From<InPin23> for u8 {
    #[inline(always)]
    fn from(val: InPin23) -> u8 {
        InPin23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin24 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin24 {
    #[inline(always)]
    fn from(val: u8) -> InPin24 {
        InPin24::from_bits(val)
    }
}
impl From<InPin24> for u8 {
    #[inline(always)]
    fn from(val: InPin24) -> u8 {
        InPin24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin25 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin25 {
    #[inline(always)]
    fn from(val: u8) -> InPin25 {
        InPin25::from_bits(val)
    }
}
impl From<InPin25> for u8 {
    #[inline(always)]
    fn from(val: InPin25) -> u8 {
        InPin25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin26 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin26 {
    #[inline(always)]
    fn from(val: u8) -> InPin26 {
        InPin26::from_bits(val)
    }
}
impl From<InPin26> for u8 {
    #[inline(always)]
    fn from(val: InPin26) -> u8 {
        InPin26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin27 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin27 {
    #[inline(always)]
    fn from(val: u8) -> InPin27 {
        InPin27::from_bits(val)
    }
}
impl From<InPin27> for u8 {
    #[inline(always)]
    fn from(val: InPin27) -> u8 {
        InPin27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin28 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin28 {
    #[inline(always)]
    fn from(val: u8) -> InPin28 {
        InPin28::from_bits(val)
    }
}
impl From<InPin28> for u8 {
    #[inline(always)]
    fn from(val: InPin28) -> u8 {
        InPin28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin29 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin29 {
    #[inline(always)]
    fn from(val: u8) -> InPin29 {
        InPin29::from_bits(val)
    }
}
impl From<InPin29> for u8 {
    #[inline(always)]
    fn from(val: InPin29) -> u8 {
        InPin29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin3 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin3 {
    #[inline(always)]
    fn from(val: u8) -> InPin3 {
        InPin3::from_bits(val)
    }
}
impl From<InPin3> for u8 {
    #[inline(always)]
    fn from(val: InPin3) -> u8 {
        InPin3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin30 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin30 {
    #[inline(always)]
    fn from(val: u8) -> InPin30 {
        InPin30::from_bits(val)
    }
}
impl From<InPin30> for u8 {
    #[inline(always)]
    fn from(val: InPin30) -> u8 {
        InPin30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin31 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin31 {
    #[inline(always)]
    fn from(val: u8) -> InPin31 {
        InPin31::from_bits(val)
    }
}
impl From<InPin31> for u8 {
    #[inline(always)]
    fn from(val: InPin31) -> u8 {
        InPin31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin4 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin4 {
    #[inline(always)]
    fn from(val: u8) -> InPin4 {
        InPin4::from_bits(val)
    }
}
impl From<InPin4> for u8 {
    #[inline(always)]
    fn from(val: InPin4) -> u8 {
        InPin4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin5 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin5 {
    #[inline(always)]
    fn from(val: u8) -> InPin5 {
        InPin5::from_bits(val)
    }
}
impl From<InPin5> for u8 {
    #[inline(always)]
    fn from(val: InPin5) -> u8 {
        InPin5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin6 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin6 {
    #[inline(always)]
    fn from(val: u8) -> InPin6 {
        InPin6::from_bits(val)
    }
}
impl From<InPin6> for u8 {
    #[inline(always)]
    fn from(val: InPin6) -> u8 {
        InPin6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin7 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin7 {
    #[inline(always)]
    fn from(val: u8) -> InPin7 {
        InPin7::from_bits(val)
    }
}
impl From<InPin7> for u8 {
    #[inline(always)]
    fn from(val: InPin7) -> u8 {
        InPin7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin8 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin8 {
    #[inline(always)]
    fn from(val: u8) -> InPin8 {
        InPin8::from_bits(val)
    }
}
impl From<InPin8> for u8 {
    #[inline(always)]
    fn from(val: InPin8) -> u8 {
        InPin8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InPin9 {
    #[doc = "Pin input is low"]
    LOW = 0x0,
    #[doc = "Pin input is high"]
    HIGH = 0x01,
}
impl InPin9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InPin9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InPin9 {
    #[inline(always)]
    fn from(val: u8) -> InPin9 {
        InPin9::from_bits(val)
    }
}
impl From<InPin9> for u8 {
    #[inline(always)]
    fn from(val: InPin9) -> u8 {
        InPin9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Input {
    #[doc = "Connect input buffer"]
    CONNECT = 0x0,
    #[doc = "Disconnect input buffer"]
    DISCONNECT = 0x01,
}
impl Input {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Input {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Input {
    #[inline(always)]
    fn from(val: u8) -> Input {
        Input::from_bits(val)
    }
}
impl From<Input> for u8 {
    #[inline(always)]
    fn from(val: Input) -> u8 {
        Input::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin0 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin0 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin0 {
        LatchPin0::from_bits(val)
    }
}
impl From<LatchPin0> for u8 {
    #[inline(always)]
    fn from(val: LatchPin0) -> u8 {
        LatchPin0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin1 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin1 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin1 {
        LatchPin1::from_bits(val)
    }
}
impl From<LatchPin1> for u8 {
    #[inline(always)]
    fn from(val: LatchPin1) -> u8 {
        LatchPin1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin10 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin10 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin10 {
        LatchPin10::from_bits(val)
    }
}
impl From<LatchPin10> for u8 {
    #[inline(always)]
    fn from(val: LatchPin10) -> u8 {
        LatchPin10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin11 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin11 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin11 {
        LatchPin11::from_bits(val)
    }
}
impl From<LatchPin11> for u8 {
    #[inline(always)]
    fn from(val: LatchPin11) -> u8 {
        LatchPin11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin12 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin12 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin12 {
        LatchPin12::from_bits(val)
    }
}
impl From<LatchPin12> for u8 {
    #[inline(always)]
    fn from(val: LatchPin12) -> u8 {
        LatchPin12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin13 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin13 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin13 {
        LatchPin13::from_bits(val)
    }
}
impl From<LatchPin13> for u8 {
    #[inline(always)]
    fn from(val: LatchPin13) -> u8 {
        LatchPin13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin14 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin14 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin14 {
        LatchPin14::from_bits(val)
    }
}
impl From<LatchPin14> for u8 {
    #[inline(always)]
    fn from(val: LatchPin14) -> u8 {
        LatchPin14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin15 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin15 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin15 {
        LatchPin15::from_bits(val)
    }
}
impl From<LatchPin15> for u8 {
    #[inline(always)]
    fn from(val: LatchPin15) -> u8 {
        LatchPin15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin16 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin16 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin16 {
        LatchPin16::from_bits(val)
    }
}
impl From<LatchPin16> for u8 {
    #[inline(always)]
    fn from(val: LatchPin16) -> u8 {
        LatchPin16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin17 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin17 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin17 {
        LatchPin17::from_bits(val)
    }
}
impl From<LatchPin17> for u8 {
    #[inline(always)]
    fn from(val: LatchPin17) -> u8 {
        LatchPin17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin18 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin18 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin18 {
        LatchPin18::from_bits(val)
    }
}
impl From<LatchPin18> for u8 {
    #[inline(always)]
    fn from(val: LatchPin18) -> u8 {
        LatchPin18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin19 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin19 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin19 {
        LatchPin19::from_bits(val)
    }
}
impl From<LatchPin19> for u8 {
    #[inline(always)]
    fn from(val: LatchPin19) -> u8 {
        LatchPin19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin2 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin2 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin2 {
        LatchPin2::from_bits(val)
    }
}
impl From<LatchPin2> for u8 {
    #[inline(always)]
    fn from(val: LatchPin2) -> u8 {
        LatchPin2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin20 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin20 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin20 {
        LatchPin20::from_bits(val)
    }
}
impl From<LatchPin20> for u8 {
    #[inline(always)]
    fn from(val: LatchPin20) -> u8 {
        LatchPin20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin21 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin21 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin21 {
        LatchPin21::from_bits(val)
    }
}
impl From<LatchPin21> for u8 {
    #[inline(always)]
    fn from(val: LatchPin21) -> u8 {
        LatchPin21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin22 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin22 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin22 {
        LatchPin22::from_bits(val)
    }
}
impl From<LatchPin22> for u8 {
    #[inline(always)]
    fn from(val: LatchPin22) -> u8 {
        LatchPin22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin23 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin23 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin23 {
        LatchPin23::from_bits(val)
    }
}
impl From<LatchPin23> for u8 {
    #[inline(always)]
    fn from(val: LatchPin23) -> u8 {
        LatchPin23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin24 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin24 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin24 {
        LatchPin24::from_bits(val)
    }
}
impl From<LatchPin24> for u8 {
    #[inline(always)]
    fn from(val: LatchPin24) -> u8 {
        LatchPin24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin25 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin25 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin25 {
        LatchPin25::from_bits(val)
    }
}
impl From<LatchPin25> for u8 {
    #[inline(always)]
    fn from(val: LatchPin25) -> u8 {
        LatchPin25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin26 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin26 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin26 {
        LatchPin26::from_bits(val)
    }
}
impl From<LatchPin26> for u8 {
    #[inline(always)]
    fn from(val: LatchPin26) -> u8 {
        LatchPin26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin27 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin27 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin27 {
        LatchPin27::from_bits(val)
    }
}
impl From<LatchPin27> for u8 {
    #[inline(always)]
    fn from(val: LatchPin27) -> u8 {
        LatchPin27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin28 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin28 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin28 {
        LatchPin28::from_bits(val)
    }
}
impl From<LatchPin28> for u8 {
    #[inline(always)]
    fn from(val: LatchPin28) -> u8 {
        LatchPin28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin29 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin29 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin29 {
        LatchPin29::from_bits(val)
    }
}
impl From<LatchPin29> for u8 {
    #[inline(always)]
    fn from(val: LatchPin29) -> u8 {
        LatchPin29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin3 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin3 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin3 {
        LatchPin3::from_bits(val)
    }
}
impl From<LatchPin3> for u8 {
    #[inline(always)]
    fn from(val: LatchPin3) -> u8 {
        LatchPin3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin30 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin30 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin30 {
        LatchPin30::from_bits(val)
    }
}
impl From<LatchPin30> for u8 {
    #[inline(always)]
    fn from(val: LatchPin30) -> u8 {
        LatchPin30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin31 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin31 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin31 {
        LatchPin31::from_bits(val)
    }
}
impl From<LatchPin31> for u8 {
    #[inline(always)]
    fn from(val: LatchPin31) -> u8 {
        LatchPin31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin4 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin4 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin4 {
        LatchPin4::from_bits(val)
    }
}
impl From<LatchPin4> for u8 {
    #[inline(always)]
    fn from(val: LatchPin4) -> u8 {
        LatchPin4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin5 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin5 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin5 {
        LatchPin5::from_bits(val)
    }
}
impl From<LatchPin5> for u8 {
    #[inline(always)]
    fn from(val: LatchPin5) -> u8 {
        LatchPin5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin6 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin6 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin6 {
        LatchPin6::from_bits(val)
    }
}
impl From<LatchPin6> for u8 {
    #[inline(always)]
    fn from(val: LatchPin6) -> u8 {
        LatchPin6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin7 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin7 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin7 {
        LatchPin7::from_bits(val)
    }
}
impl From<LatchPin7> for u8 {
    #[inline(always)]
    fn from(val: LatchPin7) -> u8 {
        LatchPin7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin8 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin8 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin8 {
        LatchPin8::from_bits(val)
    }
}
impl From<LatchPin8> for u8 {
    #[inline(always)]
    fn from(val: LatchPin8) -> u8 {
        LatchPin8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LatchPin9 {
    #[doc = "Criteria has not been met"]
    NOTLATCHED = 0x0,
    #[doc = "Criteria has been met"]
    LATCHED = 0x01,
}
impl LatchPin9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LatchPin9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LatchPin9 {
    #[inline(always)]
    fn from(val: u8) -> LatchPin9 {
        LatchPin9::from_bits(val)
    }
}
impl From<LatchPin9> for u8 {
    #[inline(always)]
    fn from(val: LatchPin9) -> u8 {
        LatchPin9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mcusel {
    #[doc = "Application MCU"]
    APPMCU = 0x0,
    #[doc = "Network MCU"]
    NETWORKMCU = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Peripheral with dedicated pins"]
    PERIPHERAL = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Trace and Debug Subsystem"]
    TND = 0x07,
}
impl Mcusel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mcusel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mcusel {
    #[inline(always)]
    fn from(val: u8) -> Mcusel {
        Mcusel::from_bits(val)
    }
}
impl From<Mcusel> for u8 {
    #[inline(always)]
    fn from(val: Mcusel) -> u8 {
        Mcusel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin0 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin0 {
    #[inline(always)]
    fn from(val: u8) -> OutPin0 {
        OutPin0::from_bits(val)
    }
}
impl From<OutPin0> for u8 {
    #[inline(always)]
    fn from(val: OutPin0) -> u8 {
        OutPin0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin1 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin1 {
    #[inline(always)]
    fn from(val: u8) -> OutPin1 {
        OutPin1::from_bits(val)
    }
}
impl From<OutPin1> for u8 {
    #[inline(always)]
    fn from(val: OutPin1) -> u8 {
        OutPin1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin10 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin10 {
    #[inline(always)]
    fn from(val: u8) -> OutPin10 {
        OutPin10::from_bits(val)
    }
}
impl From<OutPin10> for u8 {
    #[inline(always)]
    fn from(val: OutPin10) -> u8 {
        OutPin10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin11 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin11 {
    #[inline(always)]
    fn from(val: u8) -> OutPin11 {
        OutPin11::from_bits(val)
    }
}
impl From<OutPin11> for u8 {
    #[inline(always)]
    fn from(val: OutPin11) -> u8 {
        OutPin11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin12 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin12 {
    #[inline(always)]
    fn from(val: u8) -> OutPin12 {
        OutPin12::from_bits(val)
    }
}
impl From<OutPin12> for u8 {
    #[inline(always)]
    fn from(val: OutPin12) -> u8 {
        OutPin12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin13 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin13 {
    #[inline(always)]
    fn from(val: u8) -> OutPin13 {
        OutPin13::from_bits(val)
    }
}
impl From<OutPin13> for u8 {
    #[inline(always)]
    fn from(val: OutPin13) -> u8 {
        OutPin13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin14 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin14 {
    #[inline(always)]
    fn from(val: u8) -> OutPin14 {
        OutPin14::from_bits(val)
    }
}
impl From<OutPin14> for u8 {
    #[inline(always)]
    fn from(val: OutPin14) -> u8 {
        OutPin14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin15 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin15 {
    #[inline(always)]
    fn from(val: u8) -> OutPin15 {
        OutPin15::from_bits(val)
    }
}
impl From<OutPin15> for u8 {
    #[inline(always)]
    fn from(val: OutPin15) -> u8 {
        OutPin15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin16 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin16 {
    #[inline(always)]
    fn from(val: u8) -> OutPin16 {
        OutPin16::from_bits(val)
    }
}
impl From<OutPin16> for u8 {
    #[inline(always)]
    fn from(val: OutPin16) -> u8 {
        OutPin16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin17 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin17 {
    #[inline(always)]
    fn from(val: u8) -> OutPin17 {
        OutPin17::from_bits(val)
    }
}
impl From<OutPin17> for u8 {
    #[inline(always)]
    fn from(val: OutPin17) -> u8 {
        OutPin17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin18 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin18 {
    #[inline(always)]
    fn from(val: u8) -> OutPin18 {
        OutPin18::from_bits(val)
    }
}
impl From<OutPin18> for u8 {
    #[inline(always)]
    fn from(val: OutPin18) -> u8 {
        OutPin18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin19 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin19 {
    #[inline(always)]
    fn from(val: u8) -> OutPin19 {
        OutPin19::from_bits(val)
    }
}
impl From<OutPin19> for u8 {
    #[inline(always)]
    fn from(val: OutPin19) -> u8 {
        OutPin19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin2 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin2 {
    #[inline(always)]
    fn from(val: u8) -> OutPin2 {
        OutPin2::from_bits(val)
    }
}
impl From<OutPin2> for u8 {
    #[inline(always)]
    fn from(val: OutPin2) -> u8 {
        OutPin2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin20 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin20 {
    #[inline(always)]
    fn from(val: u8) -> OutPin20 {
        OutPin20::from_bits(val)
    }
}
impl From<OutPin20> for u8 {
    #[inline(always)]
    fn from(val: OutPin20) -> u8 {
        OutPin20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin21 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin21 {
    #[inline(always)]
    fn from(val: u8) -> OutPin21 {
        OutPin21::from_bits(val)
    }
}
impl From<OutPin21> for u8 {
    #[inline(always)]
    fn from(val: OutPin21) -> u8 {
        OutPin21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin22 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin22 {
    #[inline(always)]
    fn from(val: u8) -> OutPin22 {
        OutPin22::from_bits(val)
    }
}
impl From<OutPin22> for u8 {
    #[inline(always)]
    fn from(val: OutPin22) -> u8 {
        OutPin22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin23 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin23 {
    #[inline(always)]
    fn from(val: u8) -> OutPin23 {
        OutPin23::from_bits(val)
    }
}
impl From<OutPin23> for u8 {
    #[inline(always)]
    fn from(val: OutPin23) -> u8 {
        OutPin23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin24 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin24 {
    #[inline(always)]
    fn from(val: u8) -> OutPin24 {
        OutPin24::from_bits(val)
    }
}
impl From<OutPin24> for u8 {
    #[inline(always)]
    fn from(val: OutPin24) -> u8 {
        OutPin24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin25 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin25 {
    #[inline(always)]
    fn from(val: u8) -> OutPin25 {
        OutPin25::from_bits(val)
    }
}
impl From<OutPin25> for u8 {
    #[inline(always)]
    fn from(val: OutPin25) -> u8 {
        OutPin25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin26 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin26 {
    #[inline(always)]
    fn from(val: u8) -> OutPin26 {
        OutPin26::from_bits(val)
    }
}
impl From<OutPin26> for u8 {
    #[inline(always)]
    fn from(val: OutPin26) -> u8 {
        OutPin26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin27 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin27 {
    #[inline(always)]
    fn from(val: u8) -> OutPin27 {
        OutPin27::from_bits(val)
    }
}
impl From<OutPin27> for u8 {
    #[inline(always)]
    fn from(val: OutPin27) -> u8 {
        OutPin27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin28 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin28 {
    #[inline(always)]
    fn from(val: u8) -> OutPin28 {
        OutPin28::from_bits(val)
    }
}
impl From<OutPin28> for u8 {
    #[inline(always)]
    fn from(val: OutPin28) -> u8 {
        OutPin28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin29 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin29 {
    #[inline(always)]
    fn from(val: u8) -> OutPin29 {
        OutPin29::from_bits(val)
    }
}
impl From<OutPin29> for u8 {
    #[inline(always)]
    fn from(val: OutPin29) -> u8 {
        OutPin29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin3 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin3 {
    #[inline(always)]
    fn from(val: u8) -> OutPin3 {
        OutPin3::from_bits(val)
    }
}
impl From<OutPin3> for u8 {
    #[inline(always)]
    fn from(val: OutPin3) -> u8 {
        OutPin3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin30 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin30 {
    #[inline(always)]
    fn from(val: u8) -> OutPin30 {
        OutPin30::from_bits(val)
    }
}
impl From<OutPin30> for u8 {
    #[inline(always)]
    fn from(val: OutPin30) -> u8 {
        OutPin30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin31 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin31 {
    #[inline(always)]
    fn from(val: u8) -> OutPin31 {
        OutPin31::from_bits(val)
    }
}
impl From<OutPin31> for u8 {
    #[inline(always)]
    fn from(val: OutPin31) -> u8 {
        OutPin31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin4 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin4 {
    #[inline(always)]
    fn from(val: u8) -> OutPin4 {
        OutPin4::from_bits(val)
    }
}
impl From<OutPin4> for u8 {
    #[inline(always)]
    fn from(val: OutPin4) -> u8 {
        OutPin4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin5 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin5 {
    #[inline(always)]
    fn from(val: u8) -> OutPin5 {
        OutPin5::from_bits(val)
    }
}
impl From<OutPin5> for u8 {
    #[inline(always)]
    fn from(val: OutPin5) -> u8 {
        OutPin5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin6 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin6 {
    #[inline(always)]
    fn from(val: u8) -> OutPin6 {
        OutPin6::from_bits(val)
    }
}
impl From<OutPin6> for u8 {
    #[inline(always)]
    fn from(val: OutPin6) -> u8 {
        OutPin6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin7 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin7 {
    #[inline(always)]
    fn from(val: u8) -> OutPin7 {
        OutPin7::from_bits(val)
    }
}
impl From<OutPin7> for u8 {
    #[inline(always)]
    fn from(val: OutPin7) -> u8 {
        OutPin7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin8 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin8 {
    #[inline(always)]
    fn from(val: u8) -> OutPin8 {
        OutPin8::from_bits(val)
    }
}
impl From<OutPin8> for u8 {
    #[inline(always)]
    fn from(val: OutPin8) -> u8 {
        OutPin8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutPin9 {
    #[doc = "Pin driver is low"]
    LOW = 0x0,
    #[doc = "Pin driver is high"]
    HIGH = 0x01,
}
impl OutPin9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutPin9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutPin9 {
    #[inline(always)]
    fn from(val: u8) -> OutPin9 {
        OutPin9::from_bits(val)
    }
}
impl From<OutPin9> for u8 {
    #[inline(always)]
    fn from(val: OutPin9) -> u8 {
        OutPin9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin0 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin0 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin0 {
        OutclrPin0::from_bits(val)
    }
}
impl From<OutclrPin0> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin0) -> u8 {
        OutclrPin0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin1 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin1 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin1 {
        OutclrPin1::from_bits(val)
    }
}
impl From<OutclrPin1> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin1) -> u8 {
        OutclrPin1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin10 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin10 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin10 {
        OutclrPin10::from_bits(val)
    }
}
impl From<OutclrPin10> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin10) -> u8 {
        OutclrPin10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin11 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin11 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin11 {
        OutclrPin11::from_bits(val)
    }
}
impl From<OutclrPin11> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin11) -> u8 {
        OutclrPin11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin12 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin12 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin12 {
        OutclrPin12::from_bits(val)
    }
}
impl From<OutclrPin12> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin12) -> u8 {
        OutclrPin12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin13 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin13 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin13 {
        OutclrPin13::from_bits(val)
    }
}
impl From<OutclrPin13> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin13) -> u8 {
        OutclrPin13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin14 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin14 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin14 {
        OutclrPin14::from_bits(val)
    }
}
impl From<OutclrPin14> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin14) -> u8 {
        OutclrPin14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin15 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin15 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin15 {
        OutclrPin15::from_bits(val)
    }
}
impl From<OutclrPin15> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin15) -> u8 {
        OutclrPin15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin16 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin16 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin16 {
        OutclrPin16::from_bits(val)
    }
}
impl From<OutclrPin16> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin16) -> u8 {
        OutclrPin16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin17 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin17 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin17 {
        OutclrPin17::from_bits(val)
    }
}
impl From<OutclrPin17> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin17) -> u8 {
        OutclrPin17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin18 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin18 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin18 {
        OutclrPin18::from_bits(val)
    }
}
impl From<OutclrPin18> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin18) -> u8 {
        OutclrPin18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin19 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin19 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin19 {
        OutclrPin19::from_bits(val)
    }
}
impl From<OutclrPin19> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin19) -> u8 {
        OutclrPin19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin2 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin2 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin2 {
        OutclrPin2::from_bits(val)
    }
}
impl From<OutclrPin2> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin2) -> u8 {
        OutclrPin2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin20 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin20 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin20 {
        OutclrPin20::from_bits(val)
    }
}
impl From<OutclrPin20> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin20) -> u8 {
        OutclrPin20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin21 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin21 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin21 {
        OutclrPin21::from_bits(val)
    }
}
impl From<OutclrPin21> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin21) -> u8 {
        OutclrPin21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin22 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin22 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin22 {
        OutclrPin22::from_bits(val)
    }
}
impl From<OutclrPin22> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin22) -> u8 {
        OutclrPin22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin23 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin23 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin23 {
        OutclrPin23::from_bits(val)
    }
}
impl From<OutclrPin23> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin23) -> u8 {
        OutclrPin23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin24 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin24 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin24 {
        OutclrPin24::from_bits(val)
    }
}
impl From<OutclrPin24> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin24) -> u8 {
        OutclrPin24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin25 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin25 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin25 {
        OutclrPin25::from_bits(val)
    }
}
impl From<OutclrPin25> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin25) -> u8 {
        OutclrPin25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin26 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin26 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin26 {
        OutclrPin26::from_bits(val)
    }
}
impl From<OutclrPin26> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin26) -> u8 {
        OutclrPin26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin27 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin27 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin27 {
        OutclrPin27::from_bits(val)
    }
}
impl From<OutclrPin27> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin27) -> u8 {
        OutclrPin27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin28 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin28 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin28 {
        OutclrPin28::from_bits(val)
    }
}
impl From<OutclrPin28> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin28) -> u8 {
        OutclrPin28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin29 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin29 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin29 {
        OutclrPin29::from_bits(val)
    }
}
impl From<OutclrPin29> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin29) -> u8 {
        OutclrPin29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin3 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin3 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin3 {
        OutclrPin3::from_bits(val)
    }
}
impl From<OutclrPin3> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin3) -> u8 {
        OutclrPin3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin30 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin30 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin30 {
        OutclrPin30::from_bits(val)
    }
}
impl From<OutclrPin30> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin30) -> u8 {
        OutclrPin30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin31 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin31 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin31 {
        OutclrPin31::from_bits(val)
    }
}
impl From<OutclrPin31> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin31) -> u8 {
        OutclrPin31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin4 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin4 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin4 {
        OutclrPin4::from_bits(val)
    }
}
impl From<OutclrPin4> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin4) -> u8 {
        OutclrPin4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin5 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin5 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin5 {
        OutclrPin5::from_bits(val)
    }
}
impl From<OutclrPin5> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin5) -> u8 {
        OutclrPin5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin6 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin6 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin6 {
        OutclrPin6::from_bits(val)
    }
}
impl From<OutclrPin6> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin6) -> u8 {
        OutclrPin6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin7 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin7 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin7 {
        OutclrPin7::from_bits(val)
    }
}
impl From<OutclrPin7> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin7) -> u8 {
        OutclrPin7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin8 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin8 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin8 {
        OutclrPin8::from_bits(val)
    }
}
impl From<OutclrPin8> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin8) -> u8 {
        OutclrPin8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutclrPin9 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_CLEAR = 0x01,
}
impl OutclrPin9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutclrPin9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutclrPin9 {
    #[inline(always)]
    fn from(val: u8) -> OutclrPin9 {
        OutclrPin9::from_bits(val)
    }
}
impl From<OutclrPin9> for u8 {
    #[inline(always)]
    fn from(val: OutclrPin9) -> u8 {
        OutclrPin9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin0 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin0 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin0 {
        OutsetPin0::from_bits(val)
    }
}
impl From<OutsetPin0> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin0) -> u8 {
        OutsetPin0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin1 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin1 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin1 {
        OutsetPin1::from_bits(val)
    }
}
impl From<OutsetPin1> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin1) -> u8 {
        OutsetPin1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin10 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin10 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin10 {
        OutsetPin10::from_bits(val)
    }
}
impl From<OutsetPin10> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin10) -> u8 {
        OutsetPin10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin11 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin11 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin11 {
        OutsetPin11::from_bits(val)
    }
}
impl From<OutsetPin11> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin11) -> u8 {
        OutsetPin11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin12 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin12 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin12 {
        OutsetPin12::from_bits(val)
    }
}
impl From<OutsetPin12> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin12) -> u8 {
        OutsetPin12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin13 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin13 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin13 {
        OutsetPin13::from_bits(val)
    }
}
impl From<OutsetPin13> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin13) -> u8 {
        OutsetPin13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin14 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin14 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin14 {
        OutsetPin14::from_bits(val)
    }
}
impl From<OutsetPin14> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin14) -> u8 {
        OutsetPin14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin15 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin15 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin15 {
        OutsetPin15::from_bits(val)
    }
}
impl From<OutsetPin15> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin15) -> u8 {
        OutsetPin15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin16 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin16 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin16 {
        OutsetPin16::from_bits(val)
    }
}
impl From<OutsetPin16> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin16) -> u8 {
        OutsetPin16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin17 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin17 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin17 {
        OutsetPin17::from_bits(val)
    }
}
impl From<OutsetPin17> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin17) -> u8 {
        OutsetPin17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin18 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin18 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin18 {
        OutsetPin18::from_bits(val)
    }
}
impl From<OutsetPin18> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin18) -> u8 {
        OutsetPin18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin19 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin19 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin19 {
        OutsetPin19::from_bits(val)
    }
}
impl From<OutsetPin19> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin19) -> u8 {
        OutsetPin19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin2 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin2 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin2 {
        OutsetPin2::from_bits(val)
    }
}
impl From<OutsetPin2> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin2) -> u8 {
        OutsetPin2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin20 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin20 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin20 {
        OutsetPin20::from_bits(val)
    }
}
impl From<OutsetPin20> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin20) -> u8 {
        OutsetPin20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin21 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin21 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin21 {
        OutsetPin21::from_bits(val)
    }
}
impl From<OutsetPin21> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin21) -> u8 {
        OutsetPin21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin22 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin22 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin22 {
        OutsetPin22::from_bits(val)
    }
}
impl From<OutsetPin22> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin22) -> u8 {
        OutsetPin22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin23 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin23 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin23 {
        OutsetPin23::from_bits(val)
    }
}
impl From<OutsetPin23> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin23) -> u8 {
        OutsetPin23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin24 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin24 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin24 {
        OutsetPin24::from_bits(val)
    }
}
impl From<OutsetPin24> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin24) -> u8 {
        OutsetPin24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin25 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin25 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin25 {
        OutsetPin25::from_bits(val)
    }
}
impl From<OutsetPin25> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin25) -> u8 {
        OutsetPin25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin26 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin26 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin26 {
        OutsetPin26::from_bits(val)
    }
}
impl From<OutsetPin26> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin26) -> u8 {
        OutsetPin26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin27 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin27 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin27 {
        OutsetPin27::from_bits(val)
    }
}
impl From<OutsetPin27> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin27) -> u8 {
        OutsetPin27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin28 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin28 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin28 {
        OutsetPin28::from_bits(val)
    }
}
impl From<OutsetPin28> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin28) -> u8 {
        OutsetPin28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin29 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin29 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin29 {
        OutsetPin29::from_bits(val)
    }
}
impl From<OutsetPin29> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin29) -> u8 {
        OutsetPin29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin3 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin3 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin3 {
        OutsetPin3::from_bits(val)
    }
}
impl From<OutsetPin3> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin3) -> u8 {
        OutsetPin3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin30 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin30 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin30 {
        OutsetPin30::from_bits(val)
    }
}
impl From<OutsetPin30> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin30) -> u8 {
        OutsetPin30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin31 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin31 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin31 {
        OutsetPin31::from_bits(val)
    }
}
impl From<OutsetPin31> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin31) -> u8 {
        OutsetPin31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin4 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin4 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin4 {
        OutsetPin4::from_bits(val)
    }
}
impl From<OutsetPin4> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin4) -> u8 {
        OutsetPin4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin5 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin5 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin5 {
        OutsetPin5::from_bits(val)
    }
}
impl From<OutsetPin5> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin5) -> u8 {
        OutsetPin5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin6 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin6 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin6 {
        OutsetPin6::from_bits(val)
    }
}
impl From<OutsetPin6> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin6) -> u8 {
        OutsetPin6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin7 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin7 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin7 {
        OutsetPin7::from_bits(val)
    }
}
impl From<OutsetPin7> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin7) -> u8 {
        OutsetPin7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin8 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin8 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin8 {
        OutsetPin8::from_bits(val)
    }
}
impl From<OutsetPin8> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin8) -> u8 {
        OutsetPin8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutsetPin9 {
    #[doc = "Read: pin driver is low"]
    LOW = 0x0,
    #[doc = "Read: pin driver is high"]
    R_HIGH_W_SET = 0x01,
}
impl OutsetPin9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutsetPin9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutsetPin9 {
    #[inline(always)]
    fn from(val: u8) -> OutsetPin9 {
        OutsetPin9::from_bits(val)
    }
}
impl From<OutsetPin9> for u8 {
    #[inline(always)]
    fn from(val: OutsetPin9) -> u8 {
        OutsetPin9::to_bits(val)
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
