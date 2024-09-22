#[doc = "CPU ID of this subsystem"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuid(pub u32);
impl Cpuid {
    #[doc = "CPU ID"]
    #[inline(always)]
    pub const fn cpuid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "CPU ID"]
    #[inline(always)]
    pub fn set_cpuid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cpuid {
    #[inline(always)]
    fn default() -> Cpuid {
        Cpuid(0)
    }
}
