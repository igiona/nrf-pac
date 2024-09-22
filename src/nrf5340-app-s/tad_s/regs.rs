#[doc = "Start all trace and debug clocks."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clockstart(pub u32);
impl Clockstart {
    #[inline(always)]
    pub const fn start(&self) -> super::vals::Start {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Start::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_start(&mut self, val: super::vals::Start) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Clockstart {
    #[inline(always)]
    fn default() -> Clockstart {
        Clockstart(0)
    }
}
#[doc = "Stop all trace and debug clocks."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clockstop(pub u32);
impl Clockstop {
    #[inline(always)]
    pub const fn stop(&self) -> super::vals::Stop {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Stop::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_stop(&mut self, val: super::vals::Stop) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Clockstop {
    #[inline(always)]
    fn default() -> Clockstop {
        Clockstop(0)
    }
}
#[doc = "Enable debug domain and aquire selected GPIOs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
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
#[doc = "Pin configuration for TRACECLK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Traceclk(pub u32);
impl Traceclk {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> super::vals::TraceclkPin {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::TraceclkPin::from_bits(val as u8)
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: super::vals::TraceclkPin) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::TraceclkConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::TraceclkConnect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::TraceclkConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Traceclk {
    #[inline(always)]
    fn default() -> Traceclk {
        Traceclk(0)
    }
}
#[doc = "Pin configuration for TRACEDATA\\[0\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tracedata0(pub u32);
impl Tracedata0 {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> super::vals::Tracedata0pin {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Tracedata0pin::from_bits(val as u8)
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: super::vals::Tracedata0pin) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::Tracedata0connect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Tracedata0connect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Tracedata0connect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Tracedata0 {
    #[inline(always)]
    fn default() -> Tracedata0 {
        Tracedata0(0)
    }
}
#[doc = "Pin configuration for TRACEDATA\\[1\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tracedata1(pub u32);
impl Tracedata1 {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> super::vals::Tracedata1pin {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Tracedata1pin::from_bits(val as u8)
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: super::vals::Tracedata1pin) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::Tracedata1connect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Tracedata1connect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Tracedata1connect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Tracedata1 {
    #[inline(always)]
    fn default() -> Tracedata1 {
        Tracedata1(0)
    }
}
#[doc = "Pin configuration for TRACEDATA\\[2\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tracedata2(pub u32);
impl Tracedata2 {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> super::vals::Tracedata2pin {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Tracedata2pin::from_bits(val as u8)
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: super::vals::Tracedata2pin) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::Tracedata2connect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Tracedata2connect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Tracedata2connect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Tracedata2 {
    #[inline(always)]
    fn default() -> Tracedata2 {
        Tracedata2(0)
    }
}
#[doc = "Pin configuration for TRACEDATA\\[3\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tracedata3(pub u32);
impl Tracedata3 {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> super::vals::Tracedata3pin {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Tracedata3pin::from_bits(val as u8)
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: super::vals::Tracedata3pin) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::Tracedata3connect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Tracedata3connect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Tracedata3connect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Tracedata3 {
    #[inline(always)]
    fn default() -> Tracedata3 {
        Tracedata3(0)
    }
}
#[doc = "Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Traceportspeed(pub u32);
impl Traceportspeed {
    #[doc = "Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
    #[inline(always)]
    pub const fn traceportspeed(&self) -> super::vals::Traceportspeed {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Traceportspeed::from_bits(val as u8)
    }
    #[doc = "Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
    #[inline(always)]
    pub fn set_traceportspeed(&mut self, val: super::vals::Traceportspeed) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Traceportspeed {
    #[inline(always)]
    fn default() -> Traceportspeed {
        Traceportspeed(0)
    }
}
