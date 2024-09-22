#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsDenormalinput {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsDenormalinput {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsDenormalinput {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsDenormalinput {
    #[inline(always)]
    fn from(val: u8) -> EventsDenormalinput {
        EventsDenormalinput::from_bits(val)
    }
}
impl From<EventsDenormalinput> for u8 {
    #[inline(always)]
    fn from(val: EventsDenormalinput) -> u8 {
        EventsDenormalinput::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsDividebyzero {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsDividebyzero {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsDividebyzero {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsDividebyzero {
    #[inline(always)]
    fn from(val: u8) -> EventsDividebyzero {
        EventsDividebyzero::from_bits(val)
    }
}
impl From<EventsDividebyzero> for u8 {
    #[inline(always)]
    fn from(val: EventsDividebyzero) -> u8 {
        EventsDividebyzero::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsInexact {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsInexact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsInexact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsInexact {
    #[inline(always)]
    fn from(val: u8) -> EventsInexact {
        EventsInexact::from_bits(val)
    }
}
impl From<EventsInexact> for u8 {
    #[inline(always)]
    fn from(val: EventsInexact) -> u8 {
        EventsInexact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsInvalidoperation {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsInvalidoperation {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsInvalidoperation {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsInvalidoperation {
    #[inline(always)]
    fn from(val: u8) -> EventsInvalidoperation {
        EventsInvalidoperation::from_bits(val)
    }
}
impl From<EventsInvalidoperation> for u8 {
    #[inline(always)]
    fn from(val: EventsInvalidoperation) -> u8 {
        EventsInvalidoperation::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsOverflow {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsOverflow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsOverflow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsOverflow {
    #[inline(always)]
    fn from(val: u8) -> EventsOverflow {
        EventsOverflow::from_bits(val)
    }
}
impl From<EventsOverflow> for u8 {
    #[inline(always)]
    fn from(val: EventsOverflow) -> u8 {
        EventsOverflow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsUnderflow {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsUnderflow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsUnderflow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsUnderflow {
    #[inline(always)]
    fn from(val: u8) -> EventsUnderflow {
        EventsUnderflow::from_bits(val)
    }
}
impl From<EventsUnderflow> for u8 {
    #[inline(always)]
    fn from(val: EventsUnderflow) -> u8 {
        EventsUnderflow::to_bits(val)
    }
}
