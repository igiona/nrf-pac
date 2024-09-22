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
#[doc = "HFXO startup counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfxocnt(pub u32);
impl Hfxocnt {
    #[doc = "HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
    #[inline(always)]
    pub const fn hfxocnt(&self) -> super::vals::Hfxocnt {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Hfxocnt::from_bits(val as u8)
    }
    #[doc = "HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
    #[inline(always)]
    pub fn set_hfxocnt(&mut self, val: super::vals::Hfxocnt) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Hfxocnt {
    #[inline(always)]
    fn default() -> Hfxocnt {
        Hfxocnt(0)
    }
}
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nfcpins(pub u32);
impl Nfcpins {
    #[doc = "Setting of pins dedicated to NFC functionality"]
    #[inline(always)]
    pub const fn protect(&self) -> super::vals::Protect {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Protect::from_bits(val as u8)
    }
    #[doc = "Setting of pins dedicated to NFC functionality"]
    #[inline(always)]
    pub fn set_protect(&mut self, val: super::vals::Protect) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Nfcpins {
    #[inline(always)]
    fn default() -> Nfcpins {
        Nfcpins(0)
    }
}
#[doc = "Description collection: One time programmable memory"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otp(pub u32);
impl Otp {
    #[doc = "Lower half word"]
    #[inline(always)]
    pub const fn lower(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Lower half word"]
    #[inline(always)]
    pub fn set_lower(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Upper half word"]
    #[inline(always)]
    pub const fn upper(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Upper half word"]
    #[inline(always)]
    pub fn set_upper(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Otp {
    #[inline(always)]
    fn default() -> Otp {
        Otp(0)
    }
}
#[doc = "Description cluster: Define permissions for the key slot. Bits 0-15 and 16-31 can only be written when equal to 0xFFFF."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perm(pub u32);
impl Perm {
    #[doc = "Write permission for key slot"]
    #[inline(always)]
    pub const fn write(&self) -> super::vals::Write {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Write::from_bits(val as u8)
    }
    #[doc = "Write permission for key slot"]
    #[inline(always)]
    pub fn set_write(&mut self, val: super::vals::Write) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Read permission for key slot"]
    #[inline(always)]
    pub const fn read(&self) -> super::vals::Read {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Read::from_bits(val as u8)
    }
    #[doc = "Read permission for key slot"]
    #[inline(always)]
    pub fn set_read(&mut self, val: super::vals::Read) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Push permission for key slot"]
    #[inline(always)]
    pub const fn push(&self) -> super::vals::Push {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Push::from_bits(val as u8)
    }
    #[doc = "Push permission for key slot"]
    #[inline(always)]
    pub fn set_push(&mut self, val: super::vals::Push) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Revocation state for the key slot"]
    #[inline(always)]
    pub const fn state(&self) -> super::vals::State {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::State::from_bits(val as u8)
    }
    #[doc = "Revocation state for the key slot"]
    #[inline(always)]
    pub fn set_state(&mut self, val: super::vals::State) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Perm {
    #[inline(always)]
    fn default() -> Perm {
        Perm(0)
    }
}
#[doc = "Secure access port protection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Secureapprotect(pub u32);
impl Secureapprotect {
    #[doc = "Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses."]
    #[inline(always)]
    pub const fn pall(&self) -> super::vals::SecureapprotectPall {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::SecureapprotectPall::from_bits(val as u32)
    }
    #[doc = "Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses."]
    #[inline(always)]
    pub fn set_pall(&mut self, val: super::vals::SecureapprotectPall) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Secureapprotect {
    #[inline(always)]
    fn default() -> Secureapprotect {
        Secureapprotect(0)
    }
}
#[doc = "SW-DP Target instance"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tinstance(pub u32);
impl Tinstance {
    #[doc = "TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
    #[inline(always)]
    pub const fn tinstance(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
    #[inline(always)]
    pub fn set_tinstance(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Tinstance {
    #[inline(always)]
    fn default() -> Tinstance {
        Tinstance(0)
    }
}
#[doc = "Output voltage from the high voltage (VREGH) regulator stage. The maximum output voltage from this stage is given as VDDH - VREGHDROP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vreghvout(pub u32);
impl Vreghvout {
    #[doc = "VREGH regulator output voltage."]
    #[inline(always)]
    pub const fn vreghvout(&self) -> super::vals::Vreghvout {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Vreghvout::from_bits(val as u8)
    }
    #[doc = "VREGH regulator output voltage."]
    #[inline(always)]
    pub fn set_vreghvout(&mut self, val: super::vals::Vreghvout) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Vreghvout {
    #[inline(always)]
    fn default() -> Vreghvout {
        Vreghvout(0)
    }
}
