#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dercen {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Dercen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dercen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dercen {
    #[inline(always)]
    fn from(val: u8) -> Dercen {
        Dercen::from_bits(val)
    }
}
impl From<Dercen> for u8 {
    #[inline(always)]
    fn from(val: Dercen) -> u8 {
        Dercen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsValrdy {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsValrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsValrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsValrdy {
    #[inline(always)]
    fn from(val: u8) -> EventsValrdy {
        EventsValrdy::from_bits(val)
    }
}
impl From<EventsValrdy> for u8 {
    #[inline(always)]
    fn from(val: EventsValrdy) -> u8 {
        EventsValrdy::to_bits(val)
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
