#[doc = "Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vreqh(pub u32);
impl Vreqh {
    #[doc = "Request high voltage"]
    #[inline(always)]
    pub const fn vreqh(&self) -> super::vals::Vreqh {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Vreqh::from_bits(val as u8)
    }
    #[doc = "Request high voltage"]
    #[inline(always)]
    pub fn set_vreqh(&mut self, val: super::vals::Vreqh) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Vreqh {
    #[inline(always)]
    fn default() -> Vreqh {
        Vreqh(0)
    }
}
#[doc = "High voltage on RADIO is ready"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vreqhready(pub u32);
impl Vreqhready {
    #[doc = "RADIO is ready to operate on high voltage"]
    #[inline(always)]
    pub const fn ready(&self) -> super::vals::Ready {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ready::from_bits(val as u8)
    }
    #[doc = "RADIO is ready to operate on high voltage"]
    #[inline(always)]
    pub fn set_ready(&mut self, val: super::vals::Ready) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Vreqhready {
    #[inline(always)]
    fn default() -> Vreqhready {
        Vreqhready(0)
    }
}
