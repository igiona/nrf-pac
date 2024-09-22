#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Blocked {
    #[doc = "No access violation detected"]
    DISABLED = 0x0,
    #[doc = "Access violation detected and blocked"]
    ENABLED = 0x01,
}
impl Blocked {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Blocked {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Blocked {
    #[inline(always)]
    fn from(val: u8) -> Blocked {
        Blocked::from_bits(val)
    }
}
impl From<Blocked> for u8 {
    #[inline(always)]
    fn from(val: Blocked) -> u8 {
        Blocked::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsKeyslotError {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsKeyslotError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsKeyslotError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsKeyslotError {
    #[inline(always)]
    fn from(val: u8) -> EventsKeyslotError {
        EventsKeyslotError::from_bits(val)
    }
}
impl From<EventsKeyslotError> for u8 {
    #[inline(always)]
    fn from(val: EventsKeyslotError) -> u8 {
        EventsKeyslotError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsKeyslotPushed {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsKeyslotPushed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsKeyslotPushed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsKeyslotPushed {
    #[inline(always)]
    fn from(val: u8) -> EventsKeyslotPushed {
        EventsKeyslotPushed::from_bits(val)
    }
}
impl From<EventsKeyslotPushed> for u8 {
    #[inline(always)]
    fn from(val: EventsKeyslotPushed) -> u8 {
        EventsKeyslotPushed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsKeyslotRevoked {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsKeyslotRevoked {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsKeyslotRevoked {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsKeyslotRevoked {
    #[inline(always)]
    fn from(val: u8) -> EventsKeyslotRevoked {
        EventsKeyslotRevoked::from_bits(val)
    }
}
impl From<EventsKeyslotRevoked> for u8 {
    #[inline(always)]
    fn from(val: EventsKeyslotRevoked) -> u8 {
        EventsKeyslotRevoked::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendKeyslotError {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendKeyslotError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendKeyslotError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendKeyslotError {
    #[inline(always)]
    fn from(val: u8) -> IntpendKeyslotError {
        IntpendKeyslotError::from_bits(val)
    }
}
impl From<IntpendKeyslotError> for u8 {
    #[inline(always)]
    fn from(val: IntpendKeyslotError) -> u8 {
        IntpendKeyslotError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendKeyslotPushed {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendKeyslotPushed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendKeyslotPushed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendKeyslotPushed {
    #[inline(always)]
    fn from(val: u8) -> IntpendKeyslotPushed {
        IntpendKeyslotPushed::from_bits(val)
    }
}
impl From<IntpendKeyslotPushed> for u8 {
    #[inline(always)]
    fn from(val: IntpendKeyslotPushed) -> u8 {
        IntpendKeyslotPushed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendKeyslotRevoked {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendKeyslotRevoked {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendKeyslotRevoked {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendKeyslotRevoked {
    #[inline(always)]
    fn from(val: u8) -> IntpendKeyslotRevoked {
        IntpendKeyslotRevoked::from_bits(val)
    }
}
impl From<IntpendKeyslotRevoked> for u8 {
    #[inline(always)]
    fn from(val: IntpendKeyslotRevoked) -> u8 {
        IntpendKeyslotRevoked::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Selected {
    #[doc = "No key slot ID selected by KMU"]
    DISABLED = 0x0,
    #[doc = "Key slot ID successfully selected by KMU"]
    ENABLED = 0x01,
}
impl Selected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Selected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Selected {
    #[inline(always)]
    fn from(val: u8) -> Selected {
        Selected::from_bits(val)
    }
}
impl From<Selected> for u8 {
    #[inline(always)]
    fn from(val: Selected) -> u8 {
        Selected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksPushKeyslot {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksPushKeyslot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksPushKeyslot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksPushKeyslot {
    #[inline(always)]
    fn from(val: u8) -> TasksPushKeyslot {
        TasksPushKeyslot::from_bits(val)
    }
}
impl From<TasksPushKeyslot> for u8 {
    #[inline(always)]
    fn from(val: TasksPushKeyslot) -> u8 {
        TasksPushKeyslot::to_bits(val)
    }
}
