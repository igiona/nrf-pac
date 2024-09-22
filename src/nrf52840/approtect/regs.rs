#[doc = "Software disable APPROTECT mechanism"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Disable(pub u32);
impl Disable {
    #[doc = "Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::Disable {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Disable::from_bits(val as u8)
    }
    #[doc = "Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn set_disable(&mut self, val: super::vals::Disable) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Disable {
    #[inline(always)]
    fn default() -> Disable {
        Disable(0)
    }
}
#[doc = "Software force enable APPROTECT mechanism until next reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Forceprotect(pub u32);
impl Forceprotect {
    #[doc = "Write 0x0 to force enable APPROTECT mechanism"]
    #[inline(always)]
    pub const fn forceprotect(&self) -> super::vals::Forceprotect {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Forceprotect::from_bits(val as u8)
    }
    #[doc = "Write 0x0 to force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn set_forceprotect(&mut self, val: super::vals::Forceprotect) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Forceprotect {
    #[inline(always)]
    fn default() -> Forceprotect {
        Forceprotect(0)
    }
}
