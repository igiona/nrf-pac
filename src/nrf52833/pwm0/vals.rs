#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CntCnt(pub u16);
impl CntCnt {
    #[doc = "Sequence is disabled, and shall not be started as it is empty"]
    pub const DISABLED: Self = Self(0x0);
}
impl CntCnt {
    pub const fn from_bits(val: u16) -> CntCnt {
        Self(val & 0x7fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for CntCnt {
    #[inline(always)]
    fn from(val: u16) -> CntCnt {
        CntCnt::from_bits(val)
    }
}
impl From<CntCnt> for u16 {
    #[inline(always)]
    fn from(val: CntCnt) -> u16 {
        CntCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Connect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Connect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Connect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Connect {
    #[inline(always)]
    fn from(val: u8) -> Connect {
        Connect::from_bits(val)
    }
}
impl From<Connect> for u8 {
    #[inline(always)]
    fn from(val: Connect) -> u8 {
        Connect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disabled"]
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
pub enum EventsLoopsdone {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsLoopsdone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsLoopsdone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsLoopsdone {
    #[inline(always)]
    fn from(val: u8) -> EventsLoopsdone {
        EventsLoopsdone::from_bits(val)
    }
}
impl From<EventsLoopsdone> for u8 {
    #[inline(always)]
    fn from(val: EventsLoopsdone) -> u8 {
        EventsLoopsdone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsPwmperiodend {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsPwmperiodend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsPwmperiodend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsPwmperiodend {
    #[inline(always)]
    fn from(val: u8) -> EventsPwmperiodend {
        EventsPwmperiodend::from_bits(val)
    }
}
impl From<EventsPwmperiodend> for u8 {
    #[inline(always)]
    fn from(val: EventsPwmperiodend) -> u8 {
        EventsPwmperiodend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsSeqend {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsSeqend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsSeqend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsSeqend {
    #[inline(always)]
    fn from(val: u8) -> EventsSeqend {
        EventsSeqend::from_bits(val)
    }
}
impl From<EventsSeqend> for u8 {
    #[inline(always)]
    fn from(val: EventsSeqend) -> u8 {
        EventsSeqend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsSeqstarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsSeqstarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsSeqstarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsSeqstarted {
    #[inline(always)]
    fn from(val: u8) -> EventsSeqstarted {
        EventsSeqstarted::from_bits(val)
    }
}
impl From<EventsSeqstarted> for u8 {
    #[inline(always)]
    fn from(val: EventsSeqstarted) -> u8 {
        EventsSeqstarted::to_bits(val)
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
pub enum Load {
    #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
    COMMON = 0x0,
    #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    GROUPED = 0x01,
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    INDIVIDUAL = 0x02,
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    WAVEFORM = 0x03,
}
impl Load {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Load {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Load {
    #[inline(always)]
    fn from(val: u8) -> Load {
        Load::from_bits(val)
    }
}
impl From<Load> for u8 {
    #[inline(always)]
    fn from(val: Load) -> u8 {
        Load::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LoopCnt(pub u16);
impl LoopCnt {
    #[doc = "Looping disabled (stop at the end of the sequence)"]
    pub const DISABLED: Self = Self(0x0);
}
impl LoopCnt {
    pub const fn from_bits(val: u16) -> LoopCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for LoopCnt {
    #[inline(always)]
    fn from(val: u16) -> LoopCnt {
        LoopCnt::from_bits(val)
    }
}
impl From<LoopCnt> for u16 {
    #[inline(always)]
    fn from(val: LoopCnt) -> u16 {
        LoopCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    REFRESHCOUNT = 0x0,
    #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    NEXTSTEP = 0x01,
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
pub enum Prescaler {
    #[doc = "Divide by 1 (16 MHz)"]
    DIV_1 = 0x0,
    #[doc = "Divide by 2 (8 MHz)"]
    DIV_2 = 0x01,
    #[doc = "Divide by 4 (4 MHz)"]
    DIV_4 = 0x02,
    #[doc = "Divide by 8 (2 MHz)"]
    DIV_8 = 0x03,
    #[doc = "Divide by 16 (1 MHz)"]
    DIV_16 = 0x04,
    #[doc = "Divide by 32 (500 kHz)"]
    DIV_32 = 0x05,
    #[doc = "Divide by 64 (250 kHz)"]
    DIV_64 = 0x06,
    #[doc = "Divide by 128 (125 kHz)"]
    DIV_128 = 0x07,
}
impl Prescaler {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prescaler {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prescaler {
    #[inline(always)]
    fn from(val: u8) -> Prescaler {
        Prescaler::from_bits(val)
    }
}
impl From<Prescaler> for u8 {
    #[inline(always)]
    fn from(val: Prescaler) -> u8 {
        Prescaler::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RefreshCnt(pub u32);
impl RefreshCnt {
    #[doc = "Update every PWM period"]
    pub const CONTINUOUS: Self = Self(0x0);
}
impl RefreshCnt {
    pub const fn from_bits(val: u32) -> RefreshCnt {
        Self(val & 0x00ff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for RefreshCnt {
    #[inline(always)]
    fn from(val: u32) -> RefreshCnt {
        RefreshCnt::from_bits(val)
    }
}
impl From<RefreshCnt> for u32 {
    #[inline(always)]
    fn from(val: RefreshCnt) -> u32 {
        RefreshCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksNextstep {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksNextstep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksNextstep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksNextstep {
    #[inline(always)]
    fn from(val: u8) -> TasksNextstep {
        TasksNextstep::from_bits(val)
    }
}
impl From<TasksNextstep> for u8 {
    #[inline(always)]
    fn from(val: TasksNextstep) -> u8 {
        TasksNextstep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksSeqstart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksSeqstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksSeqstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksSeqstart {
    #[inline(always)]
    fn from(val: u8) -> TasksSeqstart {
        TasksSeqstart::from_bits(val)
    }
}
impl From<TasksSeqstart> for u8 {
    #[inline(always)]
    fn from(val: TasksSeqstart) -> u8 {
        TasksSeqstart::to_bits(val)
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
pub enum Updown {
    #[doc = "Up counter, edge-aligned PWM duty cycle"]
    UP = 0x0,
    #[doc = "Up and down counter, center-aligned PWM duty cycle"]
    UPANDDOWN = 0x01,
}
impl Updown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Updown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Updown {
    #[inline(always)]
    fn from(val: u8) -> Updown {
        Updown::from_bits(val)
    }
}
impl From<Updown> for u8 {
    #[inline(always)]
    fn from(val: Updown) -> u8 {
        Updown::to_bits(val)
    }
}
