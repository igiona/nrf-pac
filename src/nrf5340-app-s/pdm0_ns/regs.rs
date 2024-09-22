#[doc = "Pin number configuration for PDM CLK signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clk(pub u32);
impl Clk {
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
    pub const fn connect(&self) -> super::vals::ClkConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkConnect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::ClkConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Clk {
    #[inline(always)]
    fn default() -> Clk {
        Clk(0)
    }
}
#[doc = "Pin number configuration for PDM DIN signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Din(pub u32);
impl Din {
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
    pub const fn connect(&self) -> super::vals::DinConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::DinConnect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::DinConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Din {
    #[inline(always)]
    fn default() -> Din {
        Din(0)
    }
}
#[doc = "PDM module enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable PDM module"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enable or disable PDM module"]
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
#[doc = "Left output gain adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gainl(pub u32);
impl Gainl {
    #[doc = "Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    pub const fn gainl(&self) -> super::vals::Gainl {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Gainl::from_bits(val as u8)
    }
    #[doc = "Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    pub fn set_gainl(&mut self, val: super::vals::Gainl) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Gainl {
    #[inline(always)]
    fn default() -> Gainl {
        Gainl(0)
    }
}
#[doc = "Right output gain adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gainr(pub u32);
impl Gainr {
    #[doc = "Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    pub const fn gainr(&self) -> super::vals::Gainr {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Gainr::from_bits(val as u8)
    }
    #[doc = "Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    pub fn set_gainr(&mut self, val: super::vals::Gainr) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Gainr {
    #[inline(always)]
    fn default() -> Gainr {
        Gainr(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    pub const fn started(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn set_started(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    pub const fn stopped(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn set_stopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Number of samples to allocate memory for in EasyDMA mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Maxcnt(pub u32);
impl Maxcnt {
    #[doc = "Length of DMA RAM allocation in number of samples"]
    #[inline(always)]
    pub const fn buffsize(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Length of DMA RAM allocation in number of samples"]
    #[inline(always)]
    pub fn set_buffsize(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Maxcnt {
    #[inline(always)]
    fn default() -> Maxcnt {
        Maxcnt(0)
    }
}
#[doc = "Master clock generator configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mclkconfig(pub u32);
impl Mclkconfig {
    #[doc = "Master clock source selection"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::Src {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Src::from_bits(val as u8)
    }
    #[doc = "Master clock source selection"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::Src) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Mclkconfig {
    #[inline(always)]
    fn default() -> Mclkconfig {
        Mclkconfig(0)
    }
}
#[doc = "Defines the routing of the connected PDM microphones' signals"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "Mono or stereo operation"]
    #[inline(always)]
    pub const fn operation(&self) -> super::vals::Operation {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Operation::from_bits(val as u8)
    }
    #[doc = "Mono or stereo operation"]
    #[inline(always)]
    pub fn set_operation(&mut self, val: super::vals::Operation) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Defines on which PDM_CLK edge left (or mono) is sampled"]
    #[inline(always)]
    pub const fn edge(&self) -> super::vals::Edge {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Edge::from_bits(val as u8)
    }
    #[doc = "Defines on which PDM_CLK edge left (or mono) is sampled"]
    #[inline(always)]
    pub fn set_edge(&mut self, val: super::vals::Edge) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "PDM clock generator control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdmclkctrl(pub u32);
impl Pdmclkctrl {
    #[doc = "PDM_CLK frequency configuration. Enumerations are deprecated, use PDMCLKCTRL equation to find the register value. The 12 least significant bits of the register are ignored and shall be set to zero."]
    #[inline(always)]
    pub const fn freq(&self) -> super::vals::Freq {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Freq::from_bits(val as u32)
    }
    #[doc = "PDM_CLK frequency configuration. Enumerations are deprecated, use PDMCLKCTRL equation to find the register value. The 12 least significant bits of the register are ignored and shall be set to zero."]
    #[inline(always)]
    pub fn set_freq(&mut self, val: super::vals::Freq) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pdmclkctrl {
    #[inline(always)]
    fn default() -> Pdmclkctrl {
        Pdmclkctrl(0)
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
#[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ratio(pub u32);
impl Ratio {
    #[doc = "Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    pub const fn ratio(&self) -> super::vals::Ratio {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ratio::from_bits(val as u8)
    }
    #[doc = "Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    pub fn set_ratio(&mut self, val: super::vals::Ratio) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ratio {
    #[inline(always)]
    fn default() -> Ratio {
        Ratio(0)
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
