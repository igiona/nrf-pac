#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClkConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl ClkConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkConnect {
    #[inline(always)]
    fn from(val: u8) -> ClkConnect {
        ClkConnect::from_bits(val)
    }
}
impl From<ClkConnect> for u8 {
    #[inline(always)]
    fn from(val: ClkConnect) -> u8 {
        ClkConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DinConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl DinConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DinConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DinConnect {
    #[inline(always)]
    fn from(val: u8) -> DinConnect {
        DinConnect::from_bits(val)
    }
}
impl From<DinConnect> for u8 {
    #[inline(always)]
    fn from(val: DinConnect) -> u8 {
        DinConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Edge {
    #[doc = "Left (or mono) is sampled on falling edge of PDM_CLK"]
    LEFTFALLING = 0x0,
    #[doc = "Left (or mono) is sampled on rising edge of PDM_CLK"]
    LEFTRISING = 0x01,
}
impl Edge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edge {
    #[inline(always)]
    fn from(val: u8) -> Edge {
        Edge::from_bits(val)
    }
}
impl From<Edge> for u8 {
    #[inline(always)]
    fn from(val: Edge) -> u8 {
        Edge::to_bits(val)
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
pub struct Freq(pub u32);
impl Freq {
    #[doc = "PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    pub const _1000K: Self = Self(0x0800_0000);
    #[doc = "PDM_CLK = 32 MHz / 31 = 1.032 MHz. Nominal clock for RATIO=Ratio64."]
    pub const DEFAULT: Self = Self(0x0840_0000);
    #[doc = "PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    pub const _1067K: Self = Self(0x0880_0000);
    #[doc = "PDM_CLK = 32 MHz / 26 = 1.231 MHz"]
    pub const _1231K: Self = Self(0x0980_0000);
    #[doc = "PDM_CLK = 32 MHz / 25 = 1.280 MHz. Nominal clock for RATIO=Ratio80."]
    pub const _1280K: Self = Self(0x0a00_0000);
    #[doc = "PDM_CLK = 32 MHz / 24 = 1.333 MHz"]
    pub const _1333K: Self = Self(0x0a80_0000);
}
impl Freq {
    pub const fn from_bits(val: u32) -> Freq {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Freq {
    #[inline(always)]
    fn from(val: u32) -> Freq {
        Freq::from_bits(val)
    }
}
impl From<Freq> for u32 {
    #[inline(always)]
    fn from(val: Freq) -> u32 {
        Freq::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gainl(pub u8);
impl Gainl {
    #[doc = "-20 dB gain adjustment (minimum)"]
    pub const MINGAIN: Self = Self(0x0);
    #[doc = "0 dB gain adjustment"]
    pub const DEFAULTGAIN: Self = Self(0x28);
    #[doc = "+20 dB gain adjustment (maximum)"]
    pub const MAXGAIN: Self = Self(0x50);
}
impl Gainl {
    pub const fn from_bits(val: u8) -> Gainl {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Gainl {
    #[inline(always)]
    fn from(val: u8) -> Gainl {
        Gainl::from_bits(val)
    }
}
impl From<Gainl> for u8 {
    #[inline(always)]
    fn from(val: Gainl) -> u8 {
        Gainl::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gainr(pub u8);
impl Gainr {
    #[doc = "-20 dB gain adjustment (minimum)"]
    pub const MINGAIN: Self = Self(0x0);
    #[doc = "0 dB gain adjustment"]
    pub const DEFAULTGAIN: Self = Self(0x28);
    #[doc = "+20 dB gain adjustment (maximum)"]
    pub const MAXGAIN: Self = Self(0x50);
}
impl Gainr {
    pub const fn from_bits(val: u8) -> Gainr {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Gainr {
    #[inline(always)]
    fn from(val: u8) -> Gainr {
        Gainr::from_bits(val)
    }
}
impl From<Gainr> for u8 {
    #[inline(always)]
    fn from(val: Gainr) -> u8 {
        Gainr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Operation {
    #[doc = "Sample and store one pair (left + right) of 16-bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    STEREO = 0x0,
    #[doc = "Sample and store two successive left samples (16 bits each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    MONO = 0x01,
}
impl Operation {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Operation {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Operation {
    #[inline(always)]
    fn from(val: u8) -> Operation {
        Operation::from_bits(val)
    }
}
impl From<Operation> for u8 {
    #[inline(always)]
    fn from(val: Operation) -> u8 {
        Operation::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ratio {
    #[doc = "Ratio of 64"]
    RATIO64 = 0x0,
    #[doc = "Ratio of 80"]
    RATIO80 = 0x01,
}
impl Ratio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ratio {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ratio {
    #[inline(always)]
    fn from(val: u8) -> Ratio {
        Ratio::from_bits(val)
    }
}
impl From<Ratio> for u8 {
    #[inline(always)]
    fn from(val: Ratio) -> u8 {
        Ratio::to_bits(val)
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
