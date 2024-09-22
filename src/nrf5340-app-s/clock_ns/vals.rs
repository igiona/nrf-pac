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
pub enum EventsHfclk192mstarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsHfclk192mstarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsHfclk192mstarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsHfclk192mstarted {
    #[inline(always)]
    fn from(val: u8) -> EventsHfclk192mstarted {
        EventsHfclk192mstarted::from_bits(val)
    }
}
impl From<EventsHfclk192mstarted> for u8 {
    #[inline(always)]
    fn from(val: EventsHfclk192mstarted) -> u8 {
        EventsHfclk192mstarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsHfclkaudiostarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsHfclkaudiostarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsHfclkaudiostarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsHfclkaudiostarted {
    #[inline(always)]
    fn from(val: u8) -> EventsHfclkaudiostarted {
        EventsHfclkaudiostarted::from_bits(val)
    }
}
impl From<EventsHfclkaudiostarted> for u8 {
    #[inline(always)]
    fn from(val: EventsHfclkaudiostarted) -> u8 {
        EventsHfclkaudiostarted::to_bits(val)
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
pub enum Hclk {
    #[doc = "Divide HFCLK by 1"]
    DIV1 = 0x0,
    #[doc = "Divide HFCLK by 2"]
    DIV2 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Hclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hclk {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hclk {
    #[inline(always)]
    fn from(val: u8) -> Hclk {
        Hclk::from_bits(val)
    }
}
impl From<Hclk> for u8 {
    #[inline(always)]
    fn from(val: Hclk) -> u8 {
        Hclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hclk192m {
    #[doc = "Divide HFCLK192M by 1"]
    DIV1 = 0x0,
    #[doc = "Divide HFCLK192M by 2"]
    DIV2 = 0x01,
    #[doc = "Divide HFCLK192M by 4"]
    DIV4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Hclk192m {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hclk192m {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hclk192m {
    #[inline(always)]
    fn from(val: u8) -> Hclk192m {
        Hclk192m::from_bits(val)
    }
}
impl From<Hclk192m> for u8 {
    #[inline(always)]
    fn from(val: Hclk192m) -> u8 {
        Hclk192m::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hfclk192malwaysrunAlwaysrun {
    #[doc = "Use automatic clock control"]
    AUTOMATIC = 0x0,
    #[doc = "Ensure clock is always running"]
    ALWAYSRUN = 0x01,
}
impl Hfclk192malwaysrunAlwaysrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hfclk192malwaysrunAlwaysrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hfclk192malwaysrunAlwaysrun {
    #[inline(always)]
    fn from(val: u8) -> Hfclk192malwaysrunAlwaysrun {
        Hfclk192malwaysrunAlwaysrun::from_bits(val)
    }
}
impl From<Hfclk192malwaysrunAlwaysrun> for u8 {
    #[inline(always)]
    fn from(val: Hfclk192malwaysrunAlwaysrun) -> u8 {
        Hfclk192malwaysrunAlwaysrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hfclk192mrunStatus {
    #[doc = "Task not triggered"]
    NOTTRIGGERED = 0x0,
    #[doc = "Task triggered"]
    TRIGGERED = 0x01,
}
impl Hfclk192mrunStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hfclk192mrunStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hfclk192mrunStatus {
    #[inline(always)]
    fn from(val: u8) -> Hfclk192mrunStatus {
        Hfclk192mrunStatus::from_bits(val)
    }
}
impl From<Hfclk192mrunStatus> for u8 {
    #[inline(always)]
    fn from(val: Hfclk192mrunStatus) -> u8 {
        Hfclk192mrunStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hfclk192msrcSrc {
    #[doc = "HFCLK192MSTART task starts HFINT oscillator"]
    HFINT = 0x0,
    #[doc = "HFCLK192MSTART task starts HFXO oscillator"]
    HFXO = 0x01,
}
impl Hfclk192msrcSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hfclk192msrcSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hfclk192msrcSrc {
    #[inline(always)]
    fn from(val: u8) -> Hfclk192msrcSrc {
        Hfclk192msrcSrc::from_bits(val)
    }
}
impl From<Hfclk192msrcSrc> for u8 {
    #[inline(always)]
    fn from(val: Hfclk192msrcSrc) -> u8 {
        Hfclk192msrcSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hfclk192mstatAlwaysrunning {
    #[doc = "Automatic clock control enabled"]
    NOTRUNNING = 0x0,
    #[doc = "Oscillator is always running"]
    RUNNING = 0x01,
}
impl Hfclk192mstatAlwaysrunning {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hfclk192mstatAlwaysrunning {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hfclk192mstatAlwaysrunning {
    #[inline(always)]
    fn from(val: u8) -> Hfclk192mstatAlwaysrunning {
        Hfclk192mstatAlwaysrunning::from_bits(val)
    }
}
impl From<Hfclk192mstatAlwaysrunning> for u8 {
    #[inline(always)]
    fn from(val: Hfclk192mstatAlwaysrunning) -> u8 {
        Hfclk192mstatAlwaysrunning::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hfclk192mstatSrc {
    #[doc = "Clock source: HFINT - on-chip oscillator"]
    HFINT = 0x0,
    #[doc = "Clock source: HFXO - derived from external 32 MHz crystal oscillator"]
    HFXO = 0x01,
}
impl Hfclk192mstatSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hfclk192mstatSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hfclk192mstatSrc {
    #[inline(always)]
    fn from(val: u8) -> Hfclk192mstatSrc {
        Hfclk192mstatSrc::from_bits(val)
    }
}
impl From<Hfclk192mstatSrc> for u8 {
    #[inline(always)]
    fn from(val: Hfclk192mstatSrc) -> u8 {
        Hfclk192mstatSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hfclk192mstatState {
    #[doc = "HFCLK192M not running"]
    NOTRUNNING = 0x0,
    #[doc = "HFCLK192M running"]
    RUNNING = 0x01,
}
impl Hfclk192mstatState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hfclk192mstatState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hfclk192mstatState {
    #[inline(always)]
    fn from(val: u8) -> Hfclk192mstatState {
        Hfclk192mstatState::from_bits(val)
    }
}
impl From<Hfclk192mstatState> for u8 {
    #[inline(always)]
    fn from(val: Hfclk192mstatState) -> u8 {
        Hfclk192mstatState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkalwaysrunAlwaysrun {
    #[doc = "Use automatic clock control"]
    AUTOMATIC = 0x0,
    #[doc = "Ensure clock is always running"]
    ALWAYSRUN = 0x01,
}
impl HfclkalwaysrunAlwaysrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclkalwaysrunAlwaysrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclkalwaysrunAlwaysrun {
    #[inline(always)]
    fn from(val: u8) -> HfclkalwaysrunAlwaysrun {
        HfclkalwaysrunAlwaysrun::from_bits(val)
    }
}
impl From<HfclkalwaysrunAlwaysrun> for u8 {
    #[inline(always)]
    fn from(val: HfclkalwaysrunAlwaysrun) -> u8 {
        HfclkalwaysrunAlwaysrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkaudioalwaysrunAlwaysrun {
    #[doc = "Use automatic clock control"]
    AUTOMATIC = 0x0,
    #[doc = "Ensure clock is always running"]
    ALWAYSRUN = 0x01,
}
impl HfclkaudioalwaysrunAlwaysrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclkaudioalwaysrunAlwaysrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclkaudioalwaysrunAlwaysrun {
    #[inline(always)]
    fn from(val: u8) -> HfclkaudioalwaysrunAlwaysrun {
        HfclkaudioalwaysrunAlwaysrun::from_bits(val)
    }
}
impl From<HfclkaudioalwaysrunAlwaysrun> for u8 {
    #[inline(always)]
    fn from(val: HfclkaudioalwaysrunAlwaysrun) -> u8 {
        HfclkaudioalwaysrunAlwaysrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkaudiorunStatus {
    #[doc = "Task not triggered"]
    NOTTRIGGERED = 0x0,
    #[doc = "Task triggered"]
    TRIGGERED = 0x01,
}
impl HfclkaudiorunStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclkaudiorunStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclkaudiorunStatus {
    #[inline(always)]
    fn from(val: u8) -> HfclkaudiorunStatus {
        HfclkaudiorunStatus::from_bits(val)
    }
}
impl From<HfclkaudiorunStatus> for u8 {
    #[inline(always)]
    fn from(val: HfclkaudiorunStatus) -> u8 {
        HfclkaudiorunStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkaudiostatAlwaysrunning {
    #[doc = "Automatic clock control enabled"]
    NOTRUNNING = 0x0,
    #[doc = "Oscillator is always running"]
    RUNNING = 0x01,
}
impl HfclkaudiostatAlwaysrunning {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclkaudiostatAlwaysrunning {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclkaudiostatAlwaysrunning {
    #[inline(always)]
    fn from(val: u8) -> HfclkaudiostatAlwaysrunning {
        HfclkaudiostatAlwaysrunning::from_bits(val)
    }
}
impl From<HfclkaudiostatAlwaysrunning> for u8 {
    #[inline(always)]
    fn from(val: HfclkaudiostatAlwaysrunning) -> u8 {
        HfclkaudiostatAlwaysrunning::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkaudiostatState {
    #[doc = "HFCLKAUDIO not running"]
    NOTRUNNING = 0x0,
    #[doc = "HFCLKAUDIO running"]
    RUNNING = 0x01,
}
impl HfclkaudiostatState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclkaudiostatState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclkaudiostatState {
    #[inline(always)]
    fn from(val: u8) -> HfclkaudiostatState {
        HfclkaudiostatState::from_bits(val)
    }
}
impl From<HfclkaudiostatState> for u8 {
    #[inline(always)]
    fn from(val: HfclkaudiostatState) -> u8 {
        HfclkaudiostatState::to_bits(val)
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
pub enum HfclksrcSrc {
    #[doc = "HFCLKSTART task starts HFINT oscillator"]
    HFINT = 0x0,
    #[doc = "HFCLKSTART task starts HFXO oscillator"]
    HFXO = 0x01,
}
impl HfclksrcSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclksrcSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclksrcSrc {
    #[inline(always)]
    fn from(val: u8) -> HfclksrcSrc {
        HfclksrcSrc::from_bits(val)
    }
}
impl From<HfclksrcSrc> for u8 {
    #[inline(always)]
    fn from(val: HfclksrcSrc) -> u8 {
        HfclksrcSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkstatAlwaysrunning {
    #[doc = "Automatic clock control enabled"]
    NOTRUNNING = 0x0,
    #[doc = "Oscillator is always running"]
    RUNNING = 0x01,
}
impl HfclkstatAlwaysrunning {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclkstatAlwaysrunning {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclkstatAlwaysrunning {
    #[inline(always)]
    fn from(val: u8) -> HfclkstatAlwaysrunning {
        HfclkstatAlwaysrunning::from_bits(val)
    }
}
impl From<HfclkstatAlwaysrunning> for u8 {
    #[inline(always)]
    fn from(val: HfclkstatAlwaysrunning) -> u8 {
        HfclkstatAlwaysrunning::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkstatSrc {
    #[doc = "Clock source: HFINT - 128 MHz on-chip oscillator"]
    HFINT = 0x0,
    #[doc = "Clock source: HFXO - 128 MHz clock derived from external 32 MHz crystal oscillator"]
    HFXO = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendDone {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendDone {
    #[inline(always)]
    fn from(val: u8) -> IntpendDone {
        IntpendDone::from_bits(val)
    }
}
impl From<IntpendDone> for u8 {
    #[inline(always)]
    fn from(val: IntpendDone) -> u8 {
        IntpendDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendHfclk192mstarted {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendHfclk192mstarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendHfclk192mstarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendHfclk192mstarted {
    #[inline(always)]
    fn from(val: u8) -> IntpendHfclk192mstarted {
        IntpendHfclk192mstarted::from_bits(val)
    }
}
impl From<IntpendHfclk192mstarted> for u8 {
    #[inline(always)]
    fn from(val: IntpendHfclk192mstarted) -> u8 {
        IntpendHfclk192mstarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendHfclkaudiostarted {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendHfclkaudiostarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendHfclkaudiostarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendHfclkaudiostarted {
    #[inline(always)]
    fn from(val: u8) -> IntpendHfclkaudiostarted {
        IntpendHfclkaudiostarted::from_bits(val)
    }
}
impl From<IntpendHfclkaudiostarted> for u8 {
    #[inline(always)]
    fn from(val: IntpendHfclkaudiostarted) -> u8 {
        IntpendHfclkaudiostarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendHfclkstarted {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendHfclkstarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendHfclkstarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendHfclkstarted {
    #[inline(always)]
    fn from(val: u8) -> IntpendHfclkstarted {
        IntpendHfclkstarted::from_bits(val)
    }
}
impl From<IntpendHfclkstarted> for u8 {
    #[inline(always)]
    fn from(val: IntpendHfclkstarted) -> u8 {
        IntpendHfclkstarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendLfclkstarted {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendLfclkstarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendLfclkstarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendLfclkstarted {
    #[inline(always)]
    fn from(val: u8) -> IntpendLfclkstarted {
        IntpendLfclkstarted::from_bits(val)
    }
}
impl From<IntpendLfclkstarted> for u8 {
    #[inline(always)]
    fn from(val: IntpendLfclkstarted) -> u8 {
        IntpendLfclkstarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LfclkalwaysrunAlwaysrun {
    #[doc = "Use automatic clock control"]
    AUTOMATIC = 0x0,
    #[doc = "Ensure clock is always running"]
    ALWAYSRUN = 0x01,
}
impl LfclkalwaysrunAlwaysrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfclkalwaysrunAlwaysrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfclkalwaysrunAlwaysrun {
    #[inline(always)]
    fn from(val: u8) -> LfclkalwaysrunAlwaysrun {
        LfclkalwaysrunAlwaysrun::from_bits(val)
    }
}
impl From<LfclkalwaysrunAlwaysrun> for u8 {
    #[inline(always)]
    fn from(val: LfclkalwaysrunAlwaysrun) -> u8 {
        LfclkalwaysrunAlwaysrun::to_bits(val)
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
    _RESERVED_0 = 0x0,
    #[doc = "32.768 kHz RC oscillator"]
    LFRC = 0x01,
    #[doc = "32.768 kHz crystal oscillator"]
    LFXO = 0x02,
    #[doc = "32.768 kHz synthesized from HFCLK"]
    LFSYNT = 0x03,
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
    _RESERVED_0 = 0x0,
    #[doc = "32.768 kHz RC oscillator"]
    LFRC = 0x01,
    #[doc = "32.768 kHz crystal oscillator"]
    LFXO = 0x02,
    #[doc = "32.768 kHz synthesized from HFCLK"]
    LFSYNT = 0x03,
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
pub enum LfclkstatAlwaysrunning {
    #[doc = "Automatic clock control enabled"]
    NOTRUNNING = 0x0,
    #[doc = "Oscillator is always running"]
    RUNNING = 0x01,
}
impl LfclkstatAlwaysrunning {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfclkstatAlwaysrunning {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfclkstatAlwaysrunning {
    #[inline(always)]
    fn from(val: u8) -> LfclkstatAlwaysrunning {
        LfclkstatAlwaysrunning::from_bits(val)
    }
}
impl From<LfclkstatAlwaysrunning> for u8 {
    #[inline(always)]
    fn from(val: LfclkstatAlwaysrunning) -> u8 {
        LfclkstatAlwaysrunning::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LfclkstatSrc {
    _RESERVED_0 = 0x0,
    #[doc = "32.768 kHz RC oscillator"]
    LFRC = 0x01,
    #[doc = "32.768 kHz crystal oscillator"]
    LFXO = 0x02,
    #[doc = "32.768 kHz synthesized from HFCLK"]
    LFSYNT = 0x03,
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
pub enum PublishDoneEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishDoneEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishDoneEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishDoneEn {
    #[inline(always)]
    fn from(val: u8) -> PublishDoneEn {
        PublishDoneEn::from_bits(val)
    }
}
impl From<PublishDoneEn> for u8 {
    #[inline(always)]
    fn from(val: PublishDoneEn) -> u8 {
        PublishDoneEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishHfclk192mstartedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishHfclk192mstartedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishHfclk192mstartedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishHfclk192mstartedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishHfclk192mstartedEn {
        PublishHfclk192mstartedEn::from_bits(val)
    }
}
impl From<PublishHfclk192mstartedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishHfclk192mstartedEn) -> u8 {
        PublishHfclk192mstartedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishHfclkaudiostartedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishHfclkaudiostartedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishHfclkaudiostartedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishHfclkaudiostartedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishHfclkaudiostartedEn {
        PublishHfclkaudiostartedEn::from_bits(val)
    }
}
impl From<PublishHfclkaudiostartedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishHfclkaudiostartedEn) -> u8 {
        PublishHfclkaudiostartedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishHfclkstartedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishHfclkstartedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishHfclkstartedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishHfclkstartedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishHfclkstartedEn {
        PublishHfclkstartedEn::from_bits(val)
    }
}
impl From<PublishHfclkstartedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishHfclkstartedEn) -> u8 {
        PublishHfclkstartedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishLfclkstartedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishLfclkstartedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishLfclkstartedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishLfclkstartedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishLfclkstartedEn {
        PublishLfclkstartedEn::from_bits(val)
    }
}
impl From<PublishLfclkstartedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishLfclkstartedEn) -> u8 {
        PublishLfclkstartedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeCalEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeCalEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeCalEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeCalEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeCalEn {
        SubscribeCalEn::from_bits(val)
    }
}
impl From<SubscribeCalEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeCalEn) -> u8 {
        SubscribeCalEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeHfclk192mstartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeHfclk192mstartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeHfclk192mstartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeHfclk192mstartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeHfclk192mstartEn {
        SubscribeHfclk192mstartEn::from_bits(val)
    }
}
impl From<SubscribeHfclk192mstartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeHfclk192mstartEn) -> u8 {
        SubscribeHfclk192mstartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeHfclk192mstopEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeHfclk192mstopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeHfclk192mstopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeHfclk192mstopEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeHfclk192mstopEn {
        SubscribeHfclk192mstopEn::from_bits(val)
    }
}
impl From<SubscribeHfclk192mstopEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeHfclk192mstopEn) -> u8 {
        SubscribeHfclk192mstopEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeHfclkaudiostartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeHfclkaudiostartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeHfclkaudiostartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeHfclkaudiostartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeHfclkaudiostartEn {
        SubscribeHfclkaudiostartEn::from_bits(val)
    }
}
impl From<SubscribeHfclkaudiostartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeHfclkaudiostartEn) -> u8 {
        SubscribeHfclkaudiostartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeHfclkaudiostopEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeHfclkaudiostopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeHfclkaudiostopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeHfclkaudiostopEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeHfclkaudiostopEn {
        SubscribeHfclkaudiostopEn::from_bits(val)
    }
}
impl From<SubscribeHfclkaudiostopEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeHfclkaudiostopEn) -> u8 {
        SubscribeHfclkaudiostopEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeHfclkstartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeHfclkstartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeHfclkstartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeHfclkstartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeHfclkstartEn {
        SubscribeHfclkstartEn::from_bits(val)
    }
}
impl From<SubscribeHfclkstartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeHfclkstartEn) -> u8 {
        SubscribeHfclkstartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeHfclkstopEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeHfclkstopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeHfclkstopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeHfclkstopEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeHfclkstopEn {
        SubscribeHfclkstopEn::from_bits(val)
    }
}
impl From<SubscribeHfclkstopEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeHfclkstopEn) -> u8 {
        SubscribeHfclkstopEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeLfclkstartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeLfclkstartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeLfclkstartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeLfclkstartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeLfclkstartEn {
        SubscribeLfclkstartEn::from_bits(val)
    }
}
impl From<SubscribeLfclkstartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeLfclkstartEn) -> u8 {
        SubscribeLfclkstartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeLfclkstopEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeLfclkstopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeLfclkstopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeLfclkstopEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeLfclkstopEn {
        SubscribeLfclkstopEn::from_bits(val)
    }
}
impl From<SubscribeLfclkstopEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeLfclkstopEn) -> u8 {
        SubscribeLfclkstopEn::to_bits(val)
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
pub enum TasksHfclk192mstart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksHfclk192mstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksHfclk192mstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksHfclk192mstart {
    #[inline(always)]
    fn from(val: u8) -> TasksHfclk192mstart {
        TasksHfclk192mstart::from_bits(val)
    }
}
impl From<TasksHfclk192mstart> for u8 {
    #[inline(always)]
    fn from(val: TasksHfclk192mstart) -> u8 {
        TasksHfclk192mstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksHfclk192mstop {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksHfclk192mstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksHfclk192mstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksHfclk192mstop {
    #[inline(always)]
    fn from(val: u8) -> TasksHfclk192mstop {
        TasksHfclk192mstop::from_bits(val)
    }
}
impl From<TasksHfclk192mstop> for u8 {
    #[inline(always)]
    fn from(val: TasksHfclk192mstop) -> u8 {
        TasksHfclk192mstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksHfclkaudiostart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksHfclkaudiostart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksHfclkaudiostart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksHfclkaudiostart {
    #[inline(always)]
    fn from(val: u8) -> TasksHfclkaudiostart {
        TasksHfclkaudiostart::from_bits(val)
    }
}
impl From<TasksHfclkaudiostart> for u8 {
    #[inline(always)]
    fn from(val: TasksHfclkaudiostart) -> u8 {
        TasksHfclkaudiostart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksHfclkaudiostop {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksHfclkaudiostop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksHfclkaudiostop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksHfclkaudiostop {
    #[inline(always)]
    fn from(val: u8) -> TasksHfclkaudiostop {
        TasksHfclkaudiostop::from_bits(val)
    }
}
impl From<TasksHfclkaudiostop> for u8 {
    #[inline(always)]
    fn from(val: TasksHfclkaudiostop) -> u8 {
        TasksHfclkaudiostop::to_bits(val)
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
