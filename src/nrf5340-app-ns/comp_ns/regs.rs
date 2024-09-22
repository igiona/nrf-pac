#[doc = "COMP enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable COMP"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enable or disable COMP"]
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
#[doc = "External reference select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extrefsel(pub u32);
impl Extrefsel {
    #[doc = "External analog reference select"]
    #[inline(always)]
    pub const fn extrefsel(&self) -> super::vals::Extrefsel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Extrefsel::from_bits(val as u8)
    }
    #[doc = "External analog reference select"]
    #[inline(always)]
    pub fn set_extrefsel(&mut self, val: super::vals::Extrefsel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Extrefsel {
    #[inline(always)]
    fn default() -> Extrefsel {
        Extrefsel(0)
    }
}
#[doc = "Comparator hysteresis enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hyst(pub u32);
impl Hyst {
    #[doc = "Comparator hysteresis"]
    #[inline(always)]
    pub const fn hyst(&self) -> super::vals::Hyst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hyst::from_bits(val as u8)
    }
    #[doc = "Comparator hysteresis"]
    #[inline(always)]
    pub fn set_hyst(&mut self, val: super::vals::Hyst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Hyst {
    #[inline(always)]
    fn default() -> Hyst {
        Hyst(0)
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
    #[doc = "Write '1' to enable interrupt for event DOWN"]
    #[inline(always)]
    pub const fn down(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event DOWN"]
    #[inline(always)]
    pub fn set_down(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable interrupt for event UP"]
    #[inline(always)]
    pub const fn up(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event UP"]
    #[inline(always)]
    pub fn set_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to enable interrupt for event CROSS"]
    #[inline(always)]
    pub const fn cross(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event CROSS"]
    #[inline(always)]
    pub fn set_cross(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Current source select on analog input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isource(pub u32);
impl Isource {
    #[doc = "Comparator hysteresis"]
    #[inline(always)]
    pub const fn isource(&self) -> super::vals::Isource {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Isource::from_bits(val as u8)
    }
    #[doc = "Comparator hysteresis"]
    #[inline(always)]
    pub fn set_isource(&mut self, val: super::vals::Isource) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Isource {
    #[inline(always)]
    fn default() -> Isource {
        Isource(0)
    }
}
#[doc = "Mode configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "Speed and power modes"]
    #[inline(always)]
    pub const fn sp(&self) -> super::vals::Sp {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sp::from_bits(val as u8)
    }
    #[doc = "Speed and power modes"]
    #[inline(always)]
    pub fn set_sp(&mut self, val: super::vals::Sp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Main operation modes"]
    #[inline(always)]
    pub const fn main(&self) -> super::vals::Main {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Main::from_bits(val as u8)
    }
    #[doc = "Main operation modes"]
    #[inline(always)]
    pub fn set_main(&mut self, val: super::vals::Main) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "Pin select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel(pub u32);
impl Psel {
    #[doc = "Analog pin select"]
    #[inline(always)]
    pub const fn psel(&self) -> super::vals::Psel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Psel::from_bits(val as u8)
    }
    #[doc = "Analog pin select"]
    #[inline(always)]
    pub fn set_psel(&mut self, val: super::vals::Psel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Psel {
    #[inline(always)]
    fn default() -> Psel {
        Psel(0)
    }
}
#[doc = "Publish configuration for event CROSS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishCross(pub u32);
impl PublishCross {
    #[doc = "DPPI channel that event CROSS will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event CROSS will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishCrossEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishCrossEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishCrossEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishCross {
    #[inline(always)]
    fn default() -> PublishCross {
        PublishCross(0)
    }
}
#[doc = "Publish configuration for event DOWN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishDown(pub u32);
impl PublishDown {
    #[doc = "DPPI channel that event DOWN will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event DOWN will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishDownEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishDownEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishDownEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishDown {
    #[inline(always)]
    fn default() -> PublishDown {
        PublishDown(0)
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
#[doc = "Publish configuration for event UP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishUp(pub u32);
impl PublishUp {
    #[doc = "DPPI channel that event UP will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event UP will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishUpEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishUpEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishUpEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishUp {
    #[inline(always)]
    fn default() -> PublishUp {
        PublishUp(0)
    }
}
#[doc = "Reference source select for single-ended mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Refsel(pub u32);
impl Refsel {
    #[doc = "Reference select"]
    #[inline(always)]
    pub const fn refsel(&self) -> super::vals::Refsel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Refsel::from_bits(val as u8)
    }
    #[doc = "Reference select"]
    #[inline(always)]
    pub fn set_refsel(&mut self, val: super::vals::Refsel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Refsel {
    #[inline(always)]
    fn default() -> Refsel {
        Refsel(0)
    }
}
#[doc = "Compare result"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Result(pub u32);
impl Result {
    #[doc = "Result of last compare. Decision point SAMPLE task."]
    #[inline(always)]
    pub const fn result(&self) -> super::vals::Result {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Result::from_bits(val as u8)
    }
    #[doc = "Result of last compare. Decision point SAMPLE task."]
    #[inline(always)]
    pub fn set_result(&mut self, val: super::vals::Result) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Result {
    #[inline(always)]
    fn default() -> Result {
        Result(0)
    }
}
#[doc = "Shortcuts between local events and tasks"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between event READY and task SAMPLE"]
    #[inline(always)]
    pub const fn ready_sample(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event READY and task SAMPLE"]
    #[inline(always)]
    pub fn set_ready_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shortcut between event READY and task STOP"]
    #[inline(always)]
    pub const fn ready_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event READY and task STOP"]
    #[inline(always)]
    pub fn set_ready_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Shortcut between event DOWN and task STOP"]
    #[inline(always)]
    pub const fn down_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event DOWN and task STOP"]
    #[inline(always)]
    pub fn set_down_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Shortcut between event UP and task STOP"]
    #[inline(always)]
    pub const fn up_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event UP and task STOP"]
    #[inline(always)]
    pub fn set_up_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Shortcut between event CROSS and task STOP"]
    #[inline(always)]
    pub const fn cross_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event CROSS and task STOP"]
    #[inline(always)]
    pub fn set_cross_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
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
#[doc = "Threshold configuration for hysteresis unit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Th(pub u32);
impl Th {
    #[doc = "VDOWN = (THDOWN+1)/64*VREF"]
    #[inline(always)]
    pub const fn thdown(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "VDOWN = (THDOWN+1)/64*VREF"]
    #[inline(always)]
    pub fn set_thdown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "VUP = (THUP+1)/64*VREF"]
    #[inline(always)]
    pub const fn thup(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "VUP = (THUP+1)/64*VREF"]
    #[inline(always)]
    pub fn set_thup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Th {
    #[inline(always)]
    fn default() -> Th {
        Th(0)
    }
}
