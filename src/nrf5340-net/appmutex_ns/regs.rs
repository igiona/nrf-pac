#[doc = "Description collection: Mutex register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mutex(pub u32);
impl Mutex {
    #[doc = "Mutex register n"]
    #[inline(always)]
    pub const fn mutex(&self) -> super::vals::Mutex {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mutex::from_bits(val as u8)
    }
    #[doc = "Mutex register n"]
    #[inline(always)]
    pub fn set_mutex(&mut self, val: super::vals::Mutex) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Mutex {
    #[inline(always)]
    fn default() -> Mutex {
        Mutex(0)
    }
}
