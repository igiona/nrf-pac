#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clear {
    _RESERVED_0 = 0x0,
    #[doc = "Clear the profiling counters"]
    CLEAR = 0x01,
}
impl Clear {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clear {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clear {
    #[inline(always)]
    fn from(val: u8) -> Clear {
        Clear::from_bits(val)
    }
}
impl From<Clear> for u8 {
    #[inline(always)]
    fn from(val: Clear) -> u8 {
        Clear::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Debuglock {
    #[doc = "Debug mode unlocked"]
    UNLOCKED = 0x0,
    #[doc = "Debug mode locked"]
    LOCKED = 0x01,
}
impl Debuglock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debuglock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debuglock {
    #[inline(always)]
    fn from(val: u8) -> Debuglock {
        Debuglock::from_bits(val)
    }
}
impl From<Debuglock> for u8 {
    #[inline(always)]
    fn from(val: Debuglock) -> u8 {
        Debuglock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EnableEnable {
    #[doc = "Disable cache"]
    DISABLED = 0x0,
    #[doc = "Enable cache"]
    ENABLED = 0x01,
}
impl EnableEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnableEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnableEnable {
    #[inline(always)]
    fn from(val: u8) -> EnableEnable {
        EnableEnable::from_bits(val)
    }
}
impl From<EnableEnable> for u8 {
    #[inline(always)]
    fn from(val: EnableEnable) -> u8 {
        EnableEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Erase {
    _RESERVED_0 = 0x0,
    #[doc = "Erase cache"]
    ERASE = 0x01,
}
impl Erase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erase {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erase {
    #[inline(always)]
    fn from(val: u8) -> Erase {
        Erase::from_bits(val)
    }
}
impl From<Erase> for u8 {
    #[inline(always)]
    fn from(val: Erase) -> u8 {
        Erase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Erasestatus {
    #[doc = "Erase is not complete or hasn't started"]
    IDLE = 0x0,
    #[doc = "Cache erase is finished"]
    FINISHED = 0x01,
}
impl Erasestatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erasestatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erasestatus {
    #[inline(always)]
    fn from(val: u8) -> Erasestatus {
        Erasestatus::from_bits(val)
    }
}
impl From<Erasestatus> for u8 {
    #[inline(always)]
    fn from(val: Erasestatus) -> u8 {
        Erasestatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Invalidate {
    _RESERVED_0 = 0x0,
    #[doc = "Invalidate the cache"]
    INVALIDATE = 0x01,
}
impl Invalidate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invalidate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invalidate {
    #[inline(always)]
    fn from(val: u8) -> Invalidate {
        Invalidate::from_bits(val)
    }
}
impl From<Invalidate> for u8 {
    #[inline(always)]
    fn from(val: Invalidate) -> u8 {
        Invalidate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "Cache mode"]
    CACHE = 0x0,
    #[doc = "RAM mode"]
    RAM = 0x01,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ProfilingenableEnable {
    #[doc = "Disable profiling"]
    DISABLE = 0x0,
    #[doc = "Enable profiling"]
    ENABLE = 0x01,
}
impl ProfilingenableEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProfilingenableEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProfilingenableEnable {
    #[inline(always)]
    fn from(val: u8) -> ProfilingenableEnable {
        ProfilingenableEnable::from_bits(val)
    }
}
impl From<ProfilingenableEnable> for u8 {
    #[inline(always)]
    fn from(val: ProfilingenableEnable) -> u8 {
        ProfilingenableEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Writelock {
    #[doc = "Cache updates unlocked"]
    UNLOCKED = 0x0,
    #[doc = "Cache updates locked"]
    LOCKED = 0x01,
}
impl Writelock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Writelock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Writelock {
    #[inline(always)]
    fn from(val: u8) -> Writelock {
        Writelock::from_bits(val)
    }
}
impl From<Writelock> for u8 {
    #[inline(always)]
    fn from(val: Writelock) -> u8 {
        Writelock::to_bits(val)
    }
}
