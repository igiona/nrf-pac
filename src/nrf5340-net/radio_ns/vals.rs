#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Addr0 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Addr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addr0 {
    #[inline(always)]
    fn from(val: u8) -> Addr0 {
        Addr0::from_bits(val)
    }
}
impl From<Addr0> for u8 {
    #[inline(always)]
    fn from(val: Addr0) -> u8 {
        Addr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Addr1 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Addr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addr1 {
    #[inline(always)]
    fn from(val: u8) -> Addr1 {
        Addr1::from_bits(val)
    }
}
impl From<Addr1> for u8 {
    #[inline(always)]
    fn from(val: Addr1) -> u8 {
        Addr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Addr2 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Addr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addr2 {
    #[inline(always)]
    fn from(val: u8) -> Addr2 {
        Addr2::from_bits(val)
    }
}
impl From<Addr2> for u8 {
    #[inline(always)]
    fn from(val: Addr2) -> u8 {
        Addr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Addr3 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Addr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addr3 {
    #[inline(always)]
    fn from(val: u8) -> Addr3 {
        Addr3::from_bits(val)
    }
}
impl From<Addr3> for u8 {
    #[inline(always)]
    fn from(val: Addr3) -> u8 {
        Addr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Addr4 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Addr4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addr4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addr4 {
    #[inline(always)]
    fn from(val: u8) -> Addr4 {
        Addr4::from_bits(val)
    }
}
impl From<Addr4> for u8 {
    #[inline(always)]
    fn from(val: Addr4) -> u8 {
        Addr4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Addr5 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Addr5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addr5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addr5 {
    #[inline(always)]
    fn from(val: u8) -> Addr5 {
        Addr5::from_bits(val)
    }
}
impl From<Addr5> for u8 {
    #[inline(always)]
    fn from(val: Addr5) -> u8 {
        Addr5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Addr6 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Addr6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addr6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addr6 {
    #[inline(always)]
    fn from(val: u8) -> Addr6 {
        Addr6::from_bits(val)
    }
}
impl From<Addr6> for u8 {
    #[inline(always)]
    fn from(val: Addr6) -> u8 {
        Addr6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Addr7 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Addr7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addr7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addr7 {
    #[inline(always)]
    fn from(val: u8) -> Addr7 {
        Addr7::from_bits(val)
    }
}
impl From<Addr7> for u8 {
    #[inline(always)]
    fn from(val: Addr7) -> u8 {
        Addr7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ccamode {
    #[doc = "Energy above threshold"]
    EDMODE = 0x0,
    #[doc = "Carrier seen"]
    CARRIERMODE = 0x01,
    #[doc = "Energy above threshold AND carrier seen"]
    CARRIERANDEDMODE = 0x02,
    #[doc = "Energy above threshold OR carrier seen"]
    CARRIEROREDMODE = 0x03,
    #[doc = "Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    EDMODETEST1 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ccamode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccamode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccamode {
    #[inline(always)]
    fn from(val: u8) -> Ccamode {
        Ccamode::from_bits(val)
    }
}
impl From<Ccamode> for u8 {
    #[inline(always)]
    fn from(val: Ccamode) -> u8 {
        Ccamode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cistat {
    #[doc = "Frame is received at 125 kbps"]
    LR125KBIT = 0x0,
    #[doc = "Frame is received at 500 kbps"]
    LR500KBIT = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Cistat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cistat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cistat {
    #[inline(always)]
    fn from(val: u8) -> Cistat {
        Cistat::from_bits(val)
    }
}
impl From<Cistat> for u8 {
    #[inline(always)]
    fn from(val: Cistat) -> u8 {
        Cistat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clearpattern {
    _RESERVED_0 = 0x0,
    #[doc = "Clear the GPIO pattern"]
    CLEAR = 0x01,
}
impl Clearpattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clearpattern {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clearpattern {
    #[inline(always)]
    fn from(val: u8) -> Clearpattern {
        Clearpattern::from_bits(val)
    }
}
impl From<Clearpattern> for u8 {
    #[inline(always)]
    fn from(val: Clearpattern) -> u8 {
        Clearpattern::to_bits(val)
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
pub enum Crcinc {
    #[doc = "LENGTH does not contain CRC"]
    EXCLUDE = 0x0,
    #[doc = "LENGTH includes CRC"]
    INCLUDE = 0x01,
}
impl Crcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcinc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcinc {
    #[inline(always)]
    fn from(val: u8) -> Crcinc {
        Crcinc::from_bits(val)
    }
}
impl From<Crcinc> for u8 {
    #[inline(always)]
    fn from(val: Crcinc) -> u8 {
        Crcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crcstatus {
    #[doc = "Packet received with CRC error"]
    CRCERROR = 0x0,
    #[doc = "Packet received with CRC ok"]
    CRCOK = 0x01,
}
impl Crcstatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcstatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcstatus {
    #[inline(always)]
    fn from(val: u8) -> Crcstatus {
        Crcstatus::from_bits(val)
    }
}
impl From<Crcstatus> for u8 {
    #[inline(always)]
    fn from(val: Crcstatus) -> u8 {
        Crcstatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cteerrorhandling {
    #[doc = "No sampling and antenna switching when CRC is not OK"]
    NO = 0x0,
    #[doc = "Sampling and antenna switching also when CRC is not OK"]
    YES = 0x01,
}
impl Cteerrorhandling {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cteerrorhandling {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cteerrorhandling {
    #[inline(always)]
    fn from(val: u8) -> Cteerrorhandling {
        Cteerrorhandling::from_bits(val)
    }
}
impl From<Cteerrorhandling> for u8 {
    #[inline(always)]
    fn from(val: Cteerrorhandling) -> u8 {
        Cteerrorhandling::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cteinfoins1 {
    #[doc = "CTEInfo is NOT in S1 byte (advertising PDU)"]
    NOTINS1 = 0x0,
    #[doc = "CTEInfo is in S1 byte (data PDU)"]
    INS1 = 0x01,
}
impl Cteinfoins1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cteinfoins1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cteinfoins1 {
    #[inline(always)]
    fn from(val: u8) -> Cteinfoins1 {
        Cteinfoins1::from_bits(val)
    }
}
impl From<Cteinfoins1> for u8 {
    #[inline(always)]
    fn from(val: Cteinfoins1) -> u8 {
        Cteinfoins1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cteinlinectrlen {
    #[doc = "Parsing of CTEInfo is disabled"]
    DISABLED = 0x0,
    #[doc = "Parsing of CTEInfo is enabled"]
    ENABLED = 0x01,
}
impl Cteinlinectrlen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cteinlinectrlen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cteinlinectrlen {
    #[inline(always)]
    fn from(val: u8) -> Cteinlinectrlen {
        Cteinlinectrlen::from_bits(val)
    }
}
impl From<Cteinlinectrlen> for u8 {
    #[inline(always)]
    fn from(val: Cteinlinectrlen) -> u8 {
        Cteinlinectrlen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cteinlinerxmode1us {
    _RESERVED_0 = 0x0,
    #[doc = "4 us"]
    _4US = 0x01,
    #[doc = "2 us"]
    _2US = 0x02,
    #[doc = "1 us"]
    _1US = 0x03,
    #[doc = "0.5 us"]
    _500NS = 0x04,
    #[doc = "0.25 us"]
    _250NS = 0x05,
    #[doc = "0.125 us"]
    _125NS = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cteinlinerxmode1us {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cteinlinerxmode1us {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cteinlinerxmode1us {
    #[inline(always)]
    fn from(val: u8) -> Cteinlinerxmode1us {
        Cteinlinerxmode1us::from_bits(val)
    }
}
impl From<Cteinlinerxmode1us> for u8 {
    #[inline(always)]
    fn from(val: Cteinlinerxmode1us) -> u8 {
        Cteinlinerxmode1us::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cteinlinerxmode2us {
    _RESERVED_0 = 0x0,
    #[doc = "4 us"]
    _4US = 0x01,
    #[doc = "2 us"]
    _2US = 0x02,
    #[doc = "1 us"]
    _1US = 0x03,
    #[doc = "0.5 us"]
    _500NS = 0x04,
    #[doc = "0.25 us"]
    _250NS = 0x05,
    #[doc = "0.125 us"]
    _125NS = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cteinlinerxmode2us {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cteinlinerxmode2us {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cteinlinerxmode2us {
    #[inline(always)]
    fn from(val: u8) -> Cteinlinerxmode2us {
        Cteinlinerxmode2us::from_bits(val)
    }
}
impl From<Cteinlinerxmode2us> for u8 {
    #[inline(always)]
    fn from(val: Cteinlinerxmode2us) -> u8 {
        Cteinlinerxmode2us::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctetimevalidrange {
    #[doc = "20 in 8 us unit (default) Set to 20 if parsed CTETime is larger than 20"]
    _20 = 0x0,
    #[doc = "31 in 8 us unit"]
    _31 = 0x01,
    #[doc = "63 in 8 us unit"]
    _63 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ctetimevalidrange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctetimevalidrange {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctetimevalidrange {
    #[inline(always)]
    fn from(val: u8) -> Ctetimevalidrange {
        Ctetimevalidrange::from_bits(val)
    }
}
impl From<Ctetimevalidrange> for u8 {
    #[inline(always)]
    fn from(val: Ctetimevalidrange) -> u8 {
        Ctetimevalidrange::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dfeinextension {
    #[doc = "Antenna switching/sampling is done in the packet payload"]
    PAYLOAD = 0x0,
    #[doc = "AoA/AoD procedure triggered at end of CRC"]
    CRC = 0x01,
}
impl Dfeinextension {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfeinextension {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfeinextension {
    #[inline(always)]
    fn from(val: u8) -> Dfeinextension {
        Dfeinextension::from_bits(val)
    }
}
impl From<Dfeinextension> for u8 {
    #[inline(always)]
    fn from(val: Dfeinextension) -> u8 {
        Dfeinextension::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dfeopmode {
    #[doc = "Direction finding mode disabled"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Direction finding mode set to AoD"]
    AOD = 0x02,
    #[doc = "Direction finding mode set to AoA"]
    AOA = 0x03,
}
impl Dfeopmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfeopmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfeopmode {
    #[inline(always)]
    fn from(val: u8) -> Dfeopmode {
        Dfeopmode::from_bits(val)
    }
}
impl From<Dfeopmode> for u8 {
    #[inline(always)]
    fn from(val: Dfeopmode) -> u8 {
        Dfeopmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dtx {
    #[doc = "Transmit '1'"]
    B1 = 0x0,
    #[doc = "Transmit '0'"]
    B0 = 0x01,
    #[doc = "Transmit center frequency"]
    CENTER = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dtx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtx {
    #[inline(always)]
    fn from(val: u8) -> Dtx {
        Dtx::from_bits(val)
    }
}
impl From<Dtx> for u8 {
    #[inline(always)]
    fn from(val: Dtx) -> u8 {
        Dtx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ena0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Ena0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena0 {
    #[inline(always)]
    fn from(val: u8) -> Ena0 {
        Ena0::from_bits(val)
    }
}
impl From<Ena0> for u8 {
    #[inline(always)]
    fn from(val: Ena0) -> u8 {
        Ena0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ena1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Ena1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena1 {
    #[inline(always)]
    fn from(val: u8) -> Ena1 {
        Ena1::from_bits(val)
    }
}
impl From<Ena1> for u8 {
    #[inline(always)]
    fn from(val: Ena1) -> u8 {
        Ena1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ena2 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Ena2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena2 {
    #[inline(always)]
    fn from(val: u8) -> Ena2 {
        Ena2::from_bits(val)
    }
}
impl From<Ena2> for u8 {
    #[inline(always)]
    fn from(val: Ena2) -> u8 {
        Ena2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ena3 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Ena3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena3 {
    #[inline(always)]
    fn from(val: u8) -> Ena3 {
        Ena3::from_bits(val)
    }
}
impl From<Ena3> for u8 {
    #[inline(always)]
    fn from(val: Ena3) -> u8 {
        Ena3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ena4 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Ena4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena4 {
    #[inline(always)]
    fn from(val: u8) -> Ena4 {
        Ena4::from_bits(val)
    }
}
impl From<Ena4> for u8 {
    #[inline(always)]
    fn from(val: Ena4) -> u8 {
        Ena4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ena5 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Ena5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena5 {
    #[inline(always)]
    fn from(val: u8) -> Ena5 {
        Ena5::from_bits(val)
    }
}
impl From<Ena5> for u8 {
    #[inline(always)]
    fn from(val: Ena5) -> u8 {
        Ena5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ena6 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Ena6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena6 {
    #[inline(always)]
    fn from(val: u8) -> Ena6 {
        Ena6::from_bits(val)
    }
}
impl From<Ena6> for u8 {
    #[inline(always)]
    fn from(val: Ena6) -> u8 {
        Ena6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ena7 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Ena7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena7 {
    #[inline(always)]
    fn from(val: u8) -> Ena7 {
        Ena7::from_bits(val)
    }
}
impl From<Ena7> for u8 {
    #[inline(always)]
    fn from(val: Ena7) -> u8 {
        Ena7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Endian {
    #[doc = "Least significant bit on air first"]
    LITTLE = 0x0,
    #[doc = "Most significant bit on air first"]
    BIG = 0x01,
}
impl Endian {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endian {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endian {
    #[inline(always)]
    fn from(val: u8) -> Endian {
        Endian::from_bits(val)
    }
}
impl From<Endian> for u8 {
    #[inline(always)]
    fn from(val: Endian) -> u8 {
        Endian::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsAddress {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsAddress {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsAddress {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsAddress {
    #[inline(always)]
    fn from(val: u8) -> EventsAddress {
        EventsAddress::from_bits(val)
    }
}
impl From<EventsAddress> for u8 {
    #[inline(always)]
    fn from(val: EventsAddress) -> u8 {
        EventsAddress::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsBcmatch {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsBcmatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsBcmatch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsBcmatch {
    #[inline(always)]
    fn from(val: u8) -> EventsBcmatch {
        EventsBcmatch::from_bits(val)
    }
}
impl From<EventsBcmatch> for u8 {
    #[inline(always)]
    fn from(val: EventsBcmatch) -> u8 {
        EventsBcmatch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCcabusy {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCcabusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCcabusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCcabusy {
    #[inline(always)]
    fn from(val: u8) -> EventsCcabusy {
        EventsCcabusy::from_bits(val)
    }
}
impl From<EventsCcabusy> for u8 {
    #[inline(always)]
    fn from(val: EventsCcabusy) -> u8 {
        EventsCcabusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCcaidle {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCcaidle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCcaidle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCcaidle {
    #[inline(always)]
    fn from(val: u8) -> EventsCcaidle {
        EventsCcaidle::from_bits(val)
    }
}
impl From<EventsCcaidle> for u8 {
    #[inline(always)]
    fn from(val: EventsCcaidle) -> u8 {
        EventsCcaidle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCcastopped {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCcastopped {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCcastopped {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCcastopped {
    #[inline(always)]
    fn from(val: u8) -> EventsCcastopped {
        EventsCcastopped::from_bits(val)
    }
}
impl From<EventsCcastopped> for u8 {
    #[inline(always)]
    fn from(val: EventsCcastopped) -> u8 {
        EventsCcastopped::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCrcerror {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCrcerror {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCrcerror {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCrcerror {
    #[inline(always)]
    fn from(val: u8) -> EventsCrcerror {
        EventsCrcerror::from_bits(val)
    }
}
impl From<EventsCrcerror> for u8 {
    #[inline(always)]
    fn from(val: EventsCrcerror) -> u8 {
        EventsCrcerror::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCrcok {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCrcok {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCrcok {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCrcok {
    #[inline(always)]
    fn from(val: u8) -> EventsCrcok {
        EventsCrcok::from_bits(val)
    }
}
impl From<EventsCrcok> for u8 {
    #[inline(always)]
    fn from(val: EventsCrcok) -> u8 {
        EventsCrcok::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsCtepresent {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsCtepresent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsCtepresent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsCtepresent {
    #[inline(always)]
    fn from(val: u8) -> EventsCtepresent {
        EventsCtepresent::from_bits(val)
    }
}
impl From<EventsCtepresent> for u8 {
    #[inline(always)]
    fn from(val: EventsCtepresent) -> u8 {
        EventsCtepresent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsDevmatch {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsDevmatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsDevmatch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsDevmatch {
    #[inline(always)]
    fn from(val: u8) -> EventsDevmatch {
        EventsDevmatch::from_bits(val)
    }
}
impl From<EventsDevmatch> for u8 {
    #[inline(always)]
    fn from(val: EventsDevmatch) -> u8 {
        EventsDevmatch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsDevmiss {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsDevmiss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsDevmiss {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsDevmiss {
    #[inline(always)]
    fn from(val: u8) -> EventsDevmiss {
        EventsDevmiss::from_bits(val)
    }
}
impl From<EventsDevmiss> for u8 {
    #[inline(always)]
    fn from(val: EventsDevmiss) -> u8 {
        EventsDevmiss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsDisabled {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsDisabled {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsDisabled {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsDisabled {
    #[inline(always)]
    fn from(val: u8) -> EventsDisabled {
        EventsDisabled::from_bits(val)
    }
}
impl From<EventsDisabled> for u8 {
    #[inline(always)]
    fn from(val: EventsDisabled) -> u8 {
        EventsDisabled::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEdend {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEdend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEdend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEdend {
    #[inline(always)]
    fn from(val: u8) -> EventsEdend {
        EventsEdend::from_bits(val)
    }
}
impl From<EventsEdend> for u8 {
    #[inline(always)]
    fn from(val: EventsEdend) -> u8 {
        EventsEdend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEdstopped {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEdstopped {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEdstopped {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEdstopped {
    #[inline(always)]
    fn from(val: u8) -> EventsEdstopped {
        EventsEdstopped::from_bits(val)
    }
}
impl From<EventsEdstopped> for u8 {
    #[inline(always)]
    fn from(val: EventsEdstopped) -> u8 {
        EventsEdstopped::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEnd {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEnd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEnd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEnd {
    #[inline(always)]
    fn from(val: u8) -> EventsEnd {
        EventsEnd::from_bits(val)
    }
}
impl From<EventsEnd> for u8 {
    #[inline(always)]
    fn from(val: EventsEnd) -> u8 {
        EventsEnd::to_bits(val)
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
pub enum EventsMhrmatch {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsMhrmatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsMhrmatch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsMhrmatch {
    #[inline(always)]
    fn from(val: u8) -> EventsMhrmatch {
        EventsMhrmatch::from_bits(val)
    }
}
impl From<EventsMhrmatch> for u8 {
    #[inline(always)]
    fn from(val: EventsMhrmatch) -> u8 {
        EventsMhrmatch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsPayload {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsPayload {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsPayload {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsPayload {
    #[inline(always)]
    fn from(val: u8) -> EventsPayload {
        EventsPayload::from_bits(val)
    }
}
impl From<EventsPayload> for u8 {
    #[inline(always)]
    fn from(val: EventsPayload) -> u8 {
        EventsPayload::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsPhyend {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsPhyend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsPhyend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsPhyend {
    #[inline(always)]
    fn from(val: u8) -> EventsPhyend {
        EventsPhyend::from_bits(val)
    }
}
impl From<EventsPhyend> for u8 {
    #[inline(always)]
    fn from(val: EventsPhyend) -> u8 {
        EventsPhyend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsRateboost {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRateboost {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRateboost {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRateboost {
    #[inline(always)]
    fn from(val: u8) -> EventsRateboost {
        EventsRateboost::from_bits(val)
    }
}
impl From<EventsRateboost> for u8 {
    #[inline(always)]
    fn from(val: EventsRateboost) -> u8 {
        EventsRateboost::to_bits(val)
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
pub enum EventsRssiend {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRssiend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRssiend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRssiend {
    #[inline(always)]
    fn from(val: u8) -> EventsRssiend {
        EventsRssiend::from_bits(val)
    }
}
impl From<EventsRssiend> for u8 {
    #[inline(always)]
    fn from(val: EventsRssiend) -> u8 {
        EventsRssiend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsRxready {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsRxready {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsRxready {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsRxready {
    #[inline(always)]
    fn from(val: u8) -> EventsRxready {
        EventsRxready::from_bits(val)
    }
}
impl From<EventsRxready> for u8 {
    #[inline(always)]
    fn from(val: EventsRxready) -> u8 {
        EventsRxready::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsSync {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsSync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsSync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsSync {
    #[inline(always)]
    fn from(val: u8) -> EventsSync {
        EventsSync::from_bits(val)
    }
}
impl From<EventsSync> for u8 {
    #[inline(always)]
    fn from(val: EventsSync) -> u8 {
        EventsSync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsTxready {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsTxready {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsTxready {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsTxready {
    #[inline(always)]
    fn from(val: u8) -> EventsTxready {
        EventsTxready::from_bits(val)
    }
}
impl From<EventsTxready> for u8 {
    #[inline(always)]
    fn from(val: EventsTxready) -> u8 {
        EventsTxready::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Len {
    #[doc = "CRC length is zero and CRC calculation is disabled"]
    DISABLED = 0x0,
    #[doc = "CRC length is one byte and CRC calculation is enabled"]
    ONE = 0x01,
    #[doc = "CRC length is two bytes and CRC calculation is enabled"]
    TWO = 0x02,
    #[doc = "CRC length is three bytes and CRC calculation is enabled"]
    THREE = 0x03,
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
pub enum Map {
    #[doc = "Channel map between 2400 MHz and 2500 MHz"]
    DEFAULT = 0x0,
    #[doc = "Channel map between 2360 MHz and 2460 MHz"]
    LOW = 0x01,
}
impl Map {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Map {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Map {
    #[inline(always)]
    fn from(val: u8) -> Map {
        Map::from_bits(val)
    }
}
impl From<Map> for u8 {
    #[inline(always)]
    fn from(val: Map) -> u8 {
        Map::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "1 Mbps Nordic proprietary radio mode"]
    NRF_1MBIT = 0x0,
    #[doc = "2 Mbps Nordic proprietary radio mode"]
    NRF_2MBIT = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "1 Mbps BLE"]
    BLE_1MBIT = 0x03,
    #[doc = "2 Mbps BLE"]
    BLE_2MBIT = 0x04,
    #[doc = "Long Range 125 kbps TX, 125 kbps and 500 kbps RX"]
    BLE_LR125KBIT = 0x05,
    #[doc = "Long Range 500 kbps TX, 125 kbps and 500 kbps RX"]
    BLE_LR500KBIT = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "IEEE 802.15.4-2006 250 kbps"]
    IEEE802154_250KBIT = 0x0f,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Pdustat {
    #[doc = "Payload less than PCNF1.MAXLEN"]
    LESSTHAN = 0x0,
    #[doc = "Payload greater than PCNF1.MAXLEN"]
    GREATERTHAN = 0x01,
}
impl Pdustat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdustat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdustat {
    #[inline(always)]
    fn from(val: u8) -> Pdustat {
        Pdustat::from_bits(val)
    }
}
impl From<Pdustat> for u8 {
    #[inline(always)]
    fn from(val: Pdustat) -> u8 {
        Pdustat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Plen {
    #[doc = "8-bit preamble"]
    _8BIT = 0x0,
    #[doc = "16-bit preamble"]
    _16BIT = 0x01,
    #[doc = "32-bit zero preamble - used for IEEE 802.15.4"]
    _32BITZERO = 0x02,
    #[doc = "Preamble - used for Bluetooth LE Long Range"]
    LONGRANGE = 0x03,
}
impl Plen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plen {
    #[inline(always)]
    fn from(val: u8) -> Plen {
        Plen::from_bits(val)
    }
}
impl From<Plen> for u8 {
    #[inline(always)]
    fn from(val: Plen) -> u8 {
        Plen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Power {
    #[doc = "Peripheral is powered off"]
    DISABLED = 0x0,
    #[doc = "Peripheral is powered on"]
    ENABLED = 0x01,
}
impl Power {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Power {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Power {
    #[inline(always)]
    fn from(val: u8) -> Power {
        Power::from_bits(val)
    }
}
impl From<Power> for u8 {
    #[inline(always)]
    fn from(val: Power) -> u8 {
        Power::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishAddressEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishAddressEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishAddressEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishAddressEn {
    #[inline(always)]
    fn from(val: u8) -> PublishAddressEn {
        PublishAddressEn::from_bits(val)
    }
}
impl From<PublishAddressEn> for u8 {
    #[inline(always)]
    fn from(val: PublishAddressEn) -> u8 {
        PublishAddressEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishBcmatchEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishBcmatchEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishBcmatchEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishBcmatchEn {
    #[inline(always)]
    fn from(val: u8) -> PublishBcmatchEn {
        PublishBcmatchEn::from_bits(val)
    }
}
impl From<PublishBcmatchEn> for u8 {
    #[inline(always)]
    fn from(val: PublishBcmatchEn) -> u8 {
        PublishBcmatchEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishCcabusyEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishCcabusyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishCcabusyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishCcabusyEn {
    #[inline(always)]
    fn from(val: u8) -> PublishCcabusyEn {
        PublishCcabusyEn::from_bits(val)
    }
}
impl From<PublishCcabusyEn> for u8 {
    #[inline(always)]
    fn from(val: PublishCcabusyEn) -> u8 {
        PublishCcabusyEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishCcaidleEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishCcaidleEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishCcaidleEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishCcaidleEn {
    #[inline(always)]
    fn from(val: u8) -> PublishCcaidleEn {
        PublishCcaidleEn::from_bits(val)
    }
}
impl From<PublishCcaidleEn> for u8 {
    #[inline(always)]
    fn from(val: PublishCcaidleEn) -> u8 {
        PublishCcaidleEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishCcastoppedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishCcastoppedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishCcastoppedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishCcastoppedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishCcastoppedEn {
        PublishCcastoppedEn::from_bits(val)
    }
}
impl From<PublishCcastoppedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishCcastoppedEn) -> u8 {
        PublishCcastoppedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishCrcerrorEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishCrcerrorEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishCrcerrorEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishCrcerrorEn {
    #[inline(always)]
    fn from(val: u8) -> PublishCrcerrorEn {
        PublishCrcerrorEn::from_bits(val)
    }
}
impl From<PublishCrcerrorEn> for u8 {
    #[inline(always)]
    fn from(val: PublishCrcerrorEn) -> u8 {
        PublishCrcerrorEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishCrcokEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishCrcokEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishCrcokEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishCrcokEn {
    #[inline(always)]
    fn from(val: u8) -> PublishCrcokEn {
        PublishCrcokEn::from_bits(val)
    }
}
impl From<PublishCrcokEn> for u8 {
    #[inline(always)]
    fn from(val: PublishCrcokEn) -> u8 {
        PublishCrcokEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishCtepresentEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishCtepresentEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishCtepresentEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishCtepresentEn {
    #[inline(always)]
    fn from(val: u8) -> PublishCtepresentEn {
        PublishCtepresentEn::from_bits(val)
    }
}
impl From<PublishCtepresentEn> for u8 {
    #[inline(always)]
    fn from(val: PublishCtepresentEn) -> u8 {
        PublishCtepresentEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishDevmatchEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishDevmatchEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishDevmatchEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishDevmatchEn {
    #[inline(always)]
    fn from(val: u8) -> PublishDevmatchEn {
        PublishDevmatchEn::from_bits(val)
    }
}
impl From<PublishDevmatchEn> for u8 {
    #[inline(always)]
    fn from(val: PublishDevmatchEn) -> u8 {
        PublishDevmatchEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishDevmissEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishDevmissEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishDevmissEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishDevmissEn {
    #[inline(always)]
    fn from(val: u8) -> PublishDevmissEn {
        PublishDevmissEn::from_bits(val)
    }
}
impl From<PublishDevmissEn> for u8 {
    #[inline(always)]
    fn from(val: PublishDevmissEn) -> u8 {
        PublishDevmissEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishDisabledEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishDisabledEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishDisabledEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishDisabledEn {
    #[inline(always)]
    fn from(val: u8) -> PublishDisabledEn {
        PublishDisabledEn::from_bits(val)
    }
}
impl From<PublishDisabledEn> for u8 {
    #[inline(always)]
    fn from(val: PublishDisabledEn) -> u8 {
        PublishDisabledEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishEdendEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishEdendEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishEdendEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishEdendEn {
    #[inline(always)]
    fn from(val: u8) -> PublishEdendEn {
        PublishEdendEn::from_bits(val)
    }
}
impl From<PublishEdendEn> for u8 {
    #[inline(always)]
    fn from(val: PublishEdendEn) -> u8 {
        PublishEdendEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishEdstoppedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishEdstoppedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishEdstoppedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishEdstoppedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishEdstoppedEn {
        PublishEdstoppedEn::from_bits(val)
    }
}
impl From<PublishEdstoppedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishEdstoppedEn) -> u8 {
        PublishEdstoppedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishEndEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishEndEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishEndEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishEndEn {
    #[inline(always)]
    fn from(val: u8) -> PublishEndEn {
        PublishEndEn::from_bits(val)
    }
}
impl From<PublishEndEn> for u8 {
    #[inline(always)]
    fn from(val: PublishEndEn) -> u8 {
        PublishEndEn::to_bits(val)
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
pub enum PublishMhrmatchEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishMhrmatchEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishMhrmatchEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishMhrmatchEn {
    #[inline(always)]
    fn from(val: u8) -> PublishMhrmatchEn {
        PublishMhrmatchEn::from_bits(val)
    }
}
impl From<PublishMhrmatchEn> for u8 {
    #[inline(always)]
    fn from(val: PublishMhrmatchEn) -> u8 {
        PublishMhrmatchEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishPayloadEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishPayloadEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishPayloadEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishPayloadEn {
    #[inline(always)]
    fn from(val: u8) -> PublishPayloadEn {
        PublishPayloadEn::from_bits(val)
    }
}
impl From<PublishPayloadEn> for u8 {
    #[inline(always)]
    fn from(val: PublishPayloadEn) -> u8 {
        PublishPayloadEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishPhyendEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishPhyendEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishPhyendEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishPhyendEn {
    #[inline(always)]
    fn from(val: u8) -> PublishPhyendEn {
        PublishPhyendEn::from_bits(val)
    }
}
impl From<PublishPhyendEn> for u8 {
    #[inline(always)]
    fn from(val: PublishPhyendEn) -> u8 {
        PublishPhyendEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishRateboostEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishRateboostEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishRateboostEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishRateboostEn {
    #[inline(always)]
    fn from(val: u8) -> PublishRateboostEn {
        PublishRateboostEn::from_bits(val)
    }
}
impl From<PublishRateboostEn> for u8 {
    #[inline(always)]
    fn from(val: PublishRateboostEn) -> u8 {
        PublishRateboostEn::to_bits(val)
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
pub enum PublishRssiendEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishRssiendEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishRssiendEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishRssiendEn {
    #[inline(always)]
    fn from(val: u8) -> PublishRssiendEn {
        PublishRssiendEn::from_bits(val)
    }
}
impl From<PublishRssiendEn> for u8 {
    #[inline(always)]
    fn from(val: PublishRssiendEn) -> u8 {
        PublishRssiendEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishRxreadyEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishRxreadyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishRxreadyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishRxreadyEn {
    #[inline(always)]
    fn from(val: u8) -> PublishRxreadyEn {
        PublishRxreadyEn::from_bits(val)
    }
}
impl From<PublishRxreadyEn> for u8 {
    #[inline(always)]
    fn from(val: PublishRxreadyEn) -> u8 {
        PublishRxreadyEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishSyncEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishSyncEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishSyncEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishSyncEn {
    #[inline(always)]
    fn from(val: u8) -> PublishSyncEn {
        PublishSyncEn::from_bits(val)
    }
}
impl From<PublishSyncEn> for u8 {
    #[inline(always)]
    fn from(val: PublishSyncEn) -> u8 {
        PublishSyncEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishTxreadyEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishTxreadyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishTxreadyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishTxreadyEn {
    #[inline(always)]
    fn from(val: u8) -> PublishTxreadyEn {
        PublishTxreadyEn::from_bits(val)
    }
}
impl From<PublishTxreadyEn> for u8 {
    #[inline(always)]
    fn from(val: PublishTxreadyEn) -> u8 {
        PublishTxreadyEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Repeatpattern {
    #[doc = "Do not repeat (1 time in total)"]
    NOREPEAT = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
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
impl Repeatpattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repeatpattern {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repeatpattern {
    #[inline(always)]
    fn from(val: u8) -> Repeatpattern {
        Repeatpattern::from_bits(val)
    }
}
impl From<Repeatpattern> for u8 {
    #[inline(always)]
    fn from(val: Repeatpattern) -> u8 {
        Repeatpattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ru {
    #[doc = "Default ramp-up time (tRXEN and tTXEN), compatible with firmware written for nRF51"]
    DEFAULT = 0x0,
    #[doc = "Fast ramp-up (tRXEN,FAST and tTXEN,FAST), see electrical specifications for more information"]
    FAST = 0x01,
}
impl Ru {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ru {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ru {
    #[inline(always)]
    fn from(val: u8) -> Ru {
        Ru::from_bits(val)
    }
}
impl From<Ru> for u8 {
    #[inline(always)]
    fn from(val: Ru) -> u8 {
        Ru::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum S1incl {
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    AUTOMATIC = 0x0,
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    INCLUDE = 0x01,
}
impl S1incl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S1incl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S1incl {
    #[inline(always)]
    fn from(val: u8) -> S1incl {
        S1incl::from_bits(val)
    }
}
impl From<S1incl> for u8 {
    #[inline(always)]
    fn from(val: S1incl) -> u8 {
        S1incl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sampletype {
    #[doc = "Complex samples in I and Q"]
    IQ = 0x0,
    #[doc = "Complex samples as magnitude and phase"]
    MAGPHASE = 0x01,
}
impl Sampletype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sampletype {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sampletype {
    #[inline(always)]
    fn from(val: u8) -> Sampletype {
        Sampletype::from_bits(val)
    }
}
impl From<Sampletype> for u8 {
    #[inline(always)]
    fn from(val: Sampletype) -> u8 {
        Sampletype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Samplingstate {
    #[doc = "Sampling state Idle"]
    IDLE = 0x0,
    #[doc = "Sampling state Sampling"]
    SAMPLING = 0x01,
}
impl Samplingstate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Samplingstate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Samplingstate {
    #[inline(always)]
    fn from(val: u8) -> Samplingstate {
        Samplingstate::from_bits(val)
    }
}
impl From<Samplingstate> for u8 {
    #[inline(always)]
    fn from(val: Samplingstate) -> u8 {
        Samplingstate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Skipaddr {
    #[doc = "CRC calculation includes address field"]
    INCLUDE = 0x0,
    #[doc = "CRC calculation does not include address field. The CRC calculation will start at the first byte after the address."]
    SKIP = 0x01,
    #[doc = "CRC calculation as per 802.15.4 standard. Starting at first byte after length field."]
    IEEE802154 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Skipaddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Skipaddr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Skipaddr {
    #[inline(always)]
    fn from(val: u8) -> Skipaddr {
        Skipaddr::from_bits(val)
    }
}
impl From<Skipaddr> for u8 {
    #[inline(always)]
    fn from(val: Skipaddr) -> u8 {
        Skipaddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum State {
    #[doc = "RADIO is in the Disabled state"]
    DISABLED = 0x0,
    #[doc = "RADIO is in the RXRU state"]
    RXRU = 0x01,
    #[doc = "RADIO is in the RXIDLE state"]
    RXIDLE = 0x02,
    #[doc = "RADIO is in the RX state"]
    RX = 0x03,
    #[doc = "RADIO is in the RXDISABLED state"]
    RXDISABLE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "RADIO is in the TXRU state"]
    TXRU = 0x09,
    #[doc = "RADIO is in the TXIDLE state"]
    TXIDLE = 0x0a,
    #[doc = "RADIO is in the TX state"]
    TX = 0x0b,
    #[doc = "RADIO is in the TXDISABLED state"]
    TXDISABLE = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl State {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> State {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for State {
    #[inline(always)]
    fn from(val: u8) -> State {
        State::from_bits(val)
    }
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(val: State) -> u8 {
        State::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeBcstartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeBcstartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeBcstartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeBcstartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeBcstartEn {
        SubscribeBcstartEn::from_bits(val)
    }
}
impl From<SubscribeBcstartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeBcstartEn) -> u8 {
        SubscribeBcstartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeBcstopEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeBcstopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeBcstopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeBcstopEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeBcstopEn {
        SubscribeBcstopEn::from_bits(val)
    }
}
impl From<SubscribeBcstopEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeBcstopEn) -> u8 {
        SubscribeBcstopEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeCcastartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeCcastartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeCcastartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeCcastartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeCcastartEn {
        SubscribeCcastartEn::from_bits(val)
    }
}
impl From<SubscribeCcastartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeCcastartEn) -> u8 {
        SubscribeCcastartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeCcastopEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeCcastopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeCcastopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeCcastopEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeCcastopEn {
        SubscribeCcastopEn::from_bits(val)
    }
}
impl From<SubscribeCcastopEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeCcastopEn) -> u8 {
        SubscribeCcastopEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeDisableEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeDisableEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeDisableEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeDisableEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeDisableEn {
        SubscribeDisableEn::from_bits(val)
    }
}
impl From<SubscribeDisableEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeDisableEn) -> u8 {
        SubscribeDisableEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeEdstartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeEdstartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeEdstartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeEdstartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeEdstartEn {
        SubscribeEdstartEn::from_bits(val)
    }
}
impl From<SubscribeEdstartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeEdstartEn) -> u8 {
        SubscribeEdstartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeEdstopEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeEdstopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeEdstopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeEdstopEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeEdstopEn {
        SubscribeEdstopEn::from_bits(val)
    }
}
impl From<SubscribeEdstopEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeEdstopEn) -> u8 {
        SubscribeEdstopEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeRssistartEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeRssistartEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeRssistartEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeRssistartEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeRssistartEn {
        SubscribeRssistartEn::from_bits(val)
    }
}
impl From<SubscribeRssistartEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeRssistartEn) -> u8 {
        SubscribeRssistartEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeRssistopEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeRssistopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeRssistopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeRssistopEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeRssistopEn {
        SubscribeRssistopEn::from_bits(val)
    }
}
impl From<SubscribeRssistopEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeRssistopEn) -> u8 {
        SubscribeRssistopEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeRxenEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeRxenEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeRxenEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeRxenEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeRxenEn {
        SubscribeRxenEn::from_bits(val)
    }
}
impl From<SubscribeRxenEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeRxenEn) -> u8 {
        SubscribeRxenEn::to_bits(val)
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
pub enum SubscribeTxenEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeTxenEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeTxenEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeTxenEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeTxenEn {
        SubscribeTxenEn::from_bits(val)
    }
}
impl From<SubscribeTxenEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeTxenEn) -> u8 {
        SubscribeTxenEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Switchingstate {
    #[doc = "Switching state Idle"]
    IDLE = 0x0,
    #[doc = "Switching state Offset"]
    OFFSET = 0x01,
    #[doc = "Switching state Guard"]
    GUARD = 0x02,
    #[doc = "Switching state Ref"]
    REF = 0x03,
    #[doc = "Switching state Switching"]
    SWITCHING = 0x04,
    #[doc = "Switching state Ending"]
    ENDING = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Switchingstate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Switchingstate {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Switchingstate {
    #[inline(always)]
    fn from(val: u8) -> Switchingstate {
        Switchingstate::from_bits(val)
    }
}
impl From<Switchingstate> for u8 {
    #[inline(always)]
    fn from(val: Switchingstate) -> u8 {
        Switchingstate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksBcstart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksBcstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksBcstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksBcstart {
    #[inline(always)]
    fn from(val: u8) -> TasksBcstart {
        TasksBcstart::from_bits(val)
    }
}
impl From<TasksBcstart> for u8 {
    #[inline(always)]
    fn from(val: TasksBcstart) -> u8 {
        TasksBcstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksBcstop {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksBcstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksBcstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksBcstop {
    #[inline(always)]
    fn from(val: u8) -> TasksBcstop {
        TasksBcstop::from_bits(val)
    }
}
impl From<TasksBcstop> for u8 {
    #[inline(always)]
    fn from(val: TasksBcstop) -> u8 {
        TasksBcstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksCcastart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksCcastart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksCcastart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksCcastart {
    #[inline(always)]
    fn from(val: u8) -> TasksCcastart {
        TasksCcastart::from_bits(val)
    }
}
impl From<TasksCcastart> for u8 {
    #[inline(always)]
    fn from(val: TasksCcastart) -> u8 {
        TasksCcastart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksCcastop {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksCcastop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksCcastop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksCcastop {
    #[inline(always)]
    fn from(val: u8) -> TasksCcastop {
        TasksCcastop::from_bits(val)
    }
}
impl From<TasksCcastop> for u8 {
    #[inline(always)]
    fn from(val: TasksCcastop) -> u8 {
        TasksCcastop::to_bits(val)
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
pub enum TasksEdstart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksEdstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksEdstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksEdstart {
    #[inline(always)]
    fn from(val: u8) -> TasksEdstart {
        TasksEdstart::from_bits(val)
    }
}
impl From<TasksEdstart> for u8 {
    #[inline(always)]
    fn from(val: TasksEdstart) -> u8 {
        TasksEdstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksEdstop {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksEdstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksEdstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksEdstop {
    #[inline(always)]
    fn from(val: u8) -> TasksEdstop {
        TasksEdstop::from_bits(val)
    }
}
impl From<TasksEdstop> for u8 {
    #[inline(always)]
    fn from(val: TasksEdstop) -> u8 {
        TasksEdstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksRssistart {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksRssistart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksRssistart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksRssistart {
    #[inline(always)]
    fn from(val: u8) -> TasksRssistart {
        TasksRssistart::from_bits(val)
    }
}
impl From<TasksRssistart> for u8 {
    #[inline(always)]
    fn from(val: TasksRssistart) -> u8 {
        TasksRssistart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksRssistop {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksRssistop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksRssistop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksRssistop {
    #[inline(always)]
    fn from(val: u8) -> TasksRssistop {
        TasksRssistop::from_bits(val)
    }
}
impl From<TasksRssistop> for u8 {
    #[inline(always)]
    fn from(val: TasksRssistop) -> u8 {
        TasksRssistop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksRxen {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksRxen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksRxen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksRxen {
    #[inline(always)]
    fn from(val: u8) -> TasksRxen {
        TasksRxen::from_bits(val)
    }
}
impl From<TasksRxen> for u8 {
    #[inline(always)]
    fn from(val: TasksRxen) -> u8 {
        TasksRxen::to_bits(val)
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
pub enum TasksTxen {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksTxen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksTxen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksTxen {
    #[inline(always)]
    fn from(val: u8) -> TasksTxen {
        TasksTxen::from_bits(val)
    }
}
impl From<TasksTxen> for u8 {
    #[inline(always)]
    fn from(val: TasksTxen) -> u8 {
        TasksTxen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tsamplespacing {
    _RESERVED_0 = 0x0,
    #[doc = "4 us"]
    _4US = 0x01,
    #[doc = "2 us"]
    _2US = 0x02,
    #[doc = "1 us"]
    _1US = 0x03,
    #[doc = "0.5 us"]
    _500NS = 0x04,
    #[doc = "0.25 us"]
    _250NS = 0x05,
    #[doc = "0.125 us"]
    _125NS = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tsamplespacing {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsamplespacing {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsamplespacing {
    #[inline(always)]
    fn from(val: u8) -> Tsamplespacing {
        Tsamplespacing::from_bits(val)
    }
}
impl From<Tsamplespacing> for u8 {
    #[inline(always)]
    fn from(val: Tsamplespacing) -> u8 {
        Tsamplespacing::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tsamplespacingref {
    _RESERVED_0 = 0x0,
    #[doc = "4 us"]
    _4US = 0x01,
    #[doc = "2 us"]
    _2US = 0x02,
    #[doc = "1 us"]
    _1US = 0x03,
    #[doc = "0.5 us"]
    _500NS = 0x04,
    #[doc = "0.25 us"]
    _250NS = 0x05,
    #[doc = "0.125 us"]
    _125NS = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tsamplespacingref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsamplespacingref {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsamplespacingref {
    #[inline(always)]
    fn from(val: u8) -> Tsamplespacingref {
        Tsamplespacingref::from_bits(val)
    }
}
impl From<Tsamplespacingref> for u8 {
    #[inline(always)]
    fn from(val: Tsamplespacingref) -> u8 {
        Tsamplespacingref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tswitchspacing {
    _RESERVED_0 = 0x0,
    #[doc = "4 us"]
    _4US = 0x01,
    #[doc = "2 us"]
    _2US = 0x02,
    #[doc = "1 us"]
    _1US = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Tswitchspacing {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tswitchspacing {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tswitchspacing {
    #[inline(always)]
    fn from(val: u8) -> Tswitchspacing {
        Tswitchspacing::from_bits(val)
    }
}
impl From<Tswitchspacing> for u8 {
    #[inline(always)]
    fn from(val: Tswitchspacing) -> u8 {
        Tswitchspacing::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Txpower {
    #[doc = "0 dBm"]
    _0DBM = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
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
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    #[doc = "-40 dBm"]
    NEG40DBM = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    #[doc = "Deprecated enumerator - -40 dBm"]
    NEG30DBM = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    #[doc = "-20 dBm"]
    NEG20DBM = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    #[doc = "-16 dBm"]
    NEG16DBM = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    #[doc = "-12 dBm"]
    NEG12DBM = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    #[doc = "-8 dBm"]
    NEG8DBM = 0xf8,
    #[doc = "-7 dBm"]
    NEG7DBM = 0xf9,
    #[doc = "-6 dBm"]
    NEG6DBM = 0xfa,
    #[doc = "-5 dBm"]
    NEG5DBM = 0xfb,
    #[doc = "-4 dBm"]
    NEG4DBM = 0xfc,
    #[doc = "-3 dBm"]
    NEG3DBM = 0xfd,
    #[doc = "-2 dBm"]
    NEG2DBM = 0xfe,
    #[doc = "-1 dBm"]
    NEG1DBM = 0xff,
}
impl Txpower {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txpower {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txpower {
    #[inline(always)]
    fn from(val: u8) -> Txpower {
        Txpower::from_bits(val)
    }
}
impl From<Txpower> for u8 {
    #[inline(always)]
    fn from(val: Txpower) -> u8 {
        Txpower::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Whiteen {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Whiteen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Whiteen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Whiteen {
    #[inline(always)]
    fn from(val: u8) -> Whiteen {
        Whiteen::from_bits(val)
    }
}
impl From<Whiteen> for u8 {
    #[inline(always)]
    fn from(val: Whiteen) -> u8 {
        Whiteen::to_bits(val)
    }
}
