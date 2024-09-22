#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Align {
    #[doc = "Left-aligned."]
    LEFT = 0x0,
    #[doc = "Right-aligned."]
    RIGHT = 0x01,
}
impl Align {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Align {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Align {
    #[inline(always)]
    fn from(val: u8) -> Align {
        Align::from_bits(val)
    }
}
impl From<Align> for u8 {
    #[inline(always)]
    fn from(val: Align) -> u8 {
        Align::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bypass {
    #[doc = "Disable bypass"]
    DISABLE = 0x0,
    #[doc = "Enable bypass"]
    ENABLE = 0x01,
}
impl Bypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bypass {
    #[inline(always)]
    fn from(val: u8) -> Bypass {
        Bypass::from_bits(val)
    }
}
impl From<Bypass> for u8 {
    #[inline(always)]
    fn from(val: Bypass) -> u8 {
        Bypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channels {
    #[doc = "Stereo."]
    STEREO = 0x0,
    #[doc = "Left only."]
    LEFT = 0x01,
    #[doc = "Right only."]
    RIGHT = 0x02,
    _RESERVED_3 = 0x03,
}
impl Channels {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Channels {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Channels {
    #[inline(always)]
    fn from(val: u8) -> Channels {
        Channels::from_bits(val)
    }
}
impl From<Channels> for u8 {
    #[inline(always)]
    fn from(val: Channels) -> u8 {
        Channels::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clksrc {
    #[doc = "32MHz peripheral clock"]
    PCLK32M = 0x0,
    #[doc = "Audio PLL clock"]
    ACLK = 0x01,
}
impl Clksrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clksrc {
    #[inline(always)]
    fn from(val: u8) -> Clksrc {
        Clksrc::from_bits(val)
    }
}
impl From<Clksrc> for u8 {
    #[inline(always)]
    fn from(val: Clksrc) -> u8 {
        Clksrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x01) }
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
pub enum EventsFramestart {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsFramestart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsFramestart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsFramestart {
    #[inline(always)]
    fn from(val: u8) -> EventsFramestart {
        EventsFramestart::from_bits(val)
    }
}
impl From<EventsFramestart> for u8 {
    #[inline(always)]
    fn from(val: EventsFramestart) -> u8 {
        EventsFramestart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsRxptrupd {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRxptrupd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRxptrupd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRxptrupd {
    #[inline(always)]
    fn from(val: u8) -> EventsRxptrupd {
        EventsRxptrupd::from_bits(val)
    }
}
impl From<EventsRxptrupd> for u8 {
    #[inline(always)]
    fn from(val: EventsRxptrupd) -> u8 {
        EventsRxptrupd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsStopped {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsStopped {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsStopped {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsStopped {
    #[inline(always)]
    fn from(val: u8) -> EventsStopped {
        EventsStopped::from_bits(val)
    }
}
impl From<EventsStopped> for u8 {
    #[inline(always)]
    fn from(val: EventsStopped) -> u8 {
        EventsStopped::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsTxptrupd {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsTxptrupd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsTxptrupd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsTxptrupd {
    #[inline(always)]
    fn from(val: u8) -> EventsTxptrupd {
        EventsTxptrupd::from_bits(val)
    }
}
impl From<EventsTxptrupd> for u8 {
    #[inline(always)]
    fn from(val: EventsTxptrupd) -> u8 {
        EventsTxptrupd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Format {
    #[doc = "Original I2S format."]
    I2S = 0x0,
    #[doc = "Alternate (left- or right-aligned) format."]
    ALIGNED = 0x01,
}
impl Format {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Format {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Format {
    #[inline(always)]
    fn from(val: u8) -> Format {
        Format::from_bits(val)
    }
}
impl From<Format> for u8 {
    #[inline(always)]
    fn from(val: Format) -> u8 {
        Format::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LrckConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl LrckConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LrckConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LrckConnect {
    #[inline(always)]
    fn from(val: u8) -> LrckConnect {
        LrckConnect::from_bits(val)
    }
}
impl From<LrckConnect> for u8 {
    #[inline(always)]
    fn from(val: LrckConnect) -> u8 {
        LrckConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MckConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl MckConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MckConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MckConnect {
    #[inline(always)]
    fn from(val: u8) -> MckConnect {
        MckConnect::from_bits(val)
    }
}
impl From<MckConnect> for u8 {
    #[inline(always)]
    fn from(val: MckConnect) -> u8 {
        MckConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mcken {
    #[doc = "Master clock generator disabled and PSEL.MCK not connected(available as GPIO)."]
    DISABLED = 0x0,
    #[doc = "Master clock generator running and MCK output on PSEL.MCK."]
    ENABLED = 0x01,
}
impl Mcken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mcken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mcken {
    #[inline(always)]
    fn from(val: u8) -> Mcken {
        Mcken::from_bits(val)
    }
}
impl From<Mcken> for u8 {
    #[inline(always)]
    fn from(val: Mcken) -> u8 {
        Mcken::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mckfreq(pub u32);
impl Mckfreq {
    #[doc = "32 MHz / 125 = 0.256 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV125: Self = Self(0x020c_0000);
    #[doc = "32 MHz / 63 = 0.5079365 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV63: Self = Self(0x0410_0000);
    #[doc = "32 MHz / 42 = 0.7619048 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV42: Self = Self(0x0600_0000);
    #[doc = "32 MHz / 32 = 1.0 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV32: Self = Self(0x0800_0000);
    #[doc = "32 MHz / 31 = 1.0322581 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV31: Self = Self(0x0840_0000);
    #[doc = "32 MHz / 30 = 1.0666667 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV30: Self = Self(0x0880_0000);
    #[doc = "32 MHz / 23 = 1.3913043 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV23: Self = Self(0x0b00_0000);
    #[doc = "32 MHz / 21 = 1.5238095 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV21: Self = Self(0x0c00_0000);
    #[doc = "32 MHz / 16 = 2.0 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV16: Self = Self(0x1000_0000);
    #[doc = "32 MHz / 15 = 2.1333333 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV15: Self = Self(0x1100_0000);
    #[doc = "32 MHz / 11 = 2.9090909 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV11: Self = Self(0x1600_0000);
    #[doc = "32 MHz / 10 = 3.2 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV10: Self = Self(0x1800_0000);
    #[doc = "32 MHz / 8 = 4.0 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV8: Self = Self(0x2000_0000);
    #[doc = "32 MHz / 6 = 5.3333333 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV6: Self = Self(0x2800_0000);
    #[doc = "32 MHz / 5 = 6.4 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV5: Self = Self(0x3000_0000);
    #[doc = "32 MHz / 4 = 8.0 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV4: Self = Self(0x4000_0000);
    #[doc = "32 MHz / 3 = 10.6666667 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV3: Self = Self(0x5000_0000);
    #[doc = "32 MHz / 2 = 16.0 MHz Deprecated, use MCKFREQ equation."]
    pub const _32MDIV2: Self = Self(0x8000_0000);
}
impl Mckfreq {
    pub const fn from_bits(val: u32) -> Mckfreq {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Mckfreq {
    #[inline(always)]
    fn from(val: u32) -> Mckfreq {
        Mckfreq::from_bits(val)
    }
}
impl From<Mckfreq> for u32 {
    #[inline(always)]
    fn from(val: Mckfreq) -> u32 {
        Mckfreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "Master mode. SCK and LRCK generated from internal master clcok (MCK) and output on pins defined by PSEL.xxx."]
    MASTER = 0x0,
    #[doc = "Slave mode. SCK and LRCK generated by external master and received on pins defined by PSEL.xxx"]
    SLAVE = 0x01,
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
pub enum PublishFramestartEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishFramestartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishFramestartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishFramestartEn {
    #[inline(always)]
    fn from(val: u8) -> PublishFramestartEn {
        PublishFramestartEn::from_bits(val)
    }
}
impl From<PublishFramestartEn> for u8 {
    #[inline(always)]
    fn from(val: PublishFramestartEn) -> u8 {
        PublishFramestartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishRxptrupdEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishRxptrupdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishRxptrupdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishRxptrupdEn {
    #[inline(always)]
    fn from(val: u8) -> PublishRxptrupdEn {
        PublishRxptrupdEn::from_bits(val)
    }
}
impl From<PublishRxptrupdEn> for u8 {
    #[inline(always)]
    fn from(val: PublishRxptrupdEn) -> u8 {
        PublishRxptrupdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishStoppedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishStoppedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishStoppedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishStoppedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishStoppedEn {
        PublishStoppedEn::from_bits(val)
    }
}
impl From<PublishStoppedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishStoppedEn) -> u8 {
        PublishStoppedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishTxptrupdEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishTxptrupdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishTxptrupdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishTxptrupdEn {
    #[inline(always)]
    fn from(val: u8) -> PublishTxptrupdEn {
        PublishTxptrupdEn::from_bits(val)
    }
}
impl From<PublishTxptrupdEn> for u8 {
    #[inline(always)]
    fn from(val: PublishTxptrupdEn) -> u8 {
        PublishTxptrupdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ratio {
    #[doc = "LRCK = MCK / 32"]
    _32X = 0x0,
    #[doc = "LRCK = MCK / 48"]
    _48X = 0x01,
    #[doc = "LRCK = MCK / 64"]
    _64X = 0x02,
    #[doc = "LRCK = MCK / 96"]
    _96X = 0x03,
    #[doc = "LRCK = MCK / 128"]
    _128X = 0x04,
    #[doc = "LRCK = MCK / 192"]
    _192X = 0x05,
    #[doc = "LRCK = MCK / 256"]
    _256X = 0x06,
    #[doc = "LRCK = MCK / 384"]
    _384X = 0x07,
    #[doc = "LRCK = MCK / 512"]
    _512X = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ratio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ratio {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ratio {
    #[inline(always)]
    fn from(val: u8) -> Ratio {
        Ratio::from_bits(val)
    }
}
impl From<Ratio> for u8 {
    #[inline(always)]
    fn from(val: Ratio) -> u8 {
        Ratio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rxen {
    #[doc = "Reception disabled and now data will be written to the RXD.PTR address."]
    DISABLED = 0x0,
    #[doc = "Reception enabled."]
    ENABLED = 0x01,
}
impl Rxen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxen {
    #[inline(always)]
    fn from(val: u8) -> Rxen {
        Rxen::from_bits(val)
    }
}
impl From<Rxen> for u8 {
    #[inline(always)]
    fn from(val: Rxen) -> u8 {
        Rxen::to_bits(val)
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
pub enum SdinConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl SdinConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdinConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdinConnect {
    #[inline(always)]
    fn from(val: u8) -> SdinConnect {
        SdinConnect::from_bits(val)
    }
}
impl From<SdinConnect> for u8 {
    #[inline(always)]
    fn from(val: SdinConnect) -> u8 {
        SdinConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SdoutConnect {
    #[doc = "Connect"]
    CONNECTED = 0x0,
    #[doc = "Disconnect"]
    DISCONNECTED = 0x01,
}
impl SdoutConnect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdoutConnect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdoutConnect {
    #[inline(always)]
    fn from(val: u8) -> SdoutConnect {
        SdoutConnect::from_bits(val)
    }
}
impl From<SdoutConnect> for u8 {
    #[inline(always)]
    fn from(val: SdoutConnect) -> u8 {
        SdoutConnect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeStartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeStartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeStartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeStartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeStartEn {
        SubscribeStartEn::from_bits(val)
    }
}
impl From<SubscribeStartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeStartEn) -> u8 {
        SubscribeStartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeStopEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeStopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeStopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeStopEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeStopEn {
        SubscribeStopEn::from_bits(val)
    }
}
impl From<SubscribeStopEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeStopEn) -> u8 {
        SubscribeStopEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swidth {
    #[doc = "8 bit sample."]
    _8BIT = 0x0,
    #[doc = "16 bit sample."]
    _16BIT = 0x01,
    #[doc = "24 bit sample."]
    _24BIT = 0x02,
    #[doc = "32 bit sample."]
    _32BIT = 0x03,
    #[doc = "8 bit sample in a 16-bit half-frame."]
    _8BITIN16 = 0x04,
    #[doc = "8 bit sample in a 32-bit half-frame."]
    _8BITIN32 = 0x05,
    #[doc = "16 bit sample in a 32-bit half-frame."]
    _16BITIN32 = 0x06,
    #[doc = "24 bit sample in a 32-bit half-frame."]
    _24BITIN32 = 0x07,
}
impl Swidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swidth {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swidth {
    #[inline(always)]
    fn from(val: u8) -> Swidth {
        Swidth::from_bits(val)
    }
}
impl From<Swidth> for u8 {
    #[inline(always)]
    fn from(val: Swidth) -> u8 {
        Swidth::to_bits(val)
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
pub enum Txen {
    #[doc = "Transmission disabled and now data will be read from the RXD.TXD address."]
    DISABLED = 0x0,
    #[doc = "Transmission enabled."]
    ENABLED = 0x01,
}
impl Txen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txen {
    #[inline(always)]
    fn from(val: u8) -> Txen {
        Txen::from_bits(val)
    }
}
impl From<Txen> for u8 {
    #[inline(always)]
    fn from(val: Txen) -> u8 {
        Txen::to_bits(val)
    }
}
