#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsTriggered {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsTriggered {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsTriggered {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsTriggered {
    #[inline(always)]
    fn from(val: u8) -> EventsTriggered {
        EventsTriggered::from_bits(val)
    }
}
impl From<EventsTriggered> for u8 {
    #[inline(always)]
    fn from(val: EventsTriggered) -> u8 {
        EventsTriggered::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishTriggeredEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishTriggeredEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishTriggeredEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishTriggeredEn {
    #[inline(always)]
    fn from(val: u8) -> PublishTriggeredEn {
        PublishTriggeredEn::from_bits(val)
    }
}
impl From<PublishTriggeredEn> for u8 {
    #[inline(always)]
    fn from(val: PublishTriggeredEn) -> u8 {
        PublishTriggeredEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeTriggerEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeTriggerEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeTriggerEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeTriggerEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeTriggerEn {
        SubscribeTriggerEn::from_bits(val)
    }
}
impl From<SubscribeTriggerEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeTriggerEn) -> u8 {
        SubscribeTriggerEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksTrigger {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksTrigger {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksTrigger {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksTrigger {
    #[inline(always)]
    fn from(val: u8) -> TasksTrigger {
        TasksTrigger::from_bits(val)
    }
}
impl From<TasksTrigger> for u8 {
    #[inline(always)]
    fn from(val: TasksTrigger) -> u8 {
        TasksTrigger::to_bits(val)
    }
}
