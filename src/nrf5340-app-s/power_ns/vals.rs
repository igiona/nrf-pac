#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsPofwarn {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsPofwarn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsPofwarn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsPofwarn {
    #[inline(always)]
    fn from(val: u8) -> EventsPofwarn {
        EventsPofwarn::from_bits(val)
    }
}
impl From<EventsPofwarn> for u8 {
    #[inline(always)]
    fn from(val: EventsPofwarn) -> u8 {
        EventsPofwarn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsSleepenter {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsSleepenter {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsSleepenter {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsSleepenter {
    #[inline(always)]
    fn from(val: u8) -> EventsSleepenter {
        EventsSleepenter::from_bits(val)
    }
}
impl From<EventsSleepenter> for u8 {
    #[inline(always)]
    fn from(val: EventsSleepenter) -> u8 {
        EventsSleepenter::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsSleepexit {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsSleepexit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsSleepexit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsSleepexit {
    #[inline(always)]
    fn from(val: u8) -> EventsSleepexit {
        EventsSleepexit::from_bits(val)
    }
}
impl From<EventsSleepexit> for u8 {
    #[inline(always)]
    fn from(val: EventsSleepexit) -> u8 {
        EventsSleepexit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishPofwarnEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishPofwarnEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishPofwarnEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishPofwarnEn {
    #[inline(always)]
    fn from(val: u8) -> PublishPofwarnEn {
        PublishPofwarnEn::from_bits(val)
    }
}
impl From<PublishPofwarnEn> for u8 {
    #[inline(always)]
    fn from(val: PublishPofwarnEn) -> u8 {
        PublishPofwarnEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishSleepenterEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishSleepenterEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishSleepenterEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishSleepenterEn {
    #[inline(always)]
    fn from(val: u8) -> PublishSleepenterEn {
        PublishSleepenterEn::from_bits(val)
    }
}
impl From<PublishSleepenterEn> for u8 {
    #[inline(always)]
    fn from(val: PublishSleepenterEn) -> u8 {
        PublishSleepenterEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishSleepexitEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishSleepexitEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishSleepexitEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishSleepexitEn {
    #[inline(always)]
    fn from(val: u8) -> PublishSleepexitEn {
        PublishSleepexitEn::from_bits(val)
    }
}
impl From<PublishSleepexitEn> for u8 {
    #[inline(always)]
    fn from(val: PublishSleepexitEn) -> u8 {
        PublishSleepexitEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeConstlatEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeConstlatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeConstlatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeConstlatEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeConstlatEn {
        SubscribeConstlatEn::from_bits(val)
    }
}
impl From<SubscribeConstlatEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeConstlatEn) -> u8 {
        SubscribeConstlatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeLowpwrEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeLowpwrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeLowpwrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeLowpwrEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeLowpwrEn {
        SubscribeLowpwrEn::from_bits(val)
    }
}
impl From<SubscribeLowpwrEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeLowpwrEn) -> u8 {
        SubscribeLowpwrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksConstlat {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksConstlat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksConstlat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksConstlat {
    #[inline(always)]
    fn from(val: u8) -> TasksConstlat {
        TasksConstlat::from_bits(val)
    }
}
impl From<TasksConstlat> for u8 {
    #[inline(always)]
    fn from(val: TasksConstlat) -> u8 {
        TasksConstlat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksLowpwr {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksLowpwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksLowpwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksLowpwr {
    #[inline(always)]
    fn from(val: u8) -> TasksLowpwr {
        TasksLowpwr::from_bits(val)
    }
}
impl From<TasksLowpwr> for u8 {
    #[inline(always)]
    fn from(val: TasksLowpwr) -> u8 {
        TasksLowpwr::to_bits(val)
    }
}
