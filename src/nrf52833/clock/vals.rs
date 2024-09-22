#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bypass {
    #[doc = "Disable (use with Xtal or low-swing external source)"]
    DISABLED = 0x0,
    #[doc = "Enable (use with rail-to-rail external source)"]
    ENABLED = 0x01,
}
impl Bypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bypass {
    #[inline(always)]
    fn from(val: u8) -> Bypass {
        Bypass::from_bits(val)
    }
}
impl From<Bypass> for u8 {
    #[inline(always)]
    fn from(val: Bypass) -> u8 {
        Bypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCtstarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCtstarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCtstarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCtstarted {
    #[inline(always)]
    fn from(val: u8) -> EventsCtstarted {
        EventsCtstarted::from_bits(val)
    }
}
impl From<EventsCtstarted> for u8 {
    #[inline(always)]
    fn from(val: EventsCtstarted) -> u8 {
        EventsCtstarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCtstopped {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCtstopped {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCtstopped {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCtstopped {
    #[inline(always)]
    fn from(val: u8) -> EventsCtstopped {
        EventsCtstopped::from_bits(val)
    }
}
impl From<EventsCtstopped> for u8 {
    #[inline(always)]
    fn from(val: EventsCtstopped) -> u8 {
        EventsCtstopped::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCtto {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCtto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCtto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCtto {
    #[inline(always)]
    fn from(val: u8) -> EventsCtto {
        EventsCtto::from_bits(val)
    }
}
impl From<EventsCtto> for u8 {
    #[inline(always)]
    fn from(val: EventsCtto) -> u8 {
        EventsCtto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsDone {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsDone {
    #[inline(always)]
    fn from(val: u8) -> EventsDone {
        EventsDone::from_bits(val)
    }
}
impl From<EventsDone> for u8 {
    #[inline(always)]
    fn from(val: EventsDone) -> u8 {
        EventsDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsHfclkstarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsHfclkstarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsHfclkstarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsHfclkstarted {
    #[inline(always)]
    fn from(val: u8) -> EventsHfclkstarted {
        EventsHfclkstarted::from_bits(val)
    }
}
impl From<EventsHfclkstarted> for u8 {
    #[inline(always)]
    fn from(val: EventsHfclkstarted) -> u8 {
        EventsHfclkstarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsLfclkstarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsLfclkstarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsLfclkstarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsLfclkstarted {
    #[inline(always)]
    fn from(val: u8) -> EventsLfclkstarted {
        EventsLfclkstarted::from_bits(val)
    }
}
impl From<EventsLfclkstarted> for u8 {
    #[inline(always)]
    fn from(val: EventsLfclkstarted) -> u8 {
        EventsLfclkstarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum External {
    #[doc = "Disable external source (use with Xtal)"]
    DISABLED = 0x0,
    #[doc = "Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    ENABLED = 0x01,
}
impl External {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> External {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for External {
    #[inline(always)]
    fn from(val: u8) -> External {
        External::from_bits(val)
    }
}
impl From<External> for u8 {
    #[inline(always)]
    fn from(val: External) -> u8 {
        External::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkrunStatus {
    #[doc = "Task not triggered"]
    NOTTRIGGERED = 0x0,
    #[doc = "Task triggered"]
    TRIGGERED = 0x01,
}
impl HfclkrunStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclkrunStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclkrunStatus {
    #[inline(always)]
    fn from(val: u8) -> HfclkrunStatus {
        HfclkrunStatus::from_bits(val)
    }
}
impl From<HfclkrunStatus> for u8 {
    #[inline(always)]
    fn from(val: HfclkrunStatus) -> u8 {
        HfclkrunStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkstatSrc {
    #[doc = "64 MHz internal oscillator (HFINT)"]
    RC = 0x0,
    #[doc = "64 MHz crystal oscillator (HFXO)"]
    XTAL = 0x01,
}
impl HfclkstatSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclkstatSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclkstatSrc {
    #[inline(always)]
    fn from(val: u8) -> HfclkstatSrc {
        HfclkstatSrc::from_bits(val)
    }
}
impl From<HfclkstatSrc> for u8 {
    #[inline(always)]
    fn from(val: HfclkstatSrc) -> u8 {
        HfclkstatSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkstatState {
    #[doc = "HFCLK not running"]
    NOTRUNNING = 0x0,
    #[doc = "HFCLK running"]
    RUNNING = 0x01,
}
impl HfclkstatState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclkstatState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclkstatState {
    #[inline(always)]
    fn from(val: u8) -> HfclkstatState {
        HfclkstatState::from_bits(val)
    }
}
impl From<HfclkstatState> for u8 {
    #[inline(always)]
    fn from(val: HfclkstatState) -> u8 {
        HfclkstatState::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hfxodebounce(pub u8);
impl Hfxodebounce {
    #[doc = "256 us debounce time. Recommended for 1.6 mm x 2.0 mm crystals and larger."]
    pub const DB256US: Self = Self(0x10);
    #[doc = "1024 us debounce time. Recommended for 1.6 mm x 1.2 mm crystals and smaller."]
    pub const DB1024US: Self = Self(0x40);
}
impl Hfxodebounce {
    pub const fn from_bits(val: u8) -> Hfxodebounce {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Hfxodebounce {
    #[inline(always)]
    fn from(val: u8) -> Hfxodebounce {
        Hfxodebounce::from_bits(val)
    }
}
impl From<Hfxodebounce> for u8 {
    #[inline(always)]
    fn from(val: Hfxodebounce) -> u8 {
        Hfxodebounce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LfclkrunStatus {
    #[doc = "Task not triggered"]
    NOTTRIGGERED = 0x0,
    #[doc = "Task triggered"]
    TRIGGERED = 0x01,
}
impl LfclkrunStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfclkrunStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfclkrunStatus {
    #[inline(always)]
    fn from(val: u8) -> LfclkrunStatus {
        LfclkrunStatus::from_bits(val)
    }
}
impl From<LfclkrunStatus> for u8 {
    #[inline(always)]
    fn from(val: LfclkrunStatus) -> u8 {
        LfclkrunStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LfclksrcSrc {
    #[doc = "32.768 kHz RC oscillator (LFRC)"]
    RC = 0x0,
    #[doc = "32.768 kHz crystal oscillator (LFXO)"]
    XTAL = 0x01,
    #[doc = "32.768 kHz synthesized from HFCLK (LFSYNT)"]
    SYNTH = 0x02,
    _RESERVED_3 = 0x03,
}
impl LfclksrcSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfclksrcSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfclksrcSrc {
    #[inline(always)]
    fn from(val: u8) -> LfclksrcSrc {
        LfclksrcSrc::from_bits(val)
    }
}
impl From<LfclksrcSrc> for u8 {
    #[inline(always)]
    fn from(val: LfclksrcSrc) -> u8 {
        LfclksrcSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LfclksrccopySrc {
    #[doc = "32.768 kHz RC oscillator (LFRC)"]
    RC = 0x0,
    #[doc = "32.768 kHz crystal oscillator (LFXO)"]
    XTAL = 0x01,
    #[doc = "32.768 kHz synthesized from HFCLK (LFSYNT)"]
    SYNTH = 0x02,
    _RESERVED_3 = 0x03,
}
impl LfclksrccopySrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfclksrccopySrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfclksrccopySrc {
    #[inline(always)]
    fn from(val: u8) -> LfclksrccopySrc {
        LfclksrccopySrc::from_bits(val)
    }
}
impl From<LfclksrccopySrc> for u8 {
    #[inline(always)]
    fn from(val: LfclksrccopySrc) -> u8 {
        LfclksrccopySrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LfclkstatSrc {
    #[doc = "32.768 kHz RC oscillator (LFRC)"]
    RC = 0x0,
    #[doc = "32.768 kHz crystal oscillator (LFXO)"]
    XTAL = 0x01,
    #[doc = "32.768 kHz synthesized from HFCLK (LFSYNT)"]
    SYNTH = 0x02,
    _RESERVED_3 = 0x03,
}
impl LfclkstatSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfclkstatSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfclkstatSrc {
    #[inline(always)]
    fn from(val: u8) -> LfclkstatSrc {
        LfclkstatSrc::from_bits(val)
    }
}
impl From<LfclkstatSrc> for u8 {
    #[inline(always)]
    fn from(val: LfclkstatSrc) -> u8 {
        LfclkstatSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LfclkstatState {
    #[doc = "LFCLK not running"]
    NOTRUNNING = 0x0,
    #[doc = "LFCLK running"]
    RUNNING = 0x01,
}
impl LfclkstatState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfclkstatState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfclkstatState {
    #[inline(always)]
    fn from(val: u8) -> LfclkstatState {
        LfclkstatState::from_bits(val)
    }
}
impl From<LfclkstatState> for u8 {
    #[inline(always)]
    fn from(val: LfclkstatState) -> u8 {
        LfclkstatState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lfxodebounce {
    #[doc = "8192 32.768 kHz periods, or 0.25 s. Recommended for normal Operating Temperature conditions."]
    NORMAL = 0x0,
    #[doc = "16384 32.768 kHz periods, or 0.5 s. Recommended for Extended Operating Temperature conditions."]
    EXTENDED = 0x01,
}
impl Lfxodebounce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lfxodebounce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lfxodebounce {
    #[inline(always)]
    fn from(val: u8) -> Lfxodebounce {
        Lfxodebounce::from_bits(val)
    }
}
impl From<Lfxodebounce> for u8 {
    #[inline(always)]
    fn from(val: Lfxodebounce) -> u8 {
        Lfxodebounce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksCal {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksCal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksCal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksCal {
    #[inline(always)]
    fn from(val: u8) -> TasksCal {
        TasksCal::from_bits(val)
    }
}
impl From<TasksCal> for u8 {
    #[inline(always)]
    fn from(val: TasksCal) -> u8 {
        TasksCal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksCtstart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksCtstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksCtstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksCtstart {
    #[inline(always)]
    fn from(val: u8) -> TasksCtstart {
        TasksCtstart::from_bits(val)
    }
}
impl From<TasksCtstart> for u8 {
    #[inline(always)]
    fn from(val: TasksCtstart) -> u8 {
        TasksCtstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksCtstop {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksCtstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksCtstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksCtstop {
    #[inline(always)]
    fn from(val: u8) -> TasksCtstop {
        TasksCtstop::from_bits(val)
    }
}
impl From<TasksCtstop> for u8 {
    #[inline(always)]
    fn from(val: TasksCtstop) -> u8 {
        TasksCtstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksHfclkstart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksHfclkstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksHfclkstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksHfclkstart {
    #[inline(always)]
    fn from(val: u8) -> TasksHfclkstart {
        TasksHfclkstart::from_bits(val)
    }
}
impl From<TasksHfclkstart> for u8 {
    #[inline(always)]
    fn from(val: TasksHfclkstart) -> u8 {
        TasksHfclkstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksHfclkstop {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksHfclkstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksHfclkstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksHfclkstop {
    #[inline(always)]
    fn from(val: u8) -> TasksHfclkstop {
        TasksHfclkstop::from_bits(val)
    }
}
impl From<TasksHfclkstop> for u8 {
    #[inline(always)]
    fn from(val: TasksHfclkstop) -> u8 {
        TasksHfclkstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksLfclkstart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksLfclkstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksLfclkstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksLfclkstart {
    #[inline(always)]
    fn from(val: u8) -> TasksLfclkstart {
        TasksLfclkstart::from_bits(val)
    }
}
impl From<TasksLfclkstart> for u8 {
    #[inline(always)]
    fn from(val: TasksLfclkstart) -> u8 {
        TasksLfclkstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksLfclkstop {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksLfclkstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksLfclkstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksLfclkstop {
    #[inline(always)]
    fn from(val: u8) -> TasksLfclkstop {
        TasksLfclkstop::from_bits(val)
    }
}
impl From<TasksLfclkstop> for u8 {
    #[inline(always)]
    fn from(val: TasksLfclkstop) -> u8 {
        TasksLfclkstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tracemux {
    #[doc = "No trace signals routed to pins. All pins can be used as regular GPIOs."]
    GPIO = 0x0,
    #[doc = "SWO trace signal routed to pin. Remaining pins can be used as regular GPIOs."]
    SERIAL = 0x01,
    #[doc = "All trace signals (TRACECLK and TRACEDATA\\[n\\]) routed to pins."]
    PARALLEL = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tracemux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tracemux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tracemux {
    #[inline(always)]
    fn from(val: u8) -> Tracemux {
        Tracemux::from_bits(val)
    }
}
impl From<Tracemux> for u8 {
    #[inline(always)]
    fn from(val: Tracemux) -> u8 {
        Tracemux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Traceportspeed {
    #[doc = "32 MHz trace port clock (TRACECLK = 16 MHz)"]
    _32MHZ = 0x0,
    #[doc = "16 MHz trace port clock (TRACECLK = 8 MHz)"]
    _16MHZ = 0x01,
    #[doc = "8 MHz trace port clock (TRACECLK = 4 MHz)"]
    _8MHZ = 0x02,
    #[doc = "4 MHz trace port clock (TRACECLK = 2 MHz)"]
    _4MHZ = 0x03,
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
