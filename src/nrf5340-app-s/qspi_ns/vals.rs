#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AddrconfWipwait {
    #[doc = "No wait."]
    DISABLE = 0x0,
    #[doc = "Wait."]
    ENABLE = 0x01,
}
impl AddrconfWipwait {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrconfWipwait {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrconfWipwait {
    #[inline(always)]
    fn from(val: u8) -> AddrconfWipwait {
        AddrconfWipwait::from_bits(val)
    }
}
impl From<AddrconfWipwait> for u8 {
    #[inline(always)]
    fn from(val: AddrconfWipwait) -> u8 {
        AddrconfWipwait::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AddrconfWren {
    #[doc = "Do not send WREN."]
    DISABLE = 0x0,
    #[doc = "Send WREN."]
    ENABLE = 0x01,
}
impl AddrconfWren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrconfWren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrconfWren {
    #[inline(always)]
    fn from(val: u8) -> AddrconfWren {
        AddrconfWren::from_bits(val)
    }
}
impl From<AddrconfWren> for u8 {
    #[inline(always)]
    fn from(val: AddrconfWren) -> u8 {
        AddrconfWren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Addrmode {
    #[doc = "24-bit addressing."]
    _24BIT = 0x0,
    #[doc = "32-bit addressing."]
    _32BIT = 0x01,
}
impl Addrmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addrmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addrmode {
    #[inline(always)]
    fn from(val: u8) -> Addrmode {
        Addrmode::from_bits(val)
    }
}
impl From<Addrmode> for u8 {
    #[inline(always)]
    fn from(val: Addrmode) -> u8 {
        Addrmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CinstrconfWipwait {
    #[doc = "No wait."]
    DISABLE = 0x0,
    #[doc = "Wait."]
    ENABLE = 0x01,
}
impl CinstrconfWipwait {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CinstrconfWipwait {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CinstrconfWipwait {
    #[inline(always)]
    fn from(val: u8) -> CinstrconfWipwait {
        CinstrconfWipwait::from_bits(val)
    }
}
impl From<CinstrconfWipwait> for u8 {
    #[inline(always)]
    fn from(val: CinstrconfWipwait) -> u8 {
        CinstrconfWipwait::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CinstrconfWren {
    #[doc = "Do not send WREN."]
    DISABLE = 0x0,
    #[doc = "Send WREN."]
    ENABLE = 0x01,
}
impl CinstrconfWren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CinstrconfWren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CinstrconfWren {
    #[inline(always)]
    fn from(val: u8) -> CinstrconfWren {
        CinstrconfWren::from_bits(val)
    }
}
impl From<CinstrconfWren> for u8 {
    #[inline(always)]
    fn from(val: CinstrconfWren) -> u8 {
        CinstrconfWren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CsnConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl CsnConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsnConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsnConnect {
    #[inline(always)]
    fn from(val: u8) -> CsnConnect {
        CsnConnect::from_bits(val)
    }
}
impl From<CsnConnect> for u8 {
    #[inline(always)]
    fn from(val: CsnConnect) -> u8 {
        CsnConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DmaEncEnableEnable {
    #[doc = "Disable stream cipher for QSPI EasyDMA"]
    DISABLED = 0x0,
    #[doc = "Enable stream cipher for QSPI EasyDMA"]
    ENABLED = 0x01,
}
impl DmaEncEnableEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaEncEnableEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaEncEnableEnable {
    #[inline(always)]
    fn from(val: u8) -> DmaEncEnableEnable {
        DmaEncEnableEnable::from_bits(val)
    }
}
impl From<DmaEncEnableEnable> for u8 {
    #[inline(always)]
    fn from(val: DmaEncEnableEnable) -> u8 {
        DmaEncEnableEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dpm {
    #[doc = "External flash is not in DPM."]
    DISABLED = 0x0,
    #[doc = "External flash is in DPM."]
    ENABLED = 0x01,
}
impl Dpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dpm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dpm {
    #[inline(always)]
    fn from(val: u8) -> Dpm {
        Dpm::from_bits(val)
    }
}
impl From<Dpm> for u8 {
    #[inline(always)]
    fn from(val: Dpm) -> u8 {
        Dpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dpmen {
    #[doc = "Exit DPM."]
    EXIT = 0x0,
    #[doc = "Enter DPM."]
    ENTER = 0x01,
}
impl Dpmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dpmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dpmen {
    #[inline(always)]
    fn from(val: u8) -> Dpmen {
        Dpmen::from_bits(val)
    }
}
impl From<Dpmen> for u8 {
    #[inline(always)]
    fn from(val: Dpmen) -> u8 {
        Dpmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dpmenable {
    #[doc = "Disable DPM feature."]
    DISABLE = 0x0,
    #[doc = "Enable DPM feature."]
    ENABLE = 0x01,
}
impl Dpmenable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dpmenable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dpmenable {
    #[inline(always)]
    fn from(val: u8) -> Dpmenable {
        Dpmenable::from_bits(val)
    }
}
impl From<Dpmenable> for u8 {
    #[inline(always)]
    fn from(val: Dpmenable) -> u8 {
        Dpmenable::to_bits(val)
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
pub enum Io0connect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Io0connect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Io0connect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Io0connect {
    #[inline(always)]
    fn from(val: u8) -> Io0connect {
        Io0connect::from_bits(val)
    }
}
impl From<Io0connect> for u8 {
    #[inline(always)]
    fn from(val: Io0connect) -> u8 {
        Io0connect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Io1connect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Io1connect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Io1connect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Io1connect {
    #[inline(always)]
    fn from(val: u8) -> Io1connect {
        Io1connect::from_bits(val)
    }
}
impl From<Io1connect> for u8 {
    #[inline(always)]
    fn from(val: Io1connect) -> u8 {
        Io1connect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Io2connect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Io2connect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Io2connect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Io2connect {
    #[inline(always)]
    fn from(val: u8) -> Io2connect {
        Io2connect::from_bits(val)
    }
}
impl From<Io2connect> for u8 {
    #[inline(always)]
    fn from(val: Io2connect) -> u8 {
        Io2connect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Io3connect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl Io3connect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Io3connect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Io3connect {
    #[inline(always)]
    fn from(val: u8) -> Io3connect {
        Io3connect::from_bits(val)
    }
}
impl From<Io3connect> for u8 {
    #[inline(always)]
    fn from(val: Io3connect) -> u8 {
        Io3connect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Len {
    #[doc = "Erase 4 kB block (flash command 0x20)"]
    _4KB = 0x0,
    #[doc = "Erase 64 kB block (flash command 0xD8)"]
    _64KB = 0x01,
    #[doc = "Erase all (flash command 0xC7)"]
    ALL = 0x02,
    _RESERVED_3 = 0x03,
}
impl Len {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Len {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Len {
    #[inline(always)]
    fn from(val: u8) -> Len {
        Len::from_bits(val)
    }
}
impl From<Len> for u8 {
    #[inline(always)]
    fn from(val: Len) -> u8 {
        Len::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Length {
    _RESERVED_0 = 0x0,
    #[doc = "Send opcode only."]
    _1B = 0x01,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0."]
    _2B = 0x02,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE1."]
    _3B = 0x03,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE2."]
    _4B = 0x04,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE3."]
    _5B = 0x05,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE4."]
    _6B = 0x06,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE5."]
    _7B = 0x07,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE6."]
    _8B = 0x08,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE7."]
    _9B = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Length {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Length {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Length {
    #[inline(always)]
    fn from(val: u8) -> Length {
        Length::from_bits(val)
    }
}
impl From<Length> for u8 {
    #[inline(always)]
    fn from(val: Length) -> u8 {
        Length::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lfen {
    #[doc = "Long frame mode disabled"]
    DISABLE = 0x0,
    #[doc = "Long frame mode enabled"]
    ENABLE = 0x01,
}
impl Lfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lfen {
    #[inline(always)]
    fn from(val: u8) -> Lfen {
        Lfen::from_bits(val)
    }
}
impl From<Lfen> for u8 {
    #[inline(always)]
    fn from(val: Lfen) -> u8 {
        Lfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lfstop {
    _RESERVED_0 = 0x0,
    #[doc = "Stop"]
    STOP = 0x01,
}
impl Lfstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lfstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lfstop {
    #[inline(always)]
    fn from(val: u8) -> Lfstop {
        Lfstop::from_bits(val)
    }
}
impl From<Lfstop> for u8 {
    #[inline(always)]
    fn from(val: Lfstop) -> u8 {
        Lfstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "Do not send any instruction."]
    NOINSTR = 0x0,
    #[doc = "Send opcode."]
    OPCODE = 0x01,
    #[doc = "Send opcode, BYTE0."]
    OPBYTE0 = 0x02,
    #[doc = "Send opcode, BYTE0, BYTE1."]
    ALL = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
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
pub enum Ppsize {
    #[doc = "256 bytes."]
    _256BYTES = 0x0,
    #[doc = "512 bytes."]
    _512BYTES = 0x01,
}
impl Ppsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppsize {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppsize {
    #[inline(always)]
    fn from(val: u8) -> Ppsize {
        Ppsize::from_bits(val)
    }
}
impl From<Ppsize> for u8 {
    #[inline(always)]
    fn from(val: Ppsize) -> u8 {
        Ppsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishReadyEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishReadyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishReadyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishReadyEn {
    #[inline(always)]
    fn from(val: u8) -> PublishReadyEn {
        PublishReadyEn::from_bits(val)
    }
}
impl From<PublishReadyEn> for u8 {
    #[inline(always)]
    fn from(val: PublishReadyEn) -> u8 {
        PublishReadyEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum QspiNsEnableEnable {
    #[doc = "Disable QSPI"]
    DISABLED = 0x0,
    #[doc = "Enable QSPI"]
    ENABLED = 0x01,
}
impl QspiNsEnableEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QspiNsEnableEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QspiNsEnableEnable {
    #[inline(always)]
    fn from(val: u8) -> QspiNsEnableEnable {
        QspiNsEnableEnable::from_bits(val)
    }
}
impl From<QspiNsEnableEnable> for u8 {
    #[inline(always)]
    fn from(val: QspiNsEnableEnable) -> u8 {
        QspiNsEnableEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Readoc {
    #[doc = "Single data line SPI. FAST_READ (opcode 0x0B)."]
    FASTREAD = 0x0,
    #[doc = "Dual data line SPI. READ2O (opcode 0x3B)."]
    READ2O = 0x01,
    #[doc = "Dual data line SPI. READ2IO (opcode 0xBB)."]
    READ2IO = 0x02,
    #[doc = "Quad data line SPI. READ4O (opcode 0x6B)."]
    READ4O = 0x03,
    #[doc = "Quad data line SPI. READ4IO (opcode 0xEB)."]
    READ4IO = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Readoc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Readoc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Readoc {
    #[inline(always)]
    fn from(val: u8) -> Readoc {
        Readoc::from_bits(val)
    }
}
impl From<Readoc> for u8 {
    #[inline(always)]
    fn from(val: Readoc) -> u8 {
        Readoc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SckConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl SckConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SckConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SckConnect {
    #[inline(always)]
    fn from(val: u8) -> SckConnect {
        SckConnect::from_bits(val)
    }
}
impl From<SckConnect> for u8 {
    #[inline(always)]
    fn from(val: SckConnect) -> u8 {
        SckConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spimode {
    #[doc = "Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    MODE0 = 0x0,
    #[doc = "Mode 3: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    MODE3 = 0x01,
}
impl Spimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spimode {
    #[inline(always)]
    fn from(val: u8) -> Spimode {
        Spimode::from_bits(val)
    }
}
impl From<Spimode> for u8 {
    #[inline(always)]
    fn from(val: Spimode) -> u8 {
        Spimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum StatusReady {
    #[doc = "QSPI peripheral is busy. It is not allowed to trigger any new tasks, writing custom instructions or enter/exit DPM."]
    BUSY = 0x0,
    #[doc = "QSPI peripheral is ready. It is allowed to trigger new tasks, writing custom instructions or enter/exit DPM."]
    READY = 0x01,
}
impl StatusReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusReady {
    #[inline(always)]
    fn from(val: u8) -> StatusReady {
        StatusReady::from_bits(val)
    }
}
impl From<StatusReady> for u8 {
    #[inline(always)]
    fn from(val: StatusReady) -> u8 {
        StatusReady::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeActivateEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeActivateEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeActivateEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeActivateEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeActivateEn {
        SubscribeActivateEn::from_bits(val)
    }
}
impl From<SubscribeActivateEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeActivateEn) -> u8 {
        SubscribeActivateEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeDeactivateEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeDeactivateEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeDeactivateEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeDeactivateEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeDeactivateEn {
        SubscribeDeactivateEn::from_bits(val)
    }
}
impl From<SubscribeDeactivateEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeDeactivateEn) -> u8 {
        SubscribeDeactivateEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeErasestartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeErasestartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeErasestartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeErasestartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeErasestartEn {
        SubscribeErasestartEn::from_bits(val)
    }
}
impl From<SubscribeErasestartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeErasestartEn) -> u8 {
        SubscribeErasestartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeReadstartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeReadstartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeReadstartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeReadstartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeReadstartEn {
        SubscribeReadstartEn::from_bits(val)
    }
}
impl From<SubscribeReadstartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeReadstartEn) -> u8 {
        SubscribeReadstartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeWritestartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeWritestartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeWritestartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeWritestartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeWritestartEn {
        SubscribeWritestartEn::from_bits(val)
    }
}
impl From<SubscribeWritestartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeWritestartEn) -> u8 {
        SubscribeWritestartEn::to_bits(val)
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
pub enum TasksDeactivate {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksDeactivate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksDeactivate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksDeactivate {
    #[inline(always)]
    fn from(val: u8) -> TasksDeactivate {
        TasksDeactivate::from_bits(val)
    }
}
impl From<TasksDeactivate> for u8 {
    #[inline(always)]
    fn from(val: TasksDeactivate) -> u8 {
        TasksDeactivate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksErasestart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksErasestart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksErasestart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksErasestart {
    #[inline(always)]
    fn from(val: u8) -> TasksErasestart {
        TasksErasestart::from_bits(val)
    }
}
impl From<TasksErasestart> for u8 {
    #[inline(always)]
    fn from(val: TasksErasestart) -> u8 {
        TasksErasestart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksReadstart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksReadstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksReadstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksReadstart {
    #[inline(always)]
    fn from(val: u8) -> TasksReadstart {
        TasksReadstart::from_bits(val)
    }
}
impl From<TasksReadstart> for u8 {
    #[inline(always)]
    fn from(val: TasksReadstart) -> u8 {
        TasksReadstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksWritestart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksWritestart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksWritestart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksWritestart {
    #[inline(always)]
    fn from(val: u8) -> TasksWritestart {
        TasksWritestart::from_bits(val)
    }
}
impl From<TasksWritestart> for u8 {
    #[inline(always)]
    fn from(val: TasksWritestart) -> u8 {
        TasksWritestart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Writeoc {
    #[doc = "Single data line SPI. PP (opcode 0x02)."]
    PP = 0x0,
    #[doc = "Dual data line SPI. PP2O (opcode 0xA2)."]
    PP2O = 0x01,
    #[doc = "Quad data line SPI. PP4O (opcode 0x32)."]
    PP4O = 0x02,
    #[doc = "Quad data line SPI. PP4IO (opcode 0x38)."]
    PP4IO = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Writeoc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Writeoc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Writeoc {
    #[inline(always)]
    fn from(val: u8) -> Writeoc {
        Writeoc::from_bits(val)
    }
}
impl From<Writeoc> for u8 {
    #[inline(always)]
    fn from(val: Writeoc) -> u8 {
        Writeoc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum XipEncEnableEnable {
    #[doc = "Disable stream cipher for QSPI XIP"]
    DISABLED = 0x0,
    #[doc = "Enable stream cipher for QSPI XIP"]
    ENABLED = 0x01,
}
impl XipEncEnableEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XipEncEnableEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XipEncEnableEnable {
    #[inline(always)]
    fn from(val: u8) -> XipEncEnableEnable {
        XipEncEnableEnable::from_bits(val)
    }
}
impl From<XipEncEnableEnable> for u8 {
    #[inline(always)]
    fn from(val: XipEncEnableEnable) -> u8 {
        XipEncEnableEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Xipen {
    #[doc = "Disable XIP interface"]
    DISABLE = 0x0,
    #[doc = "Enable XIP interface"]
    ENABLE = 0x01,
}
impl Xipen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xipen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xipen {
    #[inline(always)]
    fn from(val: u8) -> Xipen {
        Xipen::from_bits(val)
    }
}
impl From<Xipen> for u8 {
    #[inline(always)]
    fn from(val: Xipen) -> u8 {
        Xipen::to_bits(val)
    }
}
