#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub const fn sleep(&self) -> super::vals::Sleep {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sleep::from_bits(val as u8)
    }
    #[doc = "Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub fn set_sleep(&mut self, val: super::vals::Sleep) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Halt {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Halt::from_bits(val as u8)
    }
    #[doc = "Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn set_halt(&mut self, val: super::vals::Halt) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Allow stopping WDT"]
    #[inline(always)]
    pub const fn stopen(&self) -> super::vals::Stopen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Stopen::from_bits(val as u8)
    }
    #[doc = "Allow stopping WDT"]
    #[inline(always)]
    pub fn set_stopen(&mut self, val: super::vals::Stopen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
    #[inline(always)]
    pub fn set_timeout(&mut self, val: bool) {
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
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmien(pub u32);
impl Nmien {
    #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event TIMEOUT"]
    #[inline(always)]
    pub fn set_timeout(&mut self, val: bool) {
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
}
impl Default for Nmien {
    #[inline(always)]
    fn default() -> Nmien {
        Nmien(0)
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
#[doc = "Publish configuration for event TIMEOUT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishTimeout(pub u32);
impl PublishTimeout {
    #[doc = "DPPI channel that event TIMEOUT will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event TIMEOUT will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishTimeoutEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishTimeoutEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishTimeoutEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishTimeout {
    #[inline(always)]
    fn default() -> PublishTimeout {
        PublishTimeout(0)
    }
}
#[doc = "Request status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reqstatus(pub u32);
impl Reqstatus {
    #[doc = "Request status for RR\\[0\\] register"]
    #[inline(always)]
    pub const fn rr0(&self) -> super::vals::ReqstatusRr0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ReqstatusRr0::from_bits(val as u8)
    }
    #[doc = "Request status for RR\\[0\\] register"]
    #[inline(always)]
    pub fn set_rr0(&mut self, val: super::vals::ReqstatusRr0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Request status for RR\\[1\\] register"]
    #[inline(always)]
    pub const fn rr1(&self) -> super::vals::ReqstatusRr1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ReqstatusRr1::from_bits(val as u8)
    }
    #[doc = "Request status for RR\\[1\\] register"]
    #[inline(always)]
    pub fn set_rr1(&mut self, val: super::vals::ReqstatusRr1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Request status for RR\\[2\\] register"]
    #[inline(always)]
    pub const fn rr2(&self) -> super::vals::ReqstatusRr2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ReqstatusRr2::from_bits(val as u8)
    }
    #[doc = "Request status for RR\\[2\\] register"]
    #[inline(always)]
    pub fn set_rr2(&mut self, val: super::vals::ReqstatusRr2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Request status for RR\\[3\\] register"]
    #[inline(always)]
    pub const fn rr3(&self) -> super::vals::ReqstatusRr3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ReqstatusRr3::from_bits(val as u8)
    }
    #[doc = "Request status for RR\\[3\\] register"]
    #[inline(always)]
    pub fn set_rr3(&mut self, val: super::vals::ReqstatusRr3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Request status for RR\\[4\\] register"]
    #[inline(always)]
    pub const fn rr4(&self) -> super::vals::ReqstatusRr4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ReqstatusRr4::from_bits(val as u8)
    }
    #[doc = "Request status for RR\\[4\\] register"]
    #[inline(always)]
    pub fn set_rr4(&mut self, val: super::vals::ReqstatusRr4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Request status for RR\\[5\\] register"]
    #[inline(always)]
    pub const fn rr5(&self) -> super::vals::ReqstatusRr5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ReqstatusRr5::from_bits(val as u8)
    }
    #[doc = "Request status for RR\\[5\\] register"]
    #[inline(always)]
    pub fn set_rr5(&mut self, val: super::vals::ReqstatusRr5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Request status for RR\\[6\\] register"]
    #[inline(always)]
    pub const fn rr6(&self) -> super::vals::ReqstatusRr6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ReqstatusRr6::from_bits(val as u8)
    }
    #[doc = "Request status for RR\\[6\\] register"]
    #[inline(always)]
    pub fn set_rr6(&mut self, val: super::vals::ReqstatusRr6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Request status for RR\\[7\\] register"]
    #[inline(always)]
    pub const fn rr7(&self) -> super::vals::ReqstatusRr7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ReqstatusRr7::from_bits(val as u8)
    }
    #[doc = "Request status for RR\\[7\\] register"]
    #[inline(always)]
    pub fn set_rr7(&mut self, val: super::vals::ReqstatusRr7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Reqstatus {
    #[inline(always)]
    fn default() -> Reqstatus {
        Reqstatus(0)
    }
}
#[doc = "Description collection: Reload request n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rr(pub u32);
impl Rr {
    #[doc = "Reload request register"]
    #[inline(always)]
    pub const fn rr(&self) -> super::vals::Rr {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Rr::from_bits(val as u32)
    }
    #[doc = "Reload request register"]
    #[inline(always)]
    pub fn set_rr(&mut self, val: super::vals::Rr) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rr {
    #[inline(always)]
    fn default() -> Rr {
        Rr(0)
    }
}
#[doc = "Enable register for reload request registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rren(pub u32);
impl Rren {
    #[doc = "Enable or disable RR\\[0\\] register"]
    #[inline(always)]
    pub const fn rr0(&self) -> super::vals::RrenRr0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RrenRr0::from_bits(val as u8)
    }
    #[doc = "Enable or disable RR\\[0\\] register"]
    #[inline(always)]
    pub fn set_rr0(&mut self, val: super::vals::RrenRr0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable RR\\[1\\] register"]
    #[inline(always)]
    pub const fn rr1(&self) -> super::vals::RrenRr1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RrenRr1::from_bits(val as u8)
    }
    #[doc = "Enable or disable RR\\[1\\] register"]
    #[inline(always)]
    pub fn set_rr1(&mut self, val: super::vals::RrenRr1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable RR\\[2\\] register"]
    #[inline(always)]
    pub const fn rr2(&self) -> super::vals::RrenRr2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RrenRr2::from_bits(val as u8)
    }
    #[doc = "Enable or disable RR\\[2\\] register"]
    #[inline(always)]
    pub fn set_rr2(&mut self, val: super::vals::RrenRr2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable or disable RR\\[3\\] register"]
    #[inline(always)]
    pub const fn rr3(&self) -> super::vals::RrenRr3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::RrenRr3::from_bits(val as u8)
    }
    #[doc = "Enable or disable RR\\[3\\] register"]
    #[inline(always)]
    pub fn set_rr3(&mut self, val: super::vals::RrenRr3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable or disable RR\\[4\\] register"]
    #[inline(always)]
    pub const fn rr4(&self) -> super::vals::RrenRr4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::RrenRr4::from_bits(val as u8)
    }
    #[doc = "Enable or disable RR\\[4\\] register"]
    #[inline(always)]
    pub fn set_rr4(&mut self, val: super::vals::RrenRr4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable or disable RR\\[5\\] register"]
    #[inline(always)]
    pub const fn rr5(&self) -> super::vals::RrenRr5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::RrenRr5::from_bits(val as u8)
    }
    #[doc = "Enable or disable RR\\[5\\] register"]
    #[inline(always)]
    pub fn set_rr5(&mut self, val: super::vals::RrenRr5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable or disable RR\\[6\\] register"]
    #[inline(always)]
    pub const fn rr6(&self) -> super::vals::RrenRr6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::RrenRr6::from_bits(val as u8)
    }
    #[doc = "Enable or disable RR\\[6\\] register"]
    #[inline(always)]
    pub fn set_rr6(&mut self, val: super::vals::RrenRr6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable or disable RR\\[7\\] register"]
    #[inline(always)]
    pub const fn rr7(&self) -> super::vals::RrenRr7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::RrenRr7::from_bits(val as u8)
    }
    #[doc = "Enable or disable RR\\[7\\] register"]
    #[inline(always)]
    pub fn set_rr7(&mut self, val: super::vals::RrenRr7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Rren {
    #[inline(always)]
    fn default() -> Rren {
        Rren(0)
    }
}
#[doc = "Run status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Runstatus(pub u32);
impl Runstatus {
    #[doc = "Indicates whether or not WDT is running"]
    #[inline(always)]
    pub const fn runstatuswdt(&self) -> super::vals::Runstatuswdt {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Runstatuswdt::from_bits(val as u8)
    }
    #[doc = "Indicates whether or not WDT is running"]
    #[inline(always)]
    pub fn set_runstatuswdt(&mut self, val: super::vals::Runstatuswdt) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Runstatus {
    #[inline(always)]
    fn default() -> Runstatus {
        Runstatus(0)
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
#[doc = "Task stop enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsen(pub u32);
impl Tsen {
    #[doc = "Allow stopping WDT"]
    #[inline(always)]
    pub const fn tsen(&self) -> super::vals::Tsen {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Tsen::from_bits(val as u32)
    }
    #[doc = "Allow stopping WDT"]
    #[inline(always)]
    pub fn set_tsen(&mut self, val: super::vals::Tsen) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tsen {
    #[inline(always)]
    fn default() -> Tsen {
        Tsen(0)
    }
}
