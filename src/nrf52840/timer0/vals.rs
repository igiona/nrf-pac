#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bitmode {
    #[doc = "16 bit timer bit width"]
    _16BIT = 0x0,
    #[doc = "8 bit timer bit width"]
    _08BIT = 0x01,
    #[doc = "24 bit timer bit width"]
    _24BIT = 0x02,
    #[doc = "32 bit timer bit width"]
    _32BIT = 0x03,
}
impl Bitmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bitmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bitmode {
    #[inline(always)]
    fn from(val: u8) -> Bitmode {
        Bitmode::from_bits(val)
    }
}
impl From<Bitmode> for u8 {
    #[inline(always)]
    fn from(val: Bitmode) -> u8 {
        Bitmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCompare {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCompare {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCompare {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCompare {
    #[inline(always)]
    fn from(val: u8) -> EventsCompare {
        EventsCompare::from_bits(val)
    }
}
impl From<EventsCompare> for u8 {
    #[inline(always)]
    fn from(val: EventsCompare) -> u8 {
        EventsCompare::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "Select Timer mode"]
    TIMER = 0x0,
    #[doc = "Deprecated enumerator - Select Counter mode"]
    COUNTER = 0x01,
    #[doc = "Select Low Power Counter mode"]
    LOWPOWERCOUNTER = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
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
pub enum TasksCapture {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksCapture {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksCapture {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksCapture {
    #[inline(always)]
    fn from(val: u8) -> TasksCapture {
        TasksCapture::from_bits(val)
    }
}
impl From<TasksCapture> for u8 {
    #[inline(always)]
    fn from(val: TasksCapture) -> u8 {
        TasksCapture::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksClear {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksClear {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksClear {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksClear {
    #[inline(always)]
    fn from(val: u8) -> TasksClear {
        TasksClear::from_bits(val)
    }
}
impl From<TasksClear> for u8 {
    #[inline(always)]
    fn from(val: TasksClear) -> u8 {
        TasksClear::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksCount {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksCount {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksCount {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksCount {
    #[inline(always)]
    fn from(val: u8) -> TasksCount {
        TasksCount::from_bits(val)
    }
}
impl From<TasksCount> for u8 {
    #[inline(always)]
    fn from(val: TasksCount) -> u8 {
        TasksCount::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksShutdown {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksShutdown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksShutdown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksShutdown {
    #[inline(always)]
    fn from(val: u8) -> TasksShutdown {
        TasksShutdown::from_bits(val)
    }
}
impl From<TasksShutdown> for u8 {
    #[inline(always)]
    fn from(val: TasksShutdown) -> u8 {
        TasksShutdown::to_bits(val)
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
