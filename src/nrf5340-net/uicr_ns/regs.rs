#[doc = "Access port protection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Approtect(pub u32);
impl Approtect {
    #[doc = "Blocks debugger read/write access to all CPU registers and memory mapped addresses."]
    #[inline(always)]
    pub const fn pall(&self) -> super::vals::ApprotectPall {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::ApprotectPall::from_bits(val as u32)
    }
    #[doc = "Blocks debugger read/write access to all CPU registers and memory mapped addresses."]
    #[inline(always)]
    pub fn set_pall(&mut self, val: super::vals::ApprotectPall) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Approtect {
    #[inline(always)]
    fn default() -> Approtect {
        Approtect(0)
    }
}
#[doc = "Erase protection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eraseprotect(pub u32);
impl Eraseprotect {
    #[doc = "Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality. Using any value except Unprotected will lead to the protection being enabled."]
    #[inline(always)]
    pub const fn pall(&self) -> super::vals::EraseprotectPall {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::EraseprotectPall::from_bits(val as u32)
    }
    #[doc = "Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality. Using any value except Unprotected will lead to the protection being enabled."]
    #[inline(always)]
    pub fn set_pall(&mut self, val: super::vals::EraseprotectPall) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Eraseprotect {
    #[inline(always)]
    fn default() -> Eraseprotect {
        Eraseprotect(0)
    }
}
