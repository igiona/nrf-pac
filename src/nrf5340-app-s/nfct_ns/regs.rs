#[doc = "Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autocolresconfig(pub u32);
impl Autocolresconfig {
    #[doc = "Enables/disables auto collision resolution"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Enables/disables auto collision resolution"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Autocolresconfig {
    #[inline(always)]
    fn default() -> Autocolresconfig {
        Autocolresconfig(0)
    }
}
#[doc = "NFC Error Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errorstatus(pub u32);
impl Errorstatus {
    #[doc = "No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    pub const fn framedelaytimeout(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn set_framedelaytimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Errorstatus {
    #[inline(always)]
    fn default() -> Errorstatus {
        Errorstatus(0)
    }
}
#[doc = "Indicates the presence or not of a valid field"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fieldpresent(pub u32);
impl Fieldpresent {
    #[doc = "Indicates if a valid field is present. Available only in the activated state."]
    #[inline(always)]
    pub const fn fieldpresent(&self) -> super::vals::Fieldpresent {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fieldpresent::from_bits(val as u8)
    }
    #[doc = "Indicates if a valid field is present. Available only in the activated state."]
    #[inline(always)]
    pub fn set_fieldpresent(&mut self, val: super::vals::Fieldpresent) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates if the low level has locked to the field"]
    #[inline(always)]
    pub const fn lockdetect(&self) -> super::vals::Lockdetect {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lockdetect::from_bits(val as u8)
    }
    #[doc = "Indicates if the low level has locked to the field"]
    #[inline(always)]
    pub fn set_lockdetect(&mut self, val: super::vals::Lockdetect) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Fieldpresent {
    #[inline(always)]
    fn default() -> Fieldpresent {
        Fieldpresent(0)
    }
}
#[doc = "Maximum frame delay"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Framedelaymax(pub u32);
impl Framedelaymax {
    #[doc = "Maximum frame delay in number of 13.56 MHz clock cycles"]
    #[inline(always)]
    pub const fn framedelaymax(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Maximum frame delay in number of 13.56 MHz clock cycles"]
    #[inline(always)]
    pub fn set_framedelaymax(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Framedelaymax {
    #[inline(always)]
    fn default() -> Framedelaymax {
        Framedelaymax(0)
    }
}
#[doc = "Minimum frame delay"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Framedelaymin(pub u32);
impl Framedelaymin {
    #[doc = "Minimum frame delay in number of 13.56 MHz clock cycles"]
    #[inline(always)]
    pub const fn framedelaymin(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Minimum frame delay in number of 13.56 MHz clock cycles"]
    #[inline(always)]
    pub fn set_framedelaymin(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Framedelaymin {
    #[inline(always)]
    fn default() -> Framedelaymin {
        Framedelaymin(0)
    }
}
#[doc = "Configuration register for the Frame Delay Timer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Framedelaymode(pub u32);
impl Framedelaymode {
    #[doc = "Configuration register for the Frame Delay Timer"]
    #[inline(always)]
    pub const fn framedelaymode(&self) -> super::vals::Framedelaymode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Framedelaymode::from_bits(val as u8)
    }
    #[doc = "Configuration register for the Frame Delay Timer"]
    #[inline(always)]
    pub fn set_framedelaymode(&mut self, val: super::vals::Framedelaymode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Framedelaymode {
    #[inline(always)]
    fn default() -> Framedelaymode {
        Framedelaymode(0)
    }
}
#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to enable interrupt for event READY"]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event READY"]
    #[inline(always)]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to enable interrupt for event FIELDDETECTED"]
    #[inline(always)]
    pub const fn fielddetected(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event FIELDDETECTED"]
    #[inline(always)]
    pub fn set_fielddetected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable interrupt for event FIELDLOST"]
    #[inline(always)]
    pub const fn fieldlost(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event FIELDLOST"]
    #[inline(always)]
    pub fn set_fieldlost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to enable interrupt for event TXFRAMESTART"]
    #[inline(always)]
    pub const fn txframestart(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event TXFRAMESTART"]
    #[inline(always)]
    pub fn set_txframestart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to enable interrupt for event TXFRAMEEND"]
    #[inline(always)]
    pub const fn txframeend(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event TXFRAMEEND"]
    #[inline(always)]
    pub fn set_txframeend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to enable interrupt for event RXFRAMESTART"]
    #[inline(always)]
    pub const fn rxframestart(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event RXFRAMESTART"]
    #[inline(always)]
    pub fn set_rxframestart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to enable interrupt for event RXFRAMEEND"]
    #[inline(always)]
    pub const fn rxframeend(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event RXFRAMEEND"]
    #[inline(always)]
    pub fn set_rxframeend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to enable interrupt for event RXERROR"]
    #[inline(always)]
    pub const fn rxerror(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event RXERROR"]
    #[inline(always)]
    pub fn set_rxerror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDRX"]
    #[inline(always)]
    pub const fn endrx(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDRX"]
    #[inline(always)]
    pub fn set_endrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDTX"]
    #[inline(always)]
    pub const fn endtx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDTX"]
    #[inline(always)]
    pub fn set_endtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write '1' to enable interrupt for event AUTOCOLRESSTARTED"]
    #[inline(always)]
    pub const fn autocolresstarted(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event AUTOCOLRESSTARTED"]
    #[inline(always)]
    pub fn set_autocolresstarted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Write '1' to enable interrupt for event COLLISION"]
    #[inline(always)]
    pub const fn collision(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event COLLISION"]
    #[inline(always)]
    pub fn set_collision(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to enable interrupt for event SELECTED"]
    #[inline(always)]
    pub const fn selected(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event SELECTED"]
    #[inline(always)]
    pub fn set_selected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    pub const fn started(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    pub fn set_started(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Maxlen(pub u32);
impl Maxlen {
    #[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each"]
    #[inline(always)]
    pub const fn maxlen(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each"]
    #[inline(always)]
    pub fn set_maxlen(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Maxlen {
    #[inline(always)]
    fn default() -> Maxlen {
        Maxlen(0)
    }
}
#[doc = "Enables the modulation output to a GPIO pin which can be connected to a second external antenna."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modulationctrl(pub u32);
impl Modulationctrl {
    #[doc = "Configuration of modulation control."]
    #[inline(always)]
    pub const fn modulationctrl(&self) -> super::vals::Modulationctrl {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Modulationctrl::from_bits(val as u8)
    }
    #[doc = "Configuration of modulation control."]
    #[inline(always)]
    pub fn set_modulationctrl(&mut self, val: super::vals::Modulationctrl) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Modulationctrl {
    #[inline(always)]
    fn default() -> Modulationctrl {
        Modulationctrl(0)
    }
}
#[doc = "Pin select for Modulation control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modulationpsel(pub u32);
impl Modulationpsel {
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
impl Default for Modulationpsel {
    #[inline(always)]
    fn default() -> Modulationpsel {
        Modulationpsel(0)
    }
}
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nfcid12ndLast(pub u32);
impl Nfcid12ndLast {
    #[doc = "NFCID1 byte V"]
    #[inline(always)]
    pub const fn nfcid1_v(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "NFCID1 byte V"]
    #[inline(always)]
    pub fn set_nfcid1_v(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "NFCID1 byte U"]
    #[inline(always)]
    pub const fn nfcid1_u(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "NFCID1 byte U"]
    #[inline(always)]
    pub fn set_nfcid1_u(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "NFCID1 byte T"]
    #[inline(always)]
    pub const fn nfcid1_t(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "NFCID1 byte T"]
    #[inline(always)]
    pub fn set_nfcid1_t(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Nfcid12ndLast {
    #[inline(always)]
    fn default() -> Nfcid12ndLast {
        Nfcid12ndLast(0)
    }
}
#[doc = "Third last NFCID1 part (10 bytes ID)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nfcid13rdLast(pub u32);
impl Nfcid13rdLast {
    #[doc = "NFCID1 byte S"]
    #[inline(always)]
    pub const fn nfcid1_s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "NFCID1 byte S"]
    #[inline(always)]
    pub fn set_nfcid1_s(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "NFCID1 byte R"]
    #[inline(always)]
    pub const fn nfcid1_r(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "NFCID1 byte R"]
    #[inline(always)]
    pub fn set_nfcid1_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "NFCID1 byte Q"]
    #[inline(always)]
    pub const fn nfcid1_q(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "NFCID1 byte Q"]
    #[inline(always)]
    pub fn set_nfcid1_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Nfcid13rdLast {
    #[inline(always)]
    fn default() -> Nfcid13rdLast {
        Nfcid13rdLast(0)
    }
}
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nfcid1last(pub u32);
impl Nfcid1last {
    #[doc = "NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    pub const fn nfcid1_z(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    pub fn set_nfcid1_z(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "NFCID1 byte Y"]
    #[inline(always)]
    pub const fn nfcid1_y(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "NFCID1 byte Y"]
    #[inline(always)]
    pub fn set_nfcid1_y(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "NFCID1 byte X"]
    #[inline(always)]
    pub const fn nfcid1_x(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "NFCID1 byte X"]
    #[inline(always)]
    pub fn set_nfcid1_x(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "NFCID1 byte W"]
    #[inline(always)]
    pub const fn nfcid1_w(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "NFCID1 byte W"]
    #[inline(always)]
    pub fn set_nfcid1_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Nfcid1last {
    #[inline(always)]
    fn default() -> Nfcid1last {
        Nfcid1last(0)
    }
}
#[doc = "Current operating state of NFC tag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nfctagstate(pub u32);
impl Nfctagstate {
    #[doc = "NfcTag state"]
    #[inline(always)]
    pub const fn nfctagstate(&self) -> super::vals::Nfctagstate {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Nfctagstate::from_bits(val as u8)
    }
    #[doc = "NfcTag state"]
    #[inline(always)]
    pub fn set_nfctagstate(&mut self, val: super::vals::Nfctagstate) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Nfctagstate {
    #[inline(always)]
    fn default() -> Nfctagstate {
        Nfctagstate(0)
    }
}
#[doc = "Publish configuration for event AUTOCOLRESSTARTED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishAutocolresstarted(pub u32);
impl PublishAutocolresstarted {
    #[doc = "DPPI channel that event AUTOCOLRESSTARTED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event AUTOCOLRESSTARTED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishAutocolresstartedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishAutocolresstartedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishAutocolresstartedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishAutocolresstarted {
    #[inline(always)]
    fn default() -> PublishAutocolresstarted {
        PublishAutocolresstarted(0)
    }
}
#[doc = "Publish configuration for event COLLISION"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishCollision(pub u32);
impl PublishCollision {
    #[doc = "DPPI channel that event COLLISION will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event COLLISION will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishCollisionEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishCollisionEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishCollisionEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishCollision {
    #[inline(always)]
    fn default() -> PublishCollision {
        PublishCollision(0)
    }
}
#[doc = "Publish configuration for event ENDRX"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEndrx(pub u32);
impl PublishEndrx {
    #[doc = "DPPI channel that event ENDRX will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event ENDRX will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEndrxEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEndrxEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEndrxEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEndrx {
    #[inline(always)]
    fn default() -> PublishEndrx {
        PublishEndrx(0)
    }
}
#[doc = "Publish configuration for event ENDTX"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEndtx(pub u32);
impl PublishEndtx {
    #[doc = "DPPI channel that event ENDTX will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event ENDTX will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEndtxEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEndtxEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEndtxEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEndtx {
    #[inline(always)]
    fn default() -> PublishEndtx {
        PublishEndtx(0)
    }
}
#[doc = "Publish configuration for event ERROR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishError(pub u32);
impl PublishError {
    #[doc = "DPPI channel that event ERROR will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event ERROR will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishErrorEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishErrorEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishErrorEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishError {
    #[inline(always)]
    fn default() -> PublishError {
        PublishError(0)
    }
}
#[doc = "Publish configuration for event FIELDDETECTED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishFielddetected(pub u32);
impl PublishFielddetected {
    #[doc = "DPPI channel that event FIELDDETECTED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event FIELDDETECTED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishFielddetectedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishFielddetectedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishFielddetectedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishFielddetected {
    #[inline(always)]
    fn default() -> PublishFielddetected {
        PublishFielddetected(0)
    }
}
#[doc = "Publish configuration for event FIELDLOST"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishFieldlost(pub u32);
impl PublishFieldlost {
    #[doc = "DPPI channel that event FIELDLOST will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event FIELDLOST will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishFieldlostEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishFieldlostEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishFieldlostEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishFieldlost {
    #[inline(always)]
    fn default() -> PublishFieldlost {
        PublishFieldlost(0)
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
#[doc = "Publish configuration for event RXERROR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishRxerror(pub u32);
impl PublishRxerror {
    #[doc = "DPPI channel that event RXERROR will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event RXERROR will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishRxerrorEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishRxerrorEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishRxerrorEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishRxerror {
    #[inline(always)]
    fn default() -> PublishRxerror {
        PublishRxerror(0)
    }
}
#[doc = "Publish configuration for event RXFRAMEEND"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishRxframeend(pub u32);
impl PublishRxframeend {
    #[doc = "DPPI channel that event RXFRAMEEND will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event RXFRAMEEND will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishRxframeendEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishRxframeendEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishRxframeendEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishRxframeend {
    #[inline(always)]
    fn default() -> PublishRxframeend {
        PublishRxframeend(0)
    }
}
#[doc = "Publish configuration for event RXFRAMESTART"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishRxframestart(pub u32);
impl PublishRxframestart {
    #[doc = "DPPI channel that event RXFRAMESTART will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event RXFRAMESTART will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishRxframestartEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishRxframestartEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishRxframestartEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishRxframestart {
    #[inline(always)]
    fn default() -> PublishRxframestart {
        PublishRxframestart(0)
    }
}
#[doc = "Publish configuration for event SELECTED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishSelected(pub u32);
impl PublishSelected {
    #[doc = "DPPI channel that event SELECTED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event SELECTED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishSelectedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishSelectedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishSelectedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishSelected {
    #[inline(always)]
    fn default() -> PublishSelected {
        PublishSelected(0)
    }
}
#[doc = "Publish configuration for event STARTED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishStarted(pub u32);
impl PublishStarted {
    #[doc = "DPPI channel that event STARTED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event STARTED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishStartedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishStartedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishStartedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishStarted {
    #[inline(always)]
    fn default() -> PublishStarted {
        PublishStarted(0)
    }
}
#[doc = "Publish configuration for event TXFRAMEEND"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishTxframeend(pub u32);
impl PublishTxframeend {
    #[doc = "DPPI channel that event TXFRAMEEND will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event TXFRAMEEND will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishTxframeendEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishTxframeendEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishTxframeendEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishTxframeend {
    #[inline(always)]
    fn default() -> PublishTxframeend {
        PublishTxframeend(0)
    }
}
#[doc = "Publish configuration for event TXFRAMESTART"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishTxframestart(pub u32);
impl PublishTxframestart {
    #[doc = "DPPI channel that event TXFRAMESTART will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event TXFRAMESTART will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishTxframestartEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishTxframestartEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishTxframestartEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishTxframestart {
    #[inline(always)]
    fn default() -> PublishTxframestart {
        PublishTxframestart(0)
    }
}
#[doc = "Result of last incoming frame"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx(pub u32);
impl Rx {
    #[doc = "No valid end of frame (EoF) detected"]
    #[inline(always)]
    pub const fn crcerror(&self) -> super::vals::Crcerror {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Crcerror::from_bits(val as u8)
    }
    #[doc = "No valid end of frame (EoF) detected"]
    #[inline(always)]
    pub fn set_crcerror(&mut self, val: super::vals::Crcerror) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Parity status of received frame"]
    #[inline(always)]
    pub const fn paritystatus(&self) -> super::vals::Paritystatus {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Paritystatus::from_bits(val as u8)
    }
    #[doc = "Parity status of received frame"]
    #[inline(always)]
    pub fn set_paritystatus(&mut self, val: super::vals::Paritystatus) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Overrun detected"]
    #[inline(always)]
    pub const fn overrun(&self) -> super::vals::Overrun {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Overrun::from_bits(val as u8)
    }
    #[doc = "Overrun detected"]
    #[inline(always)]
    pub fn set_overrun(&mut self, val: super::vals::Overrun) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Rx {
    #[inline(always)]
    fn default() -> Rx {
        Rx(0)
    }
}
#[doc = "Size of last incoming frame"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxdAmount(pub u32);
impl RxdAmount {
    #[doc = "Number of bits in the last byte in the frame, if less than 8 (including CRC, but excluding parity and SoF/EoF framing)."]
    #[inline(always)]
    pub const fn rxdatabits(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Number of bits in the last byte in the frame, if less than 8 (including CRC, but excluding parity and SoF/EoF framing)."]
    #[inline(always)]
    pub fn set_rxdatabits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Number of complete bytes received in the frame (including CRC, but excluding parity and SoF/EoF framing)"]
    #[inline(always)]
    pub const fn rxdatabytes(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x01ff;
        val as u16
    }
    #[doc = "Number of complete bytes received in the frame (including CRC, but excluding parity and SoF/EoF framing)"]
    #[inline(always)]
    pub fn set_rxdatabytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 3usize)) | (((val as u32) & 0x01ff) << 3usize);
    }
}
impl Default for RxdAmount {
    #[inline(always)]
    fn default() -> RxdAmount {
        RxdAmount(0)
    }
}
#[doc = "Configuration of incoming frames"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxdFrameconfig(pub u32);
impl RxdFrameconfig {
    #[doc = "Indicates if parity expected in RX frame"]
    #[inline(always)]
    pub const fn parity(&self) -> super::vals::RxdFrameconfigParity {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RxdFrameconfigParity::from_bits(val as u8)
    }
    #[doc = "Indicates if parity expected in RX frame"]
    #[inline(always)]
    pub fn set_parity(&mut self, val: super::vals::RxdFrameconfigParity) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SoF expected or not in RX frames"]
    #[inline(always)]
    pub const fn sof(&self) -> super::vals::RxdFrameconfigSof {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RxdFrameconfigSof::from_bits(val as u8)
    }
    #[doc = "SoF expected or not in RX frames"]
    #[inline(always)]
    pub fn set_sof(&mut self, val: super::vals::RxdFrameconfigSof) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CRC mode for incoming frames"]
    #[inline(always)]
    pub const fn crcmoderx(&self) -> super::vals::Crcmoderx {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Crcmoderx::from_bits(val as u8)
    }
    #[doc = "CRC mode for incoming frames"]
    #[inline(always)]
    pub fn set_crcmoderx(&mut self, val: super::vals::Crcmoderx) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for RxdFrameconfig {
    #[inline(always)]
    fn default() -> RxdFrameconfig {
        RxdFrameconfig(0)
    }
}
#[doc = "NFC-A SEL_RES auto-response settings"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Selres(pub u32);
impl Selres {
    #[doc = "Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub const fn rfu10(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn set_rfu10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Cascade as defined by the b3 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification (controlled by hardware, shall be 0)"]
    #[inline(always)]
    pub const fn cascade(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Cascade as defined by the b3 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification (controlled by hardware, shall be 0)"]
    #[inline(always)]
    pub fn set_cascade(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub const fn rfu43(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn set_rfu43(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub const fn protocol(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn set_protocol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub const fn rfu7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn set_rfu7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Selres {
    #[inline(always)]
    fn default() -> Selres {
        Selres(0)
    }
}
#[doc = "NFC-A SENS_RES auto-response settings"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sensres(pub u32);
impl Sensres {
    #[doc = "Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub const fn bitframesdd(&self) -> super::vals::Bitframesdd {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Bitframesdd::from_bits(val as u8)
    }
    #[doc = "Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn set_bitframesdd(&mut self, val: super::vals::Bitframesdd) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub const fn rfu5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn set_rfu5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NFCID1 size. This value is used by the auto collision resolution engine."]
    #[inline(always)]
    pub const fn nfcidsize(&self) -> super::vals::Nfcidsize {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Nfcidsize::from_bits(val as u8)
    }
    #[doc = "NFCID1 size. This value is used by the auto collision resolution engine."]
    #[inline(always)]
    pub fn set_nfcidsize(&mut self, val: super::vals::Nfcidsize) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub const fn platfconfig(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn set_platfconfig(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub const fn rfu74(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn set_rfu74(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Sensres {
    #[inline(always)]
    fn default() -> Sensres {
        Sensres(0)
    }
}
#[doc = "Shortcuts between local events and tasks"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    pub const fn fielddetected_activate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    pub fn set_fielddetected_activate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    pub const fn fieldlost_sense(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    pub fn set_fieldlost_sense(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
    #[inline(always)]
    pub const fn txframeend_enablerxdata(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
    #[inline(always)]
    pub fn set_txframeend_enablerxdata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Sleep state during automatic collision resolution"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sleepstate(pub u32);
impl Sleepstate {
    #[doc = "Reflects the sleep state during automatic collision resolution. Set to IDLE by a GOIDLE task. Set to SLEEP_A when a valid SLEEP_REQ frame is received or by a GOSLEEP task."]
    #[inline(always)]
    pub const fn sleepstate(&self) -> super::vals::Sleepstate {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sleepstate::from_bits(val as u8)
    }
    #[doc = "Reflects the sleep state during automatic collision resolution. Set to IDLE by a GOIDLE task. Set to SLEEP_A when a valid SLEEP_REQ frame is received or by a GOSLEEP task."]
    #[inline(always)]
    pub fn set_sleepstate(&mut self, val: super::vals::Sleepstate) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sleepstate {
    #[inline(always)]
    fn default() -> Sleepstate {
        Sleepstate(0)
    }
}
#[doc = "Subscribe configuration for task ACTIVATE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeActivate(pub u32);
impl SubscribeActivate {
    #[doc = "DPPI channel that task ACTIVATE will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task ACTIVATE will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeActivateEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeActivateEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeActivateEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeActivate {
    #[inline(always)]
    fn default() -> SubscribeActivate {
        SubscribeActivate(0)
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
#[doc = "Subscribe configuration for task ENABLERXDATA"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeEnablerxdata(pub u32);
impl SubscribeEnablerxdata {
    #[doc = "DPPI channel that task ENABLERXDATA will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task ENABLERXDATA will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeEnablerxdataEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeEnablerxdataEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeEnablerxdataEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeEnablerxdata {
    #[inline(always)]
    fn default() -> SubscribeEnablerxdata {
        SubscribeEnablerxdata(0)
    }
}
#[doc = "Subscribe configuration for task GOIDLE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeGoidle(pub u32);
impl SubscribeGoidle {
    #[doc = "DPPI channel that task GOIDLE will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task GOIDLE will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeGoidleEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeGoidleEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeGoidleEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeGoidle {
    #[inline(always)]
    fn default() -> SubscribeGoidle {
        SubscribeGoidle(0)
    }
}
#[doc = "Subscribe configuration for task GOSLEEP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeGosleep(pub u32);
impl SubscribeGosleep {
    #[doc = "DPPI channel that task GOSLEEP will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task GOSLEEP will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeGosleepEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeGosleepEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeGosleepEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeGosleep {
    #[inline(always)]
    fn default() -> SubscribeGosleep {
        SubscribeGosleep(0)
    }
}
#[doc = "Subscribe configuration for task SENSE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeSense(pub u32);
impl SubscribeSense {
    #[doc = "DPPI channel that task SENSE will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task SENSE will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeSenseEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeSenseEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeSenseEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeSense {
    #[inline(always)]
    fn default() -> SubscribeSense {
        SubscribeSense(0)
    }
}
#[doc = "Subscribe configuration for task STARTTX"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeStarttx(pub u32);
impl SubscribeStarttx {
    #[doc = "DPPI channel that task STARTTX will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task STARTTX will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeStarttxEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeStarttxEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeStarttxEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeStarttx {
    #[inline(always)]
    fn default() -> SubscribeStarttx {
        SubscribeStarttx(0)
    }
}
#[doc = "Size of outgoing frame"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxdAmount(pub u32);
impl TxdAmount {
    #[doc = "Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
    #[inline(always)]
    pub const fn txdatabits(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
    #[inline(always)]
    pub fn set_txdatabits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Number of complete bytes that shall be included in the frame, excluding CRC, parity, and framing."]
    #[inline(always)]
    pub const fn txdatabytes(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x01ff;
        val as u16
    }
    #[doc = "Number of complete bytes that shall be included in the frame, excluding CRC, parity, and framing."]
    #[inline(always)]
    pub fn set_txdatabytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 3usize)) | (((val as u32) & 0x01ff) << 3usize);
    }
}
impl Default for TxdAmount {
    #[inline(always)]
    fn default() -> TxdAmount {
        TxdAmount(0)
    }
}
#[doc = "Configuration of outgoing frames"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxdFrameconfig(pub u32);
impl TxdFrameconfig {
    #[doc = "Indicates if parity is added to the frame"]
    #[inline(always)]
    pub const fn parity(&self) -> super::vals::TxdFrameconfigParity {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TxdFrameconfigParity::from_bits(val as u8)
    }
    #[doc = "Indicates if parity is added to the frame"]
    #[inline(always)]
    pub fn set_parity(&mut self, val: super::vals::TxdFrameconfigParity) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Discarding unused bits at start or end of a frame"]
    #[inline(always)]
    pub const fn discardmode(&self) -> super::vals::Discardmode {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Discardmode::from_bits(val as u8)
    }
    #[doc = "Discarding unused bits at start or end of a frame"]
    #[inline(always)]
    pub fn set_discardmode(&mut self, val: super::vals::Discardmode) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Adding SoF or not in TX frames"]
    #[inline(always)]
    pub const fn sof(&self) -> super::vals::TxdFrameconfigSof {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::TxdFrameconfigSof::from_bits(val as u8)
    }
    #[doc = "Adding SoF or not in TX frames"]
    #[inline(always)]
    pub fn set_sof(&mut self, val: super::vals::TxdFrameconfigSof) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CRC mode for outgoing frames"]
    #[inline(always)]
    pub const fn crcmodetx(&self) -> super::vals::Crcmodetx {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Crcmodetx::from_bits(val as u8)
    }
    #[doc = "CRC mode for outgoing frames"]
    #[inline(always)]
    pub fn set_crcmodetx(&mut self, val: super::vals::Crcmodetx) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for TxdFrameconfig {
    #[inline(always)]
    fn default() -> TxdFrameconfig {
        TxdFrameconfig(0)
    }
}
