#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsTimeout {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsTimeout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsTimeout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsTimeout {
    #[inline(always)]
    fn from(val: u8) -> EventsTimeout {
        EventsTimeout::from_bits(val)
    }
}
impl From<EventsTimeout> for u8 {
    #[inline(always)]
    fn from(val: EventsTimeout) -> u8 {
        EventsTimeout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Halt {
    #[doc = "Pause watchdog while the CPU is halted by the debugger"]
    PAUSE = 0x0,
    #[doc = "Keep the watchdog running while the CPU is halted by the debugger"]
    RUN = 0x01,
}
impl Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Halt {
    #[inline(always)]
    fn from(val: u8) -> Halt {
        Halt::from_bits(val)
    }
}
impl From<Halt> for u8 {
    #[inline(always)]
    fn from(val: Halt) -> u8 {
        Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReqstatusRr0 {
    #[doc = "RR\\[0\\] register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0x0,
    #[doc = "RR\\[0\\] register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 0x01,
}
impl ReqstatusRr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReqstatusRr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReqstatusRr0 {
    #[inline(always)]
    fn from(val: u8) -> ReqstatusRr0 {
        ReqstatusRr0::from_bits(val)
    }
}
impl From<ReqstatusRr0> for u8 {
    #[inline(always)]
    fn from(val: ReqstatusRr0) -> u8 {
        ReqstatusRr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReqstatusRr1 {
    #[doc = "RR\\[1\\] register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0x0,
    #[doc = "RR\\[1\\] register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 0x01,
}
impl ReqstatusRr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReqstatusRr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReqstatusRr1 {
    #[inline(always)]
    fn from(val: u8) -> ReqstatusRr1 {
        ReqstatusRr1::from_bits(val)
    }
}
impl From<ReqstatusRr1> for u8 {
    #[inline(always)]
    fn from(val: ReqstatusRr1) -> u8 {
        ReqstatusRr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReqstatusRr2 {
    #[doc = "RR\\[2\\] register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0x0,
    #[doc = "RR\\[2\\] register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 0x01,
}
impl ReqstatusRr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReqstatusRr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReqstatusRr2 {
    #[inline(always)]
    fn from(val: u8) -> ReqstatusRr2 {
        ReqstatusRr2::from_bits(val)
    }
}
impl From<ReqstatusRr2> for u8 {
    #[inline(always)]
    fn from(val: ReqstatusRr2) -> u8 {
        ReqstatusRr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReqstatusRr3 {
    #[doc = "RR\\[3\\] register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0x0,
    #[doc = "RR\\[3\\] register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 0x01,
}
impl ReqstatusRr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReqstatusRr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReqstatusRr3 {
    #[inline(always)]
    fn from(val: u8) -> ReqstatusRr3 {
        ReqstatusRr3::from_bits(val)
    }
}
impl From<ReqstatusRr3> for u8 {
    #[inline(always)]
    fn from(val: ReqstatusRr3) -> u8 {
        ReqstatusRr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReqstatusRr4 {
    #[doc = "RR\\[4\\] register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0x0,
    #[doc = "RR\\[4\\] register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 0x01,
}
impl ReqstatusRr4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReqstatusRr4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReqstatusRr4 {
    #[inline(always)]
    fn from(val: u8) -> ReqstatusRr4 {
        ReqstatusRr4::from_bits(val)
    }
}
impl From<ReqstatusRr4> for u8 {
    #[inline(always)]
    fn from(val: ReqstatusRr4) -> u8 {
        ReqstatusRr4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReqstatusRr5 {
    #[doc = "RR\\[5\\] register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0x0,
    #[doc = "RR\\[5\\] register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 0x01,
}
impl ReqstatusRr5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReqstatusRr5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReqstatusRr5 {
    #[inline(always)]
    fn from(val: u8) -> ReqstatusRr5 {
        ReqstatusRr5::from_bits(val)
    }
}
impl From<ReqstatusRr5> for u8 {
    #[inline(always)]
    fn from(val: ReqstatusRr5) -> u8 {
        ReqstatusRr5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReqstatusRr6 {
    #[doc = "RR\\[6\\] register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0x0,
    #[doc = "RR\\[6\\] register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 0x01,
}
impl ReqstatusRr6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReqstatusRr6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReqstatusRr6 {
    #[inline(always)]
    fn from(val: u8) -> ReqstatusRr6 {
        ReqstatusRr6::from_bits(val)
    }
}
impl From<ReqstatusRr6> for u8 {
    #[inline(always)]
    fn from(val: ReqstatusRr6) -> u8 {
        ReqstatusRr6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReqstatusRr7 {
    #[doc = "RR\\[7\\] register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0x0,
    #[doc = "RR\\[7\\] register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 0x01,
}
impl ReqstatusRr7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReqstatusRr7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReqstatusRr7 {
    #[inline(always)]
    fn from(val: u8) -> ReqstatusRr7 {
        ReqstatusRr7::from_bits(val)
    }
}
impl From<ReqstatusRr7> for u8 {
    #[inline(always)]
    fn from(val: ReqstatusRr7) -> u8 {
        ReqstatusRr7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rr(pub u32);
impl Rr {
    #[doc = "Value to request a reload of the watchdog timer"]
    pub const RELOAD: Self = Self(0x6e52_4635);
}
impl Rr {
    pub const fn from_bits(val: u32) -> Rr {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Rr {
    #[inline(always)]
    fn from(val: u32) -> Rr {
        Rr::from_bits(val)
    }
}
impl From<Rr> for u32 {
    #[inline(always)]
    fn from(val: Rr) -> u32 {
        Rr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RrenRr0 {
    #[doc = "Disable RR\\[0\\] register"]
    DISABLED = 0x0,
    #[doc = "Enable RR\\[0\\] register"]
    ENABLED = 0x01,
}
impl RrenRr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrenRr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrenRr0 {
    #[inline(always)]
    fn from(val: u8) -> RrenRr0 {
        RrenRr0::from_bits(val)
    }
}
impl From<RrenRr0> for u8 {
    #[inline(always)]
    fn from(val: RrenRr0) -> u8 {
        RrenRr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RrenRr1 {
    #[doc = "Disable RR\\[1\\] register"]
    DISABLED = 0x0,
    #[doc = "Enable RR\\[1\\] register"]
    ENABLED = 0x01,
}
impl RrenRr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrenRr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrenRr1 {
    #[inline(always)]
    fn from(val: u8) -> RrenRr1 {
        RrenRr1::from_bits(val)
    }
}
impl From<RrenRr1> for u8 {
    #[inline(always)]
    fn from(val: RrenRr1) -> u8 {
        RrenRr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RrenRr2 {
    #[doc = "Disable RR\\[2\\] register"]
    DISABLED = 0x0,
    #[doc = "Enable RR\\[2\\] register"]
    ENABLED = 0x01,
}
impl RrenRr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrenRr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrenRr2 {
    #[inline(always)]
    fn from(val: u8) -> RrenRr2 {
        RrenRr2::from_bits(val)
    }
}
impl From<RrenRr2> for u8 {
    #[inline(always)]
    fn from(val: RrenRr2) -> u8 {
        RrenRr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RrenRr3 {
    #[doc = "Disable RR\\[3\\] register"]
    DISABLED = 0x0,
    #[doc = "Enable RR\\[3\\] register"]
    ENABLED = 0x01,
}
impl RrenRr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrenRr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrenRr3 {
    #[inline(always)]
    fn from(val: u8) -> RrenRr3 {
        RrenRr3::from_bits(val)
    }
}
impl From<RrenRr3> for u8 {
    #[inline(always)]
    fn from(val: RrenRr3) -> u8 {
        RrenRr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RrenRr4 {
    #[doc = "Disable RR\\[4\\] register"]
    DISABLED = 0x0,
    #[doc = "Enable RR\\[4\\] register"]
    ENABLED = 0x01,
}
impl RrenRr4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrenRr4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrenRr4 {
    #[inline(always)]
    fn from(val: u8) -> RrenRr4 {
        RrenRr4::from_bits(val)
    }
}
impl From<RrenRr4> for u8 {
    #[inline(always)]
    fn from(val: RrenRr4) -> u8 {
        RrenRr4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RrenRr5 {
    #[doc = "Disable RR\\[5\\] register"]
    DISABLED = 0x0,
    #[doc = "Enable RR\\[5\\] register"]
    ENABLED = 0x01,
}
impl RrenRr5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrenRr5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrenRr5 {
    #[inline(always)]
    fn from(val: u8) -> RrenRr5 {
        RrenRr5::from_bits(val)
    }
}
impl From<RrenRr5> for u8 {
    #[inline(always)]
    fn from(val: RrenRr5) -> u8 {
        RrenRr5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RrenRr6 {
    #[doc = "Disable RR\\[6\\] register"]
    DISABLED = 0x0,
    #[doc = "Enable RR\\[6\\] register"]
    ENABLED = 0x01,
}
impl RrenRr6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrenRr6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrenRr6 {
    #[inline(always)]
    fn from(val: u8) -> RrenRr6 {
        RrenRr6::from_bits(val)
    }
}
impl From<RrenRr6> for u8 {
    #[inline(always)]
    fn from(val: RrenRr6) -> u8 {
        RrenRr6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RrenRr7 {
    #[doc = "Disable RR\\[7\\] register"]
    DISABLED = 0x0,
    #[doc = "Enable RR\\[7\\] register"]
    ENABLED = 0x01,
}
impl RrenRr7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrenRr7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrenRr7 {
    #[inline(always)]
    fn from(val: u8) -> RrenRr7 {
        RrenRr7::from_bits(val)
    }
}
impl From<RrenRr7> for u8 {
    #[inline(always)]
    fn from(val: RrenRr7) -> u8 {
        RrenRr7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Runstatus {
    #[doc = "Watchdog not running"]
    NOTRUNNING = 0x0,
    #[doc = "Watchdog is running"]
    RUNNING = 0x01,
}
impl Runstatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Runstatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Runstatus {
    #[inline(always)]
    fn from(val: u8) -> Runstatus {
        Runstatus::from_bits(val)
    }
}
impl From<Runstatus> for u8 {
    #[inline(always)]
    fn from(val: Runstatus) -> u8 {
        Runstatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sleep {
    #[doc = "Pause watchdog while the CPU is sleeping"]
    PAUSE = 0x0,
    #[doc = "Keep the watchdog running while the CPU is sleeping"]
    RUN = 0x01,
}
impl Sleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleep {
    #[inline(always)]
    fn from(val: u8) -> Sleep {
        Sleep::from_bits(val)
    }
}
impl From<Sleep> for u8 {
    #[inline(always)]
    fn from(val: Sleep) -> u8 {
        Sleep::to_bits(val)
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
