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
pub enum EventsOvrflw {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsOvrflw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsOvrflw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsOvrflw {
    #[inline(always)]
    fn from(val: u8) -> EventsOvrflw {
        EventsOvrflw::from_bits(val)
    }
}
impl From<EventsOvrflw> for u8 {
    #[inline(always)]
    fn from(val: EventsOvrflw) -> u8 {
        EventsOvrflw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsTick {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsTick {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsTick {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsTick {
    #[inline(always)]
    fn from(val: u8) -> EventsTick {
        EventsTick::from_bits(val)
    }
}
impl From<EventsTick> for u8 {
    #[inline(always)]
    fn from(val: EventsTick) -> u8 {
        EventsTick::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenCompare0 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Disable"]
    ENABLED = 0x01,
}
impl EvtenCompare0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenCompare0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenCompare0 {
    #[inline(always)]
    fn from(val: u8) -> EvtenCompare0 {
        EvtenCompare0::from_bits(val)
    }
}
impl From<EvtenCompare0> for u8 {
    #[inline(always)]
    fn from(val: EvtenCompare0) -> u8 {
        EvtenCompare0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenCompare1 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Disable"]
    ENABLED = 0x01,
}
impl EvtenCompare1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenCompare1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenCompare1 {
    #[inline(always)]
    fn from(val: u8) -> EvtenCompare1 {
        EvtenCompare1::from_bits(val)
    }
}
impl From<EvtenCompare1> for u8 {
    #[inline(always)]
    fn from(val: EvtenCompare1) -> u8 {
        EvtenCompare1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenCompare2 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Disable"]
    ENABLED = 0x01,
}
impl EvtenCompare2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenCompare2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenCompare2 {
    #[inline(always)]
    fn from(val: u8) -> EvtenCompare2 {
        EvtenCompare2::from_bits(val)
    }
}
impl From<EvtenCompare2> for u8 {
    #[inline(always)]
    fn from(val: EvtenCompare2) -> u8 {
        EvtenCompare2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenCompare3 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Disable"]
    ENABLED = 0x01,
}
impl EvtenCompare3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenCompare3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenCompare3 {
    #[inline(always)]
    fn from(val: u8) -> EvtenCompare3 {
        EvtenCompare3::from_bits(val)
    }
}
impl From<EvtenCompare3> for u8 {
    #[inline(always)]
    fn from(val: EvtenCompare3) -> u8 {
        EvtenCompare3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenOvrflw {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Disable"]
    ENABLED = 0x01,
}
impl EvtenOvrflw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenOvrflw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenOvrflw {
    #[inline(always)]
    fn from(val: u8) -> EvtenOvrflw {
        EvtenOvrflw::from_bits(val)
    }
}
impl From<EvtenOvrflw> for u8 {
    #[inline(always)]
    fn from(val: EvtenOvrflw) -> u8 {
        EvtenOvrflw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenTick {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Disable"]
    ENABLED = 0x01,
}
impl EvtenTick {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenTick {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenTick {
    #[inline(always)]
    fn from(val: u8) -> EvtenTick {
        EvtenTick::from_bits(val)
    }
}
impl From<EvtenTick> for u8 {
    #[inline(always)]
    fn from(val: EvtenTick) -> u8 {
        EvtenTick::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenclrCompare0 {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl EvtenclrCompare0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenclrCompare0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenclrCompare0 {
    #[inline(always)]
    fn from(val: u8) -> EvtenclrCompare0 {
        EvtenclrCompare0::from_bits(val)
    }
}
impl From<EvtenclrCompare0> for u8 {
    #[inline(always)]
    fn from(val: EvtenclrCompare0) -> u8 {
        EvtenclrCompare0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenclrCompare1 {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl EvtenclrCompare1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenclrCompare1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenclrCompare1 {
    #[inline(always)]
    fn from(val: u8) -> EvtenclrCompare1 {
        EvtenclrCompare1::from_bits(val)
    }
}
impl From<EvtenclrCompare1> for u8 {
    #[inline(always)]
    fn from(val: EvtenclrCompare1) -> u8 {
        EvtenclrCompare1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenclrCompare2 {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl EvtenclrCompare2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenclrCompare2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenclrCompare2 {
    #[inline(always)]
    fn from(val: u8) -> EvtenclrCompare2 {
        EvtenclrCompare2::from_bits(val)
    }
}
impl From<EvtenclrCompare2> for u8 {
    #[inline(always)]
    fn from(val: EvtenclrCompare2) -> u8 {
        EvtenclrCompare2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenclrCompare3 {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl EvtenclrCompare3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenclrCompare3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenclrCompare3 {
    #[inline(always)]
    fn from(val: u8) -> EvtenclrCompare3 {
        EvtenclrCompare3::from_bits(val)
    }
}
impl From<EvtenclrCompare3> for u8 {
    #[inline(always)]
    fn from(val: EvtenclrCompare3) -> u8 {
        EvtenclrCompare3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenclrOvrflw {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl EvtenclrOvrflw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenclrOvrflw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenclrOvrflw {
    #[inline(always)]
    fn from(val: u8) -> EvtenclrOvrflw {
        EvtenclrOvrflw::from_bits(val)
    }
}
impl From<EvtenclrOvrflw> for u8 {
    #[inline(always)]
    fn from(val: EvtenclrOvrflw) -> u8 {
        EvtenclrOvrflw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtenclrTick {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_CLEAR = 0x01,
}
impl EvtenclrTick {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtenclrTick {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtenclrTick {
    #[inline(always)]
    fn from(val: u8) -> EvtenclrTick {
        EvtenclrTick::from_bits(val)
    }
}
impl From<EvtenclrTick> for u8 {
    #[inline(always)]
    fn from(val: EvtenclrTick) -> u8 {
        EvtenclrTick::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtensetCompare0 {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl EvtensetCompare0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtensetCompare0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtensetCompare0 {
    #[inline(always)]
    fn from(val: u8) -> EvtensetCompare0 {
        EvtensetCompare0::from_bits(val)
    }
}
impl From<EvtensetCompare0> for u8 {
    #[inline(always)]
    fn from(val: EvtensetCompare0) -> u8 {
        EvtensetCompare0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtensetCompare1 {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl EvtensetCompare1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtensetCompare1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtensetCompare1 {
    #[inline(always)]
    fn from(val: u8) -> EvtensetCompare1 {
        EvtensetCompare1::from_bits(val)
    }
}
impl From<EvtensetCompare1> for u8 {
    #[inline(always)]
    fn from(val: EvtensetCompare1) -> u8 {
        EvtensetCompare1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtensetCompare2 {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl EvtensetCompare2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtensetCompare2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtensetCompare2 {
    #[inline(always)]
    fn from(val: u8) -> EvtensetCompare2 {
        EvtensetCompare2::from_bits(val)
    }
}
impl From<EvtensetCompare2> for u8 {
    #[inline(always)]
    fn from(val: EvtensetCompare2) -> u8 {
        EvtensetCompare2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtensetCompare3 {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl EvtensetCompare3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtensetCompare3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtensetCompare3 {
    #[inline(always)]
    fn from(val: u8) -> EvtensetCompare3 {
        EvtensetCompare3::from_bits(val)
    }
}
impl From<EvtensetCompare3> for u8 {
    #[inline(always)]
    fn from(val: EvtensetCompare3) -> u8 {
        EvtensetCompare3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtensetOvrflw {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl EvtensetOvrflw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtensetOvrflw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtensetOvrflw {
    #[inline(always)]
    fn from(val: u8) -> EvtensetOvrflw {
        EvtensetOvrflw::from_bits(val)
    }
}
impl From<EvtensetOvrflw> for u8 {
    #[inline(always)]
    fn from(val: EvtensetOvrflw) -> u8 {
        EvtensetOvrflw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvtensetTick {
    #[doc = "Read: Disabled"]
    DISABLED = 0x0,
    #[doc = "Read: Enabled"]
    R_ENABLED_W_SET = 0x01,
}
impl EvtensetTick {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtensetTick {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtensetTick {
    #[inline(always)]
    fn from(val: u8) -> EvtensetTick {
        EvtensetTick::from_bits(val)
    }
}
impl From<EvtensetTick> for u8 {
    #[inline(always)]
    fn from(val: EvtensetTick) -> u8 {
        EvtensetTick::to_bits(val)
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
pub enum TasksTrigovrflw {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksTrigovrflw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksTrigovrflw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksTrigovrflw {
    #[inline(always)]
    fn from(val: u8) -> TasksTrigovrflw {
        TasksTrigovrflw::from_bits(val)
    }
}
impl From<TasksTrigovrflw> for u8 {
    #[inline(always)]
    fn from(val: TasksTrigovrflw) -> u8 {
        TasksTrigovrflw::to_bits(val)
    }
}
