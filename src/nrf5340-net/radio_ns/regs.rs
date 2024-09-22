#[doc = "Number of samples transferred in the last transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amount(pub u32);
impl Amount {
    #[doc = "Number of samples transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of samples transferred in the last transaction"]
    #[inline(always)]
    pub fn set_amount(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Amount {
    #[inline(always)]
    fn default() -> Amount {
        Amount(0)
    }
}
#[doc = "IEEE 802.15.4 clear channel assessment control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccactrl(pub u32);
impl Ccactrl {
    #[doc = "CCA mode of operation"]
    #[inline(always)]
    pub const fn ccamode(&self) -> super::vals::Ccamode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ccamode::from_bits(val as u8)
    }
    #[doc = "CCA mode of operation"]
    #[inline(always)]
    pub fn set_ccamode(&mut self, val: super::vals::Ccamode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline(always)]
    pub const fn ccaedthres(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline(always)]
    pub fn set_ccaedthres(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
    #[inline(always)]
    pub const fn ccacorrthres(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
    #[inline(always)]
    pub fn set_ccacorrthres(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline(always)]
    pub const fn ccacorrcnt(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline(always)]
    pub fn set_ccacorrcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ccactrl {
    #[inline(always)]
    fn default() -> Ccactrl {
        Ccactrl(0)
    }
}
#[doc = "Clear the GPIO pattern array for antenna control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clearpattern(pub u32);
impl Clearpattern {
    #[doc = "Clears GPIO pattern array for antenna control"]
    #[inline(always)]
    pub const fn clearpattern(&self) -> super::vals::Clearpattern {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Clearpattern::from_bits(val as u8)
    }
    #[doc = "Clears GPIO pattern array for antenna control"]
    #[inline(always)]
    pub fn set_clearpattern(&mut self, val: super::vals::Clearpattern) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Clearpattern {
    #[inline(always)]
    fn default() -> Clearpattern {
        Clearpattern(0)
    }
}
#[doc = "CRC configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crccnf(pub u32);
impl Crccnf {
    #[doc = "CRC length in number of bytes For MODE Ble_LR125Kbit and Ble_LR500Kbit, only LEN set to 3 is supported"]
    #[inline(always)]
    pub const fn len(&self) -> super::vals::Len {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Len::from_bits(val as u8)
    }
    #[doc = "CRC length in number of bytes For MODE Ble_LR125Kbit and Ble_LR500Kbit, only LEN set to 3 is supported"]
    #[inline(always)]
    pub fn set_len(&mut self, val: super::vals::Len) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Include or exclude packet address field out of CRC calculation."]
    #[inline(always)]
    pub const fn skipaddr(&self) -> super::vals::Skipaddr {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Skipaddr::from_bits(val as u8)
    }
    #[doc = "Include or exclude packet address field out of CRC calculation."]
    #[inline(always)]
    pub fn set_skipaddr(&mut self, val: super::vals::Skipaddr) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for Crccnf {
    #[inline(always)]
    fn default() -> Crccnf {
        Crccnf(0)
    }
}
#[doc = "CRC initial value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcinit(pub u32);
impl Crcinit {
    #[doc = "CRC initial value"]
    #[inline(always)]
    pub const fn crcinit(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CRC initial value"]
    #[inline(always)]
    pub fn set_crcinit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crcinit {
    #[inline(always)]
    fn default() -> Crcinit {
        Crcinit(0)
    }
}
#[doc = "CRC polynomial"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcpoly(pub u32);
impl Crcpoly {
    #[doc = "CRC polynomial"]
    #[inline(always)]
    pub const fn crcpoly(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CRC polynomial"]
    #[inline(always)]
    pub fn set_crcpoly(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crcpoly {
    #[inline(always)]
    fn default() -> Crcpoly {
        Crcpoly(0)
    }
}
#[doc = "CRC status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcstatus(pub u32);
impl Crcstatus {
    #[doc = "CRC status of packet received"]
    #[inline(always)]
    pub const fn crcstatus(&self) -> super::vals::Crcstatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Crcstatus::from_bits(val as u8)
    }
    #[doc = "CRC status of packet received"]
    #[inline(always)]
    pub fn set_crcstatus(&mut self, val: super::vals::Crcstatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Crcstatus {
    #[inline(always)]
    fn default() -> Crcstatus {
        Crcstatus(0)
    }
}
#[doc = "Configuration for CTE inline mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cteinlineconf(pub u32);
impl Cteinlineconf {
    #[doc = "Enable parsing of CTEInfo from received packet in BLE modes"]
    #[inline(always)]
    pub const fn cteinlinectrlen(&self) -> super::vals::Cteinlinectrlen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cteinlinectrlen::from_bits(val as u8)
    }
    #[doc = "Enable parsing of CTEInfo from received packet in BLE modes"]
    #[inline(always)]
    pub fn set_cteinlinectrlen(&mut self, val: super::vals::Cteinlinectrlen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CTEInfo is S1 byte or not"]
    #[inline(always)]
    pub const fn cteinfoins1(&self) -> super::vals::Cteinfoins1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cteinfoins1::from_bits(val as u8)
    }
    #[doc = "CTEInfo is S1 byte or not"]
    #[inline(always)]
    pub fn set_cteinfoins1(&mut self, val: super::vals::Cteinfoins1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Sampling/switching if CRC is not OK"]
    #[inline(always)]
    pub const fn cteerrorhandling(&self) -> super::vals::Cteerrorhandling {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Cteerrorhandling::from_bits(val as u8)
    }
    #[doc = "Sampling/switching if CRC is not OK"]
    #[inline(always)]
    pub fn set_cteerrorhandling(&mut self, val: super::vals::Cteerrorhandling) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Max range of CTETime"]
    #[inline(always)]
    pub const fn ctetimevalidrange(&self) -> super::vals::Ctetimevalidrange {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Ctetimevalidrange::from_bits(val as u8)
    }
    #[doc = "Max range of CTETime"]
    #[inline(always)]
    pub fn set_ctetimevalidrange(&mut self, val: super::vals::Ctetimevalidrange) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub const fn cteinlinerxmode1us(&self) -> super::vals::Cteinlinerxmode1us {
        let val = (self.0 >> 10usize) & 0x07;
        super::vals::Cteinlinerxmode1us::from_bits(val as u8)
    }
    #[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub fn set_cteinlinerxmode1us(&mut self, val: super::vals::Cteinlinerxmode1us) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
    }
    #[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub const fn cteinlinerxmode2us(&self) -> super::vals::Cteinlinerxmode2us {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::Cteinlinerxmode2us::from_bits(val as u8)
    }
    #[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub fn set_cteinlinerxmode2us(&mut self, val: super::vals::Cteinlinerxmode2us) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
    }
    #[doc = "S0 bit pattern to match"]
    #[inline(always)]
    pub const fn s0conf(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "S0 bit pattern to match"]
    #[inline(always)]
    pub fn set_s0conf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "S0 bit mask to set which bit to match"]
    #[inline(always)]
    pub const fn s0mask(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "S0 bit mask to set which bit to match"]
    #[inline(always)]
    pub fn set_s0mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cteinlineconf {
    #[inline(always)]
    fn default() -> Cteinlineconf {
        Cteinlineconf(0)
    }
}
#[doc = "CTEInfo parsed from received packet"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctestatus(pub u32);
impl Ctestatus {
    #[doc = "CTETime parsed from packet"]
    #[inline(always)]
    pub const fn ctetime(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "CTETime parsed from packet"]
    #[inline(always)]
    pub fn set_ctetime(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "RFU parsed from packet"]
    #[inline(always)]
    pub const fn rfu(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RFU parsed from packet"]
    #[inline(always)]
    pub fn set_rfu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTEType parsed from packet"]
    #[inline(always)]
    pub const fn ctetype(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "CTEType parsed from packet"]
    #[inline(always)]
    pub fn set_ctetype(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for Ctestatus {
    #[inline(always)]
    fn default() -> Ctestatus {
        Ctestatus(0)
    }
}
#[doc = "Device address match configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacnf(pub u32);
impl Dacnf {
    #[doc = "Enable or disable device address matching using device address 0"]
    #[inline(always)]
    pub const fn ena0(&self) -> super::vals::Ena0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ena0::from_bits(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 0"]
    #[inline(always)]
    pub fn set_ena0(&mut self, val: super::vals::Ena0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable device address matching using device address 1"]
    #[inline(always)]
    pub const fn ena1(&self) -> super::vals::Ena1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ena1::from_bits(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 1"]
    #[inline(always)]
    pub fn set_ena1(&mut self, val: super::vals::Ena1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable device address matching using device address 2"]
    #[inline(always)]
    pub const fn ena2(&self) -> super::vals::Ena2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ena2::from_bits(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 2"]
    #[inline(always)]
    pub fn set_ena2(&mut self, val: super::vals::Ena2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable or disable device address matching using device address 3"]
    #[inline(always)]
    pub const fn ena3(&self) -> super::vals::Ena3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ena3::from_bits(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 3"]
    #[inline(always)]
    pub fn set_ena3(&mut self, val: super::vals::Ena3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable or disable device address matching using device address 4"]
    #[inline(always)]
    pub const fn ena4(&self) -> super::vals::Ena4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ena4::from_bits(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 4"]
    #[inline(always)]
    pub fn set_ena4(&mut self, val: super::vals::Ena4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable or disable device address matching using device address 5"]
    #[inline(always)]
    pub const fn ena5(&self) -> super::vals::Ena5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ena5::from_bits(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 5"]
    #[inline(always)]
    pub fn set_ena5(&mut self, val: super::vals::Ena5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable or disable device address matching using device address 6"]
    #[inline(always)]
    pub const fn ena6(&self) -> super::vals::Ena6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ena6::from_bits(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 6"]
    #[inline(always)]
    pub fn set_ena6(&mut self, val: super::vals::Ena6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable or disable device address matching using device address 7"]
    #[inline(always)]
    pub const fn ena7(&self) -> super::vals::Ena7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ena7::from_bits(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 7"]
    #[inline(always)]
    pub fn set_ena7(&mut self, val: super::vals::Ena7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "TxAdd for device address 0"]
    #[inline(always)]
    pub const fn txadd0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 0"]
    #[inline(always)]
    pub fn set_txadd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "TxAdd for device address 1"]
    #[inline(always)]
    pub const fn txadd1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 1"]
    #[inline(always)]
    pub fn set_txadd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "TxAdd for device address 2"]
    #[inline(always)]
    pub const fn txadd2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 2"]
    #[inline(always)]
    pub fn set_txadd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "TxAdd for device address 3"]
    #[inline(always)]
    pub const fn txadd3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 3"]
    #[inline(always)]
    pub fn set_txadd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TxAdd for device address 4"]
    #[inline(always)]
    pub const fn txadd4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 4"]
    #[inline(always)]
    pub fn set_txadd4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TxAdd for device address 5"]
    #[inline(always)]
    pub const fn txadd5(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 5"]
    #[inline(always)]
    pub fn set_txadd5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "TxAdd for device address 6"]
    #[inline(always)]
    pub const fn txadd6(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 6"]
    #[inline(always)]
    pub fn set_txadd6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "TxAdd for device address 7"]
    #[inline(always)]
    pub const fn txadd7(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 7"]
    #[inline(always)]
    pub fn set_txadd7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Dacnf {
    #[inline(always)]
    fn default() -> Dacnf {
        Dacnf(0)
    }
}
#[doc = "Device address match index"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dai(pub u32);
impl Dai {
    #[doc = "Device address match index"]
    #[inline(always)]
    pub const fn dai(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Device address match index"]
    #[inline(always)]
    pub fn set_dai(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Dai {
    #[inline(always)]
    fn default() -> Dai {
        Dai(0)
    }
}
#[doc = "Description collection: Device address prefix n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dap(pub u32);
impl Dap {
    #[doc = "Device address prefix n"]
    #[inline(always)]
    pub const fn dap(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Device address prefix n"]
    #[inline(always)]
    pub fn set_dap(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dap {
    #[inline(always)]
    fn default() -> Dap {
        Dap(0)
    }
}
#[doc = "Data whitening initial value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datawhiteiv(pub u32);
impl Datawhiteiv {
    #[doc = "Data whitening initial value. Bit 6 is hardwired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
    #[inline(always)]
    pub const fn datawhiteiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Data whitening initial value. Bit 6 is hardwired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
    #[inline(always)]
    pub fn set_datawhiteiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Datawhiteiv {
    #[inline(always)]
    fn default() -> Datawhiteiv {
        Datawhiteiv(0)
    }
}
#[doc = "Various configuration for Direction finding"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfectrl1(pub u32);
impl Dfectrl1 {
    #[doc = "Length of the AoA/AoD procedure in number of 8 us units"]
    #[inline(always)]
    pub const fn numberof8us(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Length of the AoA/AoD procedure in number of 8 us units"]
    #[inline(always)]
    pub fn set_numberof8us(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Add CTE extension and do antenna switching/sampling in this extension"]
    #[inline(always)]
    pub const fn dfeinextension(&self) -> super::vals::Dfeinextension {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dfeinextension::from_bits(val as u8)
    }
    #[doc = "Add CTE extension and do antenna switching/sampling in this extension"]
    #[inline(always)]
    pub fn set_dfeinextension(&mut self, val: super::vals::Dfeinextension) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Interval between every time the antenna is changed in the SWITCHING state"]
    #[inline(always)]
    pub const fn tswitchspacing(&self) -> super::vals::Tswitchspacing {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tswitchspacing::from_bits(val as u8)
    }
    #[doc = "Interval between every time the antenna is changed in the SWITCHING state"]
    #[inline(always)]
    pub fn set_tswitchspacing(&mut self, val: super::vals::Tswitchspacing) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Interval between samples in the REFERENCE period"]
    #[inline(always)]
    pub const fn tsamplespacingref(&self) -> super::vals::Tsamplespacingref {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Tsamplespacingref::from_bits(val as u8)
    }
    #[doc = "Interval between samples in the REFERENCE period"]
    #[inline(always)]
    pub fn set_tsamplespacingref(&mut self, val: super::vals::Tsamplespacingref) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Whether to sample I/Q or magnitude/phase"]
    #[inline(always)]
    pub const fn sampletype(&self) -> super::vals::Sampletype {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sampletype::from_bits(val as u8)
    }
    #[doc = "Whether to sample I/Q or magnitude/phase"]
    #[inline(always)]
    pub fn set_sampletype(&mut self, val: super::vals::Sampletype) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
    #[inline(always)]
    pub const fn tsamplespacing(&self) -> super::vals::Tsamplespacing {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Tsamplespacing::from_bits(val as u8)
    }
    #[doc = "Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
    #[inline(always)]
    pub fn set_tsamplespacing(&mut self, val: super::vals::Tsamplespacing) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Repeat each individual antenna pattern N times sequentially, i.e. P0, P0, P1, P1, P2, P2, P3, P3, etc."]
    #[inline(always)]
    pub const fn repeatpattern(&self) -> super::vals::Repeatpattern {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Repeatpattern::from_bits(val as u8)
    }
    #[doc = "Repeat each individual antenna pattern N times sequentially, i.e. P0, P0, P1, P1, P2, P2, P3, P3, etc."]
    #[inline(always)]
    pub fn set_repeatpattern(&mut self, val: super::vals::Repeatpattern) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Gain will be lowered by the specified number of gain steps at the start of CTE"]
    #[inline(always)]
    pub const fn agcbackoffgain(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Gain will be lowered by the specified number of gain steps at the start of CTE"]
    #[inline(always)]
    pub fn set_agcbackoffgain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Dfectrl1 {
    #[inline(always)]
    fn default() -> Dfectrl1 {
        Dfectrl1(0)
    }
}
#[doc = "Start offset for Direction finding"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfectrl2(pub u32);
impl Dfectrl2 {
    #[doc = "Signed value offset after the end of the CRC before starting switching in number of 16 MHz clock cycles"]
    #[inline(always)]
    pub const fn tswitchoffset(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Signed value offset after the end of the CRC before starting switching in number of 16 MHz clock cycles"]
    #[inline(always)]
    pub fn set_tswitchoffset(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Signed value offset in number of 16 MHz clock cycles for fine tuning of the sampling instant for all IQ samples. With TSAMPLEOFFSET=0 the first sample is taken immediately at the start of the reference period"]
    #[inline(always)]
    pub const fn tsampleoffset(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Signed value offset in number of 16 MHz clock cycles for fine tuning of the sampling instant for all IQ samples. With TSAMPLEOFFSET=0 the first sample is taken immediately at the start of the reference period"]
    #[inline(always)]
    pub fn set_tsampleoffset(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Dfectrl2 {
    #[inline(always)]
    fn default() -> Dfectrl2 {
        Dfectrl2(0)
    }
}
#[doc = "Description collection: Pin select for DFE pin n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfegpio(pub u32);
impl Dfegpio {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub fn set_port(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::Connect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Connect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Connect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dfegpio {
    #[inline(always)]
    fn default() -> Dfegpio {
        Dfegpio(0)
    }
}
#[doc = "Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfemode(pub u32);
impl Dfemode {
    #[doc = "Direction finding operation mode"]
    #[inline(always)]
    pub const fn dfeopmode(&self) -> super::vals::Dfeopmode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dfeopmode::from_bits(val as u8)
    }
    #[doc = "Direction finding operation mode"]
    #[inline(always)]
    pub fn set_dfeopmode(&mut self, val: super::vals::Dfeopmode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Dfemode {
    #[inline(always)]
    fn default() -> Dfemode {
        Dfemode(0)
    }
}
#[doc = "DFE status information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfestatus(pub u32);
impl Dfestatus {
    #[doc = "Internal state of switching state machine"]
    #[inline(always)]
    pub const fn switchingstate(&self) -> super::vals::Switchingstate {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Switchingstate::from_bits(val as u8)
    }
    #[doc = "Internal state of switching state machine"]
    #[inline(always)]
    pub fn set_switchingstate(&mut self, val: super::vals::Switchingstate) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Internal state of sampling state machine"]
    #[inline(always)]
    pub const fn samplingstate(&self) -> super::vals::Samplingstate {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Samplingstate::from_bits(val as u8)
    }
    #[doc = "Internal state of sampling state machine"]
    #[inline(always)]
    pub fn set_samplingstate(&mut self, val: super::vals::Samplingstate) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Dfestatus {
    #[inline(always)]
    fn default() -> Dfestatus {
        Dfestatus(0)
    }
}
#[doc = "IEEE 802.15.4 energy detect loop count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edcnt(pub u32);
impl Edcnt {
    #[doc = "IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub const fn edcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub fn set_edcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
    }
}
impl Default for Edcnt {
    #[inline(always)]
    fn default() -> Edcnt {
        Edcnt(0)
    }
}
#[doc = "IEEE 802.15.4 energy detect level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edsample(pub u32);
impl Edsample {
    #[doc = "IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub const fn edlvl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn set_edlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Edsample {
    #[inline(always)]
    fn default() -> Edsample {
        Edsample(0)
    }
}
#[doc = "Frequency"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frequency(pub u32);
impl Frequency {
    #[doc = "Radio channel frequency"]
    #[inline(always)]
    pub const fn frequency(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Radio channel frequency"]
    #[inline(always)]
    pub fn set_frequency(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Channel map selection"]
    #[inline(always)]
    pub const fn map(&self) -> super::vals::Map {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Map::from_bits(val as u8)
    }
    #[doc = "Channel map selection"]
    #[inline(always)]
    pub fn set_map(&mut self, val: super::vals::Map) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Frequency {
    #[inline(always)]
    fn default() -> Frequency {
        Frequency(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for event ADDRESS"]
    #[inline(always)]
    pub const fn address(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event ADDRESS"]
    #[inline(always)]
    pub fn set_address(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for event PAYLOAD"]
    #[inline(always)]
    pub const fn payload(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event PAYLOAD"]
    #[inline(always)]
    pub fn set_payload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to disable interrupt for event DISABLED"]
    #[inline(always)]
    pub const fn disabled(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event DISABLED"]
    #[inline(always)]
    pub fn set_disabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to disable interrupt for event DEVMATCH"]
    #[inline(always)]
    pub const fn devmatch(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event DEVMATCH"]
    #[inline(always)]
    pub fn set_devmatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to disable interrupt for event DEVMISS"]
    #[inline(always)]
    pub const fn devmiss(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event DEVMISS"]
    #[inline(always)]
    pub fn set_devmiss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to disable interrupt for event RSSIEND"]
    #[inline(always)]
    pub const fn rssiend(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RSSIEND"]
    #[inline(always)]
    pub fn set_rssiend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to disable interrupt for event BCMATCH"]
    #[inline(always)]
    pub const fn bcmatch(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event BCMATCH"]
    #[inline(always)]
    pub fn set_bcmatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write '1' to disable interrupt for event CRCOK"]
    #[inline(always)]
    pub const fn crcok(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event CRCOK"]
    #[inline(always)]
    pub fn set_crcok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write '1' to disable interrupt for event CRCERROR"]
    #[inline(always)]
    pub const fn crcerror(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event CRCERROR"]
    #[inline(always)]
    pub fn set_crcerror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Write '1' to disable interrupt for event FRAMESTART"]
    #[inline(always)]
    pub const fn framestart(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event FRAMESTART"]
    #[inline(always)]
    pub fn set_framestart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Write '1' to disable interrupt for event EDEND"]
    #[inline(always)]
    pub const fn edend(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event EDEND"]
    #[inline(always)]
    pub fn set_edend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Write '1' to disable interrupt for event EDSTOPPED"]
    #[inline(always)]
    pub const fn edstopped(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event EDSTOPPED"]
    #[inline(always)]
    pub fn set_edstopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to disable interrupt for event CCAIDLE"]
    #[inline(always)]
    pub const fn ccaidle(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event CCAIDLE"]
    #[inline(always)]
    pub fn set_ccaidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to disable interrupt for event CCABUSY"]
    #[inline(always)]
    pub const fn ccabusy(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event CCABUSY"]
    #[inline(always)]
    pub fn set_ccabusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to disable interrupt for event CCASTOPPED"]
    #[inline(always)]
    pub const fn ccastopped(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event CCASTOPPED"]
    #[inline(always)]
    pub fn set_ccastopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write '1' to disable interrupt for event RATEBOOST"]
    #[inline(always)]
    pub const fn rateboost(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RATEBOOST"]
    #[inline(always)]
    pub fn set_rateboost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write '1' to disable interrupt for event TXREADY"]
    #[inline(always)]
    pub const fn txready(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event TXREADY"]
    #[inline(always)]
    pub fn set_txready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Write '1' to disable interrupt for event RXREADY"]
    #[inline(always)]
    pub const fn rxready(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RXREADY"]
    #[inline(always)]
    pub fn set_rxready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Write '1' to disable interrupt for event MHRMATCH"]
    #[inline(always)]
    pub const fn mhrmatch(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event MHRMATCH"]
    #[inline(always)]
    pub fn set_mhrmatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Write '1' to disable interrupt for event SYNC"]
    #[inline(always)]
    pub const fn sync(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event SYNC"]
    #[inline(always)]
    pub fn set_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Write '1' to disable interrupt for event PHYEND"]
    #[inline(always)]
    pub const fn phyend(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event PHYEND"]
    #[inline(always)]
    pub fn set_phyend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Write '1' to disable interrupt for event CTEPRESENT"]
    #[inline(always)]
    pub const fn ctepresent(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event CTEPRESENT"]
    #[inline(always)]
    pub fn set_ctepresent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Maximum number of buffer words to transfer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Maxcnt(pub u32);
impl Maxcnt {
    #[doc = "Maximum number of buffer words to transfer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Maximum number of buffer words to transfer"]
    #[inline(always)]
    pub fn set_maxcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Maxcnt {
    #[inline(always)]
    fn default() -> Maxcnt {
        Maxcnt(0)
    }
}
#[doc = "Data rate and modulation"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "Radio mode configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modecnf0(pub u32);
impl Modecnf0 {
    #[doc = "Radio ramp-up time"]
    #[inline(always)]
    pub const fn ru(&self) -> super::vals::Ru {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ru::from_bits(val as u8)
    }
    #[doc = "Radio ramp-up time"]
    #[inline(always)]
    pub fn set_ru(&mut self, val: super::vals::Ru) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Default TX value"]
    #[inline(always)]
    pub const fn dtx(&self) -> super::vals::Dtx {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Dtx::from_bits(val as u8)
    }
    #[doc = "Default TX value"]
    #[inline(always)]
    pub fn set_dtx(&mut self, val: super::vals::Dtx) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for Modecnf0 {
    #[inline(always)]
    fn default() -> Modecnf0 {
        Modecnf0(0)
    }
}
#[doc = "Packet configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcnf0(pub u32);
impl Pcnf0 {
    #[doc = "Length on air of LENGTH field in number of bits"]
    #[inline(always)]
    pub const fn lflen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Length on air of LENGTH field in number of bits"]
    #[inline(always)]
    pub fn set_lflen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Length on air of S0 field in number of bytes"]
    #[inline(always)]
    pub const fn s0len(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Length on air of S0 field in number of bytes"]
    #[inline(always)]
    pub fn set_s0len(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Length on air of S1 field in number of bits"]
    #[inline(always)]
    pub const fn s1len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length on air of S1 field in number of bits"]
    #[inline(always)]
    pub fn set_s1len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Include or exclude S1 field in RAM"]
    #[inline(always)]
    pub const fn s1incl(&self) -> super::vals::S1incl {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::S1incl::from_bits(val as u8)
    }
    #[doc = "Include or exclude S1 field in RAM"]
    #[inline(always)]
    pub fn set_s1incl(&mut self, val: super::vals::S1incl) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Length of code indicator - Long Range"]
    #[inline(always)]
    pub const fn cilen(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Length of code indicator - Long Range"]
    #[inline(always)]
    pub fn set_cilen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    pub const fn plen(&self) -> super::vals::Plen {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Plen::from_bits(val as u8)
    }
    #[doc = "Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    pub fn set_plen(&mut self, val: super::vals::Plen) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Indicates if LENGTH field contains CRC or not"]
    #[inline(always)]
    pub const fn crcinc(&self) -> super::vals::Crcinc {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Crcinc::from_bits(val as u8)
    }
    #[doc = "Indicates if LENGTH field contains CRC or not"]
    #[inline(always)]
    pub fn set_crcinc(&mut self, val: super::vals::Crcinc) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Length of TERM field in Long Range operation"]
    #[inline(always)]
    pub const fn termlen(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "Length of TERM field in Long Range operation"]
    #[inline(always)]
    pub fn set_termlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for Pcnf0 {
    #[inline(always)]
    fn default() -> Pcnf0 {
        Pcnf0(0)
    }
}
#[doc = "Packet configuration register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcnf1(pub u32);
impl Pcnf1 {
    #[doc = "Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    pub const fn maxlen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    pub fn set_maxlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Static length in number of bytes"]
    #[inline(always)]
    pub const fn statlen(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Static length in number of bytes"]
    #[inline(always)]
    pub fn set_statlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Base address length in number of bytes"]
    #[inline(always)]
    pub const fn balen(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Base address length in number of bytes"]
    #[inline(always)]
    pub fn set_balen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
    #[inline(always)]
    pub const fn endian(&self) -> super::vals::Endian {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Endian::from_bits(val as u8)
    }
    #[doc = "On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
    #[inline(always)]
    pub fn set_endian(&mut self, val: super::vals::Endian) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable or disable packet whitening"]
    #[inline(always)]
    pub const fn whiteen(&self) -> super::vals::Whiteen {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Whiteen::from_bits(val as u8)
    }
    #[doc = "Enable or disable packet whitening"]
    #[inline(always)]
    pub fn set_whiteen(&mut self, val: super::vals::Whiteen) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Pcnf1 {
    #[inline(always)]
    fn default() -> Pcnf1 {
        Pcnf1(0)
    }
}
#[doc = "Payload status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdustat(pub u32);
impl Pdustat {
    #[doc = "Status on payload length vs. PCNF1.MAXLEN"]
    #[inline(always)]
    pub const fn pdustat(&self) -> super::vals::Pdustat {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdustat::from_bits(val as u8)
    }
    #[doc = "Status on payload length vs. PCNF1.MAXLEN"]
    #[inline(always)]
    pub fn set_pdustat(&mut self, val: super::vals::Pdustat) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Status on what rate packet is received with in Long Range"]
    #[inline(always)]
    pub const fn cistat(&self) -> super::vals::Cistat {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Cistat::from_bits(val as u8)
    }
    #[doc = "Status on what rate packet is received with in Long Range"]
    #[inline(always)]
    pub fn set_cistat(&mut self, val: super::vals::Cistat) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
}
impl Default for Pdustat {
    #[inline(always)]
    fn default() -> Pdustat {
        Pdustat(0)
    }
}
#[doc = "Peripheral power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Power(pub u32);
impl Power {
    #[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
    #[inline(always)]
    pub const fn power(&self) -> super::vals::Power {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Power::from_bits(val as u8)
    }
    #[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
    #[inline(always)]
    pub fn set_power(&mut self, val: super::vals::Power) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Power {
    #[inline(always)]
    fn default() -> Power {
        Power(0)
    }
}
#[doc = "Prefixes bytes for logical addresses 0-3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prefix0(pub u32);
impl Prefix0 {
    #[doc = "Address prefix 0."]
    #[inline(always)]
    pub const fn ap0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 0."]
    #[inline(always)]
    pub fn set_ap0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address prefix 1."]
    #[inline(always)]
    pub const fn ap1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 1."]
    #[inline(always)]
    pub fn set_ap1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address prefix 2."]
    #[inline(always)]
    pub const fn ap2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 2."]
    #[inline(always)]
    pub fn set_ap2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address prefix 3."]
    #[inline(always)]
    pub const fn ap3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 3."]
    #[inline(always)]
    pub fn set_ap3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Prefix0 {
    #[inline(always)]
    fn default() -> Prefix0 {
        Prefix0(0)
    }
}
#[doc = "Prefixes bytes for logical addresses 4-7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prefix1(pub u32);
impl Prefix1 {
    #[doc = "Address prefix 4."]
    #[inline(always)]
    pub const fn ap4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 4."]
    #[inline(always)]
    pub fn set_ap4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address prefix 5."]
    #[inline(always)]
    pub const fn ap5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 5."]
    #[inline(always)]
    pub fn set_ap5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address prefix 6."]
    #[inline(always)]
    pub const fn ap6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 6."]
    #[inline(always)]
    pub fn set_ap6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address prefix 7."]
    #[inline(always)]
    pub const fn ap7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 7."]
    #[inline(always)]
    pub fn set_ap7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Prefix1 {
    #[inline(always)]
    fn default() -> Prefix1 {
        Prefix1(0)
    }
}
#[doc = "Publish configuration for event ADDRESS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishAddress(pub u32);
impl PublishAddress {
    #[doc = "DPPI channel that event ADDRESS will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event ADDRESS will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishAddressEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishAddressEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishAddressEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishAddress {
    #[inline(always)]
    fn default() -> PublishAddress {
        PublishAddress(0)
    }
}
#[doc = "Publish configuration for event BCMATCH"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishBcmatch(pub u32);
impl PublishBcmatch {
    #[doc = "DPPI channel that event BCMATCH will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event BCMATCH will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishBcmatchEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishBcmatchEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishBcmatchEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishBcmatch {
    #[inline(always)]
    fn default() -> PublishBcmatch {
        PublishBcmatch(0)
    }
}
#[doc = "Publish configuration for event CCABUSY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishCcabusy(pub u32);
impl PublishCcabusy {
    #[doc = "DPPI channel that event CCABUSY will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event CCABUSY will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishCcabusyEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishCcabusyEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishCcabusyEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishCcabusy {
    #[inline(always)]
    fn default() -> PublishCcabusy {
        PublishCcabusy(0)
    }
}
#[doc = "Publish configuration for event CCAIDLE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishCcaidle(pub u32);
impl PublishCcaidle {
    #[doc = "DPPI channel that event CCAIDLE will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event CCAIDLE will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishCcaidleEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishCcaidleEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishCcaidleEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishCcaidle {
    #[inline(always)]
    fn default() -> PublishCcaidle {
        PublishCcaidle(0)
    }
}
#[doc = "Publish configuration for event CCASTOPPED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishCcastopped(pub u32);
impl PublishCcastopped {
    #[doc = "DPPI channel that event CCASTOPPED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event CCASTOPPED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishCcastoppedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishCcastoppedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishCcastoppedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishCcastopped {
    #[inline(always)]
    fn default() -> PublishCcastopped {
        PublishCcastopped(0)
    }
}
#[doc = "Publish configuration for event CRCERROR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishCrcerror(pub u32);
impl PublishCrcerror {
    #[doc = "DPPI channel that event CRCERROR will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event CRCERROR will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishCrcerrorEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishCrcerrorEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishCrcerrorEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishCrcerror {
    #[inline(always)]
    fn default() -> PublishCrcerror {
        PublishCrcerror(0)
    }
}
#[doc = "Publish configuration for event CRCOK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishCrcok(pub u32);
impl PublishCrcok {
    #[doc = "DPPI channel that event CRCOK will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event CRCOK will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishCrcokEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishCrcokEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishCrcokEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishCrcok {
    #[inline(always)]
    fn default() -> PublishCrcok {
        PublishCrcok(0)
    }
}
#[doc = "Publish configuration for event CTEPRESENT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishCtepresent(pub u32);
impl PublishCtepresent {
    #[doc = "DPPI channel that event CTEPRESENT will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event CTEPRESENT will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishCtepresentEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishCtepresentEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishCtepresentEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishCtepresent {
    #[inline(always)]
    fn default() -> PublishCtepresent {
        PublishCtepresent(0)
    }
}
#[doc = "Publish configuration for event DEVMATCH"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishDevmatch(pub u32);
impl PublishDevmatch {
    #[doc = "DPPI channel that event DEVMATCH will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event DEVMATCH will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishDevmatchEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishDevmatchEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishDevmatchEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishDevmatch {
    #[inline(always)]
    fn default() -> PublishDevmatch {
        PublishDevmatch(0)
    }
}
#[doc = "Publish configuration for event DEVMISS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishDevmiss(pub u32);
impl PublishDevmiss {
    #[doc = "DPPI channel that event DEVMISS will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event DEVMISS will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishDevmissEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishDevmissEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishDevmissEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishDevmiss {
    #[inline(always)]
    fn default() -> PublishDevmiss {
        PublishDevmiss(0)
    }
}
#[doc = "Publish configuration for event DISABLED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishDisabled(pub u32);
impl PublishDisabled {
    #[doc = "DPPI channel that event DISABLED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event DISABLED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishDisabledEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishDisabledEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishDisabledEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishDisabled {
    #[inline(always)]
    fn default() -> PublishDisabled {
        PublishDisabled(0)
    }
}
#[doc = "Publish configuration for event EDEND"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEdend(pub u32);
impl PublishEdend {
    #[doc = "DPPI channel that event EDEND will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event EDEND will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEdendEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEdendEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEdendEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEdend {
    #[inline(always)]
    fn default() -> PublishEdend {
        PublishEdend(0)
    }
}
#[doc = "Publish configuration for event EDSTOPPED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEdstopped(pub u32);
impl PublishEdstopped {
    #[doc = "DPPI channel that event EDSTOPPED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event EDSTOPPED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEdstoppedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEdstoppedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEdstoppedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEdstopped {
    #[inline(always)]
    fn default() -> PublishEdstopped {
        PublishEdstopped(0)
    }
}
#[doc = "Publish configuration for event END"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEnd(pub u32);
impl PublishEnd {
    #[doc = "DPPI channel that event END will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event END will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEndEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEndEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEndEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEnd {
    #[inline(always)]
    fn default() -> PublishEnd {
        PublishEnd(0)
    }
}
#[doc = "Publish configuration for event FRAMESTART"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishFramestart(pub u32);
impl PublishFramestart {
    #[doc = "DPPI channel that event FRAMESTART will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event FRAMESTART will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishFramestartEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishFramestartEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishFramestartEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishFramestart {
    #[inline(always)]
    fn default() -> PublishFramestart {
        PublishFramestart(0)
    }
}
#[doc = "Publish configuration for event MHRMATCH"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishMhrmatch(pub u32);
impl PublishMhrmatch {
    #[doc = "DPPI channel that event MHRMATCH will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event MHRMATCH will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishMhrmatchEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishMhrmatchEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishMhrmatchEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishMhrmatch {
    #[inline(always)]
    fn default() -> PublishMhrmatch {
        PublishMhrmatch(0)
    }
}
#[doc = "Publish configuration for event PAYLOAD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishPayload(pub u32);
impl PublishPayload {
    #[doc = "DPPI channel that event PAYLOAD will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event PAYLOAD will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishPayloadEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishPayloadEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishPayloadEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishPayload {
    #[inline(always)]
    fn default() -> PublishPayload {
        PublishPayload(0)
    }
}
#[doc = "Publish configuration for event PHYEND"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishPhyend(pub u32);
impl PublishPhyend {
    #[doc = "DPPI channel that event PHYEND will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event PHYEND will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishPhyendEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishPhyendEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishPhyendEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishPhyend {
    #[inline(always)]
    fn default() -> PublishPhyend {
        PublishPhyend(0)
    }
}
#[doc = "Publish configuration for event RATEBOOST"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishRateboost(pub u32);
impl PublishRateboost {
    #[doc = "DPPI channel that event RATEBOOST will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event RATEBOOST will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishRateboostEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishRateboostEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishRateboostEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishRateboost {
    #[inline(always)]
    fn default() -> PublishRateboost {
        PublishRateboost(0)
    }
}
#[doc = "Publish configuration for event READY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishReady(pub u32);
impl PublishReady {
    #[doc = "DPPI channel that event READY will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event READY will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishReadyEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishReadyEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishReadyEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishReady {
    #[inline(always)]
    fn default() -> PublishReady {
        PublishReady(0)
    }
}
#[doc = "Publish configuration for event RSSIEND"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishRssiend(pub u32);
impl PublishRssiend {
    #[doc = "DPPI channel that event RSSIEND will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event RSSIEND will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishRssiendEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishRssiendEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishRssiendEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishRssiend {
    #[inline(always)]
    fn default() -> PublishRssiend {
        PublishRssiend(0)
    }
}
#[doc = "Publish configuration for event RXREADY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishRxready(pub u32);
impl PublishRxready {
    #[doc = "DPPI channel that event RXREADY will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event RXREADY will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishRxreadyEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishRxreadyEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishRxreadyEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishRxready {
    #[inline(always)]
    fn default() -> PublishRxready {
        PublishRxready(0)
    }
}
#[doc = "Publish configuration for event SYNC"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishSync(pub u32);
impl PublishSync {
    #[doc = "DPPI channel that event SYNC will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event SYNC will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishSyncEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishSyncEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishSyncEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishSync {
    #[inline(always)]
    fn default() -> PublishSync {
        PublishSync(0)
    }
}
#[doc = "Publish configuration for event TXREADY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishTxready(pub u32);
impl PublishTxready {
    #[doc = "DPPI channel that event TXREADY will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event TXREADY will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishTxreadyEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishTxreadyEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishTxreadyEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishTxready {
    #[inline(always)]
    fn default() -> PublishTxready {
        PublishTxready(0)
    }
}
#[doc = "RSSI sample"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rssisample(pub u32);
impl Rssisample {
    #[doc = "RSSI sample."]
    #[inline(always)]
    pub const fn rssisample(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "RSSI sample."]
    #[inline(always)]
    pub fn set_rssisample(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Rssisample {
    #[inline(always)]
    fn default() -> Rssisample {
        Rssisample(0)
    }
}
#[doc = "Receive address select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxaddresses(pub u32);
impl Rxaddresses {
    #[doc = "Enable or disable reception on logical address 0."]
    #[inline(always)]
    pub const fn addr0(&self) -> super::vals::Addr0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Addr0::from_bits(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 0."]
    #[inline(always)]
    pub fn set_addr0(&mut self, val: super::vals::Addr0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable reception on logical address 1."]
    #[inline(always)]
    pub const fn addr1(&self) -> super::vals::Addr1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Addr1::from_bits(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 1."]
    #[inline(always)]
    pub fn set_addr1(&mut self, val: super::vals::Addr1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable reception on logical address 2."]
    #[inline(always)]
    pub const fn addr2(&self) -> super::vals::Addr2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Addr2::from_bits(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 2."]
    #[inline(always)]
    pub fn set_addr2(&mut self, val: super::vals::Addr2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable or disable reception on logical address 3."]
    #[inline(always)]
    pub const fn addr3(&self) -> super::vals::Addr3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Addr3::from_bits(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 3."]
    #[inline(always)]
    pub fn set_addr3(&mut self, val: super::vals::Addr3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable or disable reception on logical address 4."]
    #[inline(always)]
    pub const fn addr4(&self) -> super::vals::Addr4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Addr4::from_bits(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 4."]
    #[inline(always)]
    pub fn set_addr4(&mut self, val: super::vals::Addr4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable or disable reception on logical address 5."]
    #[inline(always)]
    pub const fn addr5(&self) -> super::vals::Addr5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Addr5::from_bits(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 5."]
    #[inline(always)]
    pub fn set_addr5(&mut self, val: super::vals::Addr5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable or disable reception on logical address 6."]
    #[inline(always)]
    pub const fn addr6(&self) -> super::vals::Addr6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Addr6::from_bits(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 6."]
    #[inline(always)]
    pub fn set_addr6(&mut self, val: super::vals::Addr6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable or disable reception on logical address 7."]
    #[inline(always)]
    pub const fn addr7(&self) -> super::vals::Addr7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Addr7::from_bits(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 7."]
    #[inline(always)]
    pub fn set_addr7(&mut self, val: super::vals::Addr7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Rxaddresses {
    #[inline(always)]
    fn default() -> Rxaddresses {
        Rxaddresses(0)
    }
}
#[doc = "CRC field of previously received packet"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxcrc(pub u32);
impl Rxcrc {
    #[doc = "CRC field of previously received packet"]
    #[inline(always)]
    pub const fn rxcrc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CRC field of previously received packet"]
    #[inline(always)]
    pub fn set_rxcrc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Rxcrc {
    #[inline(always)]
    fn default() -> Rxcrc {
        Rxcrc(0)
    }
}
#[doc = "Received address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxmatch(pub u32);
impl Rxmatch {
    #[doc = "Received address"]
    #[inline(always)]
    pub const fn rxmatch(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Received address"]
    #[inline(always)]
    pub fn set_rxmatch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Rxmatch {
    #[inline(always)]
    fn default() -> Rxmatch {
        Rxmatch(0)
    }
}
#[doc = "IEEE 802.15.4 start of frame delimiter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfd(pub u32);
impl Sfd {
    #[doc = "IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub const fn sfd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub fn set_sfd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Sfd {
    #[inline(always)]
    fn default() -> Sfd {
        Sfd(0)
    }
}
#[doc = "Shortcuts between local events and tasks"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between event READY and task START"]
    #[inline(always)]
    pub const fn ready_start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event READY and task START"]
    #[inline(always)]
    pub fn set_ready_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shortcut between event END and task DISABLE"]
    #[inline(always)]
    pub const fn end_disable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event END and task DISABLE"]
    #[inline(always)]
    pub fn set_end_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Shortcut between event DISABLED and task TXEN"]
    #[inline(always)]
    pub const fn disabled_txen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event DISABLED and task TXEN"]
    #[inline(always)]
    pub fn set_disabled_txen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Shortcut between event DISABLED and task RXEN"]
    #[inline(always)]
    pub const fn disabled_rxen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event DISABLED and task RXEN"]
    #[inline(always)]
    pub fn set_disabled_rxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Shortcut between event ADDRESS and task RSSISTART"]
    #[inline(always)]
    pub const fn address_rssistart(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event ADDRESS and task RSSISTART"]
    #[inline(always)]
    pub fn set_address_rssistart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Shortcut between event END and task START"]
    #[inline(always)]
    pub const fn end_start(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event END and task START"]
    #[inline(always)]
    pub fn set_end_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Shortcut between event ADDRESS and task BCSTART"]
    #[inline(always)]
    pub const fn address_bcstart(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event ADDRESS and task BCSTART"]
    #[inline(always)]
    pub fn set_address_bcstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Shortcut between event DISABLED and task RSSISTOP"]
    #[inline(always)]
    pub const fn disabled_rssistop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event DISABLED and task RSSISTOP"]
    #[inline(always)]
    pub fn set_disabled_rssistop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Shortcut between event RXREADY and task CCASTART"]
    #[inline(always)]
    pub const fn rxready_ccastart(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event RXREADY and task CCASTART"]
    #[inline(always)]
    pub fn set_rxready_ccastart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Shortcut between event CCAIDLE and task TXEN"]
    #[inline(always)]
    pub const fn ccaidle_txen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event CCAIDLE and task TXEN"]
    #[inline(always)]
    pub fn set_ccaidle_txen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Shortcut between event CCABUSY and task DISABLE"]
    #[inline(always)]
    pub const fn ccabusy_disable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event CCABUSY and task DISABLE"]
    #[inline(always)]
    pub fn set_ccabusy_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Shortcut between event FRAMESTART and task BCSTART"]
    #[inline(always)]
    pub const fn framestart_bcstart(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event FRAMESTART and task BCSTART"]
    #[inline(always)]
    pub fn set_framestart_bcstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Shortcut between event READY and task EDSTART"]
    #[inline(always)]
    pub const fn ready_edstart(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event READY and task EDSTART"]
    #[inline(always)]
    pub fn set_ready_edstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Shortcut between event EDEND and task DISABLE"]
    #[inline(always)]
    pub const fn edend_disable(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event EDEND and task DISABLE"]
    #[inline(always)]
    pub fn set_edend_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Shortcut between event CCAIDLE and task STOP"]
    #[inline(always)]
    pub const fn ccaidle_stop(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event CCAIDLE and task STOP"]
    #[inline(always)]
    pub fn set_ccaidle_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Shortcut between event TXREADY and task START"]
    #[inline(always)]
    pub const fn txready_start(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event TXREADY and task START"]
    #[inline(always)]
    pub fn set_txready_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Shortcut between event RXREADY and task START"]
    #[inline(always)]
    pub const fn rxready_start(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event RXREADY and task START"]
    #[inline(always)]
    pub fn set_rxready_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Shortcut between event PHYEND and task DISABLE"]
    #[inline(always)]
    pub const fn phyend_disable(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event PHYEND and task DISABLE"]
    #[inline(always)]
    pub fn set_phyend_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Shortcut between event PHYEND and task START"]
    #[inline(always)]
    pub const fn phyend_start(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event PHYEND and task START"]
    #[inline(always)]
    pub fn set_phyend_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Current radio state"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State(pub u32);
impl State {
    #[doc = "Current radio state"]
    #[inline(always)]
    pub const fn state(&self) -> super::vals::State {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::State::from_bits(val as u8)
    }
    #[doc = "Current radio state"]
    #[inline(always)]
    pub fn set_state(&mut self, val: super::vals::State) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for State {
    #[inline(always)]
    fn default() -> State {
        State(0)
    }
}
#[doc = "Subscribe configuration for task BCSTART"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeBcstart(pub u32);
impl SubscribeBcstart {
    #[doc = "DPPI channel that task BCSTART will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task BCSTART will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeBcstartEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeBcstartEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeBcstartEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeBcstart {
    #[inline(always)]
    fn default() -> SubscribeBcstart {
        SubscribeBcstart(0)
    }
}
#[doc = "Subscribe configuration for task BCSTOP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeBcstop(pub u32);
impl SubscribeBcstop {
    #[doc = "DPPI channel that task BCSTOP will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task BCSTOP will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeBcstopEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeBcstopEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeBcstopEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeBcstop {
    #[inline(always)]
    fn default() -> SubscribeBcstop {
        SubscribeBcstop(0)
    }
}
#[doc = "Subscribe configuration for task CCASTART"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeCcastart(pub u32);
impl SubscribeCcastart {
    #[doc = "DPPI channel that task CCASTART will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task CCASTART will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeCcastartEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeCcastartEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeCcastartEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeCcastart {
    #[inline(always)]
    fn default() -> SubscribeCcastart {
        SubscribeCcastart(0)
    }
}
#[doc = "Subscribe configuration for task CCASTOP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeCcastop(pub u32);
impl SubscribeCcastop {
    #[doc = "DPPI channel that task CCASTOP will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task CCASTOP will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeCcastopEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeCcastopEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeCcastopEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeCcastop {
    #[inline(always)]
    fn default() -> SubscribeCcastop {
        SubscribeCcastop(0)
    }
}
#[doc = "Subscribe configuration for task DISABLE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeDisable(pub u32);
impl SubscribeDisable {
    #[doc = "DPPI channel that task DISABLE will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task DISABLE will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeDisableEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeDisableEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeDisableEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeDisable {
    #[inline(always)]
    fn default() -> SubscribeDisable {
        SubscribeDisable(0)
    }
}
#[doc = "Subscribe configuration for task EDSTART"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeEdstart(pub u32);
impl SubscribeEdstart {
    #[doc = "DPPI channel that task EDSTART will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task EDSTART will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeEdstartEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeEdstartEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeEdstartEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeEdstart {
    #[inline(always)]
    fn default() -> SubscribeEdstart {
        SubscribeEdstart(0)
    }
}
#[doc = "Subscribe configuration for task EDSTOP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeEdstop(pub u32);
impl SubscribeEdstop {
    #[doc = "DPPI channel that task EDSTOP will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task EDSTOP will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeEdstopEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeEdstopEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeEdstopEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeEdstop {
    #[inline(always)]
    fn default() -> SubscribeEdstop {
        SubscribeEdstop(0)
    }
}
#[doc = "Subscribe configuration for task RSSISTART"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeRssistart(pub u32);
impl SubscribeRssistart {
    #[doc = "DPPI channel that task RSSISTART will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task RSSISTART will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeRssistartEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeRssistartEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeRssistartEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeRssistart {
    #[inline(always)]
    fn default() -> SubscribeRssistart {
        SubscribeRssistart(0)
    }
}
#[doc = "Subscribe configuration for task RSSISTOP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeRssistop(pub u32);
impl SubscribeRssistop {
    #[doc = "DPPI channel that task RSSISTOP will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task RSSISTOP will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeRssistopEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeRssistopEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeRssistopEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeRssistop {
    #[inline(always)]
    fn default() -> SubscribeRssistop {
        SubscribeRssistop(0)
    }
}
#[doc = "Subscribe configuration for task RXEN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeRxen(pub u32);
impl SubscribeRxen {
    #[doc = "DPPI channel that task RXEN will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task RXEN will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeRxenEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeRxenEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeRxenEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeRxen {
    #[inline(always)]
    fn default() -> SubscribeRxen {
        SubscribeRxen(0)
    }
}
#[doc = "Subscribe configuration for task START"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeStart(pub u32);
impl SubscribeStart {
    #[doc = "DPPI channel that task START will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task START will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeStartEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeStartEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeStartEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeStart {
    #[inline(always)]
    fn default() -> SubscribeStart {
        SubscribeStart(0)
    }
}
#[doc = "Subscribe configuration for task STOP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeStop(pub u32);
impl SubscribeStop {
    #[doc = "DPPI channel that task STOP will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task STOP will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeStopEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeStopEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeStopEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeStop {
    #[inline(always)]
    fn default() -> SubscribeStop {
        SubscribeStop(0)
    }
}
#[doc = "Subscribe configuration for task TXEN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeTxen(pub u32);
impl SubscribeTxen {
    #[doc = "DPPI channel that task TXEN will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task TXEN will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeTxenEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeTxenEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeTxenEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeTxen {
    #[inline(always)]
    fn default() -> SubscribeTxen {
        SubscribeTxen(0)
    }
}
#[doc = "GPIO patterns to be used for each antenna"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Switchpattern(pub u32);
impl Switchpattern {
    #[doc = "Fill array of GPIO patterns for antenna control."]
    #[inline(always)]
    pub const fn switchpattern(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill array of GPIO patterns for antenna control."]
    #[inline(always)]
    pub fn set_switchpattern(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Switchpattern {
    #[inline(always)]
    fn default() -> Switchpattern {
        Switchpattern(0)
    }
}
#[doc = "Interframe spacing in us"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tifs(pub u32);
impl Tifs {
    #[doc = "Interframe spacing in us."]
    #[inline(always)]
    pub const fn tifs(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Interframe spacing in us."]
    #[inline(always)]
    pub fn set_tifs(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Tifs {
    #[inline(always)]
    fn default() -> Tifs {
        Tifs(0)
    }
}
#[doc = "Transmit address select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txaddress(pub u32);
impl Txaddress {
    #[doc = "Transmit address select"]
    #[inline(always)]
    pub const fn txaddress(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit address select"]
    #[inline(always)]
    pub fn set_txaddress(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Txaddress {
    #[inline(always)]
    fn default() -> Txaddress {
        Txaddress(0)
    }
}
#[doc = "Output power"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txpower(pub u32);
impl Txpower {
    #[doc = "RADIO output power"]
    #[inline(always)]
    pub const fn txpower(&self) -> super::vals::Txpower {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Txpower::from_bits(val as u8)
    }
    #[doc = "RADIO output power"]
    #[inline(always)]
    pub fn set_txpower(&mut self, val: super::vals::Txpower) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Txpower {
    #[inline(always)]
    fn default() -> Txpower {
        Txpower(0)
    }
}
