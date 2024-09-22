#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Aconnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Aconnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aconnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aconnect {
    #[inline(always)]
    fn from(val: u8) -> Aconnect {
        Aconnect::from_bits(val)
    }
}
impl From<Aconnect> for u8 {
    #[inline(always)]
    fn from(val: Aconnect) -> u8 {
        Aconnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bconnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Bconnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bconnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bconnect {
    #[inline(always)]
    fn from(val: u8) -> Bconnect {
        Bconnect::from_bits(val)
    }
}
impl From<Bconnect> for u8 {
    #[inline(always)]
    fn from(val: Bconnect) -> u8 {
        Bconnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dbfen {
    #[doc = "Debounce input filters disabled"]
    DISABLED = 0x0,
    #[doc = "Debounce input filters enabled"]
    ENABLED = 0x01,
}
impl Dbfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbfen {
    #[inline(always)]
    fn from(val: u8) -> Dbfen {
        Dbfen::from_bits(val)
    }
}
impl From<Dbfen> for u8 {
    #[inline(always)]
    fn from(val: Dbfen) -> u8 {
        Dbfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
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
pub enum EventsAccof {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsAccof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsAccof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsAccof {
    #[inline(always)]
    fn from(val: u8) -> EventsAccof {
        EventsAccof::from_bits(val)
    }
}
impl From<EventsAccof> for u8 {
    #[inline(always)]
    fn from(val: EventsAccof) -> u8 {
        EventsAccof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsDblrdy {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsDblrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsDblrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsDblrdy {
    #[inline(always)]
    fn from(val: u8) -> EventsDblrdy {
        EventsDblrdy::from_bits(val)
    }
}
impl From<EventsDblrdy> for u8 {
    #[inline(always)]
    fn from(val: EventsDblrdy) -> u8 {
        EventsDblrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsReportrdy {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsReportrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsReportrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsReportrdy {
    #[inline(always)]
    fn from(val: u8) -> EventsReportrdy {
        EventsReportrdy::from_bits(val)
    }
}
impl From<EventsReportrdy> for u8 {
    #[inline(always)]
    fn from(val: EventsReportrdy) -> u8 {
        EventsReportrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsSamplerdy {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsSamplerdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsSamplerdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsSamplerdy {
    #[inline(always)]
    fn from(val: u8) -> EventsSamplerdy {
        EventsSamplerdy::from_bits(val)
    }
}
impl From<EventsSamplerdy> for u8 {
    #[inline(always)]
    fn from(val: EventsSamplerdy) -> u8 {
        EventsSamplerdy::to_bits(val)
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
pub enum LedConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl LedConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LedConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LedConnect {
    #[inline(always)]
    fn from(val: u8) -> LedConnect {
        LedConnect::from_bits(val)
    }
}
impl From<LedConnect> for u8 {
    #[inline(always)]
    fn from(val: LedConnect) -> u8 {
        LedConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ledpol {
    #[doc = "Led active on output pin low"]
    ACTIVELOW = 0x0,
    #[doc = "Led active on output pin high"]
    ACTIVEHIGH = 0x01,
}
impl Ledpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ledpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ledpol {
    #[inline(always)]
    fn from(val: u8) -> Ledpol {
        Ledpol::from_bits(val)
    }
}
impl From<Ledpol> for u8 {
    #[inline(always)]
    fn from(val: Ledpol) -> u8 {
        Ledpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishAccofEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishAccofEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishAccofEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishAccofEn {
    #[inline(always)]
    fn from(val: u8) -> PublishAccofEn {
        PublishAccofEn::from_bits(val)
    }
}
impl From<PublishAccofEn> for u8 {
    #[inline(always)]
    fn from(val: PublishAccofEn) -> u8 {
        PublishAccofEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishDblrdyEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishDblrdyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishDblrdyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishDblrdyEn {
    #[inline(always)]
    fn from(val: u8) -> PublishDblrdyEn {
        PublishDblrdyEn::from_bits(val)
    }
}
impl From<PublishDblrdyEn> for u8 {
    #[inline(always)]
    fn from(val: PublishDblrdyEn) -> u8 {
        PublishDblrdyEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishReportrdyEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishReportrdyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishReportrdyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishReportrdyEn {
    #[inline(always)]
    fn from(val: u8) -> PublishReportrdyEn {
        PublishReportrdyEn::from_bits(val)
    }
}
impl From<PublishReportrdyEn> for u8 {
    #[inline(always)]
    fn from(val: PublishReportrdyEn) -> u8 {
        PublishReportrdyEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishSamplerdyEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishSamplerdyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishSamplerdyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishSamplerdyEn {
    #[inline(always)]
    fn from(val: u8) -> PublishSamplerdyEn {
        PublishSamplerdyEn::from_bits(val)
    }
}
impl From<PublishSamplerdyEn> for u8 {
    #[inline(always)]
    fn from(val: PublishSamplerdyEn) -> u8 {
        PublishSamplerdyEn::to_bits(val)
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
pub enum Reportper {
    #[doc = "10 samples/report"]
    _10SMPL = 0x0,
    #[doc = "40 samples/report"]
    _40SMPL = 0x01,
    #[doc = "80 samples/report"]
    _80SMPL = 0x02,
    #[doc = "120 samples/report"]
    _120SMPL = 0x03,
    #[doc = "160 samples/report"]
    _160SMPL = 0x04,
    #[doc = "200 samples/report"]
    _200SMPL = 0x05,
    #[doc = "240 samples/report"]
    _240SMPL = 0x06,
    #[doc = "280 samples/report"]
    _280SMPL = 0x07,
    #[doc = "1 sample/report"]
    _1SMPL = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Reportper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reportper {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reportper {
    #[inline(always)]
    fn from(val: u8) -> Reportper {
        Reportper::from_bits(val)
    }
}
impl From<Reportper> for u8 {
    #[inline(always)]
    fn from(val: Reportper) -> u8 {
        Reportper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sampleper {
    #[doc = "128 us"]
    _128US = 0x0,
    #[doc = "256 us"]
    _256US = 0x01,
    #[doc = "512 us"]
    _512US = 0x02,
    #[doc = "1024 us"]
    _1024US = 0x03,
    #[doc = "2048 us"]
    _2048US = 0x04,
    #[doc = "4096 us"]
    _4096US = 0x05,
    #[doc = "8192 us"]
    _8192US = 0x06,
    #[doc = "16384 us"]
    _16384US = 0x07,
    #[doc = "32768 us"]
    _32MS = 0x08,
    #[doc = "65536 us"]
    _65MS = 0x09,
    #[doc = "131072 us"]
    _131MS = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Sampleper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sampleper {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sampleper {
    #[inline(always)]
    fn from(val: u8) -> Sampleper {
        Sampleper::from_bits(val)
    }
}
impl From<Sampleper> for u8 {
    #[inline(always)]
    fn from(val: Sampleper) -> u8 {
        Sampleper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeRdclraccEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeRdclraccEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeRdclraccEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeRdclraccEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeRdclraccEn {
        SubscribeRdclraccEn::from_bits(val)
    }
}
impl From<SubscribeRdclraccEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeRdclraccEn) -> u8 {
        SubscribeRdclraccEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeRdclrdblEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeRdclrdblEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeRdclrdblEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeRdclrdblEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeRdclrdblEn {
        SubscribeRdclrdblEn::from_bits(val)
    }
}
impl From<SubscribeRdclrdblEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeRdclrdblEn) -> u8 {
        SubscribeRdclrdblEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeReadclraccEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeReadclraccEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeReadclraccEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeReadclraccEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeReadclraccEn {
        SubscribeReadclraccEn::from_bits(val)
    }
}
impl From<SubscribeReadclraccEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeReadclraccEn) -> u8 {
        SubscribeReadclraccEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeStartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeStartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeStartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeStartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeStartEn {
        SubscribeStartEn::from_bits(val)
    }
}
impl From<SubscribeStartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeStartEn) -> u8 {
        SubscribeStartEn::to_bits(val)
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
pub enum TasksRdclracc {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksRdclracc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksRdclracc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksRdclracc {
    #[inline(always)]
    fn from(val: u8) -> TasksRdclracc {
        TasksRdclracc::from_bits(val)
    }
}
impl From<TasksRdclracc> for u8 {
    #[inline(always)]
    fn from(val: TasksRdclracc) -> u8 {
        TasksRdclracc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksRdclrdbl {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksRdclrdbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksRdclrdbl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksRdclrdbl {
    #[inline(always)]
    fn from(val: u8) -> TasksRdclrdbl {
        TasksRdclrdbl::from_bits(val)
    }
}
impl From<TasksRdclrdbl> for u8 {
    #[inline(always)]
    fn from(val: TasksRdclrdbl) -> u8 {
        TasksRdclrdbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksReadclracc {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksReadclracc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksReadclracc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksReadclracc {
    #[inline(always)]
    fn from(val: u8) -> TasksReadclracc {
        TasksReadclracc::from_bits(val)
    }
}
impl From<TasksReadclracc> for u8 {
    #[inline(always)]
    fn from(val: TasksReadclracc) -> u8 {
        TasksReadclracc::to_bits(val)
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
