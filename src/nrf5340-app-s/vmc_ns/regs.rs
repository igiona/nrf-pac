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
    #[doc = "Keep RAM section S4 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s4power(&self) -> super::vals::PowerS4power {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PowerS4power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S4 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s4power(&mut self, val: super::vals::PowerS4power) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Keep RAM section S5 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s5power(&self) -> super::vals::PowerS5power {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PowerS5power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S5 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s5power(&mut self, val: super::vals::PowerS5power) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Keep RAM section S6 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s6power(&self) -> super::vals::PowerS6power {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PowerS6power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S6 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s6power(&mut self, val: super::vals::PowerS6power) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Keep RAM section S7 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s7power(&self) -> super::vals::PowerS7power {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PowerS7power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S7 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s7power(&mut self, val: super::vals::PowerS7power) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Keep RAM section S8 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s8power(&self) -> super::vals::PowerS8power {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PowerS8power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S8 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s8power(&mut self, val: super::vals::PowerS8power) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Keep RAM section S9 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s9power(&self) -> super::vals::PowerS9power {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PowerS9power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S9 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s9power(&mut self, val: super::vals::PowerS9power) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Keep RAM section S10 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s10power(&self) -> super::vals::PowerS10power {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PowerS10power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S10 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s10power(&mut self, val: super::vals::PowerS10power) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Keep RAM section S11 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s11power(&self) -> super::vals::PowerS11power {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PowerS11power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S11 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s11power(&mut self, val: super::vals::PowerS11power) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Keep RAM section S12 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s12power(&self) -> super::vals::PowerS12power {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PowerS12power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S12 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s12power(&mut self, val: super::vals::PowerS12power) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Keep RAM section S13 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s13power(&self) -> super::vals::PowerS13power {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::PowerS13power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S13 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s13power(&mut self, val: super::vals::PowerS13power) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Keep RAM section S14 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s14power(&self) -> super::vals::PowerS14power {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PowerS14power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S14 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s14power(&mut self, val: super::vals::PowerS14power) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Keep RAM section S15 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s15power(&self) -> super::vals::PowerS15power {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PowerS15power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S15 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s15power(&mut self, val: super::vals::PowerS15power) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
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
    #[doc = "Keep retention on RAM section S4 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s4retention(&self) -> super::vals::PowerS4retention {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PowerS4retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S4 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s4retention(&mut self, val: super::vals::PowerS4retention) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Keep retention on RAM section S5 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s5retention(&self) -> super::vals::PowerS5retention {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::PowerS5retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S5 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s5retention(&mut self, val: super::vals::PowerS5retention) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Keep retention on RAM section S6 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s6retention(&self) -> super::vals::PowerS6retention {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::PowerS6retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S6 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s6retention(&mut self, val: super::vals::PowerS6retention) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Keep retention on RAM section S7 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s7retention(&self) -> super::vals::PowerS7retention {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::PowerS7retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S7 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s7retention(&mut self, val: super::vals::PowerS7retention) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Keep retention on RAM section S8 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s8retention(&self) -> super::vals::PowerS8retention {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::PowerS8retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S8 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s8retention(&mut self, val: super::vals::PowerS8retention) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Keep retention on RAM section S9 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s9retention(&self) -> super::vals::PowerS9retention {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::PowerS9retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S9 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s9retention(&mut self, val: super::vals::PowerS9retention) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Keep retention on RAM section S10 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s10retention(&self) -> super::vals::PowerS10retention {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::PowerS10retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S10 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s10retention(&mut self, val: super::vals::PowerS10retention) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Keep retention on RAM section S11 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s11retention(&self) -> super::vals::PowerS11retention {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::PowerS11retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S11 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s11retention(&mut self, val: super::vals::PowerS11retention) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Keep retention on RAM section S12 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s12retention(&self) -> super::vals::PowerS12retention {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::PowerS12retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S12 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s12retention(&mut self, val: super::vals::PowerS12retention) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Keep retention on RAM section S13 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s13retention(&self) -> super::vals::PowerS13retention {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::PowerS13retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S13 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s13retention(&mut self, val: super::vals::PowerS13retention) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Keep retention on RAM section S14 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s14retention(&self) -> super::vals::PowerS14retention {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::PowerS14retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S14 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s14retention(&mut self, val: super::vals::PowerS14retention) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Keep retention on RAM section S15 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s15retention(&self) -> super::vals::PowerS15retention {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PowerS15retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S15 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s15retention(&mut self, val: super::vals::PowerS15retention) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
    #[doc = "Keep RAM section S4 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s4power(&self) -> super::vals::PowerclrS4power {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PowerclrS4power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S4 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s4power(&mut self, val: super::vals::PowerclrS4power) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Keep RAM section S5 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s5power(&self) -> super::vals::PowerclrS5power {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PowerclrS5power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S5 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s5power(&mut self, val: super::vals::PowerclrS5power) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Keep RAM section S6 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s6power(&self) -> super::vals::PowerclrS6power {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PowerclrS6power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S6 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s6power(&mut self, val: super::vals::PowerclrS6power) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Keep RAM section S7 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s7power(&self) -> super::vals::PowerclrS7power {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PowerclrS7power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S7 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s7power(&mut self, val: super::vals::PowerclrS7power) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Keep RAM section S8 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s8power(&self) -> super::vals::PowerclrS8power {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PowerclrS8power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S8 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s8power(&mut self, val: super::vals::PowerclrS8power) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Keep RAM section S9 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s9power(&self) -> super::vals::PowerclrS9power {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PowerclrS9power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S9 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s9power(&mut self, val: super::vals::PowerclrS9power) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Keep RAM section S10 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s10power(&self) -> super::vals::PowerclrS10power {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PowerclrS10power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S10 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s10power(&mut self, val: super::vals::PowerclrS10power) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Keep RAM section S11 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s11power(&self) -> super::vals::PowerclrS11power {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PowerclrS11power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S11 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s11power(&mut self, val: super::vals::PowerclrS11power) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Keep RAM section S12 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s12power(&self) -> super::vals::PowerclrS12power {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PowerclrS12power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S12 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s12power(&mut self, val: super::vals::PowerclrS12power) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Keep RAM section S13 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s13power(&self) -> super::vals::PowerclrS13power {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::PowerclrS13power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S13 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s13power(&mut self, val: super::vals::PowerclrS13power) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Keep RAM section S14 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s14power(&self) -> super::vals::PowerclrS14power {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PowerclrS14power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S14 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s14power(&mut self, val: super::vals::PowerclrS14power) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Keep RAM section S15 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s15power(&self) -> super::vals::PowerclrS15power {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PowerclrS15power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S15 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s15power(&mut self, val: super::vals::PowerclrS15power) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
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
    #[doc = "Keep retention on RAM section S4 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s4retention(&self) -> super::vals::PowerclrS4retention {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PowerclrS4retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S4 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s4retention(&mut self, val: super::vals::PowerclrS4retention) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Keep retention on RAM section S5 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s5retention(&self) -> super::vals::PowerclrS5retention {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::PowerclrS5retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S5 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s5retention(&mut self, val: super::vals::PowerclrS5retention) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Keep retention on RAM section S6 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s6retention(&self) -> super::vals::PowerclrS6retention {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::PowerclrS6retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S6 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s6retention(&mut self, val: super::vals::PowerclrS6retention) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Keep retention on RAM section S7 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s7retention(&self) -> super::vals::PowerclrS7retention {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::PowerclrS7retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S7 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s7retention(&mut self, val: super::vals::PowerclrS7retention) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Keep retention on RAM section S8 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s8retention(&self) -> super::vals::PowerclrS8retention {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::PowerclrS8retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S8 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s8retention(&mut self, val: super::vals::PowerclrS8retention) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Keep retention on RAM section S9 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s9retention(&self) -> super::vals::PowerclrS9retention {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::PowerclrS9retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S9 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s9retention(&mut self, val: super::vals::PowerclrS9retention) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Keep retention on RAM section S10 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s10retention(&self) -> super::vals::PowerclrS10retention {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::PowerclrS10retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S10 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s10retention(&mut self, val: super::vals::PowerclrS10retention) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Keep retention on RAM section S11 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s11retention(&self) -> super::vals::PowerclrS11retention {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::PowerclrS11retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S11 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s11retention(&mut self, val: super::vals::PowerclrS11retention) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Keep retention on RAM section S12 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s12retention(&self) -> super::vals::PowerclrS12retention {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::PowerclrS12retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S12 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s12retention(&mut self, val: super::vals::PowerclrS12retention) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Keep retention on RAM section S13 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s13retention(&self) -> super::vals::PowerclrS13retention {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::PowerclrS13retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S13 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s13retention(&mut self, val: super::vals::PowerclrS13retention) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Keep retention on RAM section S14 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s14retention(&self) -> super::vals::PowerclrS14retention {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::PowerclrS14retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S14 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s14retention(&mut self, val: super::vals::PowerclrS14retention) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Keep retention on RAM section S15 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s15retention(&self) -> super::vals::PowerclrS15retention {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PowerclrS15retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S15 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s15retention(&mut self, val: super::vals::PowerclrS15retention) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
    #[doc = "Keep RAM section S4 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s4power(&self) -> super::vals::PowersetS4power {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PowersetS4power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S4 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s4power(&mut self, val: super::vals::PowersetS4power) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Keep RAM section S5 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s5power(&self) -> super::vals::PowersetS5power {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PowersetS5power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S5 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s5power(&mut self, val: super::vals::PowersetS5power) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Keep RAM section S6 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s6power(&self) -> super::vals::PowersetS6power {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PowersetS6power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S6 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s6power(&mut self, val: super::vals::PowersetS6power) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Keep RAM section S7 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s7power(&self) -> super::vals::PowersetS7power {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PowersetS7power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S7 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s7power(&mut self, val: super::vals::PowersetS7power) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Keep RAM section S8 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s8power(&self) -> super::vals::PowersetS8power {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PowersetS8power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S8 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s8power(&mut self, val: super::vals::PowersetS8power) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Keep RAM section S9 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s9power(&self) -> super::vals::PowersetS9power {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PowersetS9power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S9 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s9power(&mut self, val: super::vals::PowersetS9power) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Keep RAM section S10 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s10power(&self) -> super::vals::PowersetS10power {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PowersetS10power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S10 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s10power(&mut self, val: super::vals::PowersetS10power) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Keep RAM section S11 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s11power(&self) -> super::vals::PowersetS11power {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PowersetS11power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S11 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s11power(&mut self, val: super::vals::PowersetS11power) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Keep RAM section S12 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s12power(&self) -> super::vals::PowersetS12power {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PowersetS12power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S12 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s12power(&mut self, val: super::vals::PowersetS12power) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Keep RAM section S13 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s13power(&self) -> super::vals::PowersetS13power {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::PowersetS13power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S13 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s13power(&mut self, val: super::vals::PowersetS13power) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Keep RAM section S14 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s14power(&self) -> super::vals::PowersetS14power {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PowersetS14power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S14 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s14power(&mut self, val: super::vals::PowersetS14power) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Keep RAM section S15 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub const fn s15power(&self) -> super::vals::PowersetS15power {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PowersetS15power::from_bits(val as u8)
    }
    #[doc = "Keep RAM section S15 of RAM\\[n\\] on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s15power(&mut self, val: super::vals::PowersetS15power) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
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
    #[doc = "Keep retention on RAM section S4 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s4retention(&self) -> super::vals::PowersetS4retention {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PowersetS4retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S4 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s4retention(&mut self, val: super::vals::PowersetS4retention) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Keep retention on RAM section S5 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s5retention(&self) -> super::vals::PowersetS5retention {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::PowersetS5retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S5 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s5retention(&mut self, val: super::vals::PowersetS5retention) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Keep retention on RAM section S6 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s6retention(&self) -> super::vals::PowersetS6retention {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::PowersetS6retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S6 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s6retention(&mut self, val: super::vals::PowersetS6retention) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Keep retention on RAM section S7 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s7retention(&self) -> super::vals::PowersetS7retention {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::PowersetS7retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S7 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s7retention(&mut self, val: super::vals::PowersetS7retention) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Keep retention on RAM section S8 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s8retention(&self) -> super::vals::PowersetS8retention {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::PowersetS8retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S8 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s8retention(&mut self, val: super::vals::PowersetS8retention) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Keep retention on RAM section S9 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s9retention(&self) -> super::vals::PowersetS9retention {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::PowersetS9retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S9 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s9retention(&mut self, val: super::vals::PowersetS9retention) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Keep retention on RAM section S10 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s10retention(&self) -> super::vals::PowersetS10retention {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::PowersetS10retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S10 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s10retention(&mut self, val: super::vals::PowersetS10retention) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Keep retention on RAM section S11 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s11retention(&self) -> super::vals::PowersetS11retention {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::PowersetS11retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S11 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s11retention(&mut self, val: super::vals::PowersetS11retention) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Keep retention on RAM section S12 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s12retention(&self) -> super::vals::PowersetS12retention {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::PowersetS12retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S12 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s12retention(&mut self, val: super::vals::PowersetS12retention) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Keep retention on RAM section S13 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s13retention(&self) -> super::vals::PowersetS13retention {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::PowersetS13retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S13 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s13retention(&mut self, val: super::vals::PowersetS13retention) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Keep retention on RAM section S14 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s14retention(&self) -> super::vals::PowersetS14retention {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::PowersetS14retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S14 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s14retention(&mut self, val: super::vals::PowersetS14retention) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Keep retention on RAM section S15 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub const fn s15retention(&self) -> super::vals::PowersetS15retention {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PowersetS15retention::from_bits(val as u8)
    }
    #[doc = "Keep retention on RAM section S15 of RAM\\[n\\] when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s15retention(&mut self, val: super::vals::PowersetS15retention) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Powerset {
    #[inline(always)]
    fn default() -> Powerset {
        Powerset(0)
    }
}
