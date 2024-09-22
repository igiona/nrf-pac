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
pub enum Enable {
    #[doc = "Disable SPI slave"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enable SPI slave"]
    ENABLED = 0x02,
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
pub enum EventsAcquired {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsAcquired {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsAcquired {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsAcquired {
    #[inline(always)]
    fn from(val: u8) -> EventsAcquired {
        EventsAcquired::from_bits(val)
    }
}
impl From<EventsAcquired> for u8 {
    #[inline(always)]
    fn from(val: EventsAcquired) -> u8 {
        EventsAcquired::to_bits(val)
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
pub enum Overflow {
    #[doc = "Read: error not present"]
    NOTPRESENT = 0x0,
    #[doc = "Read: error present"]
    R_PRESENT_W_CLEAR = 0x01,
}
impl Overflow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Overflow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Overflow {
    #[inline(always)]
    fn from(val: u8) -> Overflow {
        Overflow::from_bits(val)
    }
}
impl From<Overflow> for u8 {
    #[inline(always)]
    fn from(val: Overflow) -> u8 {
        Overflow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Overread {
    #[doc = "Read: error not present"]
    NOTPRESENT = 0x0,
    #[doc = "Read: error present"]
    R_PRESENT_W_CLEAR = 0x01,
}
impl Overread {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Overread {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Overread {
    #[inline(always)]
    fn from(val: u8) -> Overread {
        Overread::from_bits(val)
    }
}
impl From<Overread> for u8 {
    #[inline(always)]
    fn from(val: Overread) -> u8 {
        Overread::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishAcquiredEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishAcquiredEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishAcquiredEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishAcquiredEn {
    #[inline(always)]
    fn from(val: u8) -> PublishAcquiredEn {
        PublishAcquiredEn::from_bits(val)
    }
}
impl From<PublishAcquiredEn> for u8 {
    #[inline(always)]
    fn from(val: PublishAcquiredEn) -> u8 {
        PublishAcquiredEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishEndEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishEndEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishEndEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishEndEn {
    #[inline(always)]
    fn from(val: u8) -> PublishEndEn {
        PublishEndEn::from_bits(val)
    }
}
impl From<PublishEndEn> for u8 {
    #[inline(always)]
    fn from(val: PublishEndEn) -> u8 {
        PublishEndEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishEndrxEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishEndrxEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishEndrxEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishEndrxEn {
    #[inline(always)]
    fn from(val: u8) -> PublishEndrxEn {
        PublishEndrxEn::from_bits(val)
    }
}
impl From<PublishEndrxEn> for u8 {
    #[inline(always)]
    fn from(val: PublishEndrxEn) -> u8 {
        PublishEndrxEn::to_bits(val)
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
pub enum Semstat {
    #[doc = "Semaphore is free"]
    FREE = 0x0,
    #[doc = "Semaphore is assigned to CPU"]
    CPU = 0x01,
    #[doc = "Semaphore is assigned to SPI slave"]
    SPIS = 0x02,
    #[doc = "Semaphore is assigned to SPI but a handover to the CPU is pending"]
    CPUPENDING = 0x03,
}
impl Semstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Semstat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Semstat {
    #[inline(always)]
    fn from(val: u8) -> Semstat {
        Semstat::from_bits(val)
    }
}
impl From<Semstat> for u8 {
    #[inline(always)]
    fn from(val: Semstat) -> u8 {
        Semstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeAcquireEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeAcquireEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeAcquireEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeAcquireEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeAcquireEn {
        SubscribeAcquireEn::from_bits(val)
    }
}
impl From<SubscribeAcquireEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeAcquireEn) -> u8 {
        SubscribeAcquireEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeReleaseEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeReleaseEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeReleaseEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeReleaseEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeReleaseEn {
        SubscribeReleaseEn::from_bits(val)
    }
}
impl From<SubscribeReleaseEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeReleaseEn) -> u8 {
        SubscribeReleaseEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksAcquire {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksAcquire {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksAcquire {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksAcquire {
    #[inline(always)]
    fn from(val: u8) -> TasksAcquire {
        TasksAcquire::from_bits(val)
    }
}
impl From<TasksAcquire> for u8 {
    #[inline(always)]
    fn from(val: TasksAcquire) -> u8 {
        TasksAcquire::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksRelease {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksRelease {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksRelease {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksRelease {
    #[inline(always)]
    fn from(val: u8) -> TasksRelease {
        TasksRelease::from_bits(val)
    }
}
impl From<TasksRelease> for u8 {
    #[inline(always)]
    fn from(val: TasksRelease) -> u8 {
        TasksRelease::to_bits(val)
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
