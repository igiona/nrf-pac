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
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tagheader0(pub u32);
impl Tagheader0 {
    #[doc = "Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
    #[inline(always)]
    pub const fn mfgid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
    #[inline(always)]
    pub fn set_mfgid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Unique identifier byte 1"]
    #[inline(always)]
    pub const fn ud1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 1"]
    #[inline(always)]
    pub fn set_ud1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Unique identifier byte 2"]
    #[inline(always)]
    pub const fn ud2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 2"]
    #[inline(always)]
    pub fn set_ud2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Unique identifier byte 3"]
    #[inline(always)]
    pub const fn ud3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 3"]
    #[inline(always)]
    pub fn set_ud3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Tagheader0 {
    #[inline(always)]
    fn default() -> Tagheader0 {
        Tagheader0(0)
    }
}
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tagheader1(pub u32);
impl Tagheader1 {
    #[doc = "Unique identifier byte 4"]
    #[inline(always)]
    pub const fn ud4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 4"]
    #[inline(always)]
    pub fn set_ud4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Unique identifier byte 5"]
    #[inline(always)]
    pub const fn ud5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 5"]
    #[inline(always)]
    pub fn set_ud5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Unique identifier byte 6"]
    #[inline(always)]
    pub const fn ud6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 6"]
    #[inline(always)]
    pub fn set_ud6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Unique identifier byte 7"]
    #[inline(always)]
    pub const fn ud7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 7"]
    #[inline(always)]
    pub fn set_ud7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Tagheader1 {
    #[inline(always)]
    fn default() -> Tagheader1 {
        Tagheader1(0)
    }
}
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tagheader2(pub u32);
impl Tagheader2 {
    #[doc = "Unique identifier byte 8"]
    #[inline(always)]
    pub const fn ud8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 8"]
    #[inline(always)]
    pub fn set_ud8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Unique identifier byte 9"]
    #[inline(always)]
    pub const fn ud9(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 9"]
    #[inline(always)]
    pub fn set_ud9(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Unique identifier byte 10"]
    #[inline(always)]
    pub const fn ud10(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 10"]
    #[inline(always)]
    pub fn set_ud10(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Unique identifier byte 11"]
    #[inline(always)]
    pub const fn ud11(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 11"]
    #[inline(always)]
    pub fn set_ud11(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Tagheader2 {
    #[inline(always)]
    fn default() -> Tagheader2 {
        Tagheader2(0)
    }
}
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tagheader3(pub u32);
impl Tagheader3 {
    #[doc = "Unique identifier byte 12"]
    #[inline(always)]
    pub const fn ud12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 12"]
    #[inline(always)]
    pub fn set_ud12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Unique identifier byte 13"]
    #[inline(always)]
    pub const fn ud13(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 13"]
    #[inline(always)]
    pub fn set_ud13(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Unique identifier byte 14"]
    #[inline(always)]
    pub const fn ud14(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 14"]
    #[inline(always)]
    pub fn set_ud14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Unique identifier byte 15"]
    #[inline(always)]
    pub const fn ud15(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 15"]
    #[inline(always)]
    pub fn set_ud15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Tagheader3 {
    #[inline(always)]
    fn default() -> Tagheader3 {
        Tagheader3(0)
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
#[doc = "XOSC32M capacitor selection trim values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xosc32mtrim(pub u32);
impl Xosc32mtrim {
    #[doc = "Slope trim factor on twos complement form"]
    #[inline(always)]
    pub const fn slope(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Slope trim factor on twos complement form"]
    #[inline(always)]
    pub fn set_slope(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Offset trim factor on integer form"]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Offset trim factor on integer form"]
    #[inline(always)]
    pub fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
}
impl Default for Xosc32mtrim {
    #[inline(always)]
    fn default() -> Xosc32mtrim {
        Xosc32mtrim(0)
    }
}
