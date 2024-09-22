#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dcdcen0dcdcen {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Dcdcen0dcdcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcdcen0dcdcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcdcen0dcdcen {
    #[inline(always)]
    fn from(val: u8) -> Dcdcen0dcdcen {
        Dcdcen0dcdcen::from_bits(val)
    }
}
impl From<Dcdcen0dcdcen> for u8 {
    #[inline(always)]
    fn from(val: Dcdcen0dcdcen) -> u8 {
        Dcdcen0dcdcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DcdcenDcdcen {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl DcdcenDcdcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcenDcdcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcenDcdcen {
    #[inline(always)]
    fn from(val: u8) -> DcdcenDcdcen {
        DcdcenDcdcen::from_bits(val)
    }
}
impl From<DcdcenDcdcen> for u8 {
    #[inline(always)]
    fn from(val: DcdcenDcdcen) -> u8 {
        DcdcenDcdcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dif {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Dif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dif {
    #[inline(always)]
    fn from(val: u8) -> Dif {
        Dif::from_bits(val)
    }
}
impl From<Dif> for u8 {
    #[inline(always)]
    fn from(val: Dif) -> u8 {
        Dif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dog {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Dog {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dog {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dog {
    #[inline(always)]
    fn from(val: u8) -> Dog {
        Dog::from_bits(val)
    }
}
impl From<Dog> for u8 {
    #[inline(always)]
    fn from(val: Dog) -> u8 {
        Dog::to_bits(val)
    }
}
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
pub enum EventsUsbdetected {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsUsbdetected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsUsbdetected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsUsbdetected {
    #[inline(always)]
    fn from(val: u8) -> EventsUsbdetected {
        EventsUsbdetected::from_bits(val)
    }
}
impl From<EventsUsbdetected> for u8 {
    #[inline(always)]
    fn from(val: EventsUsbdetected) -> u8 {
        EventsUsbdetected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsUsbpwrrdy {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsUsbpwrrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsUsbpwrrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsUsbpwrrdy {
    #[inline(always)]
    fn from(val: u8) -> EventsUsbpwrrdy {
        EventsUsbpwrrdy::from_bits(val)
    }
}
impl From<EventsUsbpwrrdy> for u8 {
    #[inline(always)]
    fn from(val: EventsUsbpwrrdy) -> u8 {
        EventsUsbpwrrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsUsbremoved {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsUsbremoved {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsUsbremoved {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsUsbremoved {
    #[inline(always)]
    fn from(val: u8) -> EventsUsbremoved {
        EventsUsbremoved::from_bits(val)
    }
}
impl From<EventsUsbremoved> for u8 {
    #[inline(always)]
    fn from(val: EventsUsbremoved) -> u8 {
        EventsUsbremoved::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lockup {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lockup {
    #[inline(always)]
    fn from(val: u8) -> Lockup {
        Lockup::from_bits(val)
    }
}
impl From<Lockup> for u8 {
    #[inline(always)]
    fn from(val: Lockup) -> u8 {
        Lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpcomp {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Lpcomp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpcomp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpcomp {
    #[inline(always)]
    fn from(val: u8) -> Lpcomp {
        Lpcomp::from_bits(val)
    }
}
impl From<Lpcomp> for u8 {
    #[inline(always)]
    fn from(val: Lpcomp) -> u8 {
        Lpcomp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mainregstatus {
    #[doc = "Normal voltage mode. Voltage supplied on VDD."]
    NORMAL = 0x0,
    #[doc = "High voltage mode. Voltage supplied on VDDH."]
    HIGH = 0x01,
}
impl Mainregstatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mainregstatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mainregstatus {
    #[inline(always)]
    fn from(val: u8) -> Mainregstatus {
        Mainregstatus::from_bits(val)
    }
}
impl From<Mainregstatus> for u8 {
    #[inline(always)]
    fn from(val: Mainregstatus) -> u8 {
        Mainregstatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Nfc {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Nfc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfc {
    #[inline(always)]
    fn from(val: u8) -> Nfc {
        Nfc::from_bits(val)
    }
}
impl From<Nfc> for u8 {
    #[inline(always)]
    fn from(val: Nfc) -> u8 {
        Nfc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Off {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Off {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Off {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Off {
    #[inline(always)]
    fn from(val: u8) -> Off {
        Off::from_bits(val)
    }
}
impl From<Off> for u8 {
    #[inline(always)]
    fn from(val: Off) -> u8 {
        Off::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Outputrdy {
    #[doc = "USBREG output settling time not elapsed"]
    NOTREADY = 0x0,
    #[doc = "USBREG output settling time elapsed (same information as USBPWRRDY event)"]
    READY = 0x01,
}
impl Outputrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outputrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outputrdy {
    #[inline(always)]
    fn from(val: u8) -> Outputrdy {
        Outputrdy::from_bits(val)
    }
}
impl From<Outputrdy> for u8 {
    #[inline(always)]
    fn from(val: Outputrdy) -> u8 {
        Outputrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pof {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Pof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pof {
    #[inline(always)]
    fn from(val: u8) -> Pof {
        Pof::from_bits(val)
    }
}
impl From<Pof> for u8 {
    #[inline(always)]
    fn from(val: Pof) -> u8 {
        Pof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resetpin {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Resetpin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resetpin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resetpin {
    #[inline(always)]
    fn from(val: u8) -> Resetpin {
        Resetpin::from_bits(val)
    }
}
impl From<Resetpin> for u8 {
    #[inline(always)]
    fn from(val: Resetpin) -> u8 {
        Resetpin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sreq {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Sreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sreq {
    #[inline(always)]
    fn from(val: u8) -> Sreq {
        Sreq::from_bits(val)
    }
}
impl From<Sreq> for u8 {
    #[inline(always)]
    fn from(val: Sreq) -> u8 {
        Sreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Systemoff {
    _RESERVED_0 = 0x0,
    #[doc = "Enable System OFF mode"]
    ENTER = 0x01,
}
impl Systemoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systemoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systemoff {
    #[inline(always)]
    fn from(val: u8) -> Systemoff {
        Systemoff::from_bits(val)
    }
}
impl From<Systemoff> for u8 {
    #[inline(always)]
    fn from(val: Systemoff) -> u8 {
        Systemoff::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Threshold {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Set threshold to 1.7 V"]
    V17 = 0x04,
    #[doc = "Set threshold to 1.8 V"]
    V18 = 0x05,
    #[doc = "Set threshold to 1.9 V"]
    V19 = 0x06,
    #[doc = "Set threshold to 2.0 V"]
    V20 = 0x07,
    #[doc = "Set threshold to 2.1 V"]
    V21 = 0x08,
    #[doc = "Set threshold to 2.2 V"]
    V22 = 0x09,
    #[doc = "Set threshold to 2.3 V"]
    V23 = 0x0a,
    #[doc = "Set threshold to 2.4 V"]
    V24 = 0x0b,
    #[doc = "Set threshold to 2.5 V"]
    V25 = 0x0c,
    #[doc = "Set threshold to 2.6 V"]
    V26 = 0x0d,
    #[doc = "Set threshold to 2.7 V"]
    V27 = 0x0e,
    #[doc = "Set threshold to 2.8 V"]
    V28 = 0x0f,
}
impl Threshold {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Threshold {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Threshold {
    #[inline(always)]
    fn from(val: u8) -> Threshold {
        Threshold::from_bits(val)
    }
}
impl From<Threshold> for u8 {
    #[inline(always)]
    fn from(val: Threshold) -> u8 {
        Threshold::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Thresholdvddh {
    #[doc = "Set threshold to 2.7 V"]
    V27 = 0x0,
    #[doc = "Set threshold to 2.8 V"]
    V28 = 0x01,
    #[doc = "Set threshold to 2.9 V"]
    V29 = 0x02,
    #[doc = "Set threshold to 3.0 V"]
    V30 = 0x03,
    #[doc = "Set threshold to 3.1 V"]
    V31 = 0x04,
    #[doc = "Set threshold to 3.2 V"]
    V32 = 0x05,
    #[doc = "Set threshold to 3.3 V"]
    V33 = 0x06,
    #[doc = "Set threshold to 3.4 V"]
    V34 = 0x07,
    #[doc = "Set threshold to 3.5 V"]
    V35 = 0x08,
    #[doc = "Set threshold to 3.6 V"]
    V36 = 0x09,
    #[doc = "Set threshold to 3.7 V"]
    V37 = 0x0a,
    #[doc = "Set threshold to 3.8 V"]
    V38 = 0x0b,
    #[doc = "Set threshold to 3.9 V"]
    V39 = 0x0c,
    #[doc = "Set threshold to 4.0 V"]
    V40 = 0x0d,
    #[doc = "Set threshold to 4.1 V"]
    V41 = 0x0e,
    #[doc = "Set threshold to 4.2 V"]
    V42 = 0x0f,
}
impl Thresholdvddh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Thresholdvddh {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Thresholdvddh {
    #[inline(always)]
    fn from(val: u8) -> Thresholdvddh {
        Thresholdvddh::from_bits(val)
    }
}
impl From<Thresholdvddh> for u8 {
    #[inline(always)]
    fn from(val: Thresholdvddh) -> u8 {
        Thresholdvddh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vbus {
    #[doc = "Not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Vbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbus {
    #[inline(always)]
    fn from(val: u8) -> Vbus {
        Vbus::from_bits(val)
    }
}
impl From<Vbus> for u8 {
    #[inline(always)]
    fn from(val: Vbus) -> u8 {
        Vbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vbusdetect {
    #[doc = "VBUS voltage below valid threshold"]
    NOVBUS = 0x0,
    #[doc = "VBUS voltage above valid threshold"]
    VBUSPRESENT = 0x01,
}
impl Vbusdetect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbusdetect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbusdetect {
    #[inline(always)]
    fn from(val: u8) -> Vbusdetect {
        Vbusdetect::from_bits(val)
    }
}
impl From<Vbusdetect> for u8 {
    #[inline(always)]
    fn from(val: Vbusdetect) -> u8 {
        Vbusdetect::to_bits(val)
    }
}
