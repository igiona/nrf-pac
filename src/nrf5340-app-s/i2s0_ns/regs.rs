#[doc = "Alignment of sample within a frame"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Align(pub u32);
impl Align {
    #[doc = "Alignment of sample within a frame"]
    #[inline(always)]
    pub const fn align(&self) -> super::vals::Align {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Align::from_bits(val as u8)
    }
    #[doc = "Alignment of sample within a frame"]
    #[inline(always)]
    pub fn set_align(&mut self, val: super::vals::Align) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Align {
    #[inline(always)]
    fn default() -> Align {
        Align(0)
    }
}
#[doc = "Enable channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channels(pub u32);
impl Channels {
    #[doc = "Enable channels"]
    #[inline(always)]
    pub const fn channels(&self) -> super::vals::Channels {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Channels::from_bits(val as u8)
    }
    #[doc = "Enable channels"]
    #[inline(always)]
    pub fn set_channels(&mut self, val: super::vals::Channels) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Channels {
    #[inline(always)]
    fn default() -> Channels {
        Channels(0)
    }
}
#[doc = "Clock source selection for the I2S module"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkconfig(pub u32);
impl Clkconfig {
    #[doc = "Clock source selection"]
    #[inline(always)]
    pub const fn clksrc(&self) -> super::vals::Clksrc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Clksrc::from_bits(val as u8)
    }
    #[doc = "Clock source selection"]
    #[inline(always)]
    pub fn set_clksrc(&mut self, val: super::vals::Clksrc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
    #[inline(always)]
    pub const fn bypass(&self) -> super::vals::Bypass {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Bypass::from_bits(val as u8)
    }
    #[doc = "Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
    #[inline(always)]
    pub fn set_bypass(&mut self, val: super::vals::Bypass) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Clkconfig {
    #[inline(always)]
    fn default() -> Clkconfig {
        Clkconfig(0)
    }
}
#[doc = "Enable I2S module"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable I2S module"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enable I2S module"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Enable {
    #[inline(always)]
    fn default() -> Enable {
        Enable(0)
    }
}
#[doc = "Frame format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Format(pub u32);
impl Format {
    #[doc = "Frame format"]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::Format {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Format::from_bits(val as u8)
    }
    #[doc = "Frame format"]
    #[inline(always)]
    pub fn set_format(&mut self, val: super::vals::Format) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Format {
    #[inline(always)]
    fn default() -> Format {
        Format(0)
    }
}
#[doc = "Enable or disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Enable or disable interrupt for event RXPTRUPD"]
    #[inline(always)]
    pub const fn rxptrupd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event RXPTRUPD"]
    #[inline(always)]
    pub fn set_rxptrupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    pub const fn stopped(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn set_stopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable or disable interrupt for event TXPTRUPD"]
    #[inline(always)]
    pub const fn txptrupd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TXPTRUPD"]
    #[inline(always)]
    pub fn set_txptrupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable or disable interrupt for event FRAMESTART"]
    #[inline(always)]
    pub const fn framestart(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event FRAMESTART"]
    #[inline(always)]
    pub fn set_framestart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Pin select for LRCK signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lrck(pub u32);
impl Lrck {
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
    pub const fn connect(&self) -> super::vals::LrckConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::LrckConnect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::LrckConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Lrck {
    #[inline(always)]
    fn default() -> Lrck {
        Lrck(0)
    }
}
#[doc = "Size of RXD and TXD buffers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Maxcnt(pub u32);
impl Maxcnt {
    #[doc = "Size of RXD and TXD buffers in number of 32 bit words"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Size of RXD and TXD buffers in number of 32 bit words"]
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
#[doc = "Pin select for MCK signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mck(pub u32);
impl Mck {
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
    pub const fn connect(&self) -> super::vals::MckConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MckConnect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::MckConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mck {
    #[inline(always)]
    fn default() -> Mck {
        Mck(0)
    }
}
#[doc = "Master clock generator enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcken(pub u32);
impl Mcken {
    #[doc = "Master clock generator enable"]
    #[inline(always)]
    pub const fn mcken(&self) -> super::vals::Mcken {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mcken::from_bits(val as u8)
    }
    #[doc = "Master clock generator enable"]
    #[inline(always)]
    pub fn set_mcken(&mut self, val: super::vals::Mcken) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Mcken {
    #[inline(always)]
    fn default() -> Mcken {
        Mcken(0)
    }
}
#[doc = "I2S clock generator control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mckfreq(pub u32);
impl Mckfreq {
    #[doc = "I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero."]
    #[inline(always)]
    pub const fn mckfreq(&self) -> super::vals::Mckfreq {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Mckfreq::from_bits(val as u32)
    }
    #[doc = "I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero."]
    #[inline(always)]
    pub fn set_mckfreq(&mut self, val: super::vals::Mckfreq) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mckfreq {
    #[inline(always)]
    fn default() -> Mckfreq {
        Mckfreq(0)
    }
}
#[doc = "I2S mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "I2S mode"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "I2S mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
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
#[doc = "Publish configuration for event RXPTRUPD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishRxptrupd(pub u32);
impl PublishRxptrupd {
    #[doc = "DPPI channel that event RXPTRUPD will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event RXPTRUPD will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishRxptrupdEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishRxptrupdEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishRxptrupdEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishRxptrupd {
    #[inline(always)]
    fn default() -> PublishRxptrupd {
        PublishRxptrupd(0)
    }
}
#[doc = "Publish configuration for event STOPPED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishStopped(pub u32);
impl PublishStopped {
    #[doc = "DPPI channel that event STOPPED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event STOPPED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishStoppedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishStoppedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishStoppedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishStopped {
    #[inline(always)]
    fn default() -> PublishStopped {
        PublishStopped(0)
    }
}
#[doc = "Publish configuration for event TXPTRUPD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishTxptrupd(pub u32);
impl PublishTxptrupd {
    #[doc = "DPPI channel that event TXPTRUPD will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event TXPTRUPD will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishTxptrupdEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishTxptrupdEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishTxptrupdEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishTxptrupd {
    #[inline(always)]
    fn default() -> PublishTxptrupd {
        PublishTxptrupd(0)
    }
}
#[doc = "MCK / LRCK ratio"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ratio(pub u32);
impl Ratio {
    #[doc = "MCK / LRCK ratio"]
    #[inline(always)]
    pub const fn ratio(&self) -> super::vals::Ratio {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Ratio::from_bits(val as u8)
    }
    #[doc = "MCK / LRCK ratio"]
    #[inline(always)]
    pub fn set_ratio(&mut self, val: super::vals::Ratio) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Ratio {
    #[inline(always)]
    fn default() -> Ratio {
        Ratio(0)
    }
}
#[doc = "Reception (RX) enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxen(pub u32);
impl Rxen {
    #[doc = "Reception (RX) enable"]
    #[inline(always)]
    pub const fn rxen(&self) -> super::vals::Rxen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rxen::from_bits(val as u8)
    }
    #[doc = "Reception (RX) enable"]
    #[inline(always)]
    pub fn set_rxen(&mut self, val: super::vals::Rxen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Rxen {
    #[inline(always)]
    fn default() -> Rxen {
        Rxen(0)
    }
}
#[doc = "Pin select for SCK signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sck(pub u32);
impl Sck {
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
    pub const fn connect(&self) -> super::vals::SckConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SckConnect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::SckConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sck {
    #[inline(always)]
    fn default() -> Sck {
        Sck(0)
    }
}
#[doc = "Pin select for SDIN signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdin(pub u32);
impl Sdin {
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
    pub const fn connect(&self) -> super::vals::SdinConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SdinConnect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::SdinConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sdin {
    #[inline(always)]
    fn default() -> Sdin {
        Sdin(0)
    }
}
#[doc = "Pin select for SDOUT signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdout(pub u32);
impl Sdout {
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
    pub const fn connect(&self) -> super::vals::SdoutConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SdoutConnect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::SdoutConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sdout {
    #[inline(always)]
    fn default() -> Sdout {
        Sdout(0)
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
#[doc = "Sample width"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swidth(pub u32);
impl Swidth {
    #[doc = "Sample and half-frame width"]
    #[inline(always)]
    pub const fn swidth(&self) -> super::vals::Swidth {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Swidth::from_bits(val as u8)
    }
    #[doc = "Sample and half-frame width"]
    #[inline(always)]
    pub fn set_swidth(&mut self, val: super::vals::Swidth) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Swidth {
    #[inline(always)]
    fn default() -> Swidth {
        Swidth(0)
    }
}
#[doc = "Transmission (TX) enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txen(pub u32);
impl Txen {
    #[doc = "Transmission (TX) enable"]
    #[inline(always)]
    pub const fn txen(&self) -> super::vals::Txen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Txen::from_bits(val as u8)
    }
    #[doc = "Transmission (TX) enable"]
    #[inline(always)]
    pub fn set_txen(&mut self, val: super::vals::Txen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Txen {
    #[inline(always)]
    fn default() -> Txen {
        Txen(0)
    }
}
