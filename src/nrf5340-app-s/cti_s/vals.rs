#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appclear0 {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the event for channel 0."]
    CLEAR = 0x01,
}
impl Appclear0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appclear0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appclear0 {
    #[inline(always)]
    fn from(val: u8) -> Appclear0 {
        Appclear0::from_bits(val)
    }
}
impl From<Appclear0> for u8 {
    #[inline(always)]
    fn from(val: Appclear0) -> u8 {
        Appclear0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appclear1 {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the event for channel 1."]
    CLEAR = 0x01,
}
impl Appclear1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appclear1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appclear1 {
    #[inline(always)]
    fn from(val: u8) -> Appclear1 {
        Appclear1::from_bits(val)
    }
}
impl From<Appclear1> for u8 {
    #[inline(always)]
    fn from(val: Appclear1) -> u8 {
        Appclear1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appclear2 {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the event for channel 2."]
    CLEAR = 0x01,
}
impl Appclear2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appclear2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appclear2 {
    #[inline(always)]
    fn from(val: u8) -> Appclear2 {
        Appclear2::from_bits(val)
    }
}
impl From<Appclear2> for u8 {
    #[inline(always)]
    fn from(val: Appclear2) -> u8 {
        Appclear2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appclear3 {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the event for channel 3."]
    CLEAR = 0x01,
}
impl Appclear3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appclear3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appclear3 {
    #[inline(always)]
    fn from(val: u8) -> Appclear3 {
        Appclear3::from_bits(val)
    }
}
impl From<Appclear3> for u8 {
    #[inline(always)]
    fn from(val: Appclear3) -> u8 {
        Appclear3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appset0 {
    #[doc = "Application trigger 0 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Application trigger 0 is active."]
    R_ACTIVE_W_ACTIVATE = 0x01,
}
impl Appset0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appset0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appset0 {
    #[inline(always)]
    fn from(val: u8) -> Appset0 {
        Appset0::from_bits(val)
    }
}
impl From<Appset0> for u8 {
    #[inline(always)]
    fn from(val: Appset0) -> u8 {
        Appset0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appset1 {
    #[doc = "Application trigger 1 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Application trigger 1 is active."]
    R_ACTIVE_W_ACTIVATE = 0x01,
}
impl Appset1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appset1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appset1 {
    #[inline(always)]
    fn from(val: u8) -> Appset1 {
        Appset1::from_bits(val)
    }
}
impl From<Appset1> for u8 {
    #[inline(always)]
    fn from(val: Appset1) -> u8 {
        Appset1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appset2 {
    #[doc = "Application trigger 2 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Application trigger 2 is active."]
    R_ACTIVE_W_ACTIVATE = 0x01,
}
impl Appset2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appset2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appset2 {
    #[inline(always)]
    fn from(val: u8) -> Appset2 {
        Appset2::from_bits(val)
    }
}
impl From<Appset2> for u8 {
    #[inline(always)]
    fn from(val: Appset2) -> u8 {
        Appset2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appset3 {
    #[doc = "Application trigger 3 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Application trigger 3 is active."]
    R_ACTIVE_W_ACTIVATE = 0x01,
}
impl Appset3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appset3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appset3 {
    #[inline(always)]
    fn from(val: u8) -> Appset3 {
        Appset3::from_bits(val)
    }
}
impl From<Appset3> for u8 {
    #[inline(always)]
    fn from(val: Appset3) -> u8 {
        Appset3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appulse0 {
    _RESERVED_0 = 0x0,
    #[doc = "Generates an event pulse on channel 0."]
    GENERATE = 0x01,
}
impl Appulse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appulse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appulse0 {
    #[inline(always)]
    fn from(val: u8) -> Appulse0 {
        Appulse0::from_bits(val)
    }
}
impl From<Appulse0> for u8 {
    #[inline(always)]
    fn from(val: Appulse0) -> u8 {
        Appulse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appulse1 {
    _RESERVED_0 = 0x0,
    #[doc = "Generates an event pulse on channel 1."]
    GENERATE = 0x01,
}
impl Appulse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appulse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appulse1 {
    #[inline(always)]
    fn from(val: u8) -> Appulse1 {
        Appulse1::from_bits(val)
    }
}
impl From<Appulse1> for u8 {
    #[inline(always)]
    fn from(val: Appulse1) -> u8 {
        Appulse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appulse2 {
    _RESERVED_0 = 0x0,
    #[doc = "Generates an event pulse on channel 2."]
    GENERATE = 0x01,
}
impl Appulse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appulse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appulse2 {
    #[inline(always)]
    fn from(val: u8) -> Appulse2 {
        Appulse2::from_bits(val)
    }
}
impl From<Appulse2> for u8 {
    #[inline(always)]
    fn from(val: Appulse2) -> u8 {
        Appulse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Appulse3 {
    _RESERVED_0 = 0x0,
    #[doc = "Generates an event pulse on channel 3."]
    GENERATE = 0x01,
}
impl Appulse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Appulse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Appulse3 {
    #[inline(always)]
    fn from(val: u8) -> Appulse3 {
        Appulse3::from_bits(val)
    }
}
impl From<Appulse3> for u8 {
    #[inline(always)]
    fn from(val: Appulse3) -> u8 {
        Appulse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Class {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Indicates that the component is a CoreSight component."]
    CORESIGHT = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Class {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Class {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Class {
    #[inline(always)]
    fn from(val: u8) -> Class {
        Class::from_bits(val)
    }
}
impl From<Class> for u8 {
    #[inline(always)]
    fn from(val: Class) -> u8 {
        Class::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmod {
    #[doc = "Indicates that the customer has not modified this component."]
    UNMODIFIED = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmod {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmod {
    #[inline(always)]
    fn from(val: u8) -> Cmod {
        Cmod::from_bits(val)
    }
}
impl From<Cmod> for u8 {
    #[inline(always)]
    fn from(val: Cmod) -> u8 {
        Cmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cpuhalted {
    #[doc = "Ctitrigin 0 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigin 0 is active."]
    ACTIVE = 0x01,
}
impl Cpuhalted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpuhalted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpuhalted {
    #[inline(always)]
    fn from(val: u8) -> Cpuhalted {
        Cpuhalted::from_bits(val)
    }
}
impl From<Cpuhalted> for u8 {
    #[inline(always)]
    fn from(val: Cpuhalted) -> u8 {
        Cpuhalted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctichinstatus0 {
    #[doc = "Ctichin 0 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctichin 0 is active."]
    ACTIVE = 0x01,
}
impl Ctichinstatus0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctichinstatus0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctichinstatus0 {
    #[inline(always)]
    fn from(val: u8) -> Ctichinstatus0 {
        Ctichinstatus0::from_bits(val)
    }
}
impl From<Ctichinstatus0> for u8 {
    #[inline(always)]
    fn from(val: Ctichinstatus0) -> u8 {
        Ctichinstatus0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctichinstatus1 {
    #[doc = "Ctichin 1 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctichin 1 is active."]
    ACTIVE = 0x01,
}
impl Ctichinstatus1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctichinstatus1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctichinstatus1 {
    #[inline(always)]
    fn from(val: u8) -> Ctichinstatus1 {
        Ctichinstatus1::from_bits(val)
    }
}
impl From<Ctichinstatus1> for u8 {
    #[inline(always)]
    fn from(val: Ctichinstatus1) -> u8 {
        Ctichinstatus1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctichinstatus2 {
    #[doc = "Ctichin 2 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctichin 2 is active."]
    ACTIVE = 0x01,
}
impl Ctichinstatus2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctichinstatus2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctichinstatus2 {
    #[inline(always)]
    fn from(val: u8) -> Ctichinstatus2 {
        Ctichinstatus2::from_bits(val)
    }
}
impl From<Ctichinstatus2> for u8 {
    #[inline(always)]
    fn from(val: Ctichinstatus2) -> u8 {
        Ctichinstatus2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctichinstatus3 {
    #[doc = "Ctichin 3 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctichin 3 is active."]
    ACTIVE = 0x01,
}
impl Ctichinstatus3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctichinstatus3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctichinstatus3 {
    #[inline(always)]
    fn from(val: u8) -> Ctichinstatus3 {
        Ctichinstatus3::from_bits(val)
    }
}
impl From<Ctichinstatus3> for u8 {
    #[inline(always)]
    fn from(val: Ctichinstatus3) -> u8 {
        Ctichinstatus3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctigateen0 {
    #[doc = "Disable ctichout channel 0 propagation."]
    DISABLED = 0x0,
    #[doc = "Enable ctichout channel 0 propagation."]
    ENABLED = 0x01,
}
impl Ctigateen0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctigateen0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctigateen0 {
    #[inline(always)]
    fn from(val: u8) -> Ctigateen0 {
        Ctigateen0::from_bits(val)
    }
}
impl From<Ctigateen0> for u8 {
    #[inline(always)]
    fn from(val: Ctigateen0) -> u8 {
        Ctigateen0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctigateen1 {
    #[doc = "Disable ctichout channel 1 propagation."]
    DISABLED = 0x0,
    #[doc = "Enable ctichout channel 1 propagation."]
    ENABLED = 0x01,
}
impl Ctigateen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctigateen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctigateen1 {
    #[inline(always)]
    fn from(val: u8) -> Ctigateen1 {
        Ctigateen1::from_bits(val)
    }
}
impl From<Ctigateen1> for u8 {
    #[inline(always)]
    fn from(val: Ctigateen1) -> u8 {
        Ctigateen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctigateen2 {
    #[doc = "Disable ctichout channel 2 propagation."]
    DISABLED = 0x0,
    #[doc = "Enable ctichout channel 2 propagation."]
    ENABLED = 0x01,
}
impl Ctigateen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctigateen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctigateen2 {
    #[inline(always)]
    fn from(val: u8) -> Ctigateen2 {
        Ctigateen2::from_bits(val)
    }
}
impl From<Ctigateen2> for u8 {
    #[inline(always)]
    fn from(val: Ctigateen2) -> u8 {
        Ctigateen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctigateen3 {
    #[doc = "Disable ctichout channel 3 propagation."]
    DISABLED = 0x0,
    #[doc = "Enable ctichout channel 3 propagation."]
    ENABLED = 0x01,
}
impl Ctigateen3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctigateen3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctigateen3 {
    #[inline(always)]
    fn from(val: u8) -> Ctigateen3 {
        Ctigateen3::from_bits(val)
    }
}
impl From<Ctigateen3> for u8 {
    #[inline(always)]
    fn from(val: Ctigateen3) -> u8 {
        Ctigateen3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtiintackCpurestart {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the ctitrigout."]
    ACKNOWLEDGE = 0x01,
}
impl CtiintackCpurestart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtiintackCpurestart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtiintackCpurestart {
    #[inline(always)]
    fn from(val: u8) -> CtiintackCpurestart {
        CtiintackCpurestart::from_bits(val)
    }
}
impl From<CtiintackCpurestart> for u8 {
    #[inline(always)]
    fn from(val: CtiintackCpurestart) -> u8 {
        CtiintackCpurestart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtiintackDebugreq {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the ctitrigout."]
    ACKNOWLEDGE = 0x01,
}
impl CtiintackDebugreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtiintackDebugreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtiintackDebugreq {
    #[inline(always)]
    fn from(val: u8) -> CtiintackDebugreq {
        CtiintackDebugreq::from_bits(val)
    }
}
impl From<CtiintackDebugreq> for u8 {
    #[inline(always)]
    fn from(val: CtiintackDebugreq) -> u8 {
        CtiintackDebugreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtiintackEtmevtin0 {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the ctitrigout."]
    ACKNOWLEDGE = 0x01,
}
impl CtiintackEtmevtin0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtiintackEtmevtin0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtiintackEtmevtin0 {
    #[inline(always)]
    fn from(val: u8) -> CtiintackEtmevtin0 {
        CtiintackEtmevtin0::from_bits(val)
    }
}
impl From<CtiintackEtmevtin0> for u8 {
    #[inline(always)]
    fn from(val: CtiintackEtmevtin0) -> u8 {
        CtiintackEtmevtin0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtiintackEtmevtin1 {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the ctitrigout."]
    ACKNOWLEDGE = 0x01,
}
impl CtiintackEtmevtin1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtiintackEtmevtin1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtiintackEtmevtin1 {
    #[inline(always)]
    fn from(val: u8) -> CtiintackEtmevtin1 {
        CtiintackEtmevtin1::from_bits(val)
    }
}
impl From<CtiintackEtmevtin1> for u8 {
    #[inline(always)]
    fn from(val: CtiintackEtmevtin1) -> u8 {
        CtiintackEtmevtin1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtiintackEtmevtin2 {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the ctitrigout."]
    ACKNOWLEDGE = 0x01,
}
impl CtiintackEtmevtin2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtiintackEtmevtin2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtiintackEtmevtin2 {
    #[inline(always)]
    fn from(val: u8) -> CtiintackEtmevtin2 {
        CtiintackEtmevtin2::from_bits(val)
    }
}
impl From<CtiintackEtmevtin2> for u8 {
    #[inline(always)]
    fn from(val: CtiintackEtmevtin2) -> u8 {
        CtiintackEtmevtin2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtiintackEtmevtin3 {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the ctitrigout."]
    ACKNOWLEDGE = 0x01,
}
impl CtiintackEtmevtin3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtiintackEtmevtin3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtiintackEtmevtin3 {
    #[inline(always)]
    fn from(val: u8) -> CtiintackEtmevtin3 {
        CtiintackEtmevtin3::from_bits(val)
    }
}
impl From<CtiintackEtmevtin3> for u8 {
    #[inline(always)]
    fn from(val: CtiintackEtmevtin3) -> u8 {
        CtiintackEtmevtin3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtiintackUnused0 {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the ctitrigout."]
    ACKNOWLEDGE = 0x01,
}
impl CtiintackUnused0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtiintackUnused0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtiintackUnused0 {
    #[inline(always)]
    fn from(val: u8) -> CtiintackUnused0 {
        CtiintackUnused0::from_bits(val)
    }
}
impl From<CtiintackUnused0> for u8 {
    #[inline(always)]
    fn from(val: CtiintackUnused0) -> u8 {
        CtiintackUnused0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtiintackUnused1 {
    _RESERVED_0 = 0x0,
    #[doc = "Clears the ctitrigout."]
    ACKNOWLEDGE = 0x01,
}
impl CtiintackUnused1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtiintackUnused1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtiintackUnused1 {
    #[inline(always)]
    fn from(val: u8) -> CtiintackUnused1 {
        CtiintackUnused1::from_bits(val)
    }
}
impl From<CtiintackUnused1> for u8 {
    #[inline(always)]
    fn from(val: CtiintackUnused1) -> u8 {
        CtiintackUnused1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtitriginstatusUnused0 {
    #[doc = "Ctitrigin 6 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigin 6 is active."]
    ACTIVE = 0x01,
}
impl CtitriginstatusUnused0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtitriginstatusUnused0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtitriginstatusUnused0 {
    #[inline(always)]
    fn from(val: u8) -> CtitriginstatusUnused0 {
        CtitriginstatusUnused0::from_bits(val)
    }
}
impl From<CtitriginstatusUnused0> for u8 {
    #[inline(always)]
    fn from(val: CtitriginstatusUnused0) -> u8 {
        CtitriginstatusUnused0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtitriginstatusUnused1 {
    #[doc = "Ctitrigin 7 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigin 7 is active."]
    ACTIVE = 0x01,
}
impl CtitriginstatusUnused1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtitriginstatusUnused1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtitriginstatusUnused1 {
    #[inline(always)]
    fn from(val: u8) -> CtitriginstatusUnused1 {
        CtitriginstatusUnused1::from_bits(val)
    }
}
impl From<CtitriginstatusUnused1> for u8 {
    #[inline(always)]
    fn from(val: CtitriginstatusUnused1) -> u8 {
        CtitriginstatusUnused1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtitrigoutstatusCpurestart {
    #[doc = "Ctitrigout 1 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigout 1 is active."]
    ACTIVE = 0x01,
}
impl CtitrigoutstatusCpurestart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtitrigoutstatusCpurestart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtitrigoutstatusCpurestart {
    #[inline(always)]
    fn from(val: u8) -> CtitrigoutstatusCpurestart {
        CtitrigoutstatusCpurestart::from_bits(val)
    }
}
impl From<CtitrigoutstatusCpurestart> for u8 {
    #[inline(always)]
    fn from(val: CtitrigoutstatusCpurestart) -> u8 {
        CtitrigoutstatusCpurestart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtitrigoutstatusDebugreq {
    #[doc = "Ctitrigout 0 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigout 0 is active."]
    ACTIVE = 0x01,
}
impl CtitrigoutstatusDebugreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtitrigoutstatusDebugreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtitrigoutstatusDebugreq {
    #[inline(always)]
    fn from(val: u8) -> CtitrigoutstatusDebugreq {
        CtitrigoutstatusDebugreq::from_bits(val)
    }
}
impl From<CtitrigoutstatusDebugreq> for u8 {
    #[inline(always)]
    fn from(val: CtitrigoutstatusDebugreq) -> u8 {
        CtitrigoutstatusDebugreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtitrigoutstatusEtmevtin0 {
    #[doc = "Ctitrigout 4 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigout 4 is active."]
    ACTIVE = 0x01,
}
impl CtitrigoutstatusEtmevtin0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtitrigoutstatusEtmevtin0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtitrigoutstatusEtmevtin0 {
    #[inline(always)]
    fn from(val: u8) -> CtitrigoutstatusEtmevtin0 {
        CtitrigoutstatusEtmevtin0::from_bits(val)
    }
}
impl From<CtitrigoutstatusEtmevtin0> for u8 {
    #[inline(always)]
    fn from(val: CtitrigoutstatusEtmevtin0) -> u8 {
        CtitrigoutstatusEtmevtin0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtitrigoutstatusEtmevtin1 {
    #[doc = "Ctitrigout 5 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigout 5 is active."]
    ACTIVE = 0x01,
}
impl CtitrigoutstatusEtmevtin1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtitrigoutstatusEtmevtin1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtitrigoutstatusEtmevtin1 {
    #[inline(always)]
    fn from(val: u8) -> CtitrigoutstatusEtmevtin1 {
        CtitrigoutstatusEtmevtin1::from_bits(val)
    }
}
impl From<CtitrigoutstatusEtmevtin1> for u8 {
    #[inline(always)]
    fn from(val: CtitrigoutstatusEtmevtin1) -> u8 {
        CtitrigoutstatusEtmevtin1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtitrigoutstatusEtmevtin2 {
    #[doc = "Ctitrigout 6 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigout 6 is active."]
    ACTIVE = 0x01,
}
impl CtitrigoutstatusEtmevtin2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtitrigoutstatusEtmevtin2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtitrigoutstatusEtmevtin2 {
    #[inline(always)]
    fn from(val: u8) -> CtitrigoutstatusEtmevtin2 {
        CtitrigoutstatusEtmevtin2::from_bits(val)
    }
}
impl From<CtitrigoutstatusEtmevtin2> for u8 {
    #[inline(always)]
    fn from(val: CtitrigoutstatusEtmevtin2) -> u8 {
        CtitrigoutstatusEtmevtin2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtitrigoutstatusEtmevtin3 {
    #[doc = "Ctitrigout 7 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigout 7 is active."]
    ACTIVE = 0x01,
}
impl CtitrigoutstatusEtmevtin3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtitrigoutstatusEtmevtin3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtitrigoutstatusEtmevtin3 {
    #[inline(always)]
    fn from(val: u8) -> CtitrigoutstatusEtmevtin3 {
        CtitrigoutstatusEtmevtin3::from_bits(val)
    }
}
impl From<CtitrigoutstatusEtmevtin3> for u8 {
    #[inline(always)]
    fn from(val: CtitrigoutstatusEtmevtin3) -> u8 {
        CtitrigoutstatusEtmevtin3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtitrigoutstatusUnused0 {
    #[doc = "Ctitrigout 2 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigout 2 is active."]
    ACTIVE = 0x01,
}
impl CtitrigoutstatusUnused0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtitrigoutstatusUnused0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtitrigoutstatusUnused0 {
    #[inline(always)]
    fn from(val: u8) -> CtitrigoutstatusUnused0 {
        CtitrigoutstatusUnused0::from_bits(val)
    }
}
impl From<CtitrigoutstatusUnused0> for u8 {
    #[inline(always)]
    fn from(val: CtitrigoutstatusUnused0) -> u8 {
        CtitrigoutstatusUnused0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtitrigoutstatusUnused1 {
    #[doc = "Ctitrigout 3 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigout 3 is active."]
    ACTIVE = 0x01,
}
impl CtitrigoutstatusUnused1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtitrigoutstatusUnused1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtitrigoutstatusUnused1 {
    #[inline(always)]
    fn from(val: u8) -> CtitrigoutstatusUnused1 {
        CtitrigoutstatusUnused1::from_bits(val)
    }
}
impl From<CtitrigoutstatusUnused1> for u8 {
    #[inline(always)]
    fn from(val: CtitrigoutstatusUnused1) -> u8 {
        CtitrigoutstatusUnused1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Des0 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "Arm. Bits\\[3:0\\] of the JEDEC JEP106 Identity Code"]
    ARM = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Des0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Des0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Des0 {
    #[inline(always)]
    fn from(val: u8) -> Des0 {
        Des0::from_bits(val)
    }
}
impl From<Des0> for u8 {
    #[inline(always)]
    fn from(val: Des0) -> u8 {
        Des0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Des1 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Arm. Bits\\[6:4\\] of the JEDEC JEP106 Identity Code"]
    ARM = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Des1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Des1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Des1 {
    #[inline(always)]
    fn from(val: u8) -> Des1 {
        Des1::from_bits(val)
    }
}
impl From<Des1> for u8 {
    #[inline(always)]
    fn from(val: Des1) -> u8 {
        Des1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Des2 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "JEDEC continuation code."]
    CODE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Des2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Des2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Des2 {
    #[inline(always)]
    fn from(val: u8) -> Des2 {
        Des2::from_bits(val)
    }
}
impl From<Des2> for u8 {
    #[inline(always)]
    fn from(val: Des2) -> u8 {
        Des2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dwtcompout0 {
    #[doc = "Ctitrigin 1 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigin 1 is active."]
    ACTIVE = 0x01,
}
impl Dwtcompout0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dwtcompout0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dwtcompout0 {
    #[inline(always)]
    fn from(val: u8) -> Dwtcompout0 {
        Dwtcompout0::from_bits(val)
    }
}
impl From<Dwtcompout0> for u8 {
    #[inline(always)]
    fn from(val: Dwtcompout0) -> u8 {
        Dwtcompout0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dwtcompout1 {
    #[doc = "Ctitrigin 2 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigin 2 is active."]
    ACTIVE = 0x01,
}
impl Dwtcompout1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dwtcompout1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dwtcompout1 {
    #[inline(always)]
    fn from(val: u8) -> Dwtcompout1 {
        Dwtcompout1::from_bits(val)
    }
}
impl From<Dwtcompout1> for u8 {
    #[inline(always)]
    fn from(val: Dwtcompout1) -> u8 {
        Dwtcompout1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dwtcompout2 {
    #[doc = "Ctitrigin 3 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigin 3 is active."]
    ACTIVE = 0x01,
}
impl Dwtcompout2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dwtcompout2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dwtcompout2 {
    #[inline(always)]
    fn from(val: u8) -> Dwtcompout2 {
        Dwtcompout2::from_bits(val)
    }
}
impl From<Dwtcompout2> for u8 {
    #[inline(always)]
    fn from(val: Dwtcompout2) -> u8 {
        Dwtcompout2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Etmevtout0 {
    #[doc = "Ctitrigin 4 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigin 4 is active."]
    ACTIVE = 0x01,
}
impl Etmevtout0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Etmevtout0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Etmevtout0 {
    #[inline(always)]
    fn from(val: u8) -> Etmevtout0 {
        Etmevtout0::from_bits(val)
    }
}
impl From<Etmevtout0> for u8 {
    #[inline(always)]
    fn from(val: Etmevtout0) -> u8 {
        Etmevtout0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Etmevtout1 {
    #[doc = "Ctitrigin 5 is inactive."]
    INACTIVE = 0x0,
    #[doc = "Ctitrigin 5 is active."]
    ACTIVE = 0x01,
}
impl Etmevtout1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Etmevtout1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Etmevtout1 {
    #[inline(always)]
    fn from(val: u8) -> Etmevtout1 {
        Etmevtout1::from_bits(val)
    }
}
impl From<Etmevtout1> for u8 {
    #[inline(always)]
    fn from(val: Etmevtout1) -> u8 {
        Etmevtout1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Glben {
    #[doc = "All cross-triggering mapping logic functionality is disabled."]
    DISABLED = 0x0,
    #[doc = "Cross-triggering mapping logic functionality is enabled."]
    ENABLED = 0x01,
}
impl Glben {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Glben {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Glben {
    #[inline(always)]
    fn from(val: u8) -> Glben {
        Glben::from_bits(val)
    }
}
impl From<Glben> for u8 {
    #[inline(always)]
    fn from(val: Glben) -> u8 {
        Glben::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Major {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Indicates that this component allows a debugger to control other components in an Arm CoreSight SoC-400 system."]
    CONTROLLER = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Major {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Major {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Major {
    #[inline(always)]
    fn from(val: u8) -> Major {
        Major::from_bits(val)
    }
}
impl From<Major> for u8 {
    #[inline(always)]
    fn from(val: Major) -> u8 {
        Major::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Part0(pub u8);
impl Part0 {
    #[doc = "Indicates bits\\[7:0\\] of the part number of the component."]
    pub const PARTNUMBERL: Self = Self(0x21);
}
impl Part0 {
    pub const fn from_bits(val: u8) -> Part0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Part0 {
    #[inline(always)]
    fn from(val: u8) -> Part0 {
        Part0::from_bits(val)
    }
}
impl From<Part0> for u8 {
    #[inline(always)]
    fn from(val: Part0) -> u8 {
        Part0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Part1 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "Indicates bits\\[11:8\\] of the part number of the component."]
    PARTNUMBERH = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Part1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Part1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Part1 {
    #[inline(always)]
    fn from(val: u8) -> Part1 {
        Part1::from_bits(val)
    }
}
impl From<Part1> for u8 {
    #[inline(always)]
    fn from(val: Part1) -> u8 {
        Part1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prmbl0(pub u8);
impl Prmbl0 {
    #[doc = "Bits\\[7:0\\] of the identification code."]
    pub const VALUE: Self = Self(0x0d);
}
impl Prmbl0 {
    pub const fn from_bits(val: u8) -> Prmbl0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Prmbl0 {
    #[inline(always)]
    fn from(val: u8) -> Prmbl0 {
        Prmbl0::from_bits(val)
    }
}
impl From<Prmbl0> for u8 {
    #[inline(always)]
    fn from(val: Prmbl0) -> u8 {
        Prmbl0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Prmbl1 {
    #[doc = "Bits\\[11:8\\] of the identification code."]
    VALUE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Prmbl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prmbl1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prmbl1 {
    #[inline(always)]
    fn from(val: u8) -> Prmbl1 {
        Prmbl1::from_bits(val)
    }
}
impl From<Prmbl1> for u8 {
    #[inline(always)]
    fn from(val: Prmbl1) -> u8 {
        Prmbl1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prmbl2(pub u8);
impl Prmbl2 {
    #[doc = "Bits\\[23:16\\] of the identification code."]
    pub const VALUE: Self = Self(0x05);
}
impl Prmbl2 {
    pub const fn from_bits(val: u8) -> Prmbl2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Prmbl2 {
    #[inline(always)]
    fn from(val: u8) -> Prmbl2 {
        Prmbl2::from_bits(val)
    }
}
impl From<Prmbl2> for u8 {
    #[inline(always)]
    fn from(val: Prmbl2) -> u8 {
        Prmbl2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prmbl3(pub u8);
impl Prmbl3 {
    #[doc = "Bits\\[31:24\\] of the identification code."]
    pub const VALUE: Self = Self(0xb1);
}
impl Prmbl3 {
    pub const fn from_bits(val: u8) -> Prmbl3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Prmbl3 {
    #[inline(always)]
    fn from(val: u8) -> Prmbl3 {
        Prmbl3::from_bits(val)
    }
}
impl From<Prmbl3> for u8 {
    #[inline(always)]
    fn from(val: Prmbl3) -> u8 {
        Prmbl3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Revand {
    #[doc = "Indicates that there are no errata fixes to this component."]
    NOERRATA = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Revand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Revand {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Revand {
    #[inline(always)]
    fn from(val: u8) -> Revand {
        Revand::from_bits(val)
    }
}
impl From<Revand> for u8 {
    #[inline(always)]
    fn from(val: Revand) -> u8 {
        Revand::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Revision {
    #[doc = "This device is at r0p0"]
    REV0P0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Revision {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Revision {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Revision {
    #[inline(always)]
    fn from(val: u8) -> Revision {
        Revision::from_bits(val)
    }
}
impl From<Revision> for u8 {
    #[inline(always)]
    fn from(val: Revision) -> u8 {
        Revision::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sub {
    _RESERVED_0 = 0x0,
    #[doc = "Indicates that this component is a sub-triggering component."]
    CROSSTRIGGER = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Sub {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sub {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sub {
    #[inline(always)]
    fn from(val: u8) -> Sub {
        Sub::from_bits(val)
    }
}
impl From<Sub> for u8 {
    #[inline(always)]
    fn from(val: Sub) -> u8 {
        Sub::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Triginen0 {
    #[doc = "Input trigger n events are ignored by channel 0."]
    DISABLED = 0x0,
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 0."]
    ENABLED = 0x01,
}
impl Triginen0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Triginen0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Triginen0 {
    #[inline(always)]
    fn from(val: u8) -> Triginen0 {
        Triginen0::from_bits(val)
    }
}
impl From<Triginen0> for u8 {
    #[inline(always)]
    fn from(val: Triginen0) -> u8 {
        Triginen0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Triginen1 {
    #[doc = "Input trigger n events are ignored by channel 1."]
    DISABLED = 0x0,
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 1."]
    ENABLED = 0x01,
}
impl Triginen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Triginen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Triginen1 {
    #[inline(always)]
    fn from(val: u8) -> Triginen1 {
        Triginen1::from_bits(val)
    }
}
impl From<Triginen1> for u8 {
    #[inline(always)]
    fn from(val: Triginen1) -> u8 {
        Triginen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Triginen2 {
    #[doc = "Input trigger n events are ignored by channel 2."]
    DISABLED = 0x0,
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 2."]
    ENABLED = 0x01,
}
impl Triginen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Triginen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Triginen2 {
    #[inline(always)]
    fn from(val: u8) -> Triginen2 {
        Triginen2::from_bits(val)
    }
}
impl From<Triginen2> for u8 {
    #[inline(always)]
    fn from(val: Triginen2) -> u8 {
        Triginen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Triginen3 {
    #[doc = "Input trigger n events are ignored by channel 3."]
    DISABLED = 0x0,
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 3."]
    ENABLED = 0x01,
}
impl Triginen3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Triginen3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Triginen3 {
    #[inline(always)]
    fn from(val: u8) -> Triginen3 {
        Triginen3::from_bits(val)
    }
}
impl From<Triginen3> for u8 {
    #[inline(always)]
    fn from(val: Triginen3) -> u8 {
        Triginen3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Trigouten0 {
    #[doc = "Channel 0 is ignored by output trigger n."]
    DISABLED = 0x0,
    #[doc = "When an event occurs on channel 0, generate an event on output event n (ctitrigout\\[n\\])."]
    ENABLED = 0x01,
}
impl Trigouten0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigouten0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigouten0 {
    #[inline(always)]
    fn from(val: u8) -> Trigouten0 {
        Trigouten0::from_bits(val)
    }
}
impl From<Trigouten0> for u8 {
    #[inline(always)]
    fn from(val: Trigouten0) -> u8 {
        Trigouten0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Trigouten1 {
    #[doc = "Channel 1 is ignored by output trigger n."]
    DISABLED = 0x0,
    #[doc = "When an event occurs on channel 1, generate an event on output event n (ctitrigout\\[n\\])."]
    ENABLED = 0x01,
}
impl Trigouten1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigouten1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigouten1 {
    #[inline(always)]
    fn from(val: u8) -> Trigouten1 {
        Trigouten1::from_bits(val)
    }
}
impl From<Trigouten1> for u8 {
    #[inline(always)]
    fn from(val: Trigouten1) -> u8 {
        Trigouten1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Trigouten2 {
    #[doc = "Channel 2 is ignored by output trigger n."]
    DISABLED = 0x0,
    #[doc = "When an event occurs on channel 2, generate an event on output event n (ctitrigout\\[n\\])."]
    ENABLED = 0x01,
}
impl Trigouten2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigouten2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigouten2 {
    #[inline(always)]
    fn from(val: u8) -> Trigouten2 {
        Trigouten2::from_bits(val)
    }
}
impl From<Trigouten2> for u8 {
    #[inline(always)]
    fn from(val: Trigouten2) -> u8 {
        Trigouten2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Trigouten3 {
    #[doc = "Channel 3 is ignored by output trigger n."]
    DISABLED = 0x0,
    #[doc = "When an event occurs on channel 3, generate an event on output event n (ctitrigout\\[n\\])."]
    ENABLED = 0x01,
}
impl Trigouten3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigouten3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigouten3 {
    #[inline(always)]
    fn from(val: u8) -> Trigouten3 {
        Trigouten3::from_bits(val)
    }
}
impl From<Trigouten3> for u8 {
    #[inline(always)]
    fn from(val: Trigouten3) -> u8 {
        Trigouten3::to_bits(val)
    }
}
