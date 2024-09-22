#[doc = "Automatic or manual control of HFCLK128M/HFCLK64M"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfclkalwaysrun(pub u32);
impl Hfclkalwaysrun {
    #[doc = "Ensure clock is always running"]
    #[inline(always)]
    pub const fn alwaysrun(&self) -> super::vals::HfclkalwaysrunAlwaysrun {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::HfclkalwaysrunAlwaysrun::from_bits(val as u8)
    }
    #[doc = "Ensure clock is always running"]
    #[inline(always)]
    pub fn set_alwaysrun(&mut self, val: super::vals::HfclkalwaysrunAlwaysrun) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Hfclkalwaysrun {
    #[inline(always)]
    fn default() -> Hfclkalwaysrun {
        Hfclkalwaysrun(0)
    }
}
#[doc = "HFCLK128M frequency configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfclkctrl(pub u32);
impl Hfclkctrl {
    #[doc = "High frequency clock HCLK"]
    #[inline(always)]
    pub const fn hclk(&self) -> super::vals::Hclk {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Hclk::from_bits(val as u8)
    }
    #[doc = "High frequency clock HCLK"]
    #[inline(always)]
    pub fn set_hclk(&mut self, val: super::vals::Hclk) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Hfclkctrl {
    #[inline(always)]
    fn default() -> Hfclkctrl {
        Hfclkctrl(0)
    }
}
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfclkrun(pub u32);
impl Hfclkrun {
    #[doc = "HFCLKSTART task triggered or not"]
    #[inline(always)]
    pub const fn status(&self) -> super::vals::HfclkrunStatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::HfclkrunStatus::from_bits(val as u8)
    }
    #[doc = "HFCLKSTART task triggered or not"]
    #[inline(always)]
    pub fn set_status(&mut self, val: super::vals::HfclkrunStatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Hfclkrun {
    #[inline(always)]
    fn default() -> Hfclkrun {
        Hfclkrun(0)
    }
}
#[doc = "Clock source for HFCLK128M/HFCLK64M"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfclksrc(pub u32);
impl Hfclksrc {
    #[doc = "Select which HFCLK source is started by the HFCLKSTART task"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::HfclksrcSrc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::HfclksrcSrc::from_bits(val as u8)
    }
    #[doc = "Select which HFCLK source is started by the HFCLKSTART task"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::HfclksrcSrc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Hfclksrc {
    #[inline(always)]
    fn default() -> Hfclksrc {
        Hfclksrc(0)
    }
}
#[doc = "Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfclkstat(pub u32);
impl Hfclkstat {
    #[doc = "Active clock source"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::HfclkstatSrc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::HfclkstatSrc::from_bits(val as u8)
    }
    #[doc = "Active clock source"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::HfclkstatSrc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "ALWAYSRUN activated"]
    #[inline(always)]
    pub const fn alwaysrunning(&self) -> super::vals::HfclkstatAlwaysrunning {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::HfclkstatAlwaysrunning::from_bits(val as u8)
    }
    #[doc = "ALWAYSRUN activated"]
    #[inline(always)]
    pub fn set_alwaysrunning(&mut self, val: super::vals::HfclkstatAlwaysrunning) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "HFCLK state"]
    #[inline(always)]
    pub const fn state(&self) -> super::vals::HfclkstatState {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::HfclkstatState::from_bits(val as u8)
    }
    #[doc = "HFCLK state"]
    #[inline(always)]
    pub fn set_state(&mut self, val: super::vals::HfclkstatState) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Hfclkstat {
    #[inline(always)]
    fn default() -> Hfclkstat {
        Hfclkstat(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub const fn hfclkstarted(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn set_hfclkstarted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub const fn lfclkstarted(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn set_lfclkstarted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for event DONE"]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event DONE"]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Pending interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intpend(pub u32);
impl Intpend {
    #[doc = "Read pending status of interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub const fn hfclkstarted(&self) -> super::vals::IntpendHfclkstarted {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IntpendHfclkstarted::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn set_hfclkstarted(&mut self, val: super::vals::IntpendHfclkstarted) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Read pending status of interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub const fn lfclkstarted(&self) -> super::vals::IntpendLfclkstarted {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IntpendLfclkstarted::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn set_lfclkstarted(&mut self, val: super::vals::IntpendLfclkstarted) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Read pending status of interrupt for event DONE"]
    #[inline(always)]
    pub const fn done(&self) -> super::vals::IntpendDone {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::IntpendDone::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event DONE"]
    #[inline(always)]
    pub fn set_done(&mut self, val: super::vals::IntpendDone) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Intpend {
    #[inline(always)]
    fn default() -> Intpend {
        Intpend(0)
    }
}
#[doc = "Automatic or manual control of LFCLK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfclkalwaysrun(pub u32);
impl Lfclkalwaysrun {
    #[doc = "Ensure clock is always running"]
    #[inline(always)]
    pub const fn alwaysrun(&self) -> super::vals::LfclkalwaysrunAlwaysrun {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LfclkalwaysrunAlwaysrun::from_bits(val as u8)
    }
    #[doc = "Ensure clock is always running"]
    #[inline(always)]
    pub fn set_alwaysrun(&mut self, val: super::vals::LfclkalwaysrunAlwaysrun) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lfclkalwaysrun {
    #[inline(always)]
    fn default() -> Lfclkalwaysrun {
        Lfclkalwaysrun(0)
    }
}
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfclkrun(pub u32);
impl Lfclkrun {
    #[doc = "LFCLKSTART task triggered or not"]
    #[inline(always)]
    pub const fn status(&self) -> super::vals::LfclkrunStatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LfclkrunStatus::from_bits(val as u8)
    }
    #[doc = "LFCLKSTART task triggered or not"]
    #[inline(always)]
    pub fn set_status(&mut self, val: super::vals::LfclkrunStatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lfclkrun {
    #[inline(always)]
    fn default() -> Lfclkrun {
        Lfclkrun(0)
    }
}
#[doc = "Clock source for LFCLK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfclksrc(pub u32);
impl Lfclksrc {
    #[doc = "Select which LFCLK source is started by the LFCLKSTART task"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::LfclksrcSrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::LfclksrcSrc::from_bits(val as u8)
    }
    #[doc = "Select which LFCLK source is started by the LFCLKSTART task"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::LfclksrcSrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lfclksrc {
    #[inline(always)]
    fn default() -> Lfclksrc {
        Lfclksrc(0)
    }
}
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfclksrccopy(pub u32);
impl Lfclksrccopy {
    #[doc = "Clock source"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::LfclksrccopySrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::LfclksrccopySrc::from_bits(val as u8)
    }
    #[doc = "Clock source"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::LfclksrccopySrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lfclksrccopy {
    #[inline(always)]
    fn default() -> Lfclksrccopy {
        Lfclksrccopy(0)
    }
}
#[doc = "Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfclkstat(pub u32);
impl Lfclkstat {
    #[doc = "Active clock source"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::LfclkstatSrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::LfclkstatSrc::from_bits(val as u8)
    }
    #[doc = "Active clock source"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::LfclkstatSrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ALWAYSRUN activated"]
    #[inline(always)]
    pub const fn alwaysrunning(&self) -> super::vals::LfclkstatAlwaysrunning {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::LfclkstatAlwaysrunning::from_bits(val as u8)
    }
    #[doc = "ALWAYSRUN activated"]
    #[inline(always)]
    pub fn set_alwaysrunning(&mut self, val: super::vals::LfclkstatAlwaysrunning) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "LFCLK state"]
    #[inline(always)]
    pub const fn state(&self) -> super::vals::LfclkstatState {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::LfclkstatState::from_bits(val as u8)
    }
    #[doc = "LFCLK state"]
    #[inline(always)]
    pub fn set_state(&mut self, val: super::vals::LfclkstatState) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Lfclkstat {
    #[inline(always)]
    fn default() -> Lfclkstat {
        Lfclkstat(0)
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
#[doc = "Publish configuration for event HFCLKSTARTED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishHfclkstarted(pub u32);
impl PublishHfclkstarted {
    #[doc = "DPPI channel that event HFCLKSTARTED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event HFCLKSTARTED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishHfclkstartedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishHfclkstartedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishHfclkstartedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishHfclkstarted {
    #[inline(always)]
    fn default() -> PublishHfclkstarted {
        PublishHfclkstarted(0)
    }
}
#[doc = "Publish configuration for event LFCLKSTARTED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishLfclkstarted(pub u32);
impl PublishLfclkstarted {
    #[doc = "DPPI channel that event LFCLKSTARTED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event LFCLKSTARTED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishLfclkstartedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishLfclkstartedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishLfclkstartedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishLfclkstarted {
    #[inline(always)]
    fn default() -> PublishLfclkstarted {
        PublishLfclkstarted(0)
    }
}
#[doc = "Subscribe configuration for task CAL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeCal(pub u32);
impl SubscribeCal {
    #[doc = "DPPI channel that task CAL will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task CAL will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeCalEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeCalEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeCalEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeCal {
    #[inline(always)]
    fn default() -> SubscribeCal {
        SubscribeCal(0)
    }
}
#[doc = "Subscribe configuration for task HFCLKSTART"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeHfclkstart(pub u32);
impl SubscribeHfclkstart {
    #[doc = "DPPI channel that task HFCLKSTART will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task HFCLKSTART will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeHfclkstartEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeHfclkstartEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeHfclkstartEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeHfclkstart {
    #[inline(always)]
    fn default() -> SubscribeHfclkstart {
        SubscribeHfclkstart(0)
    }
}
#[doc = "Subscribe configuration for task HFCLKSTOP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeHfclkstop(pub u32);
impl SubscribeHfclkstop {
    #[doc = "DPPI channel that task HFCLKSTOP will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task HFCLKSTOP will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeHfclkstopEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeHfclkstopEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeHfclkstopEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeHfclkstop {
    #[inline(always)]
    fn default() -> SubscribeHfclkstop {
        SubscribeHfclkstop(0)
    }
}
#[doc = "Subscribe configuration for task LFCLKSTART"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeLfclkstart(pub u32);
impl SubscribeLfclkstart {
    #[doc = "DPPI channel that task LFCLKSTART will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task LFCLKSTART will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeLfclkstartEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeLfclkstartEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeLfclkstartEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeLfclkstart {
    #[inline(always)]
    fn default() -> SubscribeLfclkstart {
        SubscribeLfclkstart(0)
    }
}
#[doc = "Subscribe configuration for task LFCLKSTOP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeLfclkstop(pub u32);
impl SubscribeLfclkstop {
    #[doc = "DPPI channel that task LFCLKSTOP will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task LFCLKSTOP will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeLfclkstopEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeLfclkstopEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeLfclkstopEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeLfclkstop {
    #[inline(always)]
    fn default() -> SubscribeLfclkstop {
        SubscribeLfclkstop(0)
    }
}
