#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bypass(pub u32);
impl Bypass {
    #[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub const fn bypass(&self) -> super::vals::Bypass {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Bypass::from_bits(val as u8)
    }
    #[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn set_bypass(&mut self, val: super::vals::Bypass) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Bypass {
    #[inline(always)]
    fn default() -> Bypass {
        Bypass(0)
    }
}
#[doc = "Control usage of internal load capacitors"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intcap(pub u32);
impl Intcap {
    #[doc = "Control usage of internal load capacitors"]
    #[inline(always)]
    pub const fn intcap(&self) -> super::vals::Intcap {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Intcap::from_bits(val as u8)
    }
    #[doc = "Control usage of internal load capacitors"]
    #[inline(always)]
    pub fn set_intcap(&mut self, val: super::vals::Intcap) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Intcap {
    #[inline(always)]
    fn default() -> Intcap {
        Intcap(0)
    }
}
#[doc = "Programmable capacitance of XC1 and XC2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xosc32mcaps(pub u32);
impl Xosc32mcaps {
    #[doc = "Value representing capacitance, calculated using provided equation"]
    #[inline(always)]
    pub const fn capvalue(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Value representing capacitance, calculated using provided equation"]
    #[inline(always)]
    pub fn set_capvalue(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Enable on-chip capacitors on XC1 and XC2"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enable on-chip capacitors on XC1 and XC2"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Xosc32mcaps {
    #[inline(always)]
    fn default() -> Xosc32mcaps {
        Xosc32mcaps(0)
    }
}
