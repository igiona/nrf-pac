#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bitframesdd {
    #[doc = "SDD pattern 00000"]
    SDD00000 = 0x0,
    #[doc = "SDD pattern 00001"]
    SDD00001 = 0x01,
    #[doc = "SDD pattern 00010"]
    SDD00010 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "SDD pattern 00100"]
    SDD00100 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "SDD pattern 01000"]
    SDD01000 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "SDD pattern 10000"]
    SDD10000 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Bitframesdd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bitframesdd {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bitframesdd {
    #[inline(always)]
    fn from(val: u8) -> Bitframesdd {
        Bitframesdd::from_bits(val)
    }
}
impl From<Bitframesdd> for u8 {
    #[inline(always)]
    fn from(val: Bitframesdd) -> u8 {
        Bitframesdd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Connect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Connect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Connect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Connect {
    #[inline(always)]
    fn from(val: u8) -> Connect {
        Connect::from_bits(val)
    }
}
impl From<Connect> for u8 {
    #[inline(always)]
    fn from(val: Connect) -> u8 {
        Connect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crcerror {
    #[doc = "Valid CRC detected"]
    CRCCORRECT = 0x0,
    #[doc = "CRC received does not match local check"]
    CRCERROR = 0x01,
}
impl Crcerror {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcerror {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcerror {
    #[inline(always)]
    fn from(val: u8) -> Crcerror {
        Crcerror::from_bits(val)
    }
}
impl From<Crcerror> for u8 {
    #[inline(always)]
    fn from(val: Crcerror) -> u8 {
        Crcerror::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crcmoderx {
    #[doc = "CRC is not expected in RX frames"]
    NOCRCRX = 0x0,
    #[doc = "Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    CRC16RX = 0x01,
}
impl Crcmoderx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcmoderx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcmoderx {
    #[inline(always)]
    fn from(val: u8) -> Crcmoderx {
        Crcmoderx::from_bits(val)
    }
}
impl From<Crcmoderx> for u8 {
    #[inline(always)]
    fn from(val: Crcmoderx) -> u8 {
        Crcmoderx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crcmodetx {
    #[doc = "CRC is not added to the frame"]
    NOCRCTX = 0x0,
    #[doc = "16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    CRC16TX = 0x01,
}
impl Crcmodetx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcmodetx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcmodetx {
    #[inline(always)]
    fn from(val: u8) -> Crcmodetx {
        Crcmodetx::from_bits(val)
    }
}
impl From<Crcmodetx> for u8 {
    #[inline(always)]
    fn from(val: Crcmodetx) -> u8 {
        Crcmodetx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Discardmode {
    #[doc = "Unused bits are discarded at end of frame (EoF)"]
    DISCARDEND = 0x0,
    #[doc = "Unused bits are discarded at start of frame (SoF)"]
    DISCARDSTART = 0x01,
}
impl Discardmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Discardmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Discardmode {
    #[inline(always)]
    fn from(val: u8) -> Discardmode {
        Discardmode::from_bits(val)
    }
}
impl From<Discardmode> for u8 {
    #[inline(always)]
    fn from(val: Discardmode) -> u8 {
        Discardmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsAutocolresstarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsAutocolresstarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsAutocolresstarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsAutocolresstarted {
    #[inline(always)]
    fn from(val: u8) -> EventsAutocolresstarted {
        EventsAutocolresstarted::from_bits(val)
    }
}
impl From<EventsAutocolresstarted> for u8 {
    #[inline(always)]
    fn from(val: EventsAutocolresstarted) -> u8 {
        EventsAutocolresstarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCollision {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCollision {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCollision {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCollision {
    #[inline(always)]
    fn from(val: u8) -> EventsCollision {
        EventsCollision::from_bits(val)
    }
}
impl From<EventsCollision> for u8 {
    #[inline(always)]
    fn from(val: EventsCollision) -> u8 {
        EventsCollision::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEndrx {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEndrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEndrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEndrx {
    #[inline(always)]
    fn from(val: u8) -> EventsEndrx {
        EventsEndrx::from_bits(val)
    }
}
impl From<EventsEndrx> for u8 {
    #[inline(always)]
    fn from(val: EventsEndrx) -> u8 {
        EventsEndrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEndtx {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEndtx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEndtx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEndtx {
    #[inline(always)]
    fn from(val: u8) -> EventsEndtx {
        EventsEndtx::from_bits(val)
    }
}
impl From<EventsEndtx> for u8 {
    #[inline(always)]
    fn from(val: EventsEndtx) -> u8 {
        EventsEndtx::to_bits(val)
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
pub enum EventsFielddetected {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsFielddetected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsFielddetected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsFielddetected {
    #[inline(always)]
    fn from(val: u8) -> EventsFielddetected {
        EventsFielddetected::from_bits(val)
    }
}
impl From<EventsFielddetected> for u8 {
    #[inline(always)]
    fn from(val: EventsFielddetected) -> u8 {
        EventsFielddetected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsFieldlost {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsFieldlost {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsFieldlost {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsFieldlost {
    #[inline(always)]
    fn from(val: u8) -> EventsFieldlost {
        EventsFieldlost::from_bits(val)
    }
}
impl From<EventsFieldlost> for u8 {
    #[inline(always)]
    fn from(val: EventsFieldlost) -> u8 {
        EventsFieldlost::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsReady {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsReady {
    #[inline(always)]
    fn from(val: u8) -> EventsReady {
        EventsReady::from_bits(val)
    }
}
impl From<EventsReady> for u8 {
    #[inline(always)]
    fn from(val: EventsReady) -> u8 {
        EventsReady::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsRxerror {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRxerror {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRxerror {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRxerror {
    #[inline(always)]
    fn from(val: u8) -> EventsRxerror {
        EventsRxerror::from_bits(val)
    }
}
impl From<EventsRxerror> for u8 {
    #[inline(always)]
    fn from(val: EventsRxerror) -> u8 {
        EventsRxerror::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsRxframeend {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRxframeend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRxframeend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRxframeend {
    #[inline(always)]
    fn from(val: u8) -> EventsRxframeend {
        EventsRxframeend::from_bits(val)
    }
}
impl From<EventsRxframeend> for u8 {
    #[inline(always)]
    fn from(val: EventsRxframeend) -> u8 {
        EventsRxframeend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsRxframestart {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRxframestart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRxframestart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRxframestart {
    #[inline(always)]
    fn from(val: u8) -> EventsRxframestart {
        EventsRxframestart::from_bits(val)
    }
}
impl From<EventsRxframestart> for u8 {
    #[inline(always)]
    fn from(val: EventsRxframestart) -> u8 {
        EventsRxframestart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsSelected {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsSelected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsSelected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsSelected {
    #[inline(always)]
    fn from(val: u8) -> EventsSelected {
        EventsSelected::from_bits(val)
    }
}
impl From<EventsSelected> for u8 {
    #[inline(always)]
    fn from(val: EventsSelected) -> u8 {
        EventsSelected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsStarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsStarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsStarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsStarted {
    #[inline(always)]
    fn from(val: u8) -> EventsStarted {
        EventsStarted::from_bits(val)
    }
}
impl From<EventsStarted> for u8 {
    #[inline(always)]
    fn from(val: EventsStarted) -> u8 {
        EventsStarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsTxframeend {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsTxframeend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsTxframeend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsTxframeend {
    #[inline(always)]
    fn from(val: u8) -> EventsTxframeend {
        EventsTxframeend::from_bits(val)
    }
}
impl From<EventsTxframeend> for u8 {
    #[inline(always)]
    fn from(val: EventsTxframeend) -> u8 {
        EventsTxframeend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsTxframestart {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsTxframestart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsTxframestart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsTxframestart {
    #[inline(always)]
    fn from(val: u8) -> EventsTxframestart {
        EventsTxframestart::from_bits(val)
    }
}
impl From<EventsTxframestart> for u8 {
    #[inline(always)]
    fn from(val: EventsTxframestart) -> u8 {
        EventsTxframestart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fieldpresent {
    #[doc = "No valid field detected"]
    NOFIELD = 0x0,
    #[doc = "Valid field detected"]
    FIELDPRESENT = 0x01,
}
impl Fieldpresent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fieldpresent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fieldpresent {
    #[inline(always)]
    fn from(val: u8) -> Fieldpresent {
        Fieldpresent::from_bits(val)
    }
}
impl From<Fieldpresent> for u8 {
    #[inline(always)]
    fn from(val: Fieldpresent) -> u8 {
        Fieldpresent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Framedelaymode {
    #[doc = "Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    FREERUN = 0x0,
    #[doc = "Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    WINDOW = 0x01,
    #[doc = "Frame is transmitted exactly at FRAMEDELAYMAX"]
    EXACTVAL = 0x02,
    #[doc = "Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    WINDOWGRID = 0x03,
}
impl Framedelaymode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Framedelaymode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Framedelaymode {
    #[inline(always)]
    fn from(val: u8) -> Framedelaymode {
        Framedelaymode::from_bits(val)
    }
}
impl From<Framedelaymode> for u8 {
    #[inline(always)]
    fn from(val: Framedelaymode) -> u8 {
        Framedelaymode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lockdetect {
    #[doc = "Not locked to field"]
    NOTLOCKED = 0x0,
    #[doc = "Locked to field"]
    LOCKED = 0x01,
}
impl Lockdetect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lockdetect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lockdetect {
    #[inline(always)]
    fn from(val: u8) -> Lockdetect {
        Lockdetect::from_bits(val)
    }
}
impl From<Lockdetect> for u8 {
    #[inline(always)]
    fn from(val: Lockdetect) -> u8 {
        Lockdetect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "Auto collision resolution enabled"]
    ENABLED = 0x0,
    #[doc = "Auto collision resolution disabled"]
    DISABLED = 0x01,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Modulationctrl {
    #[doc = "Invalid, defaults to same behaviour as for Internal"]
    INVALID = 0x0,
    #[doc = "Use internal modulator only"]
    INTERNAL = 0x01,
    #[doc = "Output digital modulation signal to a GPIO pin."]
    MODTOGPIO = 0x02,
    #[doc = "Use internal modulator and output digital modulation signal to a GPIO pin."]
    INTERNALANDMODTOGPIO = 0x03,
}
impl Modulationctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Modulationctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Modulationctrl {
    #[inline(always)]
    fn from(val: u8) -> Modulationctrl {
        Modulationctrl::from_bits(val)
    }
}
impl From<Modulationctrl> for u8 {
    #[inline(always)]
    fn from(val: Modulationctrl) -> u8 {
        Modulationctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Nfcidsize {
    #[doc = "NFCID1 size: single (4 bytes)"]
    NFCID1SINGLE = 0x0,
    #[doc = "NFCID1 size: double (7 bytes)"]
    NFCID1DOUBLE = 0x01,
    #[doc = "NFCID1 size: triple (10 bytes)"]
    NFCID1TRIPLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Nfcidsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfcidsize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfcidsize {
    #[inline(always)]
    fn from(val: u8) -> Nfcidsize {
        Nfcidsize::from_bits(val)
    }
}
impl From<Nfcidsize> for u8 {
    #[inline(always)]
    fn from(val: Nfcidsize) -> u8 {
        Nfcidsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Nfctagstate {
    #[doc = "Disabled or sense"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "RampUp"]
    RAMPUP = 0x02,
    #[doc = "Idle"]
    IDLE = 0x03,
    #[doc = "Receive"]
    RECEIVE = 0x04,
    #[doc = "FrameDelay"]
    FRAMEDELAY = 0x05,
    #[doc = "Transmit"]
    TRANSMIT = 0x06,
    _RESERVED_7 = 0x07,
}
impl Nfctagstate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nfctagstate {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nfctagstate {
    #[inline(always)]
    fn from(val: u8) -> Nfctagstate {
        Nfctagstate::from_bits(val)
    }
}
impl From<Nfctagstate> for u8 {
    #[inline(always)]
    fn from(val: Nfctagstate) -> u8 {
        Nfctagstate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Overrun {
    #[doc = "No overrun detected"]
    NOOVERRUN = 0x0,
    #[doc = "Overrun error"]
    OVERRUN = 0x01,
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
pub enum Paritystatus {
    #[doc = "Frame received with parity OK"]
    PARITYOK = 0x0,
    #[doc = "Frame received with parity error"]
    PARITYERROR = 0x01,
}
impl Paritystatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Paritystatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Paritystatus {
    #[inline(always)]
    fn from(val: u8) -> Paritystatus {
        Paritystatus::from_bits(val)
    }
}
impl From<Paritystatus> for u8 {
    #[inline(always)]
    fn from(val: Paritystatus) -> u8 {
        Paritystatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RxdFrameconfigParity {
    #[doc = "Parity is not expected in RX frames"]
    NOPARITY = 0x0,
    #[doc = "Parity is expected in RX frames"]
    PARITY = 0x01,
}
impl RxdFrameconfigParity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxdFrameconfigParity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxdFrameconfigParity {
    #[inline(always)]
    fn from(val: u8) -> RxdFrameconfigParity {
        RxdFrameconfigParity::from_bits(val)
    }
}
impl From<RxdFrameconfigParity> for u8 {
    #[inline(always)]
    fn from(val: RxdFrameconfigParity) -> u8 {
        RxdFrameconfigParity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RxdFrameconfigSof {
    #[doc = "SoF symbol is not expected in RX frames"]
    NOSOF = 0x0,
    #[doc = "SoF symbol is expected in RX frames"]
    SOF = 0x01,
}
impl RxdFrameconfigSof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxdFrameconfigSof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxdFrameconfigSof {
    #[inline(always)]
    fn from(val: u8) -> RxdFrameconfigSof {
        RxdFrameconfigSof::from_bits(val)
    }
}
impl From<RxdFrameconfigSof> for u8 {
    #[inline(always)]
    fn from(val: RxdFrameconfigSof) -> u8 {
        RxdFrameconfigSof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sleepstate {
    #[doc = "State is IDLE."]
    IDLE = 0x0,
    #[doc = "State is SLEEP_A."]
    SLEEPA = 0x01,
}
impl Sleepstate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleepstate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleepstate {
    #[inline(always)]
    fn from(val: u8) -> Sleepstate {
        Sleepstate::from_bits(val)
    }
}
impl From<Sleepstate> for u8 {
    #[inline(always)]
    fn from(val: Sleepstate) -> u8 {
        Sleepstate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksActivate {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksActivate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksActivate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksActivate {
    #[inline(always)]
    fn from(val: u8) -> TasksActivate {
        TasksActivate::from_bits(val)
    }
}
impl From<TasksActivate> for u8 {
    #[inline(always)]
    fn from(val: TasksActivate) -> u8 {
        TasksActivate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksDisable {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksDisable {
    #[inline(always)]
    fn from(val: u8) -> TasksDisable {
        TasksDisable::from_bits(val)
    }
}
impl From<TasksDisable> for u8 {
    #[inline(always)]
    fn from(val: TasksDisable) -> u8 {
        TasksDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksEnablerxdata {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksEnablerxdata {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksEnablerxdata {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksEnablerxdata {
    #[inline(always)]
    fn from(val: u8) -> TasksEnablerxdata {
        TasksEnablerxdata::from_bits(val)
    }
}
impl From<TasksEnablerxdata> for u8 {
    #[inline(always)]
    fn from(val: TasksEnablerxdata) -> u8 {
        TasksEnablerxdata::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksGoidle {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksGoidle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksGoidle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksGoidle {
    #[inline(always)]
    fn from(val: u8) -> TasksGoidle {
        TasksGoidle::from_bits(val)
    }
}
impl From<TasksGoidle> for u8 {
    #[inline(always)]
    fn from(val: TasksGoidle) -> u8 {
        TasksGoidle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksGosleep {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksGosleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksGosleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksGosleep {
    #[inline(always)]
    fn from(val: u8) -> TasksGosleep {
        TasksGosleep::from_bits(val)
    }
}
impl From<TasksGosleep> for u8 {
    #[inline(always)]
    fn from(val: TasksGosleep) -> u8 {
        TasksGosleep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksSense {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksSense {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksSense {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksSense {
    #[inline(always)]
    fn from(val: u8) -> TasksSense {
        TasksSense::from_bits(val)
    }
}
impl From<TasksSense> for u8 {
    #[inline(always)]
    fn from(val: TasksSense) -> u8 {
        TasksSense::to_bits(val)
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
pub enum TxdFrameconfigParity {
    #[doc = "Parity is not added to TX frames"]
    NOPARITY = 0x0,
    #[doc = "Parity is added to TX frames"]
    PARITY = 0x01,
}
impl TxdFrameconfigParity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxdFrameconfigParity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxdFrameconfigParity {
    #[inline(always)]
    fn from(val: u8) -> TxdFrameconfigParity {
        TxdFrameconfigParity::from_bits(val)
    }
}
impl From<TxdFrameconfigParity> for u8 {
    #[inline(always)]
    fn from(val: TxdFrameconfigParity) -> u8 {
        TxdFrameconfigParity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TxdFrameconfigSof {
    #[doc = "SoF symbol not added"]
    NOSOF = 0x0,
    #[doc = "SoF symbol added"]
    SOF = 0x01,
}
impl TxdFrameconfigSof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxdFrameconfigSof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxdFrameconfigSof {
    #[inline(always)]
    fn from(val: u8) -> TxdFrameconfigSof {
        TxdFrameconfigSof::from_bits(val)
    }
}
impl From<TxdFrameconfigSof> for u8 {
    #[inline(always)]
    fn from(val: TxdFrameconfigSof) -> u8 {
        TxdFrameconfigSof::to_bits(val)
    }
}
