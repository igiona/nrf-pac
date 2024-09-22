#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEndecb {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEndecb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEndecb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEndecb {
    #[inline(always)]
    fn from(val: u8) -> EventsEndecb {
        EventsEndecb::from_bits(val)
    }
}
impl From<EventsEndecb> for u8 {
    #[inline(always)]
    fn from(val: EventsEndecb) -> u8 {
        EventsEndecb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsErrorecb {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsErrorecb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsErrorecb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsErrorecb {
    #[inline(always)]
    fn from(val: u8) -> EventsErrorecb {
        EventsErrorecb::from_bits(val)
    }
}
impl From<EventsErrorecb> for u8 {
    #[inline(always)]
    fn from(val: EventsErrorecb) -> u8 {
        EventsErrorecb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStartecb {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStartecb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStartecb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStartecb {
    #[inline(always)]
    fn from(val: u8) -> TasksStartecb {
        TasksStartecb::from_bits(val)
    }
}
impl From<TasksStartecb> for u8 {
    #[inline(always)]
    fn from(val: TasksStartecb) -> u8 {
        TasksStartecb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStopecb {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStopecb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStopecb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStopecb {
    #[inline(always)]
    fn from(val: u8) -> TasksStopecb {
        TasksStopecb::from_bits(val)
    }
}
impl From<TasksStopecb> for u8 {
    #[inline(always)]
    fn from(val: TasksStopecb) -> u8 {
        TasksStopecb::to_bits(val)
    }
}
