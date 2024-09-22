#[doc = "Code memory page size in bytes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Codepagesize(pub u32);
impl Codepagesize {
    #[doc = "Code memory page size in bytes"]
    #[inline(always)]
    pub const fn codepagesize(&self) -> super::vals::Codepagesize {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Codepagesize::from_bits(val as u32)
    }
    #[doc = "Code memory page size in bytes"]
    #[inline(always)]
    pub fn set_codepagesize(&mut self, val: super::vals::Codepagesize) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Codepagesize {
    #[inline(always)]
    fn default() -> Codepagesize {
        Codepagesize(0)
    }
}
#[doc = "Code memory size"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Codesize(pub u32);
impl Codesize {
    #[doc = "Code memory size in number of pages"]
    #[inline(always)]
    pub const fn codesize(&self) -> super::vals::Codesize {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Codesize::from_bits(val as u32)
    }
    #[doc = "Code memory size in number of pages"]
    #[inline(always)]
    pub fn set_codesize(&mut self, val: super::vals::Codesize) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Codesize {
    #[inline(always)]
    fn default() -> Codesize {
        Codesize(0)
    }
}
#[doc = "Configuration identifier"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Configid(pub u32);
impl Configid {
    #[doc = "Identification number for the HW"]
    #[inline(always)]
    pub const fn hwid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Identification number for the HW"]
    #[inline(always)]
    pub fn set_hwid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Configid {
    #[inline(always)]
    fn default() -> Configid {
        Configid(0)
    }
}
#[doc = "Device address type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deviceaddrtype(pub u32);
impl Deviceaddrtype {
    #[doc = "Device address type"]
    #[inline(always)]
    pub const fn deviceaddrtype(&self) -> super::vals::Deviceaddrtype {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Deviceaddrtype::from_bits(val as u8)
    }
    #[doc = "Device address type"]
    #[inline(always)]
    pub fn set_deviceaddrtype(&mut self, val: super::vals::Deviceaddrtype) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Deviceaddrtype {
    #[inline(always)]
    fn default() -> Deviceaddrtype {
        Deviceaddrtype(0)
    }
}
#[doc = "Device type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devicetype(pub u32);
impl Devicetype {
    #[doc = "Device type"]
    #[inline(always)]
    pub const fn devicetype(&self) -> super::vals::Devicetype {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Devicetype::from_bits(val as u32)
    }
    #[doc = "Device type"]
    #[inline(always)]
    pub fn set_devicetype(&mut self, val: super::vals::Devicetype) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Devicetype {
    #[inline(always)]
    fn default() -> Devicetype {
        Devicetype(0)
    }
}
#[doc = "Flash variant"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash(pub u32);
impl Flash {
    #[doc = "Flash variant"]
    #[inline(always)]
    pub const fn flash(&self) -> super::vals::Flash {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Flash::from_bits(val as u32)
    }
    #[doc = "Flash variant"]
    #[inline(always)]
    pub fn set_flash(&mut self, val: super::vals::Flash) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Flash {
    #[inline(always)]
    fn default() -> Flash {
        Flash(0)
    }
}
#[doc = "Package option"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Package(pub u32);
impl Package {
    #[doc = "Package option"]
    #[inline(always)]
    pub const fn package(&self) -> super::vals::Package {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Package::from_bits(val as u32)
    }
    #[doc = "Package option"]
    #[inline(always)]
    pub fn set_package(&mut self, val: super::vals::Package) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Package {
    #[inline(always)]
    fn default() -> Package {
        Package(0)
    }
}
#[doc = "Part code"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Part(pub u32);
impl Part {
    #[doc = "Part code"]
    #[inline(always)]
    pub const fn part(&self) -> super::vals::Part {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Part::from_bits(val as u32)
    }
    #[doc = "Part code"]
    #[inline(always)]
    pub fn set_part(&mut self, val: super::vals::Part) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Part {
    #[inline(always)]
    fn default() -> Part {
        Part(0)
    }
}
#[doc = "RAM variant"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram(pub u32);
impl Ram {
    #[doc = "RAM variant"]
    #[inline(always)]
    pub const fn ram(&self) -> super::vals::Ram {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Ram::from_bits(val as u32)
    }
    #[doc = "RAM variant"]
    #[inline(always)]
    pub fn set_ram(&mut self, val: super::vals::Ram) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ram {
    #[inline(always)]
    fn default() -> Ram {
        Ram(0)
    }
}
#[doc = "Part Variant, Hardware version and Production configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Variant(pub u32);
impl Variant {
    #[doc = "Part Variant, Hardware version and Production configuration, encoded as ASCII"]
    #[inline(always)]
    pub const fn variant(&self) -> super::vals::Variant {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Variant::from_bits(val as u32)
    }
    #[doc = "Part Variant, Hardware version and Production configuration, encoded as ASCII"]
    #[inline(always)]
    pub fn set_variant(&mut self, val: super::vals::Variant) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Variant {
    #[inline(always)]
    fn default() -> Variant {
        Variant(0)
    }
}
