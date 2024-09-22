#[doc = "Description collection: General purpose retention register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpregret(pub u32);
impl Gpregret {
    #[doc = "General purpose retention register"]
    #[inline(always)]
    pub const fn gpregret(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "General purpose retention register"]
    #[inline(always)]
    pub fn set_gpregret(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Gpregret {
    #[inline(always)]
    fn default() -> Gpregret {
        Gpregret(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event POFWARN"]
    #[inline(always)]
    pub const fn pofwarn(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event POFWARN"]
    #[inline(always)]
    pub fn set_pofwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to disable interrupt for event SLEEPENTER"]
    #[inline(always)]
    pub const fn sleepenter(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event SLEEPENTER"]
    #[inline(always)]
    pub fn set_sleepenter(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to disable interrupt for event SLEEPEXIT"]
    #[inline(always)]
    pub const fn sleepexit(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event SLEEPEXIT"]
    #[inline(always)]
    pub fn set_sleepexit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Publish configuration for event POFWARN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishPofwarn(pub u32);
impl PublishPofwarn {
    #[doc = "DPPI channel that event POFWARN will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event POFWARN will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishPofwarnEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishPofwarnEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishPofwarnEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishPofwarn {
    #[inline(always)]
    fn default() -> PublishPofwarn {
        PublishPofwarn(0)
    }
}
#[doc = "Publish configuration for event SLEEPENTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishSleepenter(pub u32);
impl PublishSleepenter {
    #[doc = "DPPI channel that event SLEEPENTER will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event SLEEPENTER will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishSleepenterEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishSleepenterEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishSleepenterEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishSleepenter {
    #[inline(always)]
    fn default() -> PublishSleepenter {
        PublishSleepenter(0)
    }
}
#[doc = "Publish configuration for event SLEEPEXIT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishSleepexit(pub u32);
impl PublishSleepexit {
    #[doc = "DPPI channel that event SLEEPEXIT will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event SLEEPEXIT will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishSleepexitEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishSleepexitEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishSleepexitEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishSleepexit {
    #[inline(always)]
    fn default() -> PublishSleepexit {
        PublishSleepexit(0)
    }
}
#[doc = "Subscribe configuration for task CONSTLAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeConstlat(pub u32);
impl SubscribeConstlat {
    #[doc = "DPPI channel that task CONSTLAT will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task CONSTLAT will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeConstlatEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeConstlatEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeConstlatEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeConstlat {
    #[inline(always)]
    fn default() -> SubscribeConstlat {
        SubscribeConstlat(0)
    }
}
#[doc = "Subscribe configuration for task LOWPWR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeLowpwr(pub u32);
impl SubscribeLowpwr {
    #[doc = "DPPI channel that task LOWPWR will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task LOWPWR will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeLowpwrEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeLowpwrEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeLowpwrEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeLowpwr {
    #[inline(always)]
    fn default() -> SubscribeLowpwr {
        SubscribeLowpwr(0)
    }
}
