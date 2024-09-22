#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsIn {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsIn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsIn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsIn {
    #[inline(always)]
    fn from(val: u8) -> EventsIn {
        EventsIn::from_bits(val)
    }
}
impl From<EventsIn> for u8 {
    #[inline(always)]
    fn from(val: EventsIn) -> u8 {
        EventsIn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsPort {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsPort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsPort {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsPort {
    #[inline(always)]
    fn from(val: u8) -> EventsPort {
        EventsPort::from_bits(val)
    }
}
impl From<EventsPort> for u8 {
    #[inline(always)]
    fn from(val: EventsPort) -> u8 {
        EventsPort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "Disabled. Pin specified by PSEL will not be acquired by the GPIOTE module."]
    DISABLED = 0x0,
    #[doc = "Event mode"]
    EVENT = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Task mode"]
    TASK = 0x03,
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
pub enum Outinit {
    #[doc = "Task mode: Initial value of pin before task triggering is low"]
    LOW = 0x0,
    #[doc = "Task mode: Initial value of pin before task triggering is high"]
    HIGH = 0x01,
}
impl Outinit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outinit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outinit {
    #[inline(always)]
    fn from(val: u8) -> Outinit {
        Outinit::from_bits(val)
    }
}
impl From<Outinit> for u8 {
    #[inline(always)]
    fn from(val: Outinit) -> u8 {
        Outinit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Polarity {
    #[doc = "Task mode: No effect on pin from OUT\\[n\\] task. Event mode: no IN\\[n\\] event generated on pin activity."]
    NONE = 0x0,
    #[doc = "Task mode: Set pin from OUT\\[n\\] task. Event mode: Generate IN\\[n\\] event when rising edge on pin."]
    LOTOHI = 0x01,
    #[doc = "Task mode: Clear pin from OUT\\[n\\] task. Event mode: Generate IN\\[n\\] event when falling edge on pin."]
    HITOLO = 0x02,
    #[doc = "Task mode: Toggle pin from OUT\\[n\\]. Event mode: Generate IN\\[n\\] when any change on pin."]
    TOGGLE = 0x03,
}
impl Polarity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Polarity {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Polarity {
    #[inline(always)]
    fn from(val: u8) -> Polarity {
        Polarity::from_bits(val)
    }
}
impl From<Polarity> for u8 {
    #[inline(always)]
    fn from(val: Polarity) -> u8 {
        Polarity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksClr {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksClr {
    #[inline(always)]
    fn from(val: u8) -> TasksClr {
        TasksClr::from_bits(val)
    }
}
impl From<TasksClr> for u8 {
    #[inline(always)]
    fn from(val: TasksClr) -> u8 {
        TasksClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksOut {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksOut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksOut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksOut {
    #[inline(always)]
    fn from(val: u8) -> TasksOut {
        TasksOut::from_bits(val)
    }
}
impl From<TasksOut> for u8 {
    #[inline(always)]
    fn from(val: TasksOut) -> u8 {
        TasksOut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksSet {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksSet {
    #[inline(always)]
    fn from(val: u8) -> TasksSet {
        TasksSet::from_bits(val)
    }
}
impl From<TasksSet> for u8 {
    #[inline(always)]
    fn from(val: TasksSet) -> u8 {
        TasksSet::to_bits(val)
    }
}
