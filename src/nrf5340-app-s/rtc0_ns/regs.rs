#[doc = "Description collection: Compare register n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc = "Compare value"]
    #[inline(always)]
    pub const fn compare(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Compare value"]
    #[inline(always)]
    pub fn set_compare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Cc {
    #[inline(always)]
    fn default() -> Cc {
        Cc(0)
    }
}
#[doc = "Current counter value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Counter(pub u32);
impl Counter {
    #[doc = "Counter value"]
    #[inline(always)]
    pub const fn counter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Counter value"]
    #[inline(always)]
    pub fn set_counter(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Counter {
    #[inline(always)]
    fn default() -> Counter {
        Counter(0)
    }
}
#[doc = "Enable or disable event routing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evten(pub u32);
impl Evten {
    #[doc = "Enable or disable event routing for event TICK"]
    #[inline(always)]
    pub const fn tick(&self) -> super::vals::EvtenTick {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EvtenTick::from_bits(val as u8)
    }
    #[doc = "Enable or disable event routing for event TICK"]
    #[inline(always)]
    pub fn set_tick(&mut self, val: super::vals::EvtenTick) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable event routing for event OVRFLW"]
    #[inline(always)]
    pub const fn ovrflw(&self) -> super::vals::EvtenOvrflw {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EvtenOvrflw::from_bits(val as u8)
    }
    #[doc = "Enable or disable event routing for event OVRFLW"]
    #[inline(always)]
    pub fn set_ovrflw(&mut self, val: super::vals::EvtenOvrflw) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable event routing for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub const fn compare0(&self) -> super::vals::EvtenCompare0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::EvtenCompare0::from_bits(val as u8)
    }
    #[doc = "Enable or disable event routing for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn set_compare0(&mut self, val: super::vals::EvtenCompare0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable or disable event routing for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub const fn compare1(&self) -> super::vals::EvtenCompare1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::EvtenCompare1::from_bits(val as u8)
    }
    #[doc = "Enable or disable event routing for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn set_compare1(&mut self, val: super::vals::EvtenCompare1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable or disable event routing for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub const fn compare2(&self) -> super::vals::EvtenCompare2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::EvtenCompare2::from_bits(val as u8)
    }
    #[doc = "Enable or disable event routing for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn set_compare2(&mut self, val: super::vals::EvtenCompare2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable or disable event routing for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub const fn compare3(&self) -> super::vals::EvtenCompare3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::EvtenCompare3::from_bits(val as u8)
    }
    #[doc = "Enable or disable event routing for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn set_compare3(&mut self, val: super::vals::EvtenCompare3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Evten {
    #[inline(always)]
    fn default() -> Evten {
        Evten(0)
    }
}
#[doc = "Disable event routing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evtenclr(pub u32);
impl Evtenclr {
    #[doc = "Write '1' to disable event routing for event TICK"]
    #[inline(always)]
    pub const fn tick(&self) -> super::vals::EvtenclrTick {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EvtenclrTick::from_bits(val as u8)
    }
    #[doc = "Write '1' to disable event routing for event TICK"]
    #[inline(always)]
    pub fn set_tick(&mut self, val: super::vals::EvtenclrTick) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable event routing for event OVRFLW"]
    #[inline(always)]
    pub const fn ovrflw(&self) -> super::vals::EvtenclrOvrflw {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EvtenclrOvrflw::from_bits(val as u8)
    }
    #[doc = "Write '1' to disable event routing for event OVRFLW"]
    #[inline(always)]
    pub fn set_ovrflw(&mut self, val: super::vals::EvtenclrOvrflw) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable event routing for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub const fn compare0(&self) -> super::vals::EvtenclrCompare0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::EvtenclrCompare0::from_bits(val as u8)
    }
    #[doc = "Write '1' to disable event routing for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn set_compare0(&mut self, val: super::vals::EvtenclrCompare0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to disable event routing for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub const fn compare1(&self) -> super::vals::EvtenclrCompare1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::EvtenclrCompare1::from_bits(val as u8)
    }
    #[doc = "Write '1' to disable event routing for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn set_compare1(&mut self, val: super::vals::EvtenclrCompare1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to disable event routing for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub const fn compare2(&self) -> super::vals::EvtenclrCompare2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::EvtenclrCompare2::from_bits(val as u8)
    }
    #[doc = "Write '1' to disable event routing for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn set_compare2(&mut self, val: super::vals::EvtenclrCompare2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to disable event routing for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub const fn compare3(&self) -> super::vals::EvtenclrCompare3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::EvtenclrCompare3::from_bits(val as u8)
    }
    #[doc = "Write '1' to disable event routing for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn set_compare3(&mut self, val: super::vals::EvtenclrCompare3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Evtenclr {
    #[inline(always)]
    fn default() -> Evtenclr {
        Evtenclr(0)
    }
}
#[doc = "Enable event routing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evtenset(pub u32);
impl Evtenset {
    #[doc = "Write '1' to enable event routing for event TICK"]
    #[inline(always)]
    pub const fn tick(&self) -> super::vals::EvtensetTick {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EvtensetTick::from_bits(val as u8)
    }
    #[doc = "Write '1' to enable event routing for event TICK"]
    #[inline(always)]
    pub fn set_tick(&mut self, val: super::vals::EvtensetTick) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to enable event routing for event OVRFLW"]
    #[inline(always)]
    pub const fn ovrflw(&self) -> super::vals::EvtensetOvrflw {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EvtensetOvrflw::from_bits(val as u8)
    }
    #[doc = "Write '1' to enable event routing for event OVRFLW"]
    #[inline(always)]
    pub fn set_ovrflw(&mut self, val: super::vals::EvtensetOvrflw) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable event routing for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub const fn compare0(&self) -> super::vals::EvtensetCompare0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::EvtensetCompare0::from_bits(val as u8)
    }
    #[doc = "Write '1' to enable event routing for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn set_compare0(&mut self, val: super::vals::EvtensetCompare0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to enable event routing for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub const fn compare1(&self) -> super::vals::EvtensetCompare1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::EvtensetCompare1::from_bits(val as u8)
    }
    #[doc = "Write '1' to enable event routing for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn set_compare1(&mut self, val: super::vals::EvtensetCompare1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to enable event routing for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub const fn compare2(&self) -> super::vals::EvtensetCompare2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::EvtensetCompare2::from_bits(val as u8)
    }
    #[doc = "Write '1' to enable event routing for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn set_compare2(&mut self, val: super::vals::EvtensetCompare2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to enable event routing for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub const fn compare3(&self) -> super::vals::EvtensetCompare3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::EvtensetCompare3::from_bits(val as u8)
    }
    #[doc = "Write '1' to enable event routing for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn set_compare3(&mut self, val: super::vals::EvtensetCompare3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Evtenset {
    #[inline(always)]
    fn default() -> Evtenset {
        Evtenset(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event TICK"]
    #[inline(always)]
    pub const fn tick(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event TICK"]
    #[inline(always)]
    pub fn set_tick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for event OVRFLW"]
    #[inline(always)]
    pub const fn ovrflw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event OVRFLW"]
    #[inline(always)]
    pub fn set_ovrflw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
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
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "12-bit prescaler for counter frequency (32768 / (PRESCALER + 1)). Must be written when RTC is stopped."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prescaler(pub u32);
impl Prescaler {
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub const fn prescaler(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub fn set_prescaler(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
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
#[doc = "Publish configuration for event OVRFLW"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishOvrflw(pub u32);
impl PublishOvrflw {
    #[doc = "DPPI channel that event OVRFLW will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event OVRFLW will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishOvrflwEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishOvrflwEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishOvrflwEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishOvrflw {
    #[inline(always)]
    fn default() -> PublishOvrflw {
        PublishOvrflw(0)
    }
}
#[doc = "Publish configuration for event TICK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishTick(pub u32);
impl PublishTick {
    #[doc = "DPPI channel that event TICK will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event TICK will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishTickEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishTickEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishTickEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishTick {
    #[inline(always)]
    fn default() -> PublishTick {
        PublishTick(0)
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
#[doc = "Subscribe configuration for task TRIGOVRFLW"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeTrigovrflw(pub u32);
impl SubscribeTrigovrflw {
    #[doc = "DPPI channel that task TRIGOVRFLW will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task TRIGOVRFLW will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeTrigovrflwEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeTrigovrflwEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeTrigovrflwEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeTrigovrflw {
    #[inline(always)]
    fn default() -> SubscribeTrigovrflw {
        SubscribeTrigovrflw(0)
    }
}
