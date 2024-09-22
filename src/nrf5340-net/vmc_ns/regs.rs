#[doc = "Description cluster: RAM\\[n\\] power control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Power(pub u32);
impl Power {
    #[doc = "Keep RAM section S0 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s0power(&self) -> super::vals::PowerS0power {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PowerS0power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S0 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s0power(&mut self, val: super::vals::PowerS0power) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Keep RAM section S1 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s1power(&self) -> super::vals::PowerS1power {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PowerS1power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S1 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s1power(&mut self, val: super::vals::PowerS1power) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Keep RAM section S2 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s2power(&self) -> super::vals::PowerS2power {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PowerS2power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S2 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s2power(&mut self, val: super::vals::PowerS2power) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Keep RAM section S3 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s3power(&self) -> super::vals::PowerS3power {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PowerS3power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S3 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s3power(&mut self, val: super::vals::PowerS3power) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Keep retention on RAM section S0 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s0retention(&self) -> super::vals::PowerS0retention {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::PowerS0retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S0 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s0retention(&mut self, val: super::vals::PowerS0retention) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Keep retention on RAM section S1 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s1retention(&self) -> super::vals::PowerS1retention {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PowerS1retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S1 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s1retention(&mut self, val: super::vals::PowerS1retention) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Keep retention on RAM section S2 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s2retention(&self) -> super::vals::PowerS2retention {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PowerS2retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S2 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s2retention(&mut self, val: super::vals::PowerS2retention) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Keep retention on RAM section S3 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s3retention(&self) -> super::vals::PowerS3retention {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PowerS3retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S3 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s3retention(&mut self, val: super::vals::PowerS3retention) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Power {
    #[inline(always)]
    fn default() -> Power {
        Power(0)
    }
}
#[doc = "Description cluster: RAM\\[n\\] power control clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Powerclr(pub u32);
impl Powerclr {
    #[doc = "Keep RAM section S0 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s0power(&self) -> super::vals::PowerclrS0power {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PowerclrS0power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S0 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s0power(&mut self, val: super::vals::PowerclrS0power) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Keep RAM section S1 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s1power(&self) -> super::vals::PowerclrS1power {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PowerclrS1power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S1 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s1power(&mut self, val: super::vals::PowerclrS1power) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Keep RAM section S2 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s2power(&self) -> super::vals::PowerclrS2power {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PowerclrS2power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S2 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s2power(&mut self, val: super::vals::PowerclrS2power) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Keep RAM section S3 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s3power(&self) -> super::vals::PowerclrS3power {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PowerclrS3power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S3 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s3power(&mut self, val: super::vals::PowerclrS3power) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Keep retention on RAM section S0 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s0retention(&self) -> super::vals::PowerclrS0retention {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::PowerclrS0retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S0 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s0retention(&mut self, val: super::vals::PowerclrS0retention) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Keep retention on RAM section S1 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s1retention(&self) -> super::vals::PowerclrS1retention {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PowerclrS1retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S1 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s1retention(&mut self, val: super::vals::PowerclrS1retention) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Keep retention on RAM section S2 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s2retention(&self) -> super::vals::PowerclrS2retention {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PowerclrS2retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S2 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s2retention(&mut self, val: super::vals::PowerclrS2retention) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Keep retention on RAM section S3 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s3retention(&self) -> super::vals::PowerclrS3retention {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PowerclrS3retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S3 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s3retention(&mut self, val: super::vals::PowerclrS3retention) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Powerclr {
    #[inline(always)]
    fn default() -> Powerclr {
        Powerclr(0)
    }
}
#[doc = "Description cluster: RAM\\[n\\] power control set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Powerset(pub u32);
impl Powerset {
    #[doc = "Keep RAM section S0 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s0power(&self) -> super::vals::PowersetS0power {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PowersetS0power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S0 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s0power(&mut self, val: super::vals::PowersetS0power) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Keep RAM section S1 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s1power(&self) -> super::vals::PowersetS1power {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PowersetS1power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S1 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s1power(&mut self, val: super::vals::PowersetS1power) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Keep RAM section S2 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s2power(&self) -> super::vals::PowersetS2power {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PowersetS2power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S2 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s2power(&mut self, val: super::vals::PowersetS2power) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Keep RAM section S3 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s3power(&self) -> super::vals::PowersetS3power {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PowersetS3power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S3 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s3power(&mut self, val: super::vals::PowersetS3power) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Keep retention on RAM section S0 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s0retention(&self) -> super::vals::PowersetS0retention {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::PowersetS0retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S0 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s0retention(&mut self, val: super::vals::PowersetS0retention) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Keep retention on RAM section S1 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s1retention(&self) -> super::vals::PowersetS1retention {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PowersetS1retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S1 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s1retention(&mut self, val: super::vals::PowersetS1retention) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Keep retention on RAM section S2 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s2retention(&self) -> super::vals::PowersetS2retention {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PowersetS2retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S2 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s2retention(&mut self, val: super::vals::PowersetS2retention) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Keep retention on RAM section S3 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s3retention(&self) -> super::vals::PowersetS3retention {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PowersetS3retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S3 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s3retention(&mut self, val: super::vals::PowersetS3retention) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Powerset {
    #[inline(always)]
    fn default() -> Powerset {
        Powerset(0)
    }
}
