#[doc = "Slope of first piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A0(pub u32);
impl A0 {
    #[doc = "Slope of first piecewise linear function"]
    #[inline(always)]
    pub const fn a0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of first piecewise linear function"]
    #[inline(always)]
    pub fn set_a0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A0 {
    #[inline(always)]
    fn default() -> A0 {
        A0(0)
    }
}
#[doc = "Slope of second piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A1(pub u32);
impl A1 {
    #[doc = "Slope of second piecewise linear function"]
    #[inline(always)]
    pub const fn a1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of second piecewise linear function"]
    #[inline(always)]
    pub fn set_a1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A1 {
    #[inline(always)]
    fn default() -> A1 {
        A1(0)
    }
}
#[doc = "Slope of third piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A2(pub u32);
impl A2 {
    #[doc = "Slope of third piecewise linear function"]
    #[inline(always)]
    pub const fn a2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of third piecewise linear function"]
    #[inline(always)]
    pub fn set_a2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A2 {
    #[inline(always)]
    fn default() -> A2 {
        A2(0)
    }
}
#[doc = "Slope of fourth piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A3(pub u32);
impl A3 {
    #[doc = "Slope of fourth piecewise linear function"]
    #[inline(always)]
    pub const fn a3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of fourth piecewise linear function"]
    #[inline(always)]
    pub fn set_a3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A3 {
    #[inline(always)]
    fn default() -> A3 {
        A3(0)
    }
}
#[doc = "Slope of fifth piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A4(pub u32);
impl A4 {
    #[doc = "Slope of fifth piecewise linear function"]
    #[inline(always)]
    pub const fn a4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of fifth piecewise linear function"]
    #[inline(always)]
    pub fn set_a4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A4 {
    #[inline(always)]
    fn default() -> A4 {
        A4(0)
    }
}
#[doc = "Slope of sixth piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A5(pub u32);
impl A5 {
    #[doc = "Slope of sixth piecewise linear function"]
    #[inline(always)]
    pub const fn a5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of sixth piecewise linear function"]
    #[inline(always)]
    pub fn set_a5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A5 {
    #[inline(always)]
    fn default() -> A5 {
        A5(0)
    }
}
#[doc = "y-intercept of first piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0(pub u32);
impl B0 {
    #[doc = "y-intercept of first piecewise linear function"]
    #[inline(always)]
    pub const fn b0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "y-intercept of first piecewise linear function"]
    #[inline(always)]
    pub fn set_b0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for B0 {
    #[inline(always)]
    fn default() -> B0 {
        B0(0)
    }
}
#[doc = "y-intercept of second piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B1(pub u32);
impl B1 {
    #[doc = "y-intercept of second piecewise linear function"]
    #[inline(always)]
    pub const fn b1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "y-intercept of second piecewise linear function"]
    #[inline(always)]
    pub fn set_b1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for B1 {
    #[inline(always)]
    fn default() -> B1 {
        B1(0)
    }
}
#[doc = "y-intercept of third piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B2(pub u32);
impl B2 {
    #[doc = "y-intercept of third piecewise linear function"]
    #[inline(always)]
    pub const fn b2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "y-intercept of third piecewise linear function"]
    #[inline(always)]
    pub fn set_b2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for B2 {
    #[inline(always)]
    fn default() -> B2 {
        B2(0)
    }
}
#[doc = "y-intercept of fourth piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B3(pub u32);
impl B3 {
    #[doc = "y-intercept of fourth piecewise linear function"]
    #[inline(always)]
    pub const fn b3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "y-intercept of fourth piecewise linear function"]
    #[inline(always)]
    pub fn set_b3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for B3 {
    #[inline(always)]
    fn default() -> B3 {
        B3(0)
    }
}
#[doc = "y-intercept of fifth piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B4(pub u32);
impl B4 {
    #[doc = "y-intercept of fifth piecewise linear function"]
    #[inline(always)]
    pub const fn b4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "y-intercept of fifth piecewise linear function"]
    #[inline(always)]
    pub fn set_b4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for B4 {
    #[inline(always)]
    fn default() -> B4 {
        B4(0)
    }
}
#[doc = "y-intercept of sixth piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B5(pub u32);
impl B5 {
    #[doc = "y-intercept of sixth piecewise linear function"]
    #[inline(always)]
    pub const fn b5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "y-intercept of sixth piecewise linear function"]
    #[inline(always)]
    pub fn set_b5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for B5 {
    #[inline(always)]
    fn default() -> B5 {
        B5(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event DATARDY"]
    #[inline(always)]
    pub const fn datardy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event DATARDY"]
    #[inline(always)]
    pub fn set_datardy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Publish configuration for event DATARDY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishDatardy(pub u32);
impl PublishDatardy {
    #[doc = "DPPI channel that event DATARDY will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event DATARDY will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishDatardyEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishDatardyEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishDatardyEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishDatardy {
    #[inline(always)]
    fn default() -> PublishDatardy {
        PublishDatardy(0)
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
#[doc = "Endpoint of first piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0(pub u32);
impl T0 {
    #[doc = "Endpoint of first piecewise linear function"]
    #[inline(always)]
    pub const fn t0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint of first piecewise linear function"]
    #[inline(always)]
    pub fn set_t0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T0 {
    #[inline(always)]
    fn default() -> T0 {
        T0(0)
    }
}
#[doc = "Endpoint of second piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T1(pub u32);
impl T1 {
    #[doc = "Endpoint of second piecewise linear function"]
    #[inline(always)]
    pub const fn t1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint of second piecewise linear function"]
    #[inline(always)]
    pub fn set_t1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T1 {
    #[inline(always)]
    fn default() -> T1 {
        T1(0)
    }
}
#[doc = "Endpoint of third piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T2(pub u32);
impl T2 {
    #[doc = "Endpoint of third piecewise linear function"]
    #[inline(always)]
    pub const fn t2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint of third piecewise linear function"]
    #[inline(always)]
    pub fn set_t2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T2 {
    #[inline(always)]
    fn default() -> T2 {
        T2(0)
    }
}
#[doc = "Endpoint of fourth piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T3(pub u32);
impl T3 {
    #[doc = "Endpoint of fourth piecewise linear function"]
    #[inline(always)]
    pub const fn t3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint of fourth piecewise linear function"]
    #[inline(always)]
    pub fn set_t3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T3 {
    #[inline(always)]
    fn default() -> T3 {
        T3(0)
    }
}
#[doc = "Endpoint of fifth piecewise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T4(pub u32);
impl T4 {
    #[doc = "Endpoint of fifth piecewise linear function"]
    #[inline(always)]
    pub const fn t4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint of fifth piecewise linear function"]
    #[inline(always)]
    pub fn set_t4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T4 {
    #[inline(always)]
    fn default() -> T4 {
        T4(0)
    }
}
