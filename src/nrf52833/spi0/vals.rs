#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cpha {
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    LEADING = 0x0,
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    TRAILING = 0x01,
}
impl Cpha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpha {
    #[inline(always)]
    fn from(val: u8) -> Cpha {
        Cpha::from_bits(val)
    }
}
impl From<Cpha> for u8 {
    #[inline(always)]
    fn from(val: Cpha) -> u8 {
        Cpha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cpol {
    #[doc = "Active high"]
    ACTIVEHIGH = 0x0,
    #[doc = "Active low"]
    ACTIVELOW = 0x01,
}
impl Cpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpol {
    #[inline(always)]
    fn from(val: u8) -> Cpol {
        Cpol::from_bits(val)
    }
}
impl From<Cpol> for u8 {
    #[inline(always)]
    fn from(val: Cpol) -> u8 {
        Cpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disable SPI"]
    DISABLED = 0x0,
    #[doc = "Enable SPI"]
    ENABLED = 0x01,
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
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum EventsReady {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsReady {
    #[inline(always)]
    fn from(val: u8) -> EventsReady {
        EventsReady::from_bits(val)
    }
}
impl From<EventsReady> for u8 {
    #[inline(always)]
    fn from(val: EventsReady) -> u8 {
        EventsReady::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Frequency(pub u32);
impl Frequency {
    #[doc = "125 kbps"]
    pub const K125: Self = Self(0x0200_0000);
    #[doc = "250 kbps"]
    pub const K250: Self = Self(0x0400_0000);
    #[doc = "500 kbps"]
    pub const K500: Self = Self(0x0800_0000);
    #[doc = "1 Mbps"]
    pub const M1: Self = Self(0x1000_0000);
    #[doc = "2 Mbps"]
    pub const M2: Self = Self(0x2000_0000);
    #[doc = "4 Mbps"]
    pub const M4: Self = Self(0x4000_0000);
    #[doc = "8 Mbps"]
    pub const M8: Self = Self(0x8000_0000);
}
impl Frequency {
    pub const fn from_bits(val: u32) -> Frequency {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Frequency {
    #[inline(always)]
    fn from(val: u32) -> Frequency {
        Frequency::from_bits(val)
    }
}
impl From<Frequency> for u32 {
    #[inline(always)]
    fn from(val: Frequency) -> u32 {
        Frequency::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MisoConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl MisoConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MisoConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MisoConnect {
    #[inline(always)]
    fn from(val: u8) -> MisoConnect {
        MisoConnect::from_bits(val)
    }
}
impl From<MisoConnect> for u8 {
    #[inline(always)]
    fn from(val: MisoConnect) -> u8 {
        MisoConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MosiConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl MosiConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MosiConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MosiConnect {
    #[inline(always)]
    fn from(val: u8) -> MosiConnect {
        MosiConnect::from_bits(val)
    }
}
impl From<MosiConnect> for u8 {
    #[inline(always)]
    fn from(val: MosiConnect) -> u8 {
        MosiConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Order {
    #[doc = "Most significant bit shifted out first"]
    MSBFIRST = 0x0,
    #[doc = "Least significant bit shifted out first"]
    LSBFIRST = 0x01,
}
impl Order {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Order {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Order {
    #[inline(always)]
    fn from(val: u8) -> Order {
        Order::from_bits(val)
    }
}
impl From<Order> for u8 {
    #[inline(always)]
    fn from(val: Order) -> u8 {
        Order::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SckConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl SckConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SckConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SckConnect {
    #[inline(always)]
    fn from(val: u8) -> SckConnect {
        SckConnect::from_bits(val)
    }
}
impl From<SckConnect> for u8 {
    #[inline(always)]
    fn from(val: SckConnect) -> u8 {
        SckConnect::to_bits(val)
    }
}
