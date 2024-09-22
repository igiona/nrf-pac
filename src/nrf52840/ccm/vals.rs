#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Datarate {
    #[doc = "1 Mbps"]
    _1MBIT = 0x0,
    #[doc = "2 Mbps"]
    _2MBIT = 0x01,
    #[doc = "125 kbps"]
    _125KBPS = 0x02,
    #[doc = "500 kbps"]
    _500KBPS = 0x03,
}
impl Datarate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datarate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datarate {
    #[inline(always)]
    fn from(val: u8) -> Datarate {
        Datarate::from_bits(val)
    }
}
impl From<Datarate> for u8 {
    #[inline(always)]
    fn from(val: Datarate) -> u8 {
        Datarate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disable"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enable"]
    ENABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x03) }
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
pub enum EventsEndcrypt {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEndcrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEndcrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEndcrypt {
    #[inline(always)]
    fn from(val: u8) -> EventsEndcrypt {
        EventsEndcrypt::from_bits(val)
    }
}
impl From<EventsEndcrypt> for u8 {
    #[inline(always)]
    fn from(val: EventsEndcrypt) -> u8 {
        EventsEndcrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEndksgen {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEndksgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEndksgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEndksgen {
    #[inline(always)]
    fn from(val: u8) -> EventsEndksgen {
        EventsEndksgen::from_bits(val)
    }
}
impl From<EventsEndksgen> for u8 {
    #[inline(always)]
    fn from(val: EventsEndksgen) -> u8 {
        EventsEndksgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsError {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsError {
    #[inline(always)]
    fn from(val: u8) -> EventsError {
        EventsError::from_bits(val)
    }
}
impl From<EventsError> for u8 {
    #[inline(always)]
    fn from(val: EventsError) -> u8 {
        EventsError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Length {
    #[doc = "Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A keystream for packet payloads up to 27 bytes will be generated."]
    DEFAULT = 0x0,
    #[doc = "Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A keystream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
    EXTENDED = 0x01,
}
impl Length {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Length {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Length {
    #[inline(always)]
    fn from(val: u8) -> Length {
        Length::from_bits(val)
    }
}
impl From<Length> for u8 {
    #[inline(always)]
    fn from(val: Length) -> u8 {
        Length::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Micstatus {
    #[doc = "MIC check failed"]
    CHECKFAILED = 0x0,
    #[doc = "MIC check passed"]
    CHECKPASSED = 0x01,
}
impl Micstatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Micstatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Micstatus {
    #[inline(always)]
    fn from(val: u8) -> Micstatus {
        Micstatus::from_bits(val)
    }
}
impl From<Micstatus> for u8 {
    #[inline(always)]
    fn from(val: Micstatus) -> u8 {
        Micstatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "AES CCM packet encryption mode"]
    ENCRYPTION = 0x0,
    #[doc = "AES CCM packet decryption mode"]
    DECRYPTION = 0x01,
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
pub enum Rateoverride {
    #[doc = "1 Mbps"]
    _1MBIT = 0x0,
    #[doc = "2 Mbps"]
    _2MBIT = 0x01,
    #[doc = "125 kbps"]
    _125KBPS = 0x02,
    #[doc = "500 kbps"]
    _500KBPS = 0x03,
}
impl Rateoverride {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rateoverride {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rateoverride {
    #[inline(always)]
    fn from(val: u8) -> Rateoverride {
        Rateoverride::from_bits(val)
    }
}
impl From<Rateoverride> for u8 {
    #[inline(always)]
    fn from(val: Rateoverride) -> u8 {
        Rateoverride::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksCrypt {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksCrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksCrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksCrypt {
    #[inline(always)]
    fn from(val: u8) -> TasksCrypt {
        TasksCrypt::from_bits(val)
    }
}
impl From<TasksCrypt> for u8 {
    #[inline(always)]
    fn from(val: TasksCrypt) -> u8 {
        TasksCrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksKsgen {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksKsgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksKsgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksKsgen {
    #[inline(always)]
    fn from(val: u8) -> TasksKsgen {
        TasksKsgen::from_bits(val)
    }
}
impl From<TasksKsgen> for u8 {
    #[inline(always)]
    fn from(val: TasksKsgen) -> u8 {
        TasksKsgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksRateoverride {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksRateoverride {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksRateoverride {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksRateoverride {
    #[inline(always)]
    fn from(val: u8) -> TasksRateoverride {
        TasksRateoverride::from_bits(val)
    }
}
impl From<TasksRateoverride> for u8 {
    #[inline(always)]
    fn from(val: TasksRateoverride) -> u8 {
        TasksRateoverride::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStop {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStop {
    #[inline(always)]
    fn from(val: u8) -> TasksStop {
        TasksStop::from_bits(val)
    }
}
impl From<TasksStop> for u8 {
    #[inline(always)]
    fn from(val: TasksStop) -> u8 {
        TasksStop::to_bits(val)
    }
}
