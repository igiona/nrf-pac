#[doc = "Select between default DETECT signal behavior and LDETECT mode (For non-secure pin only)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Detectmode(pub u32);
impl Detectmode {
    #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
    #[inline(always)]
    pub const fn detectmode(&self) -> super::vals::DetectmodeDetectmode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DetectmodeDetectmode::from_bits(val as u8)
    }
    #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
    #[inline(always)]
    pub fn set_detectmode(&mut self, val: super::vals::DetectmodeDetectmode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Detectmode {
    #[inline(always)]
    fn default() -> Detectmode {
        Detectmode(0)
    }
}
#[doc = "Select between default DETECT signal behavior and LDETECT mode (For secure pin only)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DetectmodeSec(pub u32);
impl DetectmodeSec {
    #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
    #[inline(always)]
    pub const fn detectmode(&self) -> super::vals::DetectmodeSecDetectmode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DetectmodeSecDetectmode::from_bits(val as u8)
    }
    #[doc = "Select between default DETECT signal behavior and LDETECT mode"]
    #[inline(always)]
    pub fn set_detectmode(&mut self, val: super::vals::DetectmodeSecDetectmode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for DetectmodeSec {
    #[inline(always)]
    fn default() -> DetectmodeSec {
        DetectmodeSec(0)
    }
}
#[doc = "Direction of GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dir(pub u32);
impl Dir {
    #[doc = "Pin 0"]
    #[inline(always)]
    pub const fn pin0(&self) -> super::vals::DirPin0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DirPin0::from_bits(val as u8)
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn set_pin0(&mut self, val: super::vals::DirPin0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub const fn pin1(&self) -> super::vals::DirPin1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DirPin1::from_bits(val as u8)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn set_pin1(&mut self, val: super::vals::DirPin1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub const fn pin2(&self) -> super::vals::DirPin2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DirPin2::from_bits(val as u8)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn set_pin2(&mut self, val: super::vals::DirPin2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub const fn pin3(&self) -> super::vals::DirPin3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DirPin3::from_bits(val as u8)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn set_pin3(&mut self, val: super::vals::DirPin3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub const fn pin4(&self) -> super::vals::DirPin4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DirPin4::from_bits(val as u8)
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn set_pin4(&mut self, val: super::vals::DirPin4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub const fn pin5(&self) -> super::vals::DirPin5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DirPin5::from_bits(val as u8)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn set_pin5(&mut self, val: super::vals::DirPin5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub const fn pin6(&self) -> super::vals::DirPin6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DirPin6::from_bits(val as u8)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn set_pin6(&mut self, val: super::vals::DirPin6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub const fn pin7(&self) -> super::vals::DirPin7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DirPin7::from_bits(val as u8)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn set_pin7(&mut self, val: super::vals::DirPin7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub const fn pin8(&self) -> super::vals::DirPin8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DirPin8::from_bits(val as u8)
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn set_pin8(&mut self, val: super::vals::DirPin8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub const fn pin9(&self) -> super::vals::DirPin9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::DirPin9::from_bits(val as u8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn set_pin9(&mut self, val: super::vals::DirPin9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub const fn pin10(&self) -> super::vals::DirPin10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::DirPin10::from_bits(val as u8)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn set_pin10(&mut self, val: super::vals::DirPin10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub const fn pin11(&self) -> super::vals::DirPin11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::DirPin11::from_bits(val as u8)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn set_pin11(&mut self, val: super::vals::DirPin11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub const fn pin12(&self) -> super::vals::DirPin12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::DirPin12::from_bits(val as u8)
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn set_pin12(&mut self, val: super::vals::DirPin12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub const fn pin13(&self) -> super::vals::DirPin13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::DirPin13::from_bits(val as u8)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn set_pin13(&mut self, val: super::vals::DirPin13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub const fn pin14(&self) -> super::vals::DirPin14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::DirPin14::from_bits(val as u8)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn set_pin14(&mut self, val: super::vals::DirPin14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub const fn pin15(&self) -> super::vals::DirPin15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::DirPin15::from_bits(val as u8)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn set_pin15(&mut self, val: super::vals::DirPin15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Pin 16"]
    #[inline(always)]
    pub const fn pin16(&self) -> super::vals::DirPin16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DirPin16::from_bits(val as u8)
    }
    #[doc = "Pin 16"]
    #[inline(always)]
    pub fn set_pin16(&mut self, val: super::vals::DirPin16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Pin 17"]
    #[inline(always)]
    pub const fn pin17(&self) -> super::vals::DirPin17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::DirPin17::from_bits(val as u8)
    }
    #[doc = "Pin 17"]
    #[inline(always)]
    pub fn set_pin17(&mut self, val: super::vals::DirPin17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Pin 18"]
    #[inline(always)]
    pub const fn pin18(&self) -> super::vals::DirPin18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::DirPin18::from_bits(val as u8)
    }
    #[doc = "Pin 18"]
    #[inline(always)]
    pub fn set_pin18(&mut self, val: super::vals::DirPin18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pin 19"]
    #[inline(always)]
    pub const fn pin19(&self) -> super::vals::DirPin19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::DirPin19::from_bits(val as u8)
    }
    #[doc = "Pin 19"]
    #[inline(always)]
    pub fn set_pin19(&mut self, val: super::vals::DirPin19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Pin 20"]
    #[inline(always)]
    pub const fn pin20(&self) -> super::vals::DirPin20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::DirPin20::from_bits(val as u8)
    }
    #[doc = "Pin 20"]
    #[inline(always)]
    pub fn set_pin20(&mut self, val: super::vals::DirPin20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Pin 21"]
    #[inline(always)]
    pub const fn pin21(&self) -> super::vals::DirPin21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::DirPin21::from_bits(val as u8)
    }
    #[doc = "Pin 21"]
    #[inline(always)]
    pub fn set_pin21(&mut self, val: super::vals::DirPin21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Pin 22"]
    #[inline(always)]
    pub const fn pin22(&self) -> super::vals::DirPin22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::DirPin22::from_bits(val as u8)
    }
    #[doc = "Pin 22"]
    #[inline(always)]
    pub fn set_pin22(&mut self, val: super::vals::DirPin22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Pin 23"]
    #[inline(always)]
    pub const fn pin23(&self) -> super::vals::DirPin23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::DirPin23::from_bits(val as u8)
    }
    #[doc = "Pin 23"]
    #[inline(always)]
    pub fn set_pin23(&mut self, val: super::vals::DirPin23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Pin 24"]
    #[inline(always)]
    pub const fn pin24(&self) -> super::vals::DirPin24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::DirPin24::from_bits(val as u8)
    }
    #[doc = "Pin 24"]
    #[inline(always)]
    pub fn set_pin24(&mut self, val: super::vals::DirPin24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Pin 25"]
    #[inline(always)]
    pub const fn pin25(&self) -> super::vals::DirPin25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::DirPin25::from_bits(val as u8)
    }
    #[doc = "Pin 25"]
    #[inline(always)]
    pub fn set_pin25(&mut self, val: super::vals::DirPin25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Pin 26"]
    #[inline(always)]
    pub const fn pin26(&self) -> super::vals::DirPin26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::DirPin26::from_bits(val as u8)
    }
    #[doc = "Pin 26"]
    #[inline(always)]
    pub fn set_pin26(&mut self, val: super::vals::DirPin26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pin 27"]
    #[inline(always)]
    pub const fn pin27(&self) -> super::vals::DirPin27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::DirPin27::from_bits(val as u8)
    }
    #[doc = "Pin 27"]
    #[inline(always)]
    pub fn set_pin27(&mut self, val: super::vals::DirPin27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Pin 28"]
    #[inline(always)]
    pub const fn pin28(&self) -> super::vals::DirPin28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::DirPin28::from_bits(val as u8)
    }
    #[doc = "Pin 28"]
    #[inline(always)]
    pub fn set_pin28(&mut self, val: super::vals::DirPin28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Pin 29"]
    #[inline(always)]
    pub const fn pin29(&self) -> super::vals::DirPin29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::DirPin29::from_bits(val as u8)
    }
    #[doc = "Pin 29"]
    #[inline(always)]
    pub fn set_pin29(&mut self, val: super::vals::DirPin29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Pin 30"]
    #[inline(always)]
    pub const fn pin30(&self) -> super::vals::DirPin30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::DirPin30::from_bits(val as u8)
    }
    #[doc = "Pin 30"]
    #[inline(always)]
    pub fn set_pin30(&mut self, val: super::vals::DirPin30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Pin 31"]
    #[inline(always)]
    pub const fn pin31(&self) -> super::vals::DirPin31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::DirPin31::from_bits(val as u8)
    }
    #[doc = "Pin 31"]
    #[inline(always)]
    pub fn set_pin31(&mut self, val: super::vals::DirPin31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dir {
    #[inline(always)]
    fn default() -> Dir {
        Dir(0)
    }
}
#[doc = "DIR clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirclr(pub u32);
impl Dirclr {
    #[doc = "Set as input pin 0"]
    #[inline(always)]
    pub const fn pin0(&self) -> super::vals::DirclrPin0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DirclrPin0::from_bits(val as u8)
    }
    #[doc = "Set as input pin 0"]
    #[inline(always)]
    pub fn set_pin0(&mut self, val: super::vals::DirclrPin0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Set as input pin 1"]
    #[inline(always)]
    pub const fn pin1(&self) -> super::vals::DirclrPin1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DirclrPin1::from_bits(val as u8)
    }
    #[doc = "Set as input pin 1"]
    #[inline(always)]
    pub fn set_pin1(&mut self, val: super::vals::DirclrPin1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Set as input pin 2"]
    #[inline(always)]
    pub const fn pin2(&self) -> super::vals::DirclrPin2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DirclrPin2::from_bits(val as u8)
    }
    #[doc = "Set as input pin 2"]
    #[inline(always)]
    pub fn set_pin2(&mut self, val: super::vals::DirclrPin2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Set as input pin 3"]
    #[inline(always)]
    pub const fn pin3(&self) -> super::vals::DirclrPin3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DirclrPin3::from_bits(val as u8)
    }
    #[doc = "Set as input pin 3"]
    #[inline(always)]
    pub fn set_pin3(&mut self, val: super::vals::DirclrPin3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Set as input pin 4"]
    #[inline(always)]
    pub const fn pin4(&self) -> super::vals::DirclrPin4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DirclrPin4::from_bits(val as u8)
    }
    #[doc = "Set as input pin 4"]
    #[inline(always)]
    pub fn set_pin4(&mut self, val: super::vals::DirclrPin4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Set as input pin 5"]
    #[inline(always)]
    pub const fn pin5(&self) -> super::vals::DirclrPin5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DirclrPin5::from_bits(val as u8)
    }
    #[doc = "Set as input pin 5"]
    #[inline(always)]
    pub fn set_pin5(&mut self, val: super::vals::DirclrPin5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Set as input pin 6"]
    #[inline(always)]
    pub const fn pin6(&self) -> super::vals::DirclrPin6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DirclrPin6::from_bits(val as u8)
    }
    #[doc = "Set as input pin 6"]
    #[inline(always)]
    pub fn set_pin6(&mut self, val: super::vals::DirclrPin6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Set as input pin 7"]
    #[inline(always)]
    pub const fn pin7(&self) -> super::vals::DirclrPin7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DirclrPin7::from_bits(val as u8)
    }
    #[doc = "Set as input pin 7"]
    #[inline(always)]
    pub fn set_pin7(&mut self, val: super::vals::DirclrPin7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Set as input pin 8"]
    #[inline(always)]
    pub const fn pin8(&self) -> super::vals::DirclrPin8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DirclrPin8::from_bits(val as u8)
    }
    #[doc = "Set as input pin 8"]
    #[inline(always)]
    pub fn set_pin8(&mut self, val: super::vals::DirclrPin8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Set as input pin 9"]
    #[inline(always)]
    pub const fn pin9(&self) -> super::vals::DirclrPin9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::DirclrPin9::from_bits(val as u8)
    }
    #[doc = "Set as input pin 9"]
    #[inline(always)]
    pub fn set_pin9(&mut self, val: super::vals::DirclrPin9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Set as input pin 10"]
    #[inline(always)]
    pub const fn pin10(&self) -> super::vals::DirclrPin10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::DirclrPin10::from_bits(val as u8)
    }
    #[doc = "Set as input pin 10"]
    #[inline(always)]
    pub fn set_pin10(&mut self, val: super::vals::DirclrPin10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Set as input pin 11"]
    #[inline(always)]
    pub const fn pin11(&self) -> super::vals::DirclrPin11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::DirclrPin11::from_bits(val as u8)
    }
    #[doc = "Set as input pin 11"]
    #[inline(always)]
    pub fn set_pin11(&mut self, val: super::vals::DirclrPin11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Set as input pin 12"]
    #[inline(always)]
    pub const fn pin12(&self) -> super::vals::DirclrPin12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::DirclrPin12::from_bits(val as u8)
    }
    #[doc = "Set as input pin 12"]
    #[inline(always)]
    pub fn set_pin12(&mut self, val: super::vals::DirclrPin12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Set as input pin 13"]
    #[inline(always)]
    pub const fn pin13(&self) -> super::vals::DirclrPin13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::DirclrPin13::from_bits(val as u8)
    }
    #[doc = "Set as input pin 13"]
    #[inline(always)]
    pub fn set_pin13(&mut self, val: super::vals::DirclrPin13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Set as input pin 14"]
    #[inline(always)]
    pub const fn pin14(&self) -> super::vals::DirclrPin14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::DirclrPin14::from_bits(val as u8)
    }
    #[doc = "Set as input pin 14"]
    #[inline(always)]
    pub fn set_pin14(&mut self, val: super::vals::DirclrPin14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Set as input pin 15"]
    #[inline(always)]
    pub const fn pin15(&self) -> super::vals::DirclrPin15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::DirclrPin15::from_bits(val as u8)
    }
    #[doc = "Set as input pin 15"]
    #[inline(always)]
    pub fn set_pin15(&mut self, val: super::vals::DirclrPin15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Set as input pin 16"]
    #[inline(always)]
    pub const fn pin16(&self) -> super::vals::DirclrPin16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DirclrPin16::from_bits(val as u8)
    }
    #[doc = "Set as input pin 16"]
    #[inline(always)]
    pub fn set_pin16(&mut self, val: super::vals::DirclrPin16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Set as input pin 17"]
    #[inline(always)]
    pub const fn pin17(&self) -> super::vals::DirclrPin17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::DirclrPin17::from_bits(val as u8)
    }
    #[doc = "Set as input pin 17"]
    #[inline(always)]
    pub fn set_pin17(&mut self, val: super::vals::DirclrPin17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Set as input pin 18"]
    #[inline(always)]
    pub const fn pin18(&self) -> super::vals::DirclrPin18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::DirclrPin18::from_bits(val as u8)
    }
    #[doc = "Set as input pin 18"]
    #[inline(always)]
    pub fn set_pin18(&mut self, val: super::vals::DirclrPin18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Set as input pin 19"]
    #[inline(always)]
    pub const fn pin19(&self) -> super::vals::DirclrPin19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::DirclrPin19::from_bits(val as u8)
    }
    #[doc = "Set as input pin 19"]
    #[inline(always)]
    pub fn set_pin19(&mut self, val: super::vals::DirclrPin19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Set as input pin 20"]
    #[inline(always)]
    pub const fn pin20(&self) -> super::vals::DirclrPin20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::DirclrPin20::from_bits(val as u8)
    }
    #[doc = "Set as input pin 20"]
    #[inline(always)]
    pub fn set_pin20(&mut self, val: super::vals::DirclrPin20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Set as input pin 21"]
    #[inline(always)]
    pub const fn pin21(&self) -> super::vals::DirclrPin21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::DirclrPin21::from_bits(val as u8)
    }
    #[doc = "Set as input pin 21"]
    #[inline(always)]
    pub fn set_pin21(&mut self, val: super::vals::DirclrPin21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Set as input pin 22"]
    #[inline(always)]
    pub const fn pin22(&self) -> super::vals::DirclrPin22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::DirclrPin22::from_bits(val as u8)
    }
    #[doc = "Set as input pin 22"]
    #[inline(always)]
    pub fn set_pin22(&mut self, val: super::vals::DirclrPin22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Set as input pin 23"]
    #[inline(always)]
    pub const fn pin23(&self) -> super::vals::DirclrPin23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::DirclrPin23::from_bits(val as u8)
    }
    #[doc = "Set as input pin 23"]
    #[inline(always)]
    pub fn set_pin23(&mut self, val: super::vals::DirclrPin23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Set as input pin 24"]
    #[inline(always)]
    pub const fn pin24(&self) -> super::vals::DirclrPin24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::DirclrPin24::from_bits(val as u8)
    }
    #[doc = "Set as input pin 24"]
    #[inline(always)]
    pub fn set_pin24(&mut self, val: super::vals::DirclrPin24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Set as input pin 25"]
    #[inline(always)]
    pub const fn pin25(&self) -> super::vals::DirclrPin25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::DirclrPin25::from_bits(val as u8)
    }
    #[doc = "Set as input pin 25"]
    #[inline(always)]
    pub fn set_pin25(&mut self, val: super::vals::DirclrPin25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Set as input pin 26"]
    #[inline(always)]
    pub const fn pin26(&self) -> super::vals::DirclrPin26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::DirclrPin26::from_bits(val as u8)
    }
    #[doc = "Set as input pin 26"]
    #[inline(always)]
    pub fn set_pin26(&mut self, val: super::vals::DirclrPin26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Set as input pin 27"]
    #[inline(always)]
    pub const fn pin27(&self) -> super::vals::DirclrPin27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::DirclrPin27::from_bits(val as u8)
    }
    #[doc = "Set as input pin 27"]
    #[inline(always)]
    pub fn set_pin27(&mut self, val: super::vals::DirclrPin27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Set as input pin 28"]
    #[inline(always)]
    pub const fn pin28(&self) -> super::vals::DirclrPin28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::DirclrPin28::from_bits(val as u8)
    }
    #[doc = "Set as input pin 28"]
    #[inline(always)]
    pub fn set_pin28(&mut self, val: super::vals::DirclrPin28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Set as input pin 29"]
    #[inline(always)]
    pub const fn pin29(&self) -> super::vals::DirclrPin29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::DirclrPin29::from_bits(val as u8)
    }
    #[doc = "Set as input pin 29"]
    #[inline(always)]
    pub fn set_pin29(&mut self, val: super::vals::DirclrPin29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Set as input pin 30"]
    #[inline(always)]
    pub const fn pin30(&self) -> super::vals::DirclrPin30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::DirclrPin30::from_bits(val as u8)
    }
    #[doc = "Set as input pin 30"]
    #[inline(always)]
    pub fn set_pin30(&mut self, val: super::vals::DirclrPin30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Set as input pin 31"]
    #[inline(always)]
    pub const fn pin31(&self) -> super::vals::DirclrPin31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::DirclrPin31::from_bits(val as u8)
    }
    #[doc = "Set as input pin 31"]
    #[inline(always)]
    pub fn set_pin31(&mut self, val: super::vals::DirclrPin31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dirclr {
    #[inline(always)]
    fn default() -> Dirclr {
        Dirclr(0)
    }
}
#[doc = "DIR set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirset(pub u32);
impl Dirset {
    #[doc = "Set as output pin 0"]
    #[inline(always)]
    pub const fn pin0(&self) -> super::vals::DirsetPin0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DirsetPin0::from_bits(val as u8)
    }
    #[doc = "Set as output pin 0"]
    #[inline(always)]
    pub fn set_pin0(&mut self, val: super::vals::DirsetPin0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Set as output pin 1"]
    #[inline(always)]
    pub const fn pin1(&self) -> super::vals::DirsetPin1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DirsetPin1::from_bits(val as u8)
    }
    #[doc = "Set as output pin 1"]
    #[inline(always)]
    pub fn set_pin1(&mut self, val: super::vals::DirsetPin1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Set as output pin 2"]
    #[inline(always)]
    pub const fn pin2(&self) -> super::vals::DirsetPin2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DirsetPin2::from_bits(val as u8)
    }
    #[doc = "Set as output pin 2"]
    #[inline(always)]
    pub fn set_pin2(&mut self, val: super::vals::DirsetPin2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Set as output pin 3"]
    #[inline(always)]
    pub const fn pin3(&self) -> super::vals::DirsetPin3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DirsetPin3::from_bits(val as u8)
    }
    #[doc = "Set as output pin 3"]
    #[inline(always)]
    pub fn set_pin3(&mut self, val: super::vals::DirsetPin3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Set as output pin 4"]
    #[inline(always)]
    pub const fn pin4(&self) -> super::vals::DirsetPin4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DirsetPin4::from_bits(val as u8)
    }
    #[doc = "Set as output pin 4"]
    #[inline(always)]
    pub fn set_pin4(&mut self, val: super::vals::DirsetPin4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Set as output pin 5"]
    #[inline(always)]
    pub const fn pin5(&self) -> super::vals::DirsetPin5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DirsetPin5::from_bits(val as u8)
    }
    #[doc = "Set as output pin 5"]
    #[inline(always)]
    pub fn set_pin5(&mut self, val: super::vals::DirsetPin5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Set as output pin 6"]
    #[inline(always)]
    pub const fn pin6(&self) -> super::vals::DirsetPin6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DirsetPin6::from_bits(val as u8)
    }
    #[doc = "Set as output pin 6"]
    #[inline(always)]
    pub fn set_pin6(&mut self, val: super::vals::DirsetPin6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Set as output pin 7"]
    #[inline(always)]
    pub const fn pin7(&self) -> super::vals::DirsetPin7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DirsetPin7::from_bits(val as u8)
    }
    #[doc = "Set as output pin 7"]
    #[inline(always)]
    pub fn set_pin7(&mut self, val: super::vals::DirsetPin7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Set as output pin 8"]
    #[inline(always)]
    pub const fn pin8(&self) -> super::vals::DirsetPin8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DirsetPin8::from_bits(val as u8)
    }
    #[doc = "Set as output pin 8"]
    #[inline(always)]
    pub fn set_pin8(&mut self, val: super::vals::DirsetPin8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Set as output pin 9"]
    #[inline(always)]
    pub const fn pin9(&self) -> super::vals::DirsetPin9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::DirsetPin9::from_bits(val as u8)
    }
    #[doc = "Set as output pin 9"]
    #[inline(always)]
    pub fn set_pin9(&mut self, val: super::vals::DirsetPin9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Set as output pin 10"]
    #[inline(always)]
    pub const fn pin10(&self) -> super::vals::DirsetPin10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::DirsetPin10::from_bits(val as u8)
    }
    #[doc = "Set as output pin 10"]
    #[inline(always)]
    pub fn set_pin10(&mut self, val: super::vals::DirsetPin10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Set as output pin 11"]
    #[inline(always)]
    pub const fn pin11(&self) -> super::vals::DirsetPin11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::DirsetPin11::from_bits(val as u8)
    }
    #[doc = "Set as output pin 11"]
    #[inline(always)]
    pub fn set_pin11(&mut self, val: super::vals::DirsetPin11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Set as output pin 12"]
    #[inline(always)]
    pub const fn pin12(&self) -> super::vals::DirsetPin12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::DirsetPin12::from_bits(val as u8)
    }
    #[doc = "Set as output pin 12"]
    #[inline(always)]
    pub fn set_pin12(&mut self, val: super::vals::DirsetPin12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Set as output pin 13"]
    #[inline(always)]
    pub const fn pin13(&self) -> super::vals::DirsetPin13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::DirsetPin13::from_bits(val as u8)
    }
    #[doc = "Set as output pin 13"]
    #[inline(always)]
    pub fn set_pin13(&mut self, val: super::vals::DirsetPin13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Set as output pin 14"]
    #[inline(always)]
    pub const fn pin14(&self) -> super::vals::DirsetPin14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::DirsetPin14::from_bits(val as u8)
    }
    #[doc = "Set as output pin 14"]
    #[inline(always)]
    pub fn set_pin14(&mut self, val: super::vals::DirsetPin14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Set as output pin 15"]
    #[inline(always)]
    pub const fn pin15(&self) -> super::vals::DirsetPin15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::DirsetPin15::from_bits(val as u8)
    }
    #[doc = "Set as output pin 15"]
    #[inline(always)]
    pub fn set_pin15(&mut self, val: super::vals::DirsetPin15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Set as output pin 16"]
    #[inline(always)]
    pub const fn pin16(&self) -> super::vals::DirsetPin16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DirsetPin16::from_bits(val as u8)
    }
    #[doc = "Set as output pin 16"]
    #[inline(always)]
    pub fn set_pin16(&mut self, val: super::vals::DirsetPin16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Set as output pin 17"]
    #[inline(always)]
    pub const fn pin17(&self) -> super::vals::DirsetPin17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::DirsetPin17::from_bits(val as u8)
    }
    #[doc = "Set as output pin 17"]
    #[inline(always)]
    pub fn set_pin17(&mut self, val: super::vals::DirsetPin17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Set as output pin 18"]
    #[inline(always)]
    pub const fn pin18(&self) -> super::vals::DirsetPin18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::DirsetPin18::from_bits(val as u8)
    }
    #[doc = "Set as output pin 18"]
    #[inline(always)]
    pub fn set_pin18(&mut self, val: super::vals::DirsetPin18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Set as output pin 19"]
    #[inline(always)]
    pub const fn pin19(&self) -> super::vals::DirsetPin19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::DirsetPin19::from_bits(val as u8)
    }
    #[doc = "Set as output pin 19"]
    #[inline(always)]
    pub fn set_pin19(&mut self, val: super::vals::DirsetPin19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Set as output pin 20"]
    #[inline(always)]
    pub const fn pin20(&self) -> super::vals::DirsetPin20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::DirsetPin20::from_bits(val as u8)
    }
    #[doc = "Set as output pin 20"]
    #[inline(always)]
    pub fn set_pin20(&mut self, val: super::vals::DirsetPin20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Set as output pin 21"]
    #[inline(always)]
    pub const fn pin21(&self) -> super::vals::DirsetPin21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::DirsetPin21::from_bits(val as u8)
    }
    #[doc = "Set as output pin 21"]
    #[inline(always)]
    pub fn set_pin21(&mut self, val: super::vals::DirsetPin21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Set as output pin 22"]
    #[inline(always)]
    pub const fn pin22(&self) -> super::vals::DirsetPin22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::DirsetPin22::from_bits(val as u8)
    }
    #[doc = "Set as output pin 22"]
    #[inline(always)]
    pub fn set_pin22(&mut self, val: super::vals::DirsetPin22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Set as output pin 23"]
    #[inline(always)]
    pub const fn pin23(&self) -> super::vals::DirsetPin23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::DirsetPin23::from_bits(val as u8)
    }
    #[doc = "Set as output pin 23"]
    #[inline(always)]
    pub fn set_pin23(&mut self, val: super::vals::DirsetPin23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Set as output pin 24"]
    #[inline(always)]
    pub const fn pin24(&self) -> super::vals::DirsetPin24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::DirsetPin24::from_bits(val as u8)
    }
    #[doc = "Set as output pin 24"]
    #[inline(always)]
    pub fn set_pin24(&mut self, val: super::vals::DirsetPin24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Set as output pin 25"]
    #[inline(always)]
    pub const fn pin25(&self) -> super::vals::DirsetPin25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::DirsetPin25::from_bits(val as u8)
    }
    #[doc = "Set as output pin 25"]
    #[inline(always)]
    pub fn set_pin25(&mut self, val: super::vals::DirsetPin25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Set as output pin 26"]
    #[inline(always)]
    pub const fn pin26(&self) -> super::vals::DirsetPin26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::DirsetPin26::from_bits(val as u8)
    }
    #[doc = "Set as output pin 26"]
    #[inline(always)]
    pub fn set_pin26(&mut self, val: super::vals::DirsetPin26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Set as output pin 27"]
    #[inline(always)]
    pub const fn pin27(&self) -> super::vals::DirsetPin27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::DirsetPin27::from_bits(val as u8)
    }
    #[doc = "Set as output pin 27"]
    #[inline(always)]
    pub fn set_pin27(&mut self, val: super::vals::DirsetPin27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Set as output pin 28"]
    #[inline(always)]
    pub const fn pin28(&self) -> super::vals::DirsetPin28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::DirsetPin28::from_bits(val as u8)
    }
    #[doc = "Set as output pin 28"]
    #[inline(always)]
    pub fn set_pin28(&mut self, val: super::vals::DirsetPin28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Set as output pin 29"]
    #[inline(always)]
    pub const fn pin29(&self) -> super::vals::DirsetPin29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::DirsetPin29::from_bits(val as u8)
    }
    #[doc = "Set as output pin 29"]
    #[inline(always)]
    pub fn set_pin29(&mut self, val: super::vals::DirsetPin29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Set as output pin 30"]
    #[inline(always)]
    pub const fn pin30(&self) -> super::vals::DirsetPin30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::DirsetPin30::from_bits(val as u8)
    }
    #[doc = "Set as output pin 30"]
    #[inline(always)]
    pub fn set_pin30(&mut self, val: super::vals::DirsetPin30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Set as output pin 31"]
    #[inline(always)]
    pub const fn pin31(&self) -> super::vals::DirsetPin31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::DirsetPin31::from_bits(val as u8)
    }
    #[doc = "Set as output pin 31"]
    #[inline(always)]
    pub fn set_pin31(&mut self, val: super::vals::DirsetPin31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dirset {
    #[inline(always)]
    fn default() -> Dirset {
        Dirset(0)
    }
}
#[doc = "Read GPIO port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In(pub u32);
impl In {
    #[doc = "Pin 0"]
    #[inline(always)]
    pub const fn pin0(&self) -> super::vals::InPin0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::InPin0::from_bits(val as u8)
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn set_pin0(&mut self, val: super::vals::InPin0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub const fn pin1(&self) -> super::vals::InPin1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::InPin1::from_bits(val as u8)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn set_pin1(&mut self, val: super::vals::InPin1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub const fn pin2(&self) -> super::vals::InPin2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::InPin2::from_bits(val as u8)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn set_pin2(&mut self, val: super::vals::InPin2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub const fn pin3(&self) -> super::vals::InPin3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::InPin3::from_bits(val as u8)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn set_pin3(&mut self, val: super::vals::InPin3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub const fn pin4(&self) -> super::vals::InPin4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::InPin4::from_bits(val as u8)
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn set_pin4(&mut self, val: super::vals::InPin4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub const fn pin5(&self) -> super::vals::InPin5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::InPin5::from_bits(val as u8)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn set_pin5(&mut self, val: super::vals::InPin5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub const fn pin6(&self) -> super::vals::InPin6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::InPin6::from_bits(val as u8)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn set_pin6(&mut self, val: super::vals::InPin6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub const fn pin7(&self) -> super::vals::InPin7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::InPin7::from_bits(val as u8)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn set_pin7(&mut self, val: super::vals::InPin7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub const fn pin8(&self) -> super::vals::InPin8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::InPin8::from_bits(val as u8)
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn set_pin8(&mut self, val: super::vals::InPin8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub const fn pin9(&self) -> super::vals::InPin9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::InPin9::from_bits(val as u8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn set_pin9(&mut self, val: super::vals::InPin9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub const fn pin10(&self) -> super::vals::InPin10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::InPin10::from_bits(val as u8)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn set_pin10(&mut self, val: super::vals::InPin10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub const fn pin11(&self) -> super::vals::InPin11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::InPin11::from_bits(val as u8)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn set_pin11(&mut self, val: super::vals::InPin11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub const fn pin12(&self) -> super::vals::InPin12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::InPin12::from_bits(val as u8)
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn set_pin12(&mut self, val: super::vals::InPin12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub const fn pin13(&self) -> super::vals::InPin13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::InPin13::from_bits(val as u8)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn set_pin13(&mut self, val: super::vals::InPin13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub const fn pin14(&self) -> super::vals::InPin14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::InPin14::from_bits(val as u8)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn set_pin14(&mut self, val: super::vals::InPin14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub const fn pin15(&self) -> super::vals::InPin15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::InPin15::from_bits(val as u8)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn set_pin15(&mut self, val: super::vals::InPin15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Pin 16"]
    #[inline(always)]
    pub const fn pin16(&self) -> super::vals::InPin16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::InPin16::from_bits(val as u8)
    }
    #[doc = "Pin 16"]
    #[inline(always)]
    pub fn set_pin16(&mut self, val: super::vals::InPin16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Pin 17"]
    #[inline(always)]
    pub const fn pin17(&self) -> super::vals::InPin17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::InPin17::from_bits(val as u8)
    }
    #[doc = "Pin 17"]
    #[inline(always)]
    pub fn set_pin17(&mut self, val: super::vals::InPin17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Pin 18"]
    #[inline(always)]
    pub const fn pin18(&self) -> super::vals::InPin18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::InPin18::from_bits(val as u8)
    }
    #[doc = "Pin 18"]
    #[inline(always)]
    pub fn set_pin18(&mut self, val: super::vals::InPin18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pin 19"]
    #[inline(always)]
    pub const fn pin19(&self) -> super::vals::InPin19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::InPin19::from_bits(val as u8)
    }
    #[doc = "Pin 19"]
    #[inline(always)]
    pub fn set_pin19(&mut self, val: super::vals::InPin19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Pin 20"]
    #[inline(always)]
    pub const fn pin20(&self) -> super::vals::InPin20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::InPin20::from_bits(val as u8)
    }
    #[doc = "Pin 20"]
    #[inline(always)]
    pub fn set_pin20(&mut self, val: super::vals::InPin20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Pin 21"]
    #[inline(always)]
    pub const fn pin21(&self) -> super::vals::InPin21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::InPin21::from_bits(val as u8)
    }
    #[doc = "Pin 21"]
    #[inline(always)]
    pub fn set_pin21(&mut self, val: super::vals::InPin21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Pin 22"]
    #[inline(always)]
    pub const fn pin22(&self) -> super::vals::InPin22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::InPin22::from_bits(val as u8)
    }
    #[doc = "Pin 22"]
    #[inline(always)]
    pub fn set_pin22(&mut self, val: super::vals::InPin22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Pin 23"]
    #[inline(always)]
    pub const fn pin23(&self) -> super::vals::InPin23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::InPin23::from_bits(val as u8)
    }
    #[doc = "Pin 23"]
    #[inline(always)]
    pub fn set_pin23(&mut self, val: super::vals::InPin23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Pin 24"]
    #[inline(always)]
    pub const fn pin24(&self) -> super::vals::InPin24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::InPin24::from_bits(val as u8)
    }
    #[doc = "Pin 24"]
    #[inline(always)]
    pub fn set_pin24(&mut self, val: super::vals::InPin24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Pin 25"]
    #[inline(always)]
    pub const fn pin25(&self) -> super::vals::InPin25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::InPin25::from_bits(val as u8)
    }
    #[doc = "Pin 25"]
    #[inline(always)]
    pub fn set_pin25(&mut self, val: super::vals::InPin25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Pin 26"]
    #[inline(always)]
    pub const fn pin26(&self) -> super::vals::InPin26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::InPin26::from_bits(val as u8)
    }
    #[doc = "Pin 26"]
    #[inline(always)]
    pub fn set_pin26(&mut self, val: super::vals::InPin26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pin 27"]
    #[inline(always)]
    pub const fn pin27(&self) -> super::vals::InPin27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::InPin27::from_bits(val as u8)
    }
    #[doc = "Pin 27"]
    #[inline(always)]
    pub fn set_pin27(&mut self, val: super::vals::InPin27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Pin 28"]
    #[inline(always)]
    pub const fn pin28(&self) -> super::vals::InPin28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::InPin28::from_bits(val as u8)
    }
    #[doc = "Pin 28"]
    #[inline(always)]
    pub fn set_pin28(&mut self, val: super::vals::InPin28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Pin 29"]
    #[inline(always)]
    pub const fn pin29(&self) -> super::vals::InPin29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::InPin29::from_bits(val as u8)
    }
    #[doc = "Pin 29"]
    #[inline(always)]
    pub fn set_pin29(&mut self, val: super::vals::InPin29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Pin 30"]
    #[inline(always)]
    pub const fn pin30(&self) -> super::vals::InPin30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::InPin30::from_bits(val as u8)
    }
    #[doc = "Pin 30"]
    #[inline(always)]
    pub fn set_pin30(&mut self, val: super::vals::InPin30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Pin 31"]
    #[inline(always)]
    pub const fn pin31(&self) -> super::vals::InPin31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::InPin31::from_bits(val as u8)
    }
    #[doc = "Pin 31"]
    #[inline(always)]
    pub fn set_pin31(&mut self, val: super::vals::InPin31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for In {
    #[inline(always)]
    fn default() -> In {
        In(0)
    }
}
#[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Latch(pub u32);
impl Latch {
    #[doc = "Status on whether PIN\\[0\\] has met criteria set in PIN_CNF\\[0\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin0(&self) -> super::vals::LatchPin0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LatchPin0::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[0\\] has met criteria set in PIN_CNF\\[0\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin0(&mut self, val: super::vals::LatchPin0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Status on whether PIN\\[1\\] has met criteria set in PIN_CNF\\[1\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin1(&self) -> super::vals::LatchPin1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::LatchPin1::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[1\\] has met criteria set in PIN_CNF\\[1\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin1(&mut self, val: super::vals::LatchPin1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Status on whether PIN\\[2\\] has met criteria set in PIN_CNF\\[2\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin2(&self) -> super::vals::LatchPin2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::LatchPin2::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[2\\] has met criteria set in PIN_CNF\\[2\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin2(&mut self, val: super::vals::LatchPin2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Status on whether PIN\\[3\\] has met criteria set in PIN_CNF\\[3\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin3(&self) -> super::vals::LatchPin3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::LatchPin3::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[3\\] has met criteria set in PIN_CNF\\[3\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin3(&mut self, val: super::vals::LatchPin3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Status on whether PIN\\[4\\] has met criteria set in PIN_CNF\\[4\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin4(&self) -> super::vals::LatchPin4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::LatchPin4::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[4\\] has met criteria set in PIN_CNF\\[4\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin4(&mut self, val: super::vals::LatchPin4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Status on whether PIN\\[5\\] has met criteria set in PIN_CNF\\[5\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin5(&self) -> super::vals::LatchPin5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::LatchPin5::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[5\\] has met criteria set in PIN_CNF\\[5\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin5(&mut self, val: super::vals::LatchPin5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Status on whether PIN\\[6\\] has met criteria set in PIN_CNF\\[6\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin6(&self) -> super::vals::LatchPin6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::LatchPin6::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[6\\] has met criteria set in PIN_CNF\\[6\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin6(&mut self, val: super::vals::LatchPin6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Status on whether PIN\\[7\\] has met criteria set in PIN_CNF\\[7\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin7(&self) -> super::vals::LatchPin7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::LatchPin7::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[7\\] has met criteria set in PIN_CNF\\[7\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin7(&mut self, val: super::vals::LatchPin7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Status on whether PIN\\[8\\] has met criteria set in PIN_CNF\\[8\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin8(&self) -> super::vals::LatchPin8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::LatchPin8::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[8\\] has met criteria set in PIN_CNF\\[8\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin8(&mut self, val: super::vals::LatchPin8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Status on whether PIN\\[9\\] has met criteria set in PIN_CNF\\[9\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin9(&self) -> super::vals::LatchPin9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::LatchPin9::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[9\\] has met criteria set in PIN_CNF\\[9\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin9(&mut self, val: super::vals::LatchPin9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Status on whether PIN\\[10\\] has met criteria set in PIN_CNF\\[10\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin10(&self) -> super::vals::LatchPin10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::LatchPin10::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[10\\] has met criteria set in PIN_CNF\\[10\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin10(&mut self, val: super::vals::LatchPin10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Status on whether PIN\\[11\\] has met criteria set in PIN_CNF\\[11\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin11(&self) -> super::vals::LatchPin11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::LatchPin11::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[11\\] has met criteria set in PIN_CNF\\[11\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin11(&mut self, val: super::vals::LatchPin11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Status on whether PIN\\[12\\] has met criteria set in PIN_CNF\\[12\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin12(&self) -> super::vals::LatchPin12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::LatchPin12::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[12\\] has met criteria set in PIN_CNF\\[12\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin12(&mut self, val: super::vals::LatchPin12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Status on whether PIN\\[13\\] has met criteria set in PIN_CNF\\[13\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin13(&self) -> super::vals::LatchPin13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::LatchPin13::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[13\\] has met criteria set in PIN_CNF\\[13\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin13(&mut self, val: super::vals::LatchPin13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Status on whether PIN\\[14\\] has met criteria set in PIN_CNF\\[14\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin14(&self) -> super::vals::LatchPin14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::LatchPin14::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[14\\] has met criteria set in PIN_CNF\\[14\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin14(&mut self, val: super::vals::LatchPin14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Status on whether PIN\\[15\\] has met criteria set in PIN_CNF\\[15\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin15(&self) -> super::vals::LatchPin15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::LatchPin15::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[15\\] has met criteria set in PIN_CNF\\[15\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin15(&mut self, val: super::vals::LatchPin15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Status on whether PIN\\[16\\] has met criteria set in PIN_CNF\\[16\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin16(&self) -> super::vals::LatchPin16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::LatchPin16::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[16\\] has met criteria set in PIN_CNF\\[16\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin16(&mut self, val: super::vals::LatchPin16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Status on whether PIN\\[17\\] has met criteria set in PIN_CNF\\[17\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin17(&self) -> super::vals::LatchPin17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::LatchPin17::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[17\\] has met criteria set in PIN_CNF\\[17\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin17(&mut self, val: super::vals::LatchPin17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Status on whether PIN\\[18\\] has met criteria set in PIN_CNF\\[18\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin18(&self) -> super::vals::LatchPin18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::LatchPin18::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[18\\] has met criteria set in PIN_CNF\\[18\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin18(&mut self, val: super::vals::LatchPin18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Status on whether PIN\\[19\\] has met criteria set in PIN_CNF\\[19\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin19(&self) -> super::vals::LatchPin19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::LatchPin19::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[19\\] has met criteria set in PIN_CNF\\[19\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin19(&mut self, val: super::vals::LatchPin19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Status on whether PIN\\[20\\] has met criteria set in PIN_CNF\\[20\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin20(&self) -> super::vals::LatchPin20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::LatchPin20::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[20\\] has met criteria set in PIN_CNF\\[20\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin20(&mut self, val: super::vals::LatchPin20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Status on whether PIN\\[21\\] has met criteria set in PIN_CNF\\[21\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin21(&self) -> super::vals::LatchPin21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::LatchPin21::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[21\\] has met criteria set in PIN_CNF\\[21\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin21(&mut self, val: super::vals::LatchPin21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Status on whether PIN\\[22\\] has met criteria set in PIN_CNF\\[22\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin22(&self) -> super::vals::LatchPin22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::LatchPin22::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[22\\] has met criteria set in PIN_CNF\\[22\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin22(&mut self, val: super::vals::LatchPin22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Status on whether PIN\\[23\\] has met criteria set in PIN_CNF\\[23\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin23(&self) -> super::vals::LatchPin23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::LatchPin23::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[23\\] has met criteria set in PIN_CNF\\[23\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin23(&mut self, val: super::vals::LatchPin23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Status on whether PIN\\[24\\] has met criteria set in PIN_CNF\\[24\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin24(&self) -> super::vals::LatchPin24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::LatchPin24::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[24\\] has met criteria set in PIN_CNF\\[24\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin24(&mut self, val: super::vals::LatchPin24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Status on whether PIN\\[25\\] has met criteria set in PIN_CNF\\[25\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin25(&self) -> super::vals::LatchPin25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::LatchPin25::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[25\\] has met criteria set in PIN_CNF\\[25\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin25(&mut self, val: super::vals::LatchPin25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Status on whether PIN\\[26\\] has met criteria set in PIN_CNF\\[26\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin26(&self) -> super::vals::LatchPin26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::LatchPin26::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[26\\] has met criteria set in PIN_CNF\\[26\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin26(&mut self, val: super::vals::LatchPin26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Status on whether PIN\\[27\\] has met criteria set in PIN_CNF\\[27\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin27(&self) -> super::vals::LatchPin27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::LatchPin27::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[27\\] has met criteria set in PIN_CNF\\[27\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin27(&mut self, val: super::vals::LatchPin27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Status on whether PIN\\[28\\] has met criteria set in PIN_CNF\\[28\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin28(&self) -> super::vals::LatchPin28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::LatchPin28::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[28\\] has met criteria set in PIN_CNF\\[28\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin28(&mut self, val: super::vals::LatchPin28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Status on whether PIN\\[29\\] has met criteria set in PIN_CNF\\[29\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin29(&self) -> super::vals::LatchPin29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::LatchPin29::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[29\\] has met criteria set in PIN_CNF\\[29\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin29(&mut self, val: super::vals::LatchPin29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Status on whether PIN\\[30\\] has met criteria set in PIN_CNF\\[30\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin30(&self) -> super::vals::LatchPin30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::LatchPin30::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[30\\] has met criteria set in PIN_CNF\\[30\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin30(&mut self, val: super::vals::LatchPin30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Status on whether PIN\\[31\\] has met criteria set in PIN_CNF\\[31\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub const fn pin31(&self) -> super::vals::LatchPin31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::LatchPin31::from_bits(val as u8)
    }
    #[doc = "Status on whether PIN\\[31\\] has met criteria set in PIN_CNF\\[31\\].SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin31(&mut self, val: super::vals::LatchPin31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Latch {
    #[inline(always)]
    fn default() -> Latch {
        Latch(0)
    }
}
#[doc = "Write GPIO port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out(pub u32);
impl Out {
    #[doc = "Pin 0"]
    #[inline(always)]
    pub const fn pin0(&self) -> super::vals::OutPin0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::OutPin0::from_bits(val as u8)
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn set_pin0(&mut self, val: super::vals::OutPin0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub const fn pin1(&self) -> super::vals::OutPin1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::OutPin1::from_bits(val as u8)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn set_pin1(&mut self, val: super::vals::OutPin1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub const fn pin2(&self) -> super::vals::OutPin2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::OutPin2::from_bits(val as u8)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn set_pin2(&mut self, val: super::vals::OutPin2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub const fn pin3(&self) -> super::vals::OutPin3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::OutPin3::from_bits(val as u8)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn set_pin3(&mut self, val: super::vals::OutPin3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub const fn pin4(&self) -> super::vals::OutPin4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::OutPin4::from_bits(val as u8)
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn set_pin4(&mut self, val: super::vals::OutPin4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub const fn pin5(&self) -> super::vals::OutPin5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::OutPin5::from_bits(val as u8)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn set_pin5(&mut self, val: super::vals::OutPin5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub const fn pin6(&self) -> super::vals::OutPin6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::OutPin6::from_bits(val as u8)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn set_pin6(&mut self, val: super::vals::OutPin6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub const fn pin7(&self) -> super::vals::OutPin7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::OutPin7::from_bits(val as u8)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn set_pin7(&mut self, val: super::vals::OutPin7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub const fn pin8(&self) -> super::vals::OutPin8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::OutPin8::from_bits(val as u8)
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn set_pin8(&mut self, val: super::vals::OutPin8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub const fn pin9(&self) -> super::vals::OutPin9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::OutPin9::from_bits(val as u8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn set_pin9(&mut self, val: super::vals::OutPin9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub const fn pin10(&self) -> super::vals::OutPin10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::OutPin10::from_bits(val as u8)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn set_pin10(&mut self, val: super::vals::OutPin10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub const fn pin11(&self) -> super::vals::OutPin11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::OutPin11::from_bits(val as u8)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn set_pin11(&mut self, val: super::vals::OutPin11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub const fn pin12(&self) -> super::vals::OutPin12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::OutPin12::from_bits(val as u8)
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn set_pin12(&mut self, val: super::vals::OutPin12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub const fn pin13(&self) -> super::vals::OutPin13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::OutPin13::from_bits(val as u8)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn set_pin13(&mut self, val: super::vals::OutPin13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub const fn pin14(&self) -> super::vals::OutPin14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::OutPin14::from_bits(val as u8)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn set_pin14(&mut self, val: super::vals::OutPin14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub const fn pin15(&self) -> super::vals::OutPin15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::OutPin15::from_bits(val as u8)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn set_pin15(&mut self, val: super::vals::OutPin15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Pin 16"]
    #[inline(always)]
    pub const fn pin16(&self) -> super::vals::OutPin16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::OutPin16::from_bits(val as u8)
    }
    #[doc = "Pin 16"]
    #[inline(always)]
    pub fn set_pin16(&mut self, val: super::vals::OutPin16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Pin 17"]
    #[inline(always)]
    pub const fn pin17(&self) -> super::vals::OutPin17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::OutPin17::from_bits(val as u8)
    }
    #[doc = "Pin 17"]
    #[inline(always)]
    pub fn set_pin17(&mut self, val: super::vals::OutPin17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Pin 18"]
    #[inline(always)]
    pub const fn pin18(&self) -> super::vals::OutPin18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::OutPin18::from_bits(val as u8)
    }
    #[doc = "Pin 18"]
    #[inline(always)]
    pub fn set_pin18(&mut self, val: super::vals::OutPin18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pin 19"]
    #[inline(always)]
    pub const fn pin19(&self) -> super::vals::OutPin19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::OutPin19::from_bits(val as u8)
    }
    #[doc = "Pin 19"]
    #[inline(always)]
    pub fn set_pin19(&mut self, val: super::vals::OutPin19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Pin 20"]
    #[inline(always)]
    pub const fn pin20(&self) -> super::vals::OutPin20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::OutPin20::from_bits(val as u8)
    }
    #[doc = "Pin 20"]
    #[inline(always)]
    pub fn set_pin20(&mut self, val: super::vals::OutPin20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Pin 21"]
    #[inline(always)]
    pub const fn pin21(&self) -> super::vals::OutPin21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::OutPin21::from_bits(val as u8)
    }
    #[doc = "Pin 21"]
    #[inline(always)]
    pub fn set_pin21(&mut self, val: super::vals::OutPin21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Pin 22"]
    #[inline(always)]
    pub const fn pin22(&self) -> super::vals::OutPin22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::OutPin22::from_bits(val as u8)
    }
    #[doc = "Pin 22"]
    #[inline(always)]
    pub fn set_pin22(&mut self, val: super::vals::OutPin22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Pin 23"]
    #[inline(always)]
    pub const fn pin23(&self) -> super::vals::OutPin23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::OutPin23::from_bits(val as u8)
    }
    #[doc = "Pin 23"]
    #[inline(always)]
    pub fn set_pin23(&mut self, val: super::vals::OutPin23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Pin 24"]
    #[inline(always)]
    pub const fn pin24(&self) -> super::vals::OutPin24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::OutPin24::from_bits(val as u8)
    }
    #[doc = "Pin 24"]
    #[inline(always)]
    pub fn set_pin24(&mut self, val: super::vals::OutPin24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Pin 25"]
    #[inline(always)]
    pub const fn pin25(&self) -> super::vals::OutPin25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::OutPin25::from_bits(val as u8)
    }
    #[doc = "Pin 25"]
    #[inline(always)]
    pub fn set_pin25(&mut self, val: super::vals::OutPin25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Pin 26"]
    #[inline(always)]
    pub const fn pin26(&self) -> super::vals::OutPin26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::OutPin26::from_bits(val as u8)
    }
    #[doc = "Pin 26"]
    #[inline(always)]
    pub fn set_pin26(&mut self, val: super::vals::OutPin26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pin 27"]
    #[inline(always)]
    pub const fn pin27(&self) -> super::vals::OutPin27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::OutPin27::from_bits(val as u8)
    }
    #[doc = "Pin 27"]
    #[inline(always)]
    pub fn set_pin27(&mut self, val: super::vals::OutPin27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Pin 28"]
    #[inline(always)]
    pub const fn pin28(&self) -> super::vals::OutPin28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::OutPin28::from_bits(val as u8)
    }
    #[doc = "Pin 28"]
    #[inline(always)]
    pub fn set_pin28(&mut self, val: super::vals::OutPin28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Pin 29"]
    #[inline(always)]
    pub const fn pin29(&self) -> super::vals::OutPin29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::OutPin29::from_bits(val as u8)
    }
    #[doc = "Pin 29"]
    #[inline(always)]
    pub fn set_pin29(&mut self, val: super::vals::OutPin29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Pin 30"]
    #[inline(always)]
    pub const fn pin30(&self) -> super::vals::OutPin30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::OutPin30::from_bits(val as u8)
    }
    #[doc = "Pin 30"]
    #[inline(always)]
    pub fn set_pin30(&mut self, val: super::vals::OutPin30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Pin 31"]
    #[inline(always)]
    pub const fn pin31(&self) -> super::vals::OutPin31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::OutPin31::from_bits(val as u8)
    }
    #[doc = "Pin 31"]
    #[inline(always)]
    pub fn set_pin31(&mut self, val: super::vals::OutPin31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Out {
    #[inline(always)]
    fn default() -> Out {
        Out(0)
    }
}
#[doc = "Clear individual bits in GPIO port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outclr(pub u32);
impl Outclr {
    #[doc = "Pin 0"]
    #[inline(always)]
    pub const fn pin0(&self) -> super::vals::OutclrPin0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::OutclrPin0::from_bits(val as u8)
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn set_pin0(&mut self, val: super::vals::OutclrPin0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub const fn pin1(&self) -> super::vals::OutclrPin1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::OutclrPin1::from_bits(val as u8)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn set_pin1(&mut self, val: super::vals::OutclrPin1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub const fn pin2(&self) -> super::vals::OutclrPin2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::OutclrPin2::from_bits(val as u8)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn set_pin2(&mut self, val: super::vals::OutclrPin2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub const fn pin3(&self) -> super::vals::OutclrPin3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::OutclrPin3::from_bits(val as u8)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn set_pin3(&mut self, val: super::vals::OutclrPin3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub const fn pin4(&self) -> super::vals::OutclrPin4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::OutclrPin4::from_bits(val as u8)
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn set_pin4(&mut self, val: super::vals::OutclrPin4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub const fn pin5(&self) -> super::vals::OutclrPin5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::OutclrPin5::from_bits(val as u8)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn set_pin5(&mut self, val: super::vals::OutclrPin5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub const fn pin6(&self) -> super::vals::OutclrPin6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::OutclrPin6::from_bits(val as u8)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn set_pin6(&mut self, val: super::vals::OutclrPin6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub const fn pin7(&self) -> super::vals::OutclrPin7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::OutclrPin7::from_bits(val as u8)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn set_pin7(&mut self, val: super::vals::OutclrPin7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub const fn pin8(&self) -> super::vals::OutclrPin8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::OutclrPin8::from_bits(val as u8)
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn set_pin8(&mut self, val: super::vals::OutclrPin8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub const fn pin9(&self) -> super::vals::OutclrPin9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::OutclrPin9::from_bits(val as u8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn set_pin9(&mut self, val: super::vals::OutclrPin9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub const fn pin10(&self) -> super::vals::OutclrPin10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::OutclrPin10::from_bits(val as u8)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn set_pin10(&mut self, val: super::vals::OutclrPin10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub const fn pin11(&self) -> super::vals::OutclrPin11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::OutclrPin11::from_bits(val as u8)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn set_pin11(&mut self, val: super::vals::OutclrPin11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub const fn pin12(&self) -> super::vals::OutclrPin12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::OutclrPin12::from_bits(val as u8)
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn set_pin12(&mut self, val: super::vals::OutclrPin12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub const fn pin13(&self) -> super::vals::OutclrPin13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::OutclrPin13::from_bits(val as u8)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn set_pin13(&mut self, val: super::vals::OutclrPin13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub const fn pin14(&self) -> super::vals::OutclrPin14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::OutclrPin14::from_bits(val as u8)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn set_pin14(&mut self, val: super::vals::OutclrPin14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub const fn pin15(&self) -> super::vals::OutclrPin15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::OutclrPin15::from_bits(val as u8)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn set_pin15(&mut self, val: super::vals::OutclrPin15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Pin 16"]
    #[inline(always)]
    pub const fn pin16(&self) -> super::vals::OutclrPin16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::OutclrPin16::from_bits(val as u8)
    }
    #[doc = "Pin 16"]
    #[inline(always)]
    pub fn set_pin16(&mut self, val: super::vals::OutclrPin16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Pin 17"]
    #[inline(always)]
    pub const fn pin17(&self) -> super::vals::OutclrPin17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::OutclrPin17::from_bits(val as u8)
    }
    #[doc = "Pin 17"]
    #[inline(always)]
    pub fn set_pin17(&mut self, val: super::vals::OutclrPin17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Pin 18"]
    #[inline(always)]
    pub const fn pin18(&self) -> super::vals::OutclrPin18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::OutclrPin18::from_bits(val as u8)
    }
    #[doc = "Pin 18"]
    #[inline(always)]
    pub fn set_pin18(&mut self, val: super::vals::OutclrPin18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pin 19"]
    #[inline(always)]
    pub const fn pin19(&self) -> super::vals::OutclrPin19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::OutclrPin19::from_bits(val as u8)
    }
    #[doc = "Pin 19"]
    #[inline(always)]
    pub fn set_pin19(&mut self, val: super::vals::OutclrPin19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Pin 20"]
    #[inline(always)]
    pub const fn pin20(&self) -> super::vals::OutclrPin20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::OutclrPin20::from_bits(val as u8)
    }
    #[doc = "Pin 20"]
    #[inline(always)]
    pub fn set_pin20(&mut self, val: super::vals::OutclrPin20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Pin 21"]
    #[inline(always)]
    pub const fn pin21(&self) -> super::vals::OutclrPin21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::OutclrPin21::from_bits(val as u8)
    }
    #[doc = "Pin 21"]
    #[inline(always)]
    pub fn set_pin21(&mut self, val: super::vals::OutclrPin21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Pin 22"]
    #[inline(always)]
    pub const fn pin22(&self) -> super::vals::OutclrPin22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::OutclrPin22::from_bits(val as u8)
    }
    #[doc = "Pin 22"]
    #[inline(always)]
    pub fn set_pin22(&mut self, val: super::vals::OutclrPin22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Pin 23"]
    #[inline(always)]
    pub const fn pin23(&self) -> super::vals::OutclrPin23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::OutclrPin23::from_bits(val as u8)
    }
    #[doc = "Pin 23"]
    #[inline(always)]
    pub fn set_pin23(&mut self, val: super::vals::OutclrPin23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Pin 24"]
    #[inline(always)]
    pub const fn pin24(&self) -> super::vals::OutclrPin24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::OutclrPin24::from_bits(val as u8)
    }
    #[doc = "Pin 24"]
    #[inline(always)]
    pub fn set_pin24(&mut self, val: super::vals::OutclrPin24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Pin 25"]
    #[inline(always)]
    pub const fn pin25(&self) -> super::vals::OutclrPin25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::OutclrPin25::from_bits(val as u8)
    }
    #[doc = "Pin 25"]
    #[inline(always)]
    pub fn set_pin25(&mut self, val: super::vals::OutclrPin25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Pin 26"]
    #[inline(always)]
    pub const fn pin26(&self) -> super::vals::OutclrPin26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::OutclrPin26::from_bits(val as u8)
    }
    #[doc = "Pin 26"]
    #[inline(always)]
    pub fn set_pin26(&mut self, val: super::vals::OutclrPin26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pin 27"]
    #[inline(always)]
    pub const fn pin27(&self) -> super::vals::OutclrPin27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::OutclrPin27::from_bits(val as u8)
    }
    #[doc = "Pin 27"]
    #[inline(always)]
    pub fn set_pin27(&mut self, val: super::vals::OutclrPin27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Pin 28"]
    #[inline(always)]
    pub const fn pin28(&self) -> super::vals::OutclrPin28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::OutclrPin28::from_bits(val as u8)
    }
    #[doc = "Pin 28"]
    #[inline(always)]
    pub fn set_pin28(&mut self, val: super::vals::OutclrPin28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Pin 29"]
    #[inline(always)]
    pub const fn pin29(&self) -> super::vals::OutclrPin29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::OutclrPin29::from_bits(val as u8)
    }
    #[doc = "Pin 29"]
    #[inline(always)]
    pub fn set_pin29(&mut self, val: super::vals::OutclrPin29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Pin 30"]
    #[inline(always)]
    pub const fn pin30(&self) -> super::vals::OutclrPin30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::OutclrPin30::from_bits(val as u8)
    }
    #[doc = "Pin 30"]
    #[inline(always)]
    pub fn set_pin30(&mut self, val: super::vals::OutclrPin30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Pin 31"]
    #[inline(always)]
    pub const fn pin31(&self) -> super::vals::OutclrPin31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::OutclrPin31::from_bits(val as u8)
    }
    #[doc = "Pin 31"]
    #[inline(always)]
    pub fn set_pin31(&mut self, val: super::vals::OutclrPin31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Outclr {
    #[inline(always)]
    fn default() -> Outclr {
        Outclr(0)
    }
}
#[doc = "Set individual bits in GPIO port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outset(pub u32);
impl Outset {
    #[doc = "Pin 0"]
    #[inline(always)]
    pub const fn pin0(&self) -> super::vals::OutsetPin0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::OutsetPin0::from_bits(val as u8)
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn set_pin0(&mut self, val: super::vals::OutsetPin0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub const fn pin1(&self) -> super::vals::OutsetPin1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::OutsetPin1::from_bits(val as u8)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn set_pin1(&mut self, val: super::vals::OutsetPin1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub const fn pin2(&self) -> super::vals::OutsetPin2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::OutsetPin2::from_bits(val as u8)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn set_pin2(&mut self, val: super::vals::OutsetPin2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub const fn pin3(&self) -> super::vals::OutsetPin3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::OutsetPin3::from_bits(val as u8)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn set_pin3(&mut self, val: super::vals::OutsetPin3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub const fn pin4(&self) -> super::vals::OutsetPin4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::OutsetPin4::from_bits(val as u8)
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn set_pin4(&mut self, val: super::vals::OutsetPin4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub const fn pin5(&self) -> super::vals::OutsetPin5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::OutsetPin5::from_bits(val as u8)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn set_pin5(&mut self, val: super::vals::OutsetPin5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub const fn pin6(&self) -> super::vals::OutsetPin6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::OutsetPin6::from_bits(val as u8)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn set_pin6(&mut self, val: super::vals::OutsetPin6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub const fn pin7(&self) -> super::vals::OutsetPin7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::OutsetPin7::from_bits(val as u8)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn set_pin7(&mut self, val: super::vals::OutsetPin7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub const fn pin8(&self) -> super::vals::OutsetPin8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::OutsetPin8::from_bits(val as u8)
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn set_pin8(&mut self, val: super::vals::OutsetPin8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub const fn pin9(&self) -> super::vals::OutsetPin9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::OutsetPin9::from_bits(val as u8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn set_pin9(&mut self, val: super::vals::OutsetPin9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub const fn pin10(&self) -> super::vals::OutsetPin10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::OutsetPin10::from_bits(val as u8)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn set_pin10(&mut self, val: super::vals::OutsetPin10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub const fn pin11(&self) -> super::vals::OutsetPin11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::OutsetPin11::from_bits(val as u8)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn set_pin11(&mut self, val: super::vals::OutsetPin11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub const fn pin12(&self) -> super::vals::OutsetPin12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::OutsetPin12::from_bits(val as u8)
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn set_pin12(&mut self, val: super::vals::OutsetPin12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub const fn pin13(&self) -> super::vals::OutsetPin13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::OutsetPin13::from_bits(val as u8)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn set_pin13(&mut self, val: super::vals::OutsetPin13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub const fn pin14(&self) -> super::vals::OutsetPin14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::OutsetPin14::from_bits(val as u8)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn set_pin14(&mut self, val: super::vals::OutsetPin14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub const fn pin15(&self) -> super::vals::OutsetPin15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::OutsetPin15::from_bits(val as u8)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn set_pin15(&mut self, val: super::vals::OutsetPin15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Pin 16"]
    #[inline(always)]
    pub const fn pin16(&self) -> super::vals::OutsetPin16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::OutsetPin16::from_bits(val as u8)
    }
    #[doc = "Pin 16"]
    #[inline(always)]
    pub fn set_pin16(&mut self, val: super::vals::OutsetPin16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Pin 17"]
    #[inline(always)]
    pub const fn pin17(&self) -> super::vals::OutsetPin17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::OutsetPin17::from_bits(val as u8)
    }
    #[doc = "Pin 17"]
    #[inline(always)]
    pub fn set_pin17(&mut self, val: super::vals::OutsetPin17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Pin 18"]
    #[inline(always)]
    pub const fn pin18(&self) -> super::vals::OutsetPin18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::OutsetPin18::from_bits(val as u8)
    }
    #[doc = "Pin 18"]
    #[inline(always)]
    pub fn set_pin18(&mut self, val: super::vals::OutsetPin18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Pin 19"]
    #[inline(always)]
    pub const fn pin19(&self) -> super::vals::OutsetPin19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::OutsetPin19::from_bits(val as u8)
    }
    #[doc = "Pin 19"]
    #[inline(always)]
    pub fn set_pin19(&mut self, val: super::vals::OutsetPin19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Pin 20"]
    #[inline(always)]
    pub const fn pin20(&self) -> super::vals::OutsetPin20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::OutsetPin20::from_bits(val as u8)
    }
    #[doc = "Pin 20"]
    #[inline(always)]
    pub fn set_pin20(&mut self, val: super::vals::OutsetPin20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Pin 21"]
    #[inline(always)]
    pub const fn pin21(&self) -> super::vals::OutsetPin21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::OutsetPin21::from_bits(val as u8)
    }
    #[doc = "Pin 21"]
    #[inline(always)]
    pub fn set_pin21(&mut self, val: super::vals::OutsetPin21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Pin 22"]
    #[inline(always)]
    pub const fn pin22(&self) -> super::vals::OutsetPin22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::OutsetPin22::from_bits(val as u8)
    }
    #[doc = "Pin 22"]
    #[inline(always)]
    pub fn set_pin22(&mut self, val: super::vals::OutsetPin22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Pin 23"]
    #[inline(always)]
    pub const fn pin23(&self) -> super::vals::OutsetPin23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::OutsetPin23::from_bits(val as u8)
    }
    #[doc = "Pin 23"]
    #[inline(always)]
    pub fn set_pin23(&mut self, val: super::vals::OutsetPin23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Pin 24"]
    #[inline(always)]
    pub const fn pin24(&self) -> super::vals::OutsetPin24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::OutsetPin24::from_bits(val as u8)
    }
    #[doc = "Pin 24"]
    #[inline(always)]
    pub fn set_pin24(&mut self, val: super::vals::OutsetPin24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Pin 25"]
    #[inline(always)]
    pub const fn pin25(&self) -> super::vals::OutsetPin25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::OutsetPin25::from_bits(val as u8)
    }
    #[doc = "Pin 25"]
    #[inline(always)]
    pub fn set_pin25(&mut self, val: super::vals::OutsetPin25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Pin 26"]
    #[inline(always)]
    pub const fn pin26(&self) -> super::vals::OutsetPin26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::OutsetPin26::from_bits(val as u8)
    }
    #[doc = "Pin 26"]
    #[inline(always)]
    pub fn set_pin26(&mut self, val: super::vals::OutsetPin26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Pin 27"]
    #[inline(always)]
    pub const fn pin27(&self) -> super::vals::OutsetPin27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::OutsetPin27::from_bits(val as u8)
    }
    #[doc = "Pin 27"]
    #[inline(always)]
    pub fn set_pin27(&mut self, val: super::vals::OutsetPin27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Pin 28"]
    #[inline(always)]
    pub const fn pin28(&self) -> super::vals::OutsetPin28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::OutsetPin28::from_bits(val as u8)
    }
    #[doc = "Pin 28"]
    #[inline(always)]
    pub fn set_pin28(&mut self, val: super::vals::OutsetPin28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Pin 29"]
    #[inline(always)]
    pub const fn pin29(&self) -> super::vals::OutsetPin29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::OutsetPin29::from_bits(val as u8)
    }
    #[doc = "Pin 29"]
    #[inline(always)]
    pub fn set_pin29(&mut self, val: super::vals::OutsetPin29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Pin 30"]
    #[inline(always)]
    pub const fn pin30(&self) -> super::vals::OutsetPin30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::OutsetPin30::from_bits(val as u8)
    }
    #[doc = "Pin 30"]
    #[inline(always)]
    pub fn set_pin30(&mut self, val: super::vals::OutsetPin30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Pin 31"]
    #[inline(always)]
    pub const fn pin31(&self) -> super::vals::OutsetPin31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::OutsetPin31::from_bits(val as u8)
    }
    #[doc = "Pin 31"]
    #[inline(always)]
    pub fn set_pin31(&mut self, val: super::vals::OutsetPin31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Outset {
    #[inline(always)]
    fn default() -> Outset {
        Outset(0)
    }
}
#[doc = "Description collection: Configuration of GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PinCnf(pub u32);
impl PinCnf {
    #[doc = "Pin direction. Same physical register as DIR register"]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::Dir {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dir::from_bits(val as u8)
    }
    #[doc = "Pin direction. Same physical register as DIR register"]
    #[inline(always)]
    pub fn set_dir(&mut self, val: super::vals::Dir) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Connect or disconnect input buffer"]
    #[inline(always)]
    pub const fn input(&self) -> super::vals::Input {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Input::from_bits(val as u8)
    }
    #[doc = "Connect or disconnect input buffer"]
    #[inline(always)]
    pub fn set_input(&mut self, val: super::vals::Input) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull configuration"]
    #[inline(always)]
    pub const fn pull(&self) -> super::vals::Pull {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pull::from_bits(val as u8)
    }
    #[doc = "Pull configuration"]
    #[inline(always)]
    pub fn set_pull(&mut self, val: super::vals::Pull) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Drive configuration"]
    #[inline(always)]
    pub const fn drive(&self) -> super::vals::Drive {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Drive::from_bits(val as u8)
    }
    #[doc = "Drive configuration"]
    #[inline(always)]
    pub fn set_drive(&mut self, val: super::vals::Drive) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Pin sensing mechanism"]
    #[inline(always)]
    pub const fn sense(&self) -> super::vals::Sense {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Sense::from_bits(val as u8)
    }
    #[doc = "Pin sensing mechanism"]
    #[inline(always)]
    pub fn set_sense(&mut self, val: super::vals::Sense) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Select which MCU/Subsystem controls this pin Note: this field is only accessible from secure code."]
    #[inline(always)]
    pub const fn mcusel(&self) -> super::vals::Mcusel {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mcusel::from_bits(val as u8)
    }
    #[doc = "Select which MCU/Subsystem controls this pin Note: this field is only accessible from secure code."]
    #[inline(always)]
    pub fn set_mcusel(&mut self, val: super::vals::Mcusel) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for PinCnf {
    #[inline(always)]
    fn default() -> PinCnf {
        PinCnf(0)
    }
}
