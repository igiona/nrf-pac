#[doc = "Number of buffer words transferred since last START"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amount(pub u32);
impl Amount {
    #[doc = "Number of buffer words transferred since last START. This register can be read after an END or STOPPED event."]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Number of buffer words transferred since last START. This register can be read after an END or STOPPED event."]
    #[inline(always)]
    pub fn set_amount(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Amount {
    #[inline(always)]
    fn default() -> Amount {
        Amount(0)
    }
}
#[doc = "Description cluster: Input configuration for CH\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Positive channel resistor control"]
    #[inline(always)]
    pub const fn resp(&self) -> super::vals::Resp {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Resp::from_bits(val as u8)
    }
    #[doc = "Positive channel resistor control"]
    #[inline(always)]
    pub fn set_resp(&mut self, val: super::vals::Resp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Negative channel resistor control"]
    #[inline(always)]
    pub const fn resn(&self) -> super::vals::Resn {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Resn::from_bits(val as u8)
    }
    #[doc = "Negative channel resistor control"]
    #[inline(always)]
    pub fn set_resn(&mut self, val: super::vals::Resn) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Gain control"]
    #[inline(always)]
    pub const fn gain(&self) -> super::vals::Gain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Gain::from_bits(val as u8)
    }
    #[doc = "Gain control"]
    #[inline(always)]
    pub fn set_gain(&mut self, val: super::vals::Gain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Reference control"]
    #[inline(always)]
    pub const fn refsel(&self) -> super::vals::Refsel {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Refsel::from_bits(val as u8)
    }
    #[doc = "Reference control"]
    #[inline(always)]
    pub fn set_refsel(&mut self, val: super::vals::Refsel) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Acquisition time, the time the ADC uses to sample the input voltage"]
    #[inline(always)]
    pub const fn tacq(&self) -> super::vals::Tacq {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Tacq::from_bits(val as u8)
    }
    #[doc = "Acquisition time, the time the ADC uses to sample the input voltage"]
    #[inline(always)]
    pub fn set_tacq(&mut self, val: super::vals::Tacq) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Enable differential mode"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::ConfigMode {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::ConfigMode::from_bits(val as u8)
    }
    #[doc = "Enable differential mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::ConfigMode) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable burst mode"]
    #[inline(always)]
    pub const fn burst(&self) -> super::vals::Burst {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Burst::from_bits(val as u8)
    }
    #[doc = "Enable burst mode"]
    #[inline(always)]
    pub fn set_burst(&mut self, val: super::vals::Burst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "Enable or disable ADC"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable ADC"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enable or disable ADC"]
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
#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    pub const fn started(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    pub fn set_started(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to enable interrupt for event END"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event END"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable interrupt for event DONE"]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event DONE"]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to enable interrupt for event RESULTDONE"]
    #[inline(always)]
    pub const fn resultdone(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event RESULTDONE"]
    #[inline(always)]
    pub fn set_resultdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to enable interrupt for event CALIBRATEDONE"]
    #[inline(always)]
    pub const fn calibratedone(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CALIBRATEDONE"]
    #[inline(always)]
    pub fn set_calibratedone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub const fn stopped(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn set_stopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH0LIMITH"]
    #[inline(always)]
    pub const fn ch0limith(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH0LIMITH"]
    #[inline(always)]
    pub fn set_ch0limith(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH0LIMITL"]
    #[inline(always)]
    pub const fn ch0limitl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH0LIMITL"]
    #[inline(always)]
    pub fn set_ch0limitl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH1LIMITH"]
    #[inline(always)]
    pub const fn ch1limith(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH1LIMITH"]
    #[inline(always)]
    pub fn set_ch1limith(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH1LIMITL"]
    #[inline(always)]
    pub const fn ch1limitl(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH1LIMITL"]
    #[inline(always)]
    pub fn set_ch1limitl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH2LIMITH"]
    #[inline(always)]
    pub const fn ch2limith(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH2LIMITH"]
    #[inline(always)]
    pub fn set_ch2limith(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH2LIMITL"]
    #[inline(always)]
    pub const fn ch2limitl(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH2LIMITL"]
    #[inline(always)]
    pub fn set_ch2limitl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH3LIMITH"]
    #[inline(always)]
    pub const fn ch3limith(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH3LIMITH"]
    #[inline(always)]
    pub fn set_ch3limith(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH3LIMITL"]
    #[inline(always)]
    pub const fn ch3limitl(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH3LIMITL"]
    #[inline(always)]
    pub fn set_ch3limitl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH4LIMITH"]
    #[inline(always)]
    pub const fn ch4limith(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH4LIMITH"]
    #[inline(always)]
    pub fn set_ch4limith(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH4LIMITL"]
    #[inline(always)]
    pub const fn ch4limitl(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH4LIMITL"]
    #[inline(always)]
    pub fn set_ch4limitl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH5LIMITH"]
    #[inline(always)]
    pub const fn ch5limith(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH5LIMITH"]
    #[inline(always)]
    pub fn set_ch5limith(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH5LIMITL"]
    #[inline(always)]
    pub const fn ch5limitl(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH5LIMITL"]
    #[inline(always)]
    pub fn set_ch5limitl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH6LIMITH"]
    #[inline(always)]
    pub const fn ch6limith(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH6LIMITH"]
    #[inline(always)]
    pub fn set_ch6limith(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH6LIMITL"]
    #[inline(always)]
    pub const fn ch6limitl(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH6LIMITL"]
    #[inline(always)]
    pub fn set_ch6limitl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH7LIMITH"]
    #[inline(always)]
    pub const fn ch7limith(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH7LIMITH"]
    #[inline(always)]
    pub fn set_ch7limith(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write '1' to enable interrupt for event CH7LIMITL"]
    #[inline(always)]
    pub const fn ch7limitl(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CH7LIMITL"]
    #[inline(always)]
    pub fn set_ch7limitl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Description cluster: High/low limits for event monitoring a channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Limit(pub u32);
impl Limit {
    #[doc = "Low level limit"]
    #[inline(always)]
    pub const fn low(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Low level limit"]
    #[inline(always)]
    pub fn set_low(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "High level limit"]
    #[inline(always)]
    pub const fn high(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "High level limit"]
    #[inline(always)]
    pub fn set_high(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Limit {
    #[inline(always)]
    fn default() -> Limit {
        Limit(0)
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
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Maximum number of buffer words to transfer"]
    #[inline(always)]
    pub fn set_maxcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Maxcnt {
    #[inline(always)]
    fn default() -> Maxcnt {
        Maxcnt(0)
    }
}
#[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oversample(pub u32);
impl Oversample {
    #[doc = "Oversample control"]
    #[inline(always)]
    pub const fn oversample(&self) -> super::vals::Oversample {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Oversample::from_bits(val as u8)
    }
    #[doc = "Oversample control"]
    #[inline(always)]
    pub fn set_oversample(&mut self, val: super::vals::Oversample) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Oversample {
    #[inline(always)]
    fn default() -> Oversample {
        Oversample(0)
    }
}
#[doc = "Description cluster: Input negative pin selection for CH\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pseln(pub u32);
impl Pseln {
    #[doc = "Analog negative input, enables differential channel"]
    #[inline(always)]
    pub const fn pseln(&self) -> super::vals::Pseln {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Pseln::from_bits(val as u8)
    }
    #[doc = "Analog negative input, enables differential channel"]
    #[inline(always)]
    pub fn set_pseln(&mut self, val: super::vals::Pseln) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Pseln {
    #[inline(always)]
    fn default() -> Pseln {
        Pseln(0)
    }
}
#[doc = "Description cluster: Input positive pin selection for CH\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pselp(pub u32);
impl Pselp {
    #[doc = "Analog positive input channel"]
    #[inline(always)]
    pub const fn pselp(&self) -> super::vals::Pselp {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Pselp::from_bits(val as u8)
    }
    #[doc = "Analog positive input channel"]
    #[inline(always)]
    pub fn set_pselp(&mut self, val: super::vals::Pselp) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Pselp {
    #[inline(always)]
    fn default() -> Pselp {
        Pselp(0)
    }
}
#[doc = "Publish configuration for event CALIBRATEDONE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishCalibratedone(pub u32);
impl PublishCalibratedone {
    #[doc = "DPPI channel that event CALIBRATEDONE will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event CALIBRATEDONE will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishCalibratedoneEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishCalibratedoneEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishCalibratedoneEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishCalibratedone {
    #[inline(always)]
    fn default() -> PublishCalibratedone {
        PublishCalibratedone(0)
    }
}
#[doc = "Description cluster: Publish configuration for event CH\\[n\\].LIMITH"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishChLimith(pub u32);
impl PublishChLimith {
    #[doc = "DPPI channel that event CH\\[n\\].LIMITH will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event CH\\[n\\].LIMITH will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::LimithEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::LimithEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::LimithEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishChLimith {
    #[inline(always)]
    fn default() -> PublishChLimith {
        PublishChLimith(0)
    }
}
#[doc = "Description cluster: Publish configuration for event CH\\[n\\].LIMITL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishChLimitl(pub u32);
impl PublishChLimitl {
    #[doc = "DPPI channel that event CH\\[n\\].LIMITL will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event CH\\[n\\].LIMITL will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::LimitlEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::LimitlEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::LimitlEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishChLimitl {
    #[inline(always)]
    fn default() -> PublishChLimitl {
        PublishChLimitl(0)
    }
}
#[doc = "Publish configuration for event DONE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishDone(pub u32);
impl PublishDone {
    #[doc = "DPPI channel that event DONE will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event DONE will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishDoneEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishDoneEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishDoneEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishDone {
    #[inline(always)]
    fn default() -> PublishDone {
        PublishDone(0)
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
#[doc = "Publish configuration for event RESULTDONE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishResultdone(pub u32);
impl PublishResultdone {
    #[doc = "DPPI channel that event RESULTDONE will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event RESULTDONE will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishResultdoneEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishResultdoneEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishResultdoneEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishResultdone {
    #[inline(always)]
    fn default() -> PublishResultdone {
        PublishResultdone(0)
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
#[doc = "Resolution configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resolution(pub u32);
impl Resolution {
    #[doc = "Set the resolution"]
    #[inline(always)]
    pub const fn val(&self) -> super::vals::Val {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Val::from_bits(val as u8)
    }
    #[doc = "Set the resolution"]
    #[inline(always)]
    pub fn set_val(&mut self, val: super::vals::Val) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Resolution {
    #[inline(always)]
    fn default() -> Resolution {
        Resolution(0)
    }
}
#[doc = "Controls normal or continuous sample rate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Samplerate(pub u32);
impl Samplerate {
    #[doc = "Capture and compare value; sample rate is 16 MHz/CC"]
    #[inline(always)]
    pub const fn cc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Capture and compare value; sample rate is 16 MHz/CC"]
    #[inline(always)]
    pub fn set_cc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Select mode for sample rate control"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::SamplerateMode {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SamplerateMode::from_bits(val as u8)
    }
    #[doc = "Select mode for sample rate control"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::SamplerateMode) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
}
impl Default for Samplerate {
    #[inline(always)]
    fn default() -> Samplerate {
        Samplerate(0)
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Status"]
    #[inline(always)]
    pub const fn status(&self) -> super::vals::Status {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Status::from_bits(val as u8)
    }
    #[doc = "Status"]
    #[inline(always)]
    pub fn set_status(&mut self, val: super::vals::Status) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "Subscribe configuration for task CALIBRATEOFFSET"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeCalibrateoffset(pub u32);
impl SubscribeCalibrateoffset {
    #[doc = "DPPI channel that task CALIBRATEOFFSET will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task CALIBRATEOFFSET will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeCalibrateoffsetEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeCalibrateoffsetEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeCalibrateoffsetEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeCalibrateoffset {
    #[inline(always)]
    fn default() -> SubscribeCalibrateoffset {
        SubscribeCalibrateoffset(0)
    }
}
#[doc = "Subscribe configuration for task SAMPLE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeSample(pub u32);
impl SubscribeSample {
    #[doc = "DPPI channel that task SAMPLE will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task SAMPLE will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeSampleEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeSampleEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeSampleEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeSample {
    #[inline(always)]
    fn default() -> SubscribeSample {
        SubscribeSample(0)
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
