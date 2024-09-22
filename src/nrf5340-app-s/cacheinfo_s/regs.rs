#[doc = "Description collection: Cache information for SET\\[n\\], WAY\\[o\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Way(pub u32);
impl Way {
    #[doc = "Cache tag."]
    #[inline(always)]
    pub const fn tag(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Cache tag."]
    #[inline(always)]
    pub fn set_tag(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
    #[doc = "Valid bit"]
    #[inline(always)]
    pub const fn v(&self) -> super::vals::V {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::V::from_bits(val as u8)
    }
    #[doc = "Valid bit"]
    #[inline(always)]
    pub fn set_v(&mut self, val: super::vals::V) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Most recently used way."]
    #[inline(always)]
    pub const fn mru(&self) -> super::vals::Mru {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mru::from_bits(val as u8)
    }
    #[doc = "Most recently used way."]
    #[inline(always)]
    pub fn set_mru(&mut self, val: super::vals::Mru) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Way {
    #[inline(always)]
    fn default() -> Way {
        Way(0)
    }
}
