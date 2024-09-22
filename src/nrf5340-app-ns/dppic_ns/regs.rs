#[doc = "Channel enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chen(pub u32);
impl Chen {
    #[doc = "Enable or disable channel 0"]
    #[inline(always)]
    pub const fn ch0(&self) -> super::vals::ChenCh0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ChenCh0::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 0"]
    #[inline(always)]
    pub fn set_ch0(&mut self, val: super::vals::ChenCh0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable channel 1"]
    #[inline(always)]
    pub const fn ch1(&self) -> super::vals::ChenCh1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ChenCh1::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 1"]
    #[inline(always)]
    pub fn set_ch1(&mut self, val: super::vals::ChenCh1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable channel 2"]
    #[inline(always)]
    pub const fn ch2(&self) -> super::vals::ChenCh2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ChenCh2::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 2"]
    #[inline(always)]
    pub fn set_ch2(&mut self, val: super::vals::ChenCh2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable or disable channel 3"]
    #[inline(always)]
    pub const fn ch3(&self) -> super::vals::ChenCh3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ChenCh3::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 3"]
    #[inline(always)]
    pub fn set_ch3(&mut self, val: super::vals::ChenCh3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable or disable channel 4"]
    #[inline(always)]
    pub const fn ch4(&self) -> super::vals::ChenCh4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ChenCh4::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 4"]
    #[inline(always)]
    pub fn set_ch4(&mut self, val: super::vals::ChenCh4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable or disable channel 5"]
    #[inline(always)]
    pub const fn ch5(&self) -> super::vals::ChenCh5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ChenCh5::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 5"]
    #[inline(always)]
    pub fn set_ch5(&mut self, val: super::vals::ChenCh5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable or disable channel 6"]
    #[inline(always)]
    pub const fn ch6(&self) -> super::vals::ChenCh6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ChenCh6::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 6"]
    #[inline(always)]
    pub fn set_ch6(&mut self, val: super::vals::ChenCh6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable or disable channel 7"]
    #[inline(always)]
    pub const fn ch7(&self) -> super::vals::ChenCh7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ChenCh7::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 7"]
    #[inline(always)]
    pub fn set_ch7(&mut self, val: super::vals::ChenCh7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable or disable channel 8"]
    #[inline(always)]
    pub const fn ch8(&self) -> super::vals::ChenCh8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ChenCh8::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 8"]
    #[inline(always)]
    pub fn set_ch8(&mut self, val: super::vals::ChenCh8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable or disable channel 9"]
    #[inline(always)]
    pub const fn ch9(&self) -> super::vals::ChenCh9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::ChenCh9::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 9"]
    #[inline(always)]
    pub fn set_ch9(&mut self, val: super::vals::ChenCh9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable or disable channel 10"]
    #[inline(always)]
    pub const fn ch10(&self) -> super::vals::ChenCh10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ChenCh10::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 10"]
    #[inline(always)]
    pub fn set_ch10(&mut self, val: super::vals::ChenCh10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable or disable channel 11"]
    #[inline(always)]
    pub const fn ch11(&self) -> super::vals::ChenCh11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::ChenCh11::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 11"]
    #[inline(always)]
    pub fn set_ch11(&mut self, val: super::vals::ChenCh11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable or disable channel 12"]
    #[inline(always)]
    pub const fn ch12(&self) -> super::vals::ChenCh12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::ChenCh12::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 12"]
    #[inline(always)]
    pub fn set_ch12(&mut self, val: super::vals::ChenCh12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable or disable channel 13"]
    #[inline(always)]
    pub const fn ch13(&self) -> super::vals::ChenCh13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::ChenCh13::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 13"]
    #[inline(always)]
    pub fn set_ch13(&mut self, val: super::vals::ChenCh13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable or disable channel 14"]
    #[inline(always)]
    pub const fn ch14(&self) -> super::vals::ChenCh14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::ChenCh14::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 14"]
    #[inline(always)]
    pub fn set_ch14(&mut self, val: super::vals::ChenCh14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable or disable channel 15"]
    #[inline(always)]
    pub const fn ch15(&self) -> super::vals::ChenCh15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::ChenCh15::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 15"]
    #[inline(always)]
    pub fn set_ch15(&mut self, val: super::vals::ChenCh15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable or disable channel 16"]
    #[inline(always)]
    pub const fn ch16(&self) -> super::vals::ChenCh16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::ChenCh16::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 16"]
    #[inline(always)]
    pub fn set_ch16(&mut self, val: super::vals::ChenCh16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable or disable channel 17"]
    #[inline(always)]
    pub const fn ch17(&self) -> super::vals::ChenCh17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::ChenCh17::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 17"]
    #[inline(always)]
    pub fn set_ch17(&mut self, val: super::vals::ChenCh17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable or disable channel 18"]
    #[inline(always)]
    pub const fn ch18(&self) -> super::vals::ChenCh18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::ChenCh18::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 18"]
    #[inline(always)]
    pub fn set_ch18(&mut self, val: super::vals::ChenCh18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable or disable channel 19"]
    #[inline(always)]
    pub const fn ch19(&self) -> super::vals::ChenCh19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::ChenCh19::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 19"]
    #[inline(always)]
    pub fn set_ch19(&mut self, val: super::vals::ChenCh19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Enable or disable channel 20"]
    #[inline(always)]
    pub const fn ch20(&self) -> super::vals::ChenCh20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::ChenCh20::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 20"]
    #[inline(always)]
    pub fn set_ch20(&mut self, val: super::vals::ChenCh20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable or disable channel 21"]
    #[inline(always)]
    pub const fn ch21(&self) -> super::vals::ChenCh21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::ChenCh21::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 21"]
    #[inline(always)]
    pub fn set_ch21(&mut self, val: super::vals::ChenCh21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable or disable channel 22"]
    #[inline(always)]
    pub const fn ch22(&self) -> super::vals::ChenCh22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::ChenCh22::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 22"]
    #[inline(always)]
    pub fn set_ch22(&mut self, val: super::vals::ChenCh22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable or disable channel 23"]
    #[inline(always)]
    pub const fn ch23(&self) -> super::vals::ChenCh23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::ChenCh23::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 23"]
    #[inline(always)]
    pub fn set_ch23(&mut self, val: super::vals::ChenCh23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable or disable channel 24"]
    #[inline(always)]
    pub const fn ch24(&self) -> super::vals::ChenCh24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::ChenCh24::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 24"]
    #[inline(always)]
    pub fn set_ch24(&mut self, val: super::vals::ChenCh24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable or disable channel 25"]
    #[inline(always)]
    pub const fn ch25(&self) -> super::vals::ChenCh25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::ChenCh25::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 25"]
    #[inline(always)]
    pub fn set_ch25(&mut self, val: super::vals::ChenCh25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable or disable channel 26"]
    #[inline(always)]
    pub const fn ch26(&self) -> super::vals::ChenCh26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::ChenCh26::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 26"]
    #[inline(always)]
    pub fn set_ch26(&mut self, val: super::vals::ChenCh26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Enable or disable channel 27"]
    #[inline(always)]
    pub const fn ch27(&self) -> super::vals::ChenCh27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::ChenCh27::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 27"]
    #[inline(always)]
    pub fn set_ch27(&mut self, val: super::vals::ChenCh27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Enable or disable channel 28"]
    #[inline(always)]
    pub const fn ch28(&self) -> super::vals::ChenCh28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::ChenCh28::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 28"]
    #[inline(always)]
    pub fn set_ch28(&mut self, val: super::vals::ChenCh28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Enable or disable channel 29"]
    #[inline(always)]
    pub const fn ch29(&self) -> super::vals::ChenCh29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ChenCh29::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 29"]
    #[inline(always)]
    pub fn set_ch29(&mut self, val: super::vals::ChenCh29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Enable or disable channel 30"]
    #[inline(always)]
    pub const fn ch30(&self) -> super::vals::ChenCh30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ChenCh30::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 30"]
    #[inline(always)]
    pub fn set_ch30(&mut self, val: super::vals::ChenCh30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable or disable channel 31"]
    #[inline(always)]
    pub const fn ch31(&self) -> super::vals::ChenCh31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ChenCh31::from_bits(val as u8)
    }
    #[doc = "Enable or disable channel 31"]
    #[inline(always)]
    pub fn set_ch31(&mut self, val: super::vals::ChenCh31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Chen {
    #[inline(always)]
    fn default() -> Chen {
        Chen(0)
    }
}
#[doc = "Channel enable clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chenclr(pub u32);
impl Chenclr {
    #[doc = "Channel 0 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch0(&self) -> super::vals::ChenclrCh0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ChenclrCh0::from_bits(val as u8)
    }
    #[doc = "Channel 0 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch0(&mut self, val: super::vals::ChenclrCh0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch1(&self) -> super::vals::ChenclrCh1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ChenclrCh1::from_bits(val as u8)
    }
    #[doc = "Channel 1 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch1(&mut self, val: super::vals::ChenclrCh1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch2(&self) -> super::vals::ChenclrCh2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ChenclrCh2::from_bits(val as u8)
    }
    #[doc = "Channel 2 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch2(&mut self, val: super::vals::ChenclrCh2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch3(&self) -> super::vals::ChenclrCh3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ChenclrCh3::from_bits(val as u8)
    }
    #[doc = "Channel 3 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch3(&mut self, val: super::vals::ChenclrCh3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch4(&self) -> super::vals::ChenclrCh4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ChenclrCh4::from_bits(val as u8)
    }
    #[doc = "Channel 4 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch4(&mut self, val: super::vals::ChenclrCh4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch5(&self) -> super::vals::ChenclrCh5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ChenclrCh5::from_bits(val as u8)
    }
    #[doc = "Channel 5 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch5(&mut self, val: super::vals::ChenclrCh5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch6(&self) -> super::vals::ChenclrCh6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ChenclrCh6::from_bits(val as u8)
    }
    #[doc = "Channel 6 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch6(&mut self, val: super::vals::ChenclrCh6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch7(&self) -> super::vals::ChenclrCh7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ChenclrCh7::from_bits(val as u8)
    }
    #[doc = "Channel 7 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch7(&mut self, val: super::vals::ChenclrCh7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Channel 8 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch8(&self) -> super::vals::ChenclrCh8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ChenclrCh8::from_bits(val as u8)
    }
    #[doc = "Channel 8 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch8(&mut self, val: super::vals::ChenclrCh8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel 9 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch9(&self) -> super::vals::ChenclrCh9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::ChenclrCh9::from_bits(val as u8)
    }
    #[doc = "Channel 9 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch9(&mut self, val: super::vals::ChenclrCh9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Channel 10 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch10(&self) -> super::vals::ChenclrCh10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ChenclrCh10::from_bits(val as u8)
    }
    #[doc = "Channel 10 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch10(&mut self, val: super::vals::ChenclrCh10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Channel 11 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch11(&self) -> super::vals::ChenclrCh11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::ChenclrCh11::from_bits(val as u8)
    }
    #[doc = "Channel 11 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch11(&mut self, val: super::vals::ChenclrCh11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Channel 12 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch12(&self) -> super::vals::ChenclrCh12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::ChenclrCh12::from_bits(val as u8)
    }
    #[doc = "Channel 12 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch12(&mut self, val: super::vals::ChenclrCh12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Channel 13 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch13(&self) -> super::vals::ChenclrCh13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::ChenclrCh13::from_bits(val as u8)
    }
    #[doc = "Channel 13 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch13(&mut self, val: super::vals::ChenclrCh13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Channel 14 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch14(&self) -> super::vals::ChenclrCh14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::ChenclrCh14::from_bits(val as u8)
    }
    #[doc = "Channel 14 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch14(&mut self, val: super::vals::ChenclrCh14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Channel 15 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch15(&self) -> super::vals::ChenclrCh15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::ChenclrCh15::from_bits(val as u8)
    }
    #[doc = "Channel 15 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch15(&mut self, val: super::vals::ChenclrCh15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Channel 16 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch16(&self) -> super::vals::ChenclrCh16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::ChenclrCh16::from_bits(val as u8)
    }
    #[doc = "Channel 16 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch16(&mut self, val: super::vals::ChenclrCh16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Channel 17 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch17(&self) -> super::vals::ChenclrCh17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::ChenclrCh17::from_bits(val as u8)
    }
    #[doc = "Channel 17 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch17(&mut self, val: super::vals::ChenclrCh17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Channel 18 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch18(&self) -> super::vals::ChenclrCh18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::ChenclrCh18::from_bits(val as u8)
    }
    #[doc = "Channel 18 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch18(&mut self, val: super::vals::ChenclrCh18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Channel 19 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch19(&self) -> super::vals::ChenclrCh19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::ChenclrCh19::from_bits(val as u8)
    }
    #[doc = "Channel 19 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch19(&mut self, val: super::vals::ChenclrCh19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Channel 20 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch20(&self) -> super::vals::ChenclrCh20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::ChenclrCh20::from_bits(val as u8)
    }
    #[doc = "Channel 20 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch20(&mut self, val: super::vals::ChenclrCh20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Channel 21 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch21(&self) -> super::vals::ChenclrCh21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::ChenclrCh21::from_bits(val as u8)
    }
    #[doc = "Channel 21 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch21(&mut self, val: super::vals::ChenclrCh21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Channel 22 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch22(&self) -> super::vals::ChenclrCh22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::ChenclrCh22::from_bits(val as u8)
    }
    #[doc = "Channel 22 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch22(&mut self, val: super::vals::ChenclrCh22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Channel 23 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch23(&self) -> super::vals::ChenclrCh23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::ChenclrCh23::from_bits(val as u8)
    }
    #[doc = "Channel 23 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch23(&mut self, val: super::vals::ChenclrCh23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Channel 24 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch24(&self) -> super::vals::ChenclrCh24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::ChenclrCh24::from_bits(val as u8)
    }
    #[doc = "Channel 24 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch24(&mut self, val: super::vals::ChenclrCh24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Channel 25 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch25(&self) -> super::vals::ChenclrCh25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::ChenclrCh25::from_bits(val as u8)
    }
    #[doc = "Channel 25 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch25(&mut self, val: super::vals::ChenclrCh25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Channel 26 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch26(&self) -> super::vals::ChenclrCh26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::ChenclrCh26::from_bits(val as u8)
    }
    #[doc = "Channel 26 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch26(&mut self, val: super::vals::ChenclrCh26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Channel 27 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch27(&self) -> super::vals::ChenclrCh27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::ChenclrCh27::from_bits(val as u8)
    }
    #[doc = "Channel 27 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch27(&mut self, val: super::vals::ChenclrCh27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Channel 28 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch28(&self) -> super::vals::ChenclrCh28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::ChenclrCh28::from_bits(val as u8)
    }
    #[doc = "Channel 28 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch28(&mut self, val: super::vals::ChenclrCh28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Channel 29 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch29(&self) -> super::vals::ChenclrCh29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ChenclrCh29::from_bits(val as u8)
    }
    #[doc = "Channel 29 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch29(&mut self, val: super::vals::ChenclrCh29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Channel 30 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch30(&self) -> super::vals::ChenclrCh30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ChenclrCh30::from_bits(val as u8)
    }
    #[doc = "Channel 30 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch30(&mut self, val: super::vals::ChenclrCh30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Channel 31 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch31(&self) -> super::vals::ChenclrCh31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ChenclrCh31::from_bits(val as u8)
    }
    #[doc = "Channel 31 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch31(&mut self, val: super::vals::ChenclrCh31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Chenclr {
    #[inline(always)]
    fn default() -> Chenclr {
        Chenclr(0)
    }
}
#[doc = "Channel enable set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chenset(pub u32);
impl Chenset {
    #[doc = "Channel 0 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch0(&self) -> super::vals::ChensetCh0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ChensetCh0::from_bits(val as u8)
    }
    #[doc = "Channel 0 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch0(&mut self, val: super::vals::ChensetCh0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch1(&self) -> super::vals::ChensetCh1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ChensetCh1::from_bits(val as u8)
    }
    #[doc = "Channel 1 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch1(&mut self, val: super::vals::ChensetCh1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch2(&self) -> super::vals::ChensetCh2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ChensetCh2::from_bits(val as u8)
    }
    #[doc = "Channel 2 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch2(&mut self, val: super::vals::ChensetCh2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch3(&self) -> super::vals::ChensetCh3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ChensetCh3::from_bits(val as u8)
    }
    #[doc = "Channel 3 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch3(&mut self, val: super::vals::ChensetCh3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch4(&self) -> super::vals::ChensetCh4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ChensetCh4::from_bits(val as u8)
    }
    #[doc = "Channel 4 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch4(&mut self, val: super::vals::ChensetCh4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch5(&self) -> super::vals::ChensetCh5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ChensetCh5::from_bits(val as u8)
    }
    #[doc = "Channel 5 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch5(&mut self, val: super::vals::ChensetCh5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch6(&self) -> super::vals::ChensetCh6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ChensetCh6::from_bits(val as u8)
    }
    #[doc = "Channel 6 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch6(&mut self, val: super::vals::ChensetCh6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch7(&self) -> super::vals::ChensetCh7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ChensetCh7::from_bits(val as u8)
    }
    #[doc = "Channel 7 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch7(&mut self, val: super::vals::ChensetCh7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Channel 8 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch8(&self) -> super::vals::ChensetCh8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ChensetCh8::from_bits(val as u8)
    }
    #[doc = "Channel 8 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch8(&mut self, val: super::vals::ChensetCh8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel 9 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch9(&self) -> super::vals::ChensetCh9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::ChensetCh9::from_bits(val as u8)
    }
    #[doc = "Channel 9 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch9(&mut self, val: super::vals::ChensetCh9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Channel 10 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch10(&self) -> super::vals::ChensetCh10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ChensetCh10::from_bits(val as u8)
    }
    #[doc = "Channel 10 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch10(&mut self, val: super::vals::ChensetCh10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Channel 11 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch11(&self) -> super::vals::ChensetCh11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::ChensetCh11::from_bits(val as u8)
    }
    #[doc = "Channel 11 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch11(&mut self, val: super::vals::ChensetCh11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Channel 12 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch12(&self) -> super::vals::ChensetCh12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::ChensetCh12::from_bits(val as u8)
    }
    #[doc = "Channel 12 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch12(&mut self, val: super::vals::ChensetCh12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Channel 13 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch13(&self) -> super::vals::ChensetCh13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::ChensetCh13::from_bits(val as u8)
    }
    #[doc = "Channel 13 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch13(&mut self, val: super::vals::ChensetCh13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Channel 14 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch14(&self) -> super::vals::ChensetCh14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::ChensetCh14::from_bits(val as u8)
    }
    #[doc = "Channel 14 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch14(&mut self, val: super::vals::ChensetCh14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Channel 15 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch15(&self) -> super::vals::ChensetCh15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::ChensetCh15::from_bits(val as u8)
    }
    #[doc = "Channel 15 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch15(&mut self, val: super::vals::ChensetCh15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Channel 16 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch16(&self) -> super::vals::ChensetCh16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::ChensetCh16::from_bits(val as u8)
    }
    #[doc = "Channel 16 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch16(&mut self, val: super::vals::ChensetCh16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Channel 17 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch17(&self) -> super::vals::ChensetCh17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::ChensetCh17::from_bits(val as u8)
    }
    #[doc = "Channel 17 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch17(&mut self, val: super::vals::ChensetCh17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Channel 18 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch18(&self) -> super::vals::ChensetCh18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::ChensetCh18::from_bits(val as u8)
    }
    #[doc = "Channel 18 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch18(&mut self, val: super::vals::ChensetCh18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Channel 19 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch19(&self) -> super::vals::ChensetCh19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::ChensetCh19::from_bits(val as u8)
    }
    #[doc = "Channel 19 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch19(&mut self, val: super::vals::ChensetCh19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Channel 20 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch20(&self) -> super::vals::ChensetCh20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::ChensetCh20::from_bits(val as u8)
    }
    #[doc = "Channel 20 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch20(&mut self, val: super::vals::ChensetCh20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Channel 21 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch21(&self) -> super::vals::ChensetCh21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::ChensetCh21::from_bits(val as u8)
    }
    #[doc = "Channel 21 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch21(&mut self, val: super::vals::ChensetCh21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Channel 22 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch22(&self) -> super::vals::ChensetCh22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::ChensetCh22::from_bits(val as u8)
    }
    #[doc = "Channel 22 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch22(&mut self, val: super::vals::ChensetCh22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Channel 23 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch23(&self) -> super::vals::ChensetCh23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::ChensetCh23::from_bits(val as u8)
    }
    #[doc = "Channel 23 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch23(&mut self, val: super::vals::ChensetCh23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Channel 24 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch24(&self) -> super::vals::ChensetCh24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::ChensetCh24::from_bits(val as u8)
    }
    #[doc = "Channel 24 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch24(&mut self, val: super::vals::ChensetCh24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Channel 25 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch25(&self) -> super::vals::ChensetCh25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::ChensetCh25::from_bits(val as u8)
    }
    #[doc = "Channel 25 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch25(&mut self, val: super::vals::ChensetCh25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Channel 26 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch26(&self) -> super::vals::ChensetCh26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::ChensetCh26::from_bits(val as u8)
    }
    #[doc = "Channel 26 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch26(&mut self, val: super::vals::ChensetCh26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Channel 27 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch27(&self) -> super::vals::ChensetCh27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::ChensetCh27::from_bits(val as u8)
    }
    #[doc = "Channel 27 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch27(&mut self, val: super::vals::ChensetCh27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Channel 28 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch28(&self) -> super::vals::ChensetCh28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::ChensetCh28::from_bits(val as u8)
    }
    #[doc = "Channel 28 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch28(&mut self, val: super::vals::ChensetCh28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Channel 29 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch29(&self) -> super::vals::ChensetCh29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ChensetCh29::from_bits(val as u8)
    }
    #[doc = "Channel 29 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch29(&mut self, val: super::vals::ChensetCh29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Channel 30 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch30(&self) -> super::vals::ChensetCh30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ChensetCh30::from_bits(val as u8)
    }
    #[doc = "Channel 30 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch30(&mut self, val: super::vals::ChensetCh30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Channel 31 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ch31(&self) -> super::vals::ChensetCh31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ChensetCh31::from_bits(val as u8)
    }
    #[doc = "Channel 31 enable set register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn set_ch31(&mut self, val: super::vals::ChensetCh31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Chenset {
    #[inline(always)]
    fn default() -> Chenset {
        Chenset(0)
    }
}
#[doc = "Description collection: Channel group n Note: Writes to this register are ignored if either SUBSCRIBE_CHG\\[n\\].EN or SUBSCRIBE_CHG\\[n\\].DIS is enabled"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chg(pub u32);
impl Chg {
    #[doc = "Include or exclude channel 0"]
    #[inline(always)]
    pub const fn ch0(&self) -> super::vals::ChgCh0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ChgCh0::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 0"]
    #[inline(always)]
    pub fn set_ch0(&mut self, val: super::vals::ChgCh0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Include or exclude channel 1"]
    #[inline(always)]
    pub const fn ch1(&self) -> super::vals::ChgCh1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ChgCh1::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 1"]
    #[inline(always)]
    pub fn set_ch1(&mut self, val: super::vals::ChgCh1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Include or exclude channel 2"]
    #[inline(always)]
    pub const fn ch2(&self) -> super::vals::ChgCh2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ChgCh2::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 2"]
    #[inline(always)]
    pub fn set_ch2(&mut self, val: super::vals::ChgCh2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Include or exclude channel 3"]
    #[inline(always)]
    pub const fn ch3(&self) -> super::vals::ChgCh3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ChgCh3::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 3"]
    #[inline(always)]
    pub fn set_ch3(&mut self, val: super::vals::ChgCh3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Include or exclude channel 4"]
    #[inline(always)]
    pub const fn ch4(&self) -> super::vals::ChgCh4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ChgCh4::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 4"]
    #[inline(always)]
    pub fn set_ch4(&mut self, val: super::vals::ChgCh4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Include or exclude channel 5"]
    #[inline(always)]
    pub const fn ch5(&self) -> super::vals::ChgCh5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ChgCh5::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 5"]
    #[inline(always)]
    pub fn set_ch5(&mut self, val: super::vals::ChgCh5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Include or exclude channel 6"]
    #[inline(always)]
    pub const fn ch6(&self) -> super::vals::ChgCh6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ChgCh6::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 6"]
    #[inline(always)]
    pub fn set_ch6(&mut self, val: super::vals::ChgCh6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Include or exclude channel 7"]
    #[inline(always)]
    pub const fn ch7(&self) -> super::vals::ChgCh7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ChgCh7::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 7"]
    #[inline(always)]
    pub fn set_ch7(&mut self, val: super::vals::ChgCh7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Include or exclude channel 8"]
    #[inline(always)]
    pub const fn ch8(&self) -> super::vals::ChgCh8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ChgCh8::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 8"]
    #[inline(always)]
    pub fn set_ch8(&mut self, val: super::vals::ChgCh8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Include or exclude channel 9"]
    #[inline(always)]
    pub const fn ch9(&self) -> super::vals::ChgCh9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::ChgCh9::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 9"]
    #[inline(always)]
    pub fn set_ch9(&mut self, val: super::vals::ChgCh9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Include or exclude channel 10"]
    #[inline(always)]
    pub const fn ch10(&self) -> super::vals::ChgCh10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ChgCh10::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 10"]
    #[inline(always)]
    pub fn set_ch10(&mut self, val: super::vals::ChgCh10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Include or exclude channel 11"]
    #[inline(always)]
    pub const fn ch11(&self) -> super::vals::ChgCh11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::ChgCh11::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 11"]
    #[inline(always)]
    pub fn set_ch11(&mut self, val: super::vals::ChgCh11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Include or exclude channel 12"]
    #[inline(always)]
    pub const fn ch12(&self) -> super::vals::ChgCh12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::ChgCh12::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 12"]
    #[inline(always)]
    pub fn set_ch12(&mut self, val: super::vals::ChgCh12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Include or exclude channel 13"]
    #[inline(always)]
    pub const fn ch13(&self) -> super::vals::ChgCh13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::ChgCh13::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 13"]
    #[inline(always)]
    pub fn set_ch13(&mut self, val: super::vals::ChgCh13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Include or exclude channel 14"]
    #[inline(always)]
    pub const fn ch14(&self) -> super::vals::ChgCh14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::ChgCh14::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 14"]
    #[inline(always)]
    pub fn set_ch14(&mut self, val: super::vals::ChgCh14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Include or exclude channel 15"]
    #[inline(always)]
    pub const fn ch15(&self) -> super::vals::ChgCh15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::ChgCh15::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 15"]
    #[inline(always)]
    pub fn set_ch15(&mut self, val: super::vals::ChgCh15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Include or exclude channel 16"]
    #[inline(always)]
    pub const fn ch16(&self) -> super::vals::ChgCh16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::ChgCh16::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 16"]
    #[inline(always)]
    pub fn set_ch16(&mut self, val: super::vals::ChgCh16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Include or exclude channel 17"]
    #[inline(always)]
    pub const fn ch17(&self) -> super::vals::ChgCh17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::ChgCh17::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 17"]
    #[inline(always)]
    pub fn set_ch17(&mut self, val: super::vals::ChgCh17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Include or exclude channel 18"]
    #[inline(always)]
    pub const fn ch18(&self) -> super::vals::ChgCh18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::ChgCh18::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 18"]
    #[inline(always)]
    pub fn set_ch18(&mut self, val: super::vals::ChgCh18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Include or exclude channel 19"]
    #[inline(always)]
    pub const fn ch19(&self) -> super::vals::ChgCh19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::ChgCh19::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 19"]
    #[inline(always)]
    pub fn set_ch19(&mut self, val: super::vals::ChgCh19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Include or exclude channel 20"]
    #[inline(always)]
    pub const fn ch20(&self) -> super::vals::ChgCh20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::ChgCh20::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 20"]
    #[inline(always)]
    pub fn set_ch20(&mut self, val: super::vals::ChgCh20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Include or exclude channel 21"]
    #[inline(always)]
    pub const fn ch21(&self) -> super::vals::ChgCh21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::ChgCh21::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 21"]
    #[inline(always)]
    pub fn set_ch21(&mut self, val: super::vals::ChgCh21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Include or exclude channel 22"]
    #[inline(always)]
    pub const fn ch22(&self) -> super::vals::ChgCh22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::ChgCh22::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 22"]
    #[inline(always)]
    pub fn set_ch22(&mut self, val: super::vals::ChgCh22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Include or exclude channel 23"]
    #[inline(always)]
    pub const fn ch23(&self) -> super::vals::ChgCh23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::ChgCh23::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 23"]
    #[inline(always)]
    pub fn set_ch23(&mut self, val: super::vals::ChgCh23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Include or exclude channel 24"]
    #[inline(always)]
    pub const fn ch24(&self) -> super::vals::ChgCh24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::ChgCh24::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 24"]
    #[inline(always)]
    pub fn set_ch24(&mut self, val: super::vals::ChgCh24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Include or exclude channel 25"]
    #[inline(always)]
    pub const fn ch25(&self) -> super::vals::ChgCh25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::ChgCh25::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 25"]
    #[inline(always)]
    pub fn set_ch25(&mut self, val: super::vals::ChgCh25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Include or exclude channel 26"]
    #[inline(always)]
    pub const fn ch26(&self) -> super::vals::ChgCh26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::ChgCh26::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 26"]
    #[inline(always)]
    pub fn set_ch26(&mut self, val: super::vals::ChgCh26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Include or exclude channel 27"]
    #[inline(always)]
    pub const fn ch27(&self) -> super::vals::ChgCh27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::ChgCh27::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 27"]
    #[inline(always)]
    pub fn set_ch27(&mut self, val: super::vals::ChgCh27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Include or exclude channel 28"]
    #[inline(always)]
    pub const fn ch28(&self) -> super::vals::ChgCh28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::ChgCh28::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 28"]
    #[inline(always)]
    pub fn set_ch28(&mut self, val: super::vals::ChgCh28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Include or exclude channel 29"]
    #[inline(always)]
    pub const fn ch29(&self) -> super::vals::ChgCh29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ChgCh29::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 29"]
    #[inline(always)]
    pub fn set_ch29(&mut self, val: super::vals::ChgCh29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Include or exclude channel 30"]
    #[inline(always)]
    pub const fn ch30(&self) -> super::vals::ChgCh30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ChgCh30::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 30"]
    #[inline(always)]
    pub fn set_ch30(&mut self, val: super::vals::ChgCh30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Include or exclude channel 31"]
    #[inline(always)]
    pub const fn ch31(&self) -> super::vals::ChgCh31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ChgCh31::from_bits(val as u8)
    }
    #[doc = "Include or exclude channel 31"]
    #[inline(always)]
    pub fn set_ch31(&mut self, val: super::vals::ChgCh31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Chg {
    #[inline(always)]
    fn default() -> Chg {
        Chg(0)
    }
}
#[doc = "Description cluster: Subscribe configuration for task CHG\\[n\\].DIS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeChgDis(pub u32);
impl SubscribeChgDis {
    #[doc = "DPPI channel that task CHG\\[n\\].DIS will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task CHG\\[n\\].DIS will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::DisEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::DisEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::DisEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeChgDis {
    #[inline(always)]
    fn default() -> SubscribeChgDis {
        SubscribeChgDis(0)
    }
}
#[doc = "Description cluster: Subscribe configuration for task CHG\\[n\\].EN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeChgEn(pub u32);
impl SubscribeChgEn {
    #[doc = "DPPI channel that task CHG\\[n\\].EN will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task CHG\\[n\\].EN will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeChgEnEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeChgEnEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeChgEnEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeChgEn {
    #[inline(always)]
    fn default() -> SubscribeChgEn {
        SubscribeChgEn(0)
    }
}
