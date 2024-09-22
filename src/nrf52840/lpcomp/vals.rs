#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Anadetect {
    #[doc = "Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    CROSS = 0x0,
    #[doc = "Generate ANADETECT on upward crossing only"]
    UP = 0x01,
    #[doc = "Generate ANADETECT on downward crossing only"]
    DOWN = 0x02,
    _RESERVED_3 = 0x03,
}
impl Anadetect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Anadetect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Anadetect {
    #[inline(always)]
    fn from(val: u8) -> Anadetect {
        Anadetect::from_bits(val)
    }
}
impl From<Anadetect> for u8 {
    #[inline(always)]
    fn from(val: Anadetect) -> u8 {
        Anadetect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x03) }
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
pub enum EventsCross {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCross {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCross {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCross {
    #[inline(always)]
    fn from(val: u8) -> EventsCross {
        EventsCross::from_bits(val)
    }
}
impl From<EventsCross> for u8 {
    #[inline(always)]
    fn from(val: EventsCross) -> u8 {
        EventsCross::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsDown {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsDown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsDown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsDown {
    #[inline(always)]
    fn from(val: u8) -> EventsDown {
        EventsDown::from_bits(val)
    }
}
impl From<EventsDown> for u8 {
    #[inline(always)]
    fn from(val: EventsDown) -> u8 {
        EventsDown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsReady {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsReady {
    #[inline(always)]
    fn from(val: u8) -> EventsReady {
        EventsReady::from_bits(val)
    }
}
impl From<EventsReady> for u8 {
    #[inline(always)]
    fn from(val: EventsReady) -> u8 {
        EventsReady::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsUp {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsUp {
    #[inline(always)]
    fn from(val: u8) -> EventsUp {
        EventsUp::from_bits(val)
    }
}
impl From<EventsUp> for u8 {
    #[inline(always)]
    fn from(val: EventsUp) -> u8 {
        EventsUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Extrefsel {
    #[doc = "Use AIN0 as external analog reference"]
    ANALOGREFERENCE0 = 0x0,
    #[doc = "Use AIN1 as external analog reference"]
    ANALOGREFERENCE1 = 0x01,
}
impl Extrefsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extrefsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extrefsel {
    #[inline(always)]
    fn from(val: u8) -> Extrefsel {
        Extrefsel::from_bits(val)
    }
}
impl From<Extrefsel> for u8 {
    #[inline(always)]
    fn from(val: Extrefsel) -> u8 {
        Extrefsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hyst {
    #[doc = "Comparator hysteresis disabled"]
    DISABLED = 0x0,
    #[doc = "Comparator hysteresis enabled"]
    ENABLED = 0x01,
}
impl Hyst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hyst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hyst {
    #[inline(always)]
    fn from(val: u8) -> Hyst {
        Hyst::from_bits(val)
    }
}
impl From<Hyst> for u8 {
    #[inline(always)]
    fn from(val: Hyst) -> u8 {
        Hyst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Psel {
    #[doc = "AIN0 selected as analog input"]
    ANALOGINPUT0 = 0x0,
    #[doc = "AIN1 selected as analog input"]
    ANALOGINPUT1 = 0x01,
    #[doc = "AIN2 selected as analog input"]
    ANALOGINPUT2 = 0x02,
    #[doc = "AIN3 selected as analog input"]
    ANALOGINPUT3 = 0x03,
    #[doc = "AIN4 selected as analog input"]
    ANALOGINPUT4 = 0x04,
    #[doc = "AIN5 selected as analog input"]
    ANALOGINPUT5 = 0x05,
    #[doc = "AIN6 selected as analog input"]
    ANALOGINPUT6 = 0x06,
    #[doc = "AIN7 selected as analog input"]
    ANALOGINPUT7 = 0x07,
}
impl Psel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psel {
    #[inline(always)]
    fn from(val: u8) -> Psel {
        Psel::from_bits(val)
    }
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(val: Psel) -> u8 {
        Psel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Refsel {
    #[doc = "VDD * 1/8 selected as reference"]
    REF1_8VDD = 0x0,
    #[doc = "VDD * 2/8 selected as reference"]
    REF2_8VDD = 0x01,
    #[doc = "VDD * 3/8 selected as reference"]
    REF3_8VDD = 0x02,
    #[doc = "VDD * 4/8 selected as reference"]
    REF4_8VDD = 0x03,
    #[doc = "VDD * 5/8 selected as reference"]
    REF5_8VDD = 0x04,
    #[doc = "VDD * 6/8 selected as reference"]
    REF6_8VDD = 0x05,
    #[doc = "VDD * 7/8 selected as reference"]
    REF7_8VDD = 0x06,
    #[doc = "External analog reference selected"]
    AREF = 0x07,
    #[doc = "VDD * 1/16 selected as reference"]
    REF1_16VDD = 0x08,
    #[doc = "VDD * 3/16 selected as reference"]
    REF3_16VDD = 0x09,
    #[doc = "VDD * 5/16 selected as reference"]
    REF5_16VDD = 0x0a,
    #[doc = "VDD * 7/16 selected as reference"]
    REF7_16VDD = 0x0b,
    #[doc = "VDD * 9/16 selected as reference"]
    REF9_16VDD = 0x0c,
    #[doc = "VDD * 11/16 selected as reference"]
    REF11_16VDD = 0x0d,
    #[doc = "VDD * 13/16 selected as reference"]
    REF13_16VDD = 0x0e,
    #[doc = "VDD * 15/16 selected as reference"]
    REF15_16VDD = 0x0f,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Result {
    #[doc = "Input voltage is below the reference threshold (VIN+ &lt; VIN-)"]
    BELOW = 0x0,
    #[doc = "Input voltage is above the reference threshold (VIN+ &gt; VIN-)"]
    ABOVE = 0x01,
}
impl Result {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Result {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Result {
    #[inline(always)]
    fn from(val: u8) -> Result {
        Result::from_bits(val)
    }
}
impl From<Result> for u8 {
    #[inline(always)]
    fn from(val: Result) -> u8 {
        Result::to_bits(val)
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
