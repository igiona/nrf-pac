#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Address0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Address0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Address0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Address0 {
    #[inline(always)]
    fn from(val: u8) -> Address0 {
        Address0::from_bits(val)
    }
}
impl From<Address0> for u8 {
    #[inline(always)]
    fn from(val: Address0) -> u8 {
        Address0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Address1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Address1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Address1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Address1 {
    #[inline(always)]
    fn from(val: u8) -> Address1 {
        Address1::from_bits(val)
    }
}
impl From<Address1> for u8 {
    #[inline(always)]
    fn from(val: Address1) -> u8 {
        Address1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dnack {
    #[doc = "Error did not occur"]
    NOTRECEIVED = 0x0,
    #[doc = "Error occurred"]
    RECEIVED = 0x01,
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
    #[doc = "Disable TWIS"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Enable TWIS"]
    ENABLED = 0x09,
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
pub enum EventsRead {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRead {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRead {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRead {
    #[inline(always)]
    fn from(val: u8) -> EventsRead {
        EventsRead::from_bits(val)
    }
}
impl From<EventsRead> for u8 {
    #[inline(always)]
    fn from(val: EventsRead) -> u8 {
        EventsRead::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsRxstarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRxstarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRxstarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRxstarted {
    #[inline(always)]
    fn from(val: u8) -> EventsRxstarted {
        EventsRxstarted::from_bits(val)
    }
}
impl From<EventsRxstarted> for u8 {
    #[inline(always)]
    fn from(val: EventsRxstarted) -> u8 {
        EventsRxstarted::to_bits(val)
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
pub enum EventsTxstarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsTxstarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsTxstarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsTxstarted {
    #[inline(always)]
    fn from(val: u8) -> EventsTxstarted {
        EventsTxstarted::from_bits(val)
    }
}
impl From<EventsTxstarted> for u8 {
    #[inline(always)]
    fn from(val: EventsTxstarted) -> u8 {
        EventsTxstarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsWrite {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsWrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsWrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsWrite {
    #[inline(always)]
    fn from(val: u8) -> EventsWrite {
        EventsWrite::from_bits(val)
    }
}
impl From<EventsWrite> for u8 {
    #[inline(always)]
    fn from(val: EventsWrite) -> u8 {
        EventsWrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Overflow {
    #[doc = "Error did not occur"]
    NOTDETECTED = 0x0,
    #[doc = "Error occurred"]
    DETECTED = 0x01,
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
    #[doc = "Error did not occur"]
    NOTDETECTED = 0x0,
    #[doc = "Error occurred"]
    DETECTED = 0x01,
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
pub enum PublishErrorEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishErrorEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishErrorEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishErrorEn {
    #[inline(always)]
    fn from(val: u8) -> PublishErrorEn {
        PublishErrorEn::from_bits(val)
    }
}
impl From<PublishErrorEn> for u8 {
    #[inline(always)]
    fn from(val: PublishErrorEn) -> u8 {
        PublishErrorEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishReadEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishReadEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishReadEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishReadEn {
    #[inline(always)]
    fn from(val: u8) -> PublishReadEn {
        PublishReadEn::from_bits(val)
    }
}
impl From<PublishReadEn> for u8 {
    #[inline(always)]
    fn from(val: PublishReadEn) -> u8 {
        PublishReadEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishRxstartedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishRxstartedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishRxstartedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishRxstartedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishRxstartedEn {
        PublishRxstartedEn::from_bits(val)
    }
}
impl From<PublishRxstartedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishRxstartedEn) -> u8 {
        PublishRxstartedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishStoppedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishStoppedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishStoppedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishStoppedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishStoppedEn {
        PublishStoppedEn::from_bits(val)
    }
}
impl From<PublishStoppedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishStoppedEn) -> u8 {
        PublishStoppedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishTxstartedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishTxstartedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishTxstartedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishTxstartedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishTxstartedEn {
        PublishTxstartedEn::from_bits(val)
    }
}
impl From<PublishTxstartedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishTxstartedEn) -> u8 {
        PublishTxstartedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishWriteEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishWriteEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishWriteEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishWriteEn {
    #[inline(always)]
    fn from(val: u8) -> PublishWriteEn {
        PublishWriteEn::from_bits(val)
    }
}
impl From<PublishWriteEn> for u8 {
    #[inline(always)]
    fn from(val: PublishWriteEn) -> u8 {
        PublishWriteEn::to_bits(val)
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
pub enum SubscribePreparerxEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribePreparerxEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribePreparerxEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribePreparerxEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribePreparerxEn {
        SubscribePreparerxEn::from_bits(val)
    }
}
impl From<SubscribePreparerxEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribePreparerxEn) -> u8 {
        SubscribePreparerxEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribePreparetxEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribePreparetxEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribePreparetxEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribePreparetxEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribePreparetxEn {
        SubscribePreparetxEn::from_bits(val)
    }
}
impl From<SubscribePreparetxEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribePreparetxEn) -> u8 {
        SubscribePreparetxEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeResumeEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeResumeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeResumeEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeResumeEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeResumeEn {
        SubscribeResumeEn::from_bits(val)
    }
}
impl From<SubscribeResumeEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeResumeEn) -> u8 {
        SubscribeResumeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeStopEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeStopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeStopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeStopEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeStopEn {
        SubscribeStopEn::from_bits(val)
    }
}
impl From<SubscribeStopEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeStopEn) -> u8 {
        SubscribeStopEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeSuspendEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeSuspendEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeSuspendEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeSuspendEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeSuspendEn {
        SubscribeSuspendEn::from_bits(val)
    }
}
impl From<SubscribeSuspendEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeSuspendEn) -> u8 {
        SubscribeSuspendEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksPreparerx {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksPreparerx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksPreparerx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksPreparerx {
    #[inline(always)]
    fn from(val: u8) -> TasksPreparerx {
        TasksPreparerx::from_bits(val)
    }
}
impl From<TasksPreparerx> for u8 {
    #[inline(always)]
    fn from(val: TasksPreparerx) -> u8 {
        TasksPreparerx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksPreparetx {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksPreparetx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksPreparetx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksPreparetx {
    #[inline(always)]
    fn from(val: u8) -> TasksPreparetx {
        TasksPreparetx::from_bits(val)
    }
}
impl From<TasksPreparetx> for u8 {
    #[inline(always)]
    fn from(val: TasksPreparetx) -> u8 {
        TasksPreparetx::to_bits(val)
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
