#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disable debug domain and release selected GPIOs"]
    DISABLED = 0x0,
    #[doc = "Enable debug domain and aquire selected GPIOs"]
    ENABLED = 0x01,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Start {
    _RESERVED_0 = 0x0,
    #[doc = "Start all trace and debug clocks."]
    START = 0x01,
}
impl Start {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Start {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Start {
    #[inline(always)]
    fn from(val: u8) -> Start {
        Start::from_bits(val)
    }
}
impl From<Start> for u8 {
    #[inline(always)]
    fn from(val: Start) -> u8 {
        Start::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Stop {
    _RESERVED_0 = 0x0,
    #[doc = "Stop all trace and debug clocks."]
    STOP = 0x01,
}
impl Stop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stop {
    #[inline(always)]
    fn from(val: u8) -> Stop {
        Stop::from_bits(val)
    }
}
impl From<Stop> for u8 {
    #[inline(always)]
    fn from(val: Stop) -> u8 {
        Stop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TraceclkConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl TraceclkConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkConnect {
    #[inline(always)]
    fn from(val: u8) -> TraceclkConnect {
        TraceclkConnect::from_bits(val)
    }
}
impl From<TraceclkConnect> for u8 {
    #[inline(always)]
    fn from(val: TraceclkConnect) -> u8 {
        TraceclkConnect::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TraceclkPin(pub u8);
impl TraceclkPin {
    #[doc = "TRACECLK pin"]
    pub const TRACECLK: Self = Self(0x0c);
}
impl TraceclkPin {
    pub const fn from_bits(val: u8) -> TraceclkPin {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for TraceclkPin {
    #[inline(always)]
    fn from(val: u8) -> TraceclkPin {
        TraceclkPin::from_bits(val)
    }
}
impl From<TraceclkPin> for u8 {
    #[inline(always)]
    fn from(val: TraceclkPin) -> u8 {
        TraceclkPin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tracedata0connect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Tracedata0connect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tracedata0connect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tracedata0connect {
    #[inline(always)]
    fn from(val: u8) -> Tracedata0connect {
        Tracedata0connect::from_bits(val)
    }
}
impl From<Tracedata0connect> for u8 {
    #[inline(always)]
    fn from(val: Tracedata0connect) -> u8 {
        Tracedata0connect::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tracedata0pin(pub u8);
impl Tracedata0pin {
    #[doc = "TRACEDATA0 pin"]
    pub const TRACEDATA0: Self = Self(0x0b);
}
impl Tracedata0pin {
    pub const fn from_bits(val: u8) -> Tracedata0pin {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Tracedata0pin {
    #[inline(always)]
    fn from(val: u8) -> Tracedata0pin {
        Tracedata0pin::from_bits(val)
    }
}
impl From<Tracedata0pin> for u8 {
    #[inline(always)]
    fn from(val: Tracedata0pin) -> u8 {
        Tracedata0pin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tracedata1connect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Tracedata1connect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tracedata1connect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tracedata1connect {
    #[inline(always)]
    fn from(val: u8) -> Tracedata1connect {
        Tracedata1connect::from_bits(val)
    }
}
impl From<Tracedata1connect> for u8 {
    #[inline(always)]
    fn from(val: Tracedata1connect) -> u8 {
        Tracedata1connect::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tracedata1pin(pub u8);
impl Tracedata1pin {
    #[doc = "TRACEDATA1 pin"]
    pub const TRACEDATA1: Self = Self(0x0a);
}
impl Tracedata1pin {
    pub const fn from_bits(val: u8) -> Tracedata1pin {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Tracedata1pin {
    #[inline(always)]
    fn from(val: u8) -> Tracedata1pin {
        Tracedata1pin::from_bits(val)
    }
}
impl From<Tracedata1pin> for u8 {
    #[inline(always)]
    fn from(val: Tracedata1pin) -> u8 {
        Tracedata1pin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tracedata2connect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Tracedata2connect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tracedata2connect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tracedata2connect {
    #[inline(always)]
    fn from(val: u8) -> Tracedata2connect {
        Tracedata2connect::from_bits(val)
    }
}
impl From<Tracedata2connect> for u8 {
    #[inline(always)]
    fn from(val: Tracedata2connect) -> u8 {
        Tracedata2connect::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tracedata2pin(pub u8);
impl Tracedata2pin {
    #[doc = "TRACEDATA2 pin"]
    pub const TRACEDATA2: Self = Self(0x09);
}
impl Tracedata2pin {
    pub const fn from_bits(val: u8) -> Tracedata2pin {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Tracedata2pin {
    #[inline(always)]
    fn from(val: u8) -> Tracedata2pin {
        Tracedata2pin::from_bits(val)
    }
}
impl From<Tracedata2pin> for u8 {
    #[inline(always)]
    fn from(val: Tracedata2pin) -> u8 {
        Tracedata2pin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tracedata3connect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Tracedata3connect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tracedata3connect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tracedata3connect {
    #[inline(always)]
    fn from(val: u8) -> Tracedata3connect {
        Tracedata3connect::from_bits(val)
    }
}
impl From<Tracedata3connect> for u8 {
    #[inline(always)]
    fn from(val: Tracedata3connect) -> u8 {
        Tracedata3connect::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tracedata3pin(pub u8);
impl Tracedata3pin {
    #[doc = "TRACEDATA3 pin"]
    pub const TRACEDATA3: Self = Self(0x08);
}
impl Tracedata3pin {
    pub const fn from_bits(val: u8) -> Tracedata3pin {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Tracedata3pin {
    #[inline(always)]
    fn from(val: u8) -> Tracedata3pin {
        Tracedata3pin::from_bits(val)
    }
}
impl From<Tracedata3pin> for u8 {
    #[inline(always)]
    fn from(val: Tracedata3pin) -> u8 {
        Tracedata3pin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Traceportspeed {
    #[doc = "Trace Port clock is: 64MHz"]
    _64MHZ = 0x0,
    #[doc = "Trace Port clock is: 32MHz"]
    _32MHZ = 0x01,
    #[doc = "Trace Port clock is: 16MHz"]
    _16MHZ = 0x02,
    #[doc = "Trace Port clock is: 8MHz"]
    _8MHZ = 0x03,
}
impl Traceportspeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Traceportspeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Traceportspeed {
    #[inline(always)]
    fn from(val: u8) -> Traceportspeed {
        Traceportspeed::from_bits(val)
    }
}
impl From<Traceportspeed> for u8 {
    #[inline(always)]
    fn from(val: Traceportspeed) -> u8 {
        Traceportspeed::to_bits(val)
    }
}
