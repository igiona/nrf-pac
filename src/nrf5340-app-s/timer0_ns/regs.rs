#[doc = "Configure the number of bits used by the TIMER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bitmode(pub u32);
impl Bitmode {
    #[doc = "Timer bit width"]
    #[inline(always)]
    pub const fn bitmode(&self) -> super::vals::Bitmode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bitmode::from_bits(val as u8)
    }
    #[doc = "Timer bit width"]
    #[inline(always)]
    pub fn set_bitmode(&mut self, val: super::vals::Bitmode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Bitmode {
    #[inline(always)]
    fn default() -> Bitmode {
        Bitmode(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub const fn compare0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn set_compare0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub const fn compare1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn set_compare1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub const fn compare2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn set_compare2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub const fn compare3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn set_compare3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[4\\]"]
    #[inline(always)]
    pub const fn compare4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[4\\]"]
    #[inline(always)]
    pub fn set_compare4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[5\\]"]
    #[inline(always)]
    pub const fn compare5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event COMPARE\\[5\\]"]
    #[inline(always)]
    pub fn set_compare5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Timer mode selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "Timer mode"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oneshoten(pub u32);
impl Oneshoten {
    #[doc = "Enable one-shot operation"]
    #[inline(always)]
    pub const fn oneshoten(&self) -> super::vals::Oneshoten {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Oneshoten::from_bits(val as u8)
    }
    #[doc = "Enable one-shot operation"]
    #[inline(always)]
    pub fn set_oneshoten(&mut self, val: super::vals::Oneshoten) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Oneshoten {
    #[inline(always)]
    fn default() -> Oneshoten {
        Oneshoten(0)
    }
}
#[doc = "Timer prescaler register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prescaler(pub u32);
impl Prescaler {
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub const fn prescaler(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub fn set_prescaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Prescaler {
    #[inline(always)]
    fn default() -> Prescaler {
        Prescaler(0)
    }
}
#[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishCompare(pub u32);
impl PublishCompare {
    #[doc = "DPPI channel that event COMPARE\\[n\\] will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event COMPARE\\[n\\] will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishCompareEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishCompareEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishCompareEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishCompare {
    #[inline(always)]
    fn default() -> PublishCompare {
        PublishCompare(0)
    }
}
#[doc = "Shortcuts between local events and tasks"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between event COMPARE\\[0\\] and task CLEAR"]
    #[inline(always)]
    pub const fn compare0_clear(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[0\\] and task CLEAR"]
    #[inline(always)]
    pub fn set_compare0_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shortcut between event COMPARE\\[1\\] and task CLEAR"]
    #[inline(always)]
    pub const fn compare1_clear(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[1\\] and task CLEAR"]
    #[inline(always)]
    pub fn set_compare1_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Shortcut between event COMPARE\\[2\\] and task CLEAR"]
    #[inline(always)]
    pub const fn compare2_clear(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[2\\] and task CLEAR"]
    #[inline(always)]
    pub fn set_compare2_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Shortcut between event COMPARE\\[3\\] and task CLEAR"]
    #[inline(always)]
    pub const fn compare3_clear(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[3\\] and task CLEAR"]
    #[inline(always)]
    pub fn set_compare3_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Shortcut between event COMPARE\\[4\\] and task CLEAR"]
    #[inline(always)]
    pub const fn compare4_clear(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[4\\] and task CLEAR"]
    #[inline(always)]
    pub fn set_compare4_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Shortcut between event COMPARE\\[5\\] and task CLEAR"]
    #[inline(always)]
    pub const fn compare5_clear(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[5\\] and task CLEAR"]
    #[inline(always)]
    pub fn set_compare5_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Shortcut between event COMPARE\\[0\\] and task STOP"]
    #[inline(always)]
    pub const fn compare0_stop(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[0\\] and task STOP"]
    #[inline(always)]
    pub fn set_compare0_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Shortcut between event COMPARE\\[1\\] and task STOP"]
    #[inline(always)]
    pub const fn compare1_stop(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[1\\] and task STOP"]
    #[inline(always)]
    pub fn set_compare1_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Shortcut between event COMPARE\\[2\\] and task STOP"]
    #[inline(always)]
    pub const fn compare2_stop(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[2\\] and task STOP"]
    #[inline(always)]
    pub fn set_compare2_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Shortcut between event COMPARE\\[3\\] and task STOP"]
    #[inline(always)]
    pub const fn compare3_stop(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[3\\] and task STOP"]
    #[inline(always)]
    pub fn set_compare3_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Shortcut between event COMPARE\\[4\\] and task STOP"]
    #[inline(always)]
    pub const fn compare4_stop(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[4\\] and task STOP"]
    #[inline(always)]
    pub fn set_compare4_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Shortcut between event COMPARE\\[5\\] and task STOP"]
    #[inline(always)]
    pub const fn compare5_stop(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event COMPARE\\[5\\] and task STOP"]
    #[inline(always)]
    pub fn set_compare5_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeCapture(pub u32);
impl SubscribeCapture {
    #[doc = "DPPI channel that task CAPTURE\\[n\\] will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task CAPTURE\\[n\\] will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeCaptureEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeCaptureEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeCaptureEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeCapture {
    #[inline(always)]
    fn default() -> SubscribeCapture {
        SubscribeCapture(0)
    }
}
#[doc = "Subscribe configuration for task CLEAR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeClear(pub u32);
impl SubscribeClear {
    #[doc = "DPPI channel that task CLEAR will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task CLEAR will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeClearEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeClearEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeClearEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeClear {
    #[inline(always)]
    fn default() -> SubscribeClear {
        SubscribeClear(0)
    }
}
#[doc = "Subscribe configuration for task COUNT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeCount(pub u32);
impl SubscribeCount {
    #[doc = "DPPI channel that task COUNT will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task COUNT will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeCountEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeCountEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeCountEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeCount {
    #[inline(always)]
    fn default() -> SubscribeCount {
        SubscribeCount(0)
    }
}
#[doc = "Deprecated register - Subscribe configuration for task SHUTDOWN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeShutdown(pub u32);
impl SubscribeShutdown {
    #[doc = "DPPI channel that task SHUTDOWN will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task SHUTDOWN will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeShutdownEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeShutdownEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeShutdownEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeShutdown {
    #[inline(always)]
    fn default() -> SubscribeShutdown {
        SubscribeShutdown(0)
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
