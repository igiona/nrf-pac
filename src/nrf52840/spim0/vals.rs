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
pub enum CsnConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl CsnConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsnConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsnConnect {
    #[inline(always)]
    fn from(val: u8) -> CsnConnect {
        CsnConnect::from_bits(val)
    }
}
impl From<CsnConnect> for u8 {
    #[inline(always)]
    fn from(val: CsnConnect) -> u8 {
        CsnConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Csnpol {
    #[doc = "Active low (idle state high)"]
    LOW = 0x0,
    #[doc = "Active high (idle state low)"]
    HIGH = 0x01,
}
impl Csnpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csnpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csnpol {
    #[inline(always)]
    fn from(val: u8) -> Csnpol {
        Csnpol::from_bits(val)
    }
}
impl From<Csnpol> for u8 {
    #[inline(always)]
    fn from(val: Csnpol) -> u8 {
        Csnpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disable SPIM"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Enable SPIM"]
    ENABLED = 0x07,
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
pub enum EventsEnd {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEnd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEnd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEnd {
    #[inline(always)]
    fn from(val: u8) -> EventsEnd {
        EventsEnd::from_bits(val)
    }
}
impl From<EventsEnd> for u8 {
    #[inline(always)]
    fn from(val: EventsEnd) -> u8 {
        EventsEnd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEndrx {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEndrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEndrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEndrx {
    #[inline(always)]
    fn from(val: u8) -> EventsEndrx {
        EventsEndrx::from_bits(val)
    }
}
impl From<EventsEndrx> for u8 {
    #[inline(always)]
    fn from(val: EventsEndrx) -> u8 {
        EventsEndrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEndtx {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEndtx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEndtx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEndtx {
    #[inline(always)]
    fn from(val: u8) -> EventsEndtx {
        EventsEndtx::from_bits(val)
    }
}
impl From<EventsEndtx> for u8 {
    #[inline(always)]
    fn from(val: EventsEndtx) -> u8 {
        EventsEndtx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsStarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsStarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsStarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsStarted {
    #[inline(always)]
    fn from(val: u8) -> EventsStarted {
        EventsStarted::from_bits(val)
    }
}
impl From<EventsStarted> for u8 {
    #[inline(always)]
    fn from(val: EventsStarted) -> u8 {
        EventsStarted::to_bits(val)
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
    #[doc = "16 Mbps"]
    pub const M16: Self = Self(0x0a00_0000);
    #[doc = "1 Mbps"]
    pub const M1: Self = Self(0x1000_0000);
    #[doc = "32 Mbps"]
    pub const M32: Self = Self(0x1400_0000);
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
pub enum PseldcxConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl PseldcxConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PseldcxConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PseldcxConnect {
    #[inline(always)]
    fn from(val: u8) -> PseldcxConnect {
        PseldcxConnect::from_bits(val)
    }
}
impl From<PseldcxConnect> for u8 {
    #[inline(always)]
    fn from(val: PseldcxConnect) -> u8 {
        PseldcxConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rx {
    #[doc = "No stall"]
    NOSTALL = 0x0,
    #[doc = "A stall has occurred"]
    STALL = 0x01,
}
impl Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rx {
    #[inline(always)]
    fn from(val: u8) -> Rx {
        Rx::from_bits(val)
    }
}
impl From<Rx> for u8 {
    #[inline(always)]
    fn from(val: Rx) -> u8 {
        Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RxdListList {
    #[doc = "Disable EasyDMA list"]
    DISABLED = 0x0,
    #[doc = "Use array list"]
    ARRAYLIST = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RxdListList {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxdListList {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxdListList {
    #[inline(always)]
    fn from(val: u8) -> RxdListList {
        RxdListList::from_bits(val)
    }
}
impl From<RxdListList> for u8 {
    #[inline(always)]
    fn from(val: RxdListList) -> u8 {
        RxdListList::to_bits(val)
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
pub enum TasksStart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStart {
    #[inline(always)]
    fn from(val: u8) -> TasksStart {
        TasksStart::from_bits(val)
    }
}
impl From<TasksStart> for u8 {
    #[inline(always)]
    fn from(val: TasksStart) -> u8 {
        TasksStart::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tx {
    #[doc = "No stall"]
    NOSTALL = 0x0,
    #[doc = "A stall has occurred"]
    STALL = 0x01,
}
impl Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tx {
    #[inline(always)]
    fn from(val: u8) -> Tx {
        Tx::from_bits(val)
    }
}
impl From<Tx> for u8 {
    #[inline(always)]
    fn from(val: Tx) -> u8 {
        Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TxdListList {
    #[doc = "Disable EasyDMA list"]
    DISABLED = 0x0,
    #[doc = "Use array list"]
    ARRAYLIST = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl TxdListList {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxdListList {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxdListList {
    #[inline(always)]
    fn from(val: u8) -> TxdListList {
        TxdListList::from_bits(val)
    }
}
impl From<TxdListList> for u8 {
    #[inline(always)]
    fn from(val: TxdListList) -> u8 {
        TxdListList::to_bits(val)
    }
}
