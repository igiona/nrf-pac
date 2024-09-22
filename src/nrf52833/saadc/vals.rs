#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Burst {
    #[doc = "Burst mode is disabled (normal operation)"]
    DISABLED = 0x0,
    #[doc = "Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    ENABLED = 0x01,
}
impl Burst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Burst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Burst {
    #[inline(always)]
    fn from(val: u8) -> Burst {
        Burst::from_bits(val)
    }
}
impl From<Burst> for u8 {
    #[inline(always)]
    fn from(val: Burst) -> u8 {
        Burst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConfigMode {
    #[doc = "Single-ended, PSELN will be ignored, negative input to SAADC shorted to GND"]
    SE = 0x0,
    #[doc = "Differential"]
    DIFF = 0x01,
}
impl ConfigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfigMode {
    #[inline(always)]
    fn from(val: u8) -> ConfigMode {
        ConfigMode::from_bits(val)
    }
}
impl From<ConfigMode> for u8 {
    #[inline(always)]
    fn from(val: ConfigMode) -> u8 {
        ConfigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disable SAADC"]
    DISABLED = 0x0,
    #[doc = "Enable SAADC"]
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
pub enum EventsCalibratedone {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCalibratedone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCalibratedone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCalibratedone {
    #[inline(always)]
    fn from(val: u8) -> EventsCalibratedone {
        EventsCalibratedone::from_bits(val)
    }
}
impl From<EventsCalibratedone> for u8 {
    #[inline(always)]
    fn from(val: EventsCalibratedone) -> u8 {
        EventsCalibratedone::to_bits(val)
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
pub enum EventsResultdone {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsResultdone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsResultdone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsResultdone {
    #[inline(always)]
    fn from(val: u8) -> EventsResultdone {
        EventsResultdone::from_bits(val)
    }
}
impl From<EventsResultdone> for u8 {
    #[inline(always)]
    fn from(val: EventsResultdone) -> u8 {
        EventsResultdone::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gain {
    #[doc = "1/6"]
    GAIN1_6 = 0x0,
    #[doc = "1/5"]
    GAIN1_5 = 0x01,
    #[doc = "1/4"]
    GAIN1_4 = 0x02,
    #[doc = "1/3"]
    GAIN1_3 = 0x03,
    #[doc = "1/2"]
    GAIN1_2 = 0x04,
    #[doc = "1"]
    GAIN1 = 0x05,
    #[doc = "2"]
    GAIN2 = 0x06,
    #[doc = "4"]
    GAIN4 = 0x07,
}
impl Gain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gain {
    #[inline(always)]
    fn from(val: u8) -> Gain {
        Gain::from_bits(val)
    }
}
impl From<Gain> for u8 {
    #[inline(always)]
    fn from(val: Gain) -> u8 {
        Gain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Limith {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl Limith {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Limith {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Limith {
    #[inline(always)]
    fn from(val: u8) -> Limith {
        Limith::from_bits(val)
    }
}
impl From<Limith> for u8 {
    #[inline(always)]
    fn from(val: Limith) -> u8 {
        Limith::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Limitl {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl Limitl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Limitl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Limitl {
    #[inline(always)]
    fn from(val: u8) -> Limitl {
        Limitl::from_bits(val)
    }
}
impl From<Limitl> for u8 {
    #[inline(always)]
    fn from(val: Limitl) -> u8 {
        Limitl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Oversample {
    #[doc = "Bypass oversampling"]
    BYPASS = 0x0,
    #[doc = "Oversample 2x"]
    OVER2X = 0x01,
    #[doc = "Oversample 4x"]
    OVER4X = 0x02,
    #[doc = "Oversample 8x"]
    OVER8X = 0x03,
    #[doc = "Oversample 16x"]
    OVER16X = 0x04,
    #[doc = "Oversample 32x"]
    OVER32X = 0x05,
    #[doc = "Oversample 64x"]
    OVER64X = 0x06,
    #[doc = "Oversample 128x"]
    OVER128X = 0x07,
    #[doc = "Oversample 256x"]
    OVER256X = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Oversample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oversample {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oversample {
    #[inline(always)]
    fn from(val: u8) -> Oversample {
        Oversample::from_bits(val)
    }
}
impl From<Oversample> for u8 {
    #[inline(always)]
    fn from(val: Oversample) -> u8 {
        Oversample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pseln {
    #[doc = "Not connected"]
    NC = 0x0,
    #[doc = "AIN0"]
    ANALOGINPUT0 = 0x01,
    #[doc = "AIN1"]
    ANALOGINPUT1 = 0x02,
    #[doc = "AIN2"]
    ANALOGINPUT2 = 0x03,
    #[doc = "AIN3"]
    ANALOGINPUT3 = 0x04,
    #[doc = "AIN4"]
    ANALOGINPUT4 = 0x05,
    #[doc = "AIN5"]
    ANALOGINPUT5 = 0x06,
    #[doc = "AIN6"]
    ANALOGINPUT6 = 0x07,
    #[doc = "AIN7"]
    ANALOGINPUT7 = 0x08,
    #[doc = "VDD"]
    VDD = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "VDDH/5"]
    VDDHDIV5 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pseln {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pseln {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pseln {
    #[inline(always)]
    fn from(val: u8) -> Pseln {
        Pseln::from_bits(val)
    }
}
impl From<Pseln> for u8 {
    #[inline(always)]
    fn from(val: Pseln) -> u8 {
        Pseln::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pselp {
    #[doc = "Not connected"]
    NC = 0x0,
    #[doc = "AIN0"]
    ANALOGINPUT0 = 0x01,
    #[doc = "AIN1"]
    ANALOGINPUT1 = 0x02,
    #[doc = "AIN2"]
    ANALOGINPUT2 = 0x03,
    #[doc = "AIN3"]
    ANALOGINPUT3 = 0x04,
    #[doc = "AIN4"]
    ANALOGINPUT4 = 0x05,
    #[doc = "AIN5"]
    ANALOGINPUT5 = 0x06,
    #[doc = "AIN6"]
    ANALOGINPUT6 = 0x07,
    #[doc = "AIN7"]
    ANALOGINPUT7 = 0x08,
    #[doc = "VDD"]
    VDD = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "VDDH/5"]
    VDDHDIV5 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pselp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pselp {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pselp {
    #[inline(always)]
    fn from(val: u8) -> Pselp {
        Pselp::from_bits(val)
    }
}
impl From<Pselp> for u8 {
    #[inline(always)]
    fn from(val: Pselp) -> u8 {
        Pselp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Refsel {
    #[doc = "Internal reference (0.6 V)"]
    INTERNAL = 0x0,
    #[doc = "VDD/4 as reference"]
    VDD1_4 = 0x01,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resn {
    #[doc = "Bypass resistor ladder"]
    BYPASS = 0x0,
    #[doc = "Pull-down to GND"]
    PULLDOWN = 0x01,
    #[doc = "Pull-up to VDD"]
    PULLUP = 0x02,
    #[doc = "Set input at VDD/2"]
    VDD1_2 = 0x03,
}
impl Resn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resn {
    #[inline(always)]
    fn from(val: u8) -> Resn {
        Resn::from_bits(val)
    }
}
impl From<Resn> for u8 {
    #[inline(always)]
    fn from(val: Resn) -> u8 {
        Resn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resp {
    #[doc = "Bypass resistor ladder"]
    BYPASS = 0x0,
    #[doc = "Pull-down to GND"]
    PULLDOWN = 0x01,
    #[doc = "Pull-up to VDD"]
    PULLUP = 0x02,
    #[doc = "Set input at VDD/2"]
    VDD1_2 = 0x03,
}
impl Resp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resp {
    #[inline(always)]
    fn from(val: u8) -> Resp {
        Resp::from_bits(val)
    }
}
impl From<Resp> for u8 {
    #[inline(always)]
    fn from(val: Resp) -> u8 {
        Resp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SamplerateMode {
    #[doc = "Rate is controlled from SAMPLE task"]
    TASK = 0x0,
    #[doc = "Rate is controlled from local timer (use CC to control the rate)"]
    TIMERS = 0x01,
}
impl SamplerateMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SamplerateMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SamplerateMode {
    #[inline(always)]
    fn from(val: u8) -> SamplerateMode {
        SamplerateMode::from_bits(val)
    }
}
impl From<SamplerateMode> for u8 {
    #[inline(always)]
    fn from(val: SamplerateMode) -> u8 {
        SamplerateMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Status {
    #[doc = "SAADC is ready. No on-going conversions."]
    READY = 0x0,
    #[doc = "SAADC is busy. Conversion in progress."]
    BUSY = 0x01,
}
impl Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Status {
    #[inline(always)]
    fn from(val: u8) -> Status {
        Status::from_bits(val)
    }
}
impl From<Status> for u8 {
    #[inline(always)]
    fn from(val: Status) -> u8 {
        Status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tacq {
    #[doc = "3 us"]
    _3US = 0x0,
    #[doc = "5 us"]
    _5US = 0x01,
    #[doc = "10 us"]
    _10US = 0x02,
    #[doc = "15 us"]
    _15US = 0x03,
    #[doc = "20 us"]
    _20US = 0x04,
    #[doc = "40 us"]
    _40US = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tacq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tacq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tacq {
    #[inline(always)]
    fn from(val: u8) -> Tacq {
        Tacq::from_bits(val)
    }
}
impl From<Tacq> for u8 {
    #[inline(always)]
    fn from(val: Tacq) -> u8 {
        Tacq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksCalibrateoffset {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksCalibrateoffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksCalibrateoffset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksCalibrateoffset {
    #[inline(always)]
    fn from(val: u8) -> TasksCalibrateoffset {
        TasksCalibrateoffset::from_bits(val)
    }
}
impl From<TasksCalibrateoffset> for u8 {
    #[inline(always)]
    fn from(val: TasksCalibrateoffset) -> u8 {
        TasksCalibrateoffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksSample {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksSample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksSample {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksSample {
    #[inline(always)]
    fn from(val: u8) -> TasksSample {
        TasksSample::from_bits(val)
    }
}
impl From<TasksSample> for u8 {
    #[inline(always)]
    fn from(val: TasksSample) -> u8 {
        TasksSample::to_bits(val)
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
pub enum Val {
    #[doc = "8 bits"]
    _8BIT = 0x0,
    #[doc = "10 bits"]
    _10BIT = 0x01,
    #[doc = "12 bits"]
    _12BIT = 0x02,
    #[doc = "14 bits"]
    _14BIT = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Val {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val {
    #[inline(always)]
    fn from(val: u8) -> Val {
        Val::from_bits(val)
    }
}
impl From<Val> for u8 {
    #[inline(always)]
    fn from(val: Val) -> u8 {
        Val::to_bits(val)
    }
}
