#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Anack {
    #[doc = "Read: error not present"]
    NOTPRESENT = 0x0,
    #[doc = "Read: error present"]
    PRESENT = 0x01,
}
impl Anack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Anack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Anack {
    #[inline(always)]
    fn from(val: u8) -> Anack {
        Anack::from_bits(val)
    }
}
impl From<Anack> for u8 {
    #[inline(always)]
    fn from(val: Anack) -> u8 {
        Anack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dnack {
    #[doc = "Read: error not present"]
    NOTPRESENT = 0x0,
    #[doc = "Read: error present"]
    PRESENT = 0x01,
}
impl Dnack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dnack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dnack {
    #[inline(always)]
    fn from(val: u8) -> Dnack {
        Dnack::from_bits(val)
    }
}
impl From<Dnack> for u8 {
    #[inline(always)]
    fn from(val: Dnack) -> u8 {
        Dnack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disable TWI"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Enable TWI"]
    ENABLED = 0x05,
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
pub enum EventsBb {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsBb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsBb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsBb {
    #[inline(always)]
    fn from(val: u8) -> EventsBb {
        EventsBb::from_bits(val)
    }
}
impl From<EventsBb> for u8 {
    #[inline(always)]
    fn from(val: EventsBb) -> u8 {
        EventsBb::to_bits(val)
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
pub enum EventsRxdready {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRxdready {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRxdready {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRxdready {
    #[inline(always)]
    fn from(val: u8) -> EventsRxdready {
        EventsRxdready::from_bits(val)
    }
}
impl From<EventsRxdready> for u8 {
    #[inline(always)]
    fn from(val: EventsRxdready) -> u8 {
        EventsRxdready::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsStopped {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsStopped {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsStopped {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsStopped {
    #[inline(always)]
    fn from(val: u8) -> EventsStopped {
        EventsStopped::from_bits(val)
    }
}
impl From<EventsStopped> for u8 {
    #[inline(always)]
    fn from(val: EventsStopped) -> u8 {
        EventsStopped::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsSuspended {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsSuspended {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsSuspended {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsSuspended {
    #[inline(always)]
    fn from(val: u8) -> EventsSuspended {
        EventsSuspended::from_bits(val)
    }
}
impl From<EventsSuspended> for u8 {
    #[inline(always)]
    fn from(val: EventsSuspended) -> u8 {
        EventsSuspended::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsTxdsent {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsTxdsent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsTxdsent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsTxdsent {
    #[inline(always)]
    fn from(val: u8) -> EventsTxdsent {
        EventsTxdsent::from_bits(val)
    }
}
impl From<EventsTxdsent> for u8 {
    #[inline(always)]
    fn from(val: EventsTxdsent) -> u8 {
        EventsTxdsent::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Frequency(pub u32);
impl Frequency {
    #[doc = "100 kbps"]
    pub const K100: Self = Self(0x0198_0000);
    #[doc = "250 kbps"]
    pub const K250: Self = Self(0x0400_0000);
    #[doc = "400 kbps (actual rate 410.256 kbps)"]
    pub const K400: Self = Self(0x0668_0000);
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
pub enum Overrun {
    #[doc = "Read: no overrun occured"]
    NOTPRESENT = 0x0,
    #[doc = "Read: overrun occured"]
    PRESENT = 0x01,
}
impl Overrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Overrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Overrun {
    #[inline(always)]
    fn from(val: u8) -> Overrun {
        Overrun::from_bits(val)
    }
}
impl From<Overrun> for u8 {
    #[inline(always)]
    fn from(val: Overrun) -> u8 {
        Overrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SclConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl SclConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SclConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SclConnect {
    #[inline(always)]
    fn from(val: u8) -> SclConnect {
        SclConnect::from_bits(val)
    }
}
impl From<SclConnect> for u8 {
    #[inline(always)]
    fn from(val: SclConnect) -> u8 {
        SclConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SdaConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl SdaConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdaConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdaConnect {
    #[inline(always)]
    fn from(val: u8) -> SdaConnect {
        SdaConnect::from_bits(val)
    }
}
impl From<SdaConnect> for u8 {
    #[inline(always)]
    fn from(val: SdaConnect) -> u8 {
        SdaConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksResume {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksResume {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksResume {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksResume {
    #[inline(always)]
    fn from(val: u8) -> TasksResume {
        TasksResume::from_bits(val)
    }
}
impl From<TasksResume> for u8 {
    #[inline(always)]
    fn from(val: TasksResume) -> u8 {
        TasksResume::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStartrx {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStartrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStartrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStartrx {
    #[inline(always)]
    fn from(val: u8) -> TasksStartrx {
        TasksStartrx::from_bits(val)
    }
}
impl From<TasksStartrx> for u8 {
    #[inline(always)]
    fn from(val: TasksStartrx) -> u8 {
        TasksStartrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStarttx {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStarttx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStarttx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStarttx {
    #[inline(always)]
    fn from(val: u8) -> TasksStarttx {
        TasksStarttx::from_bits(val)
    }
}
impl From<TasksStarttx> for u8 {
    #[inline(always)]
    fn from(val: TasksStarttx) -> u8 {
        TasksStarttx::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksSuspend {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksSuspend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksSuspend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksSuspend {
    #[inline(always)]
    fn from(val: u8) -> TasksSuspend {
        TasksSuspend::from_bits(val)
    }
}
impl From<TasksSuspend> for u8 {
    #[inline(always)]
    fn from(val: TasksSuspend) -> u8 {
        TasksSuspend::to_bits(val)
    }
}
