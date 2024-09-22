#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Baudrate(pub u32);
impl Baudrate {
    #[doc = "1200 baud (actual rate: 1205)"]
    pub const BAUD1200: Self = Self(0x0004_f000);
    #[doc = "2400 baud (actual rate: 2396)"]
    pub const BAUD2400: Self = Self(0x0009_d000);
    #[doc = "4800 baud (actual rate: 4808)"]
    pub const BAUD4800: Self = Self(0x0013_b000);
    #[doc = "9600 baud (actual rate: 9598)"]
    pub const BAUD9600: Self = Self(0x0027_5000);
    #[doc = "14400 baud (actual rate: 14414)"]
    pub const BAUD14400: Self = Self(0x003b_0000);
    #[doc = "19200 baud (actual rate: 19208)"]
    pub const BAUD19200: Self = Self(0x004e_a000);
    #[doc = "28800 baud (actual rate: 28829)"]
    pub const BAUD28800: Self = Self(0x0075_f000);
    #[doc = "31250 baud"]
    pub const BAUD31250: Self = Self(0x0080_0000);
    #[doc = "38400 baud (actual rate: 38462)"]
    pub const BAUD38400: Self = Self(0x009d_5000);
    #[doc = "56000 baud (actual rate: 55944)"]
    pub const BAUD56000: Self = Self(0x00e5_0000);
    #[doc = "57600 baud (actual rate: 57762)"]
    pub const BAUD57600: Self = Self(0x00eb_f000);
    #[doc = "76800 baud (actual rate: 76923)"]
    pub const BAUD76800: Self = Self(0x013a_9000);
    #[doc = "115200 baud (actual rate: 115942)"]
    pub const BAUD115200: Self = Self(0x01d7_e000);
    #[doc = "230400 baud (actual rate: 231884)"]
    pub const BAUD230400: Self = Self(0x03af_b000);
    #[doc = "250000 baud"]
    pub const BAUD250000: Self = Self(0x0400_0000);
    #[doc = "460800 baud (actual rate: 470588)"]
    pub const BAUD460800: Self = Self(0x075f_7000);
    #[doc = "921600 baud (actual rate: 941176)"]
    pub const BAUD921600: Self = Self(0x0ebe_d000);
    #[doc = "1Mega baud"]
    pub const BAUD1M: Self = Self(0x1000_0000);
}
impl Baudrate {
    pub const fn from_bits(val: u32) -> Baudrate {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Baudrate {
    #[inline(always)]
    fn from(val: u32) -> Baudrate {
        Baudrate::from_bits(val)
    }
}
impl From<Baudrate> for u32 {
    #[inline(always)]
    fn from(val: Baudrate) -> u32 {
        Baudrate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Break {
    #[doc = "Read: error not present"]
    NOTPRESENT = 0x0,
    #[doc = "Read: error present"]
    PRESENT = 0x01,
}
impl Break {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Break {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Break {
    #[inline(always)]
    fn from(val: u8) -> Break {
        Break::from_bits(val)
    }
}
impl From<Break> for u8 {
    #[inline(always)]
    fn from(val: Break) -> u8 {
        Break::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConfigParity {
    #[doc = "Exclude parity bit"]
    EXCLUDED = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Include parity bit"]
    INCLUDED = 0x07,
}
impl ConfigParity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfigParity {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfigParity {
    #[inline(always)]
    fn from(val: u8) -> ConfigParity {
        ConfigParity::from_bits(val)
    }
}
impl From<ConfigParity> for u8 {
    #[inline(always)]
    fn from(val: ConfigParity) -> u8 {
        ConfigParity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtsConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl CtsConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtsConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtsConnect {
    #[inline(always)]
    fn from(val: u8) -> CtsConnect {
        CtsConnect::from_bits(val)
    }
}
impl From<CtsConnect> for u8 {
    #[inline(always)]
    fn from(val: CtsConnect) -> u8 {
        CtsConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disable UART"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Enable UART"]
    ENABLED = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ErrorsrcParity {
    #[doc = "Read: error not present"]
    NOTPRESENT = 0x0,
    #[doc = "Read: error present"]
    PRESENT = 0x01,
}
impl ErrorsrcParity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ErrorsrcParity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ErrorsrcParity {
    #[inline(always)]
    fn from(val: u8) -> ErrorsrcParity {
        ErrorsrcParity::from_bits(val)
    }
}
impl From<ErrorsrcParity> for u8 {
    #[inline(always)]
    fn from(val: ErrorsrcParity) -> u8 {
        ErrorsrcParity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCts {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCts {
    #[inline(always)]
    fn from(val: u8) -> EventsCts {
        EventsCts::from_bits(val)
    }
}
impl From<EventsCts> for u8 {
    #[inline(always)]
    fn from(val: EventsCts) -> u8 {
        EventsCts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsError {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsError {
    #[inline(always)]
    fn from(val: u8) -> EventsError {
        EventsError::from_bits(val)
    }
}
impl From<EventsError> for u8 {
    #[inline(always)]
    fn from(val: EventsError) -> u8 {
        EventsError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsNcts {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsNcts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsNcts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsNcts {
    #[inline(always)]
    fn from(val: u8) -> EventsNcts {
        EventsNcts::from_bits(val)
    }
}
impl From<EventsNcts> for u8 {
    #[inline(always)]
    fn from(val: EventsNcts) -> u8 {
        EventsNcts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsRxdrdy {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRxdrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRxdrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRxdrdy {
    #[inline(always)]
    fn from(val: u8) -> EventsRxdrdy {
        EventsRxdrdy::from_bits(val)
    }
}
impl From<EventsRxdrdy> for u8 {
    #[inline(always)]
    fn from(val: EventsRxdrdy) -> u8 {
        EventsRxdrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsRxto {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRxto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRxto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRxto {
    #[inline(always)]
    fn from(val: u8) -> EventsRxto {
        EventsRxto::from_bits(val)
    }
}
impl From<EventsRxto> for u8 {
    #[inline(always)]
    fn from(val: EventsRxto) -> u8 {
        EventsRxto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsTxdrdy {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsTxdrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsTxdrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsTxdrdy {
    #[inline(always)]
    fn from(val: u8) -> EventsTxdrdy {
        EventsTxdrdy::from_bits(val)
    }
}
impl From<EventsTxdrdy> for u8 {
    #[inline(always)]
    fn from(val: EventsTxdrdy) -> u8 {
        EventsTxdrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Framing {
    #[doc = "Read: error not present"]
    NOTPRESENT = 0x0,
    #[doc = "Read: error present"]
    PRESENT = 0x01,
}
impl Framing {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Framing {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Framing {
    #[inline(always)]
    fn from(val: u8) -> Framing {
        Framing::from_bits(val)
    }
}
impl From<Framing> for u8 {
    #[inline(always)]
    fn from(val: Framing) -> u8 {
        Framing::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hwfc {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Hwfc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hwfc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hwfc {
    #[inline(always)]
    fn from(val: u8) -> Hwfc {
        Hwfc::from_bits(val)
    }
}
impl From<Hwfc> for u8 {
    #[inline(always)]
    fn from(val: Hwfc) -> u8 {
        Hwfc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Overrun {
    #[doc = "Read: error not present"]
    NOTPRESENT = 0x0,
    #[doc = "Read: error present"]
    PRESENT = 0x01,
}
impl Overrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Overrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Overrun {
    #[inline(always)]
    fn from(val: u8) -> Overrun {
        Overrun::from_bits(val)
    }
}
impl From<Overrun> for u8 {
    #[inline(always)]
    fn from(val: Overrun) -> u8 {
        Overrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Paritytype {
    #[doc = "Even parity"]
    EVEN = 0x0,
    #[doc = "Odd parity"]
    ODD = 0x01,
}
impl Paritytype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Paritytype {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Paritytype {
    #[inline(always)]
    fn from(val: u8) -> Paritytype {
        Paritytype::from_bits(val)
    }
}
impl From<Paritytype> for u8 {
    #[inline(always)]
    fn from(val: Paritytype) -> u8 {
        Paritytype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RtsConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl RtsConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtsConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtsConnect {
    #[inline(always)]
    fn from(val: u8) -> RtsConnect {
        RtsConnect::from_bits(val)
    }
}
impl From<RtsConnect> for u8 {
    #[inline(always)]
    fn from(val: RtsConnect) -> u8 {
        RtsConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RxdConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl RxdConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxdConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxdConnect {
    #[inline(always)]
    fn from(val: u8) -> RxdConnect {
        RxdConnect::from_bits(val)
    }
}
impl From<RxdConnect> for u8 {
    #[inline(always)]
    fn from(val: RxdConnect) -> u8 {
        RxdConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Stop {
    #[doc = "One stop bit"]
    ONE = 0x0,
    #[doc = "Two stop bits"]
    TWO = 0x01,
}
impl Stop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stop {
    #[inline(always)]
    fn from(val: u8) -> Stop {
        Stop::from_bits(val)
    }
}
impl From<Stop> for u8 {
    #[inline(always)]
    fn from(val: Stop) -> u8 {
        Stop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStartrx {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStartrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStartrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStartrx {
    #[inline(always)]
    fn from(val: u8) -> TasksStartrx {
        TasksStartrx::from_bits(val)
    }
}
impl From<TasksStartrx> for u8 {
    #[inline(always)]
    fn from(val: TasksStartrx) -> u8 {
        TasksStartrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStarttx {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStarttx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStarttx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStarttx {
    #[inline(always)]
    fn from(val: u8) -> TasksStarttx {
        TasksStarttx::from_bits(val)
    }
}
impl From<TasksStarttx> for u8 {
    #[inline(always)]
    fn from(val: TasksStarttx) -> u8 {
        TasksStarttx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStoprx {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStoprx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStoprx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStoprx {
    #[inline(always)]
    fn from(val: u8) -> TasksStoprx {
        TasksStoprx::from_bits(val)
    }
}
impl From<TasksStoprx> for u8 {
    #[inline(always)]
    fn from(val: TasksStoprx) -> u8 {
        TasksStoprx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStoptx {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStoptx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStoptx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStoptx {
    #[inline(always)]
    fn from(val: u8) -> TasksStoptx {
        TasksStoptx::from_bits(val)
    }
}
impl From<TasksStoptx> for u8 {
    #[inline(always)]
    fn from(val: TasksStoptx) -> u8 {
        TasksStoptx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksSuspend {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksSuspend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksSuspend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksSuspend {
    #[inline(always)]
    fn from(val: u8) -> TasksSuspend {
        TasksSuspend::from_bits(val)
    }
}
impl From<TasksSuspend> for u8 {
    #[inline(always)]
    fn from(val: TasksSuspend) -> u8 {
        TasksSuspend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TxdConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl TxdConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxdConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxdConnect {
    #[inline(always)]
    fn from(val: u8) -> TxdConnect {
        TxdConnect::from_bits(val)
    }
}
impl From<TxdConnect> for u8 {
    #[inline(always)]
    fn from(val: TxdConnect) -> u8 {
        TxdConnect::to_bits(val)
    }
}
