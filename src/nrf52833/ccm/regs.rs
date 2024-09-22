#[doc = "Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable CCM"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enable or disable CCM"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Enable {
    #[inline(always)]
    fn default() -> Enable {
        Enable(0)
    }
}
#[doc = "Header (S0) mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Headermask(pub u32);
impl Headermask {
    #[doc = "Header (S0) mask"]
    #[inline(always)]
    pub const fn headermask(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Header (S0) mask"]
    #[inline(always)]
    pub fn set_headermask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Headermask {
    #[inline(always)]
    fn default() -> Headermask {
        Headermask(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event ENDKSGEN"]
    #[inline(always)]
    pub const fn endksgen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event ENDKSGEN"]
    #[inline(always)]
    pub fn set_endksgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for event ENDCRYPT"]
    #[inline(always)]
    pub const fn endcrypt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event ENDCRYPT"]
    #[inline(always)]
    pub fn set_endcrypt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Length of keystream generated when MODE.LENGTH = Extended"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Maxpacketsize(pub u32);
impl Maxpacketsize {
    #[doc = "Length of keystream generated when MODE.LENGTH = Extended. This value must be greater than or equal to the subsequent packet payload to be encrypted/decrypted."]
    #[inline(always)]
    pub const fn maxpacketsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Length of keystream generated when MODE.LENGTH = Extended. This value must be greater than or equal to the subsequent packet payload to be encrypted/decrypted."]
    #[inline(always)]
    pub fn set_maxpacketsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Maxpacketsize {
    #[inline(always)]
    fn default() -> Maxpacketsize {
        Maxpacketsize(0)
    }
}
#[doc = "MIC check result"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Micstatus(pub u32);
impl Micstatus {
    #[doc = "The result of the MIC check performed during the previous decryption operation"]
    #[inline(always)]
    pub const fn micstatus(&self) -> super::vals::Micstatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Micstatus::from_bits(val as u8)
    }
    #[doc = "The result of the MIC check performed during the previous decryption operation"]
    #[inline(always)]
    pub fn set_micstatus(&mut self, val: super::vals::Micstatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Micstatus {
    #[inline(always)]
    fn default() -> Micstatus {
        Micstatus(0)
    }
}
#[doc = "Operation mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "The mode of operation to be used. Settings in this register apply whenever either the KSGEN task or the CRYPT task is triggered."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "The mode of operation to be used. Settings in this register apply whenever either the KSGEN task or the CRYPT task is triggered."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Radio data rate that the CCM shall run synchronous with"]
    #[inline(always)]
    pub const fn datarate(&self) -> super::vals::Datarate {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Datarate::from_bits(val as u8)
    }
    #[doc = "Radio data rate that the CCM shall run synchronous with"]
    #[inline(always)]
    pub fn set_datarate(&mut self, val: super::vals::Datarate) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Packet length configuration"]
    #[inline(always)]
    pub const fn length(&self) -> super::vals::Length {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Length::from_bits(val as u8)
    }
    #[doc = "Packet length configuration"]
    #[inline(always)]
    pub fn set_length(&mut self, val: super::vals::Length) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "Data rate override setting."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rateoverride(pub u32);
impl Rateoverride {
    #[doc = "Data rate override setting"]
    #[inline(always)]
    pub const fn rateoverride(&self) -> super::vals::Rateoverride {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Rateoverride::from_bits(val as u8)
    }
    #[doc = "Data rate override setting"]
    #[inline(always)]
    pub fn set_rateoverride(&mut self, val: super::vals::Rateoverride) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Rateoverride {
    #[inline(always)]
    fn default() -> Rateoverride {
        Rateoverride(0)
    }
}
#[doc = "Shortcuts between local events and tasks"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between event ENDKSGEN and task CRYPT"]
    #[inline(always)]
    pub const fn endksgen_crypt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event ENDKSGEN and task CRYPT"]
    #[inline(always)]
    pub fn set_endksgen_crypt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
