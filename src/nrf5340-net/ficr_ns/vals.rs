#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Codepagesize(pub u32);
impl Codepagesize {
    #[doc = "2 kByte"]
    pub const K2048: Self = Self(0x0800);
}
impl Codepagesize {
    pub const fn from_bits(val: u32) -> Codepagesize {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Codepagesize {
    #[inline(always)]
    fn from(val: u32) -> Codepagesize {
        Codepagesize::from_bits(val)
    }
}
impl From<Codepagesize> for u32 {
    #[inline(always)]
    fn from(val: Codepagesize) -> u32 {
        Codepagesize::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Codesize(pub u32);
impl Codesize {
    #[doc = "128 pages"]
    pub const P128: Self = Self(0x80);
}
impl Codesize {
    pub const fn from_bits(val: u32) -> Codesize {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Codesize {
    #[inline(always)]
    fn from(val: u32) -> Codesize {
        Codesize::from_bits(val)
    }
}
impl From<Codesize> for u32 {
    #[inline(always)]
    fn from(val: Codesize) -> u32 {
        Codesize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Deviceaddrtype {
    #[doc = "Public address"]
    PUBLIC = 0x0,
    #[doc = "Random address"]
    RANDOM = 0x01,
}
impl Deviceaddrtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Deviceaddrtype {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Deviceaddrtype {
    #[inline(always)]
    fn from(val: u8) -> Deviceaddrtype {
        Deviceaddrtype::from_bits(val)
    }
}
impl From<Deviceaddrtype> for u8 {
    #[inline(always)]
    fn from(val: Deviceaddrtype) -> u8 {
        Deviceaddrtype::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Devicetype(pub u32);
impl Devicetype {
    #[doc = "Device is an physical DIE"]
    pub const DIE: Self = Self(0x0);
    #[doc = "Device is an FPGA"]
    pub const FPGA: Self = Self(0xffff_ffff);
}
impl Devicetype {
    pub const fn from_bits(val: u32) -> Devicetype {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Devicetype {
    #[inline(always)]
    fn from(val: u32) -> Devicetype {
        Devicetype::from_bits(val)
    }
}
impl From<Devicetype> for u32 {
    #[inline(always)]
    fn from(val: Devicetype) -> u32 {
        Devicetype::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Flash(pub u32);
impl Flash {
    #[doc = "128 kByte FLASH"]
    pub const K128: Self = Self(0x80);
    #[doc = "256 kByte FLASH"]
    pub const K256: Self = Self(0x0100);
    #[doc = "512 kByte FLASH"]
    pub const K512: Self = Self(0x0200);
    #[doc = "1 MByte FLASH"]
    pub const K1024: Self = Self(0x0400);
    #[doc = "2 MByte FLASH"]
    pub const K2048: Self = Self(0x0800);
    #[doc = "Unspecified"]
    pub const UNSPECIFIED: Self = Self(0xffff_ffff);
}
impl Flash {
    pub const fn from_bits(val: u32) -> Flash {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Flash {
    #[inline(always)]
    fn from(val: u32) -> Flash {
        Flash::from_bits(val)
    }
}
impl From<Flash> for u32 {
    #[inline(always)]
    fn from(val: Flash) -> u32 {
        Flash::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Package(pub u32);
impl Package {
    #[doc = "QKxx - 94-pin aQFN"]
    pub const QK: Self = Self(0x2000);
    #[doc = "CLxx - WLCSP"]
    pub const CL: Self = Self(0x2005);
    #[doc = "Unspecified"]
    pub const UNSPECIFIED: Self = Self(0xffff_ffff);
}
impl Package {
    pub const fn from_bits(val: u32) -> Package {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Package {
    #[inline(always)]
    fn from(val: u32) -> Package {
        Package::from_bits(val)
    }
}
impl From<Package> for u32 {
    #[inline(always)]
    fn from(val: Package) -> u32 {
        Package::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Part(pub u32);
impl Part {
    #[doc = "nRF5340"]
    pub const N5340: Self = Self(0x5340);
    #[doc = "Unspecified"]
    pub const UNSPECIFIED: Self = Self(0xffff_ffff);
}
impl Part {
    pub const fn from_bits(val: u32) -> Part {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Part {
    #[inline(always)]
    fn from(val: u32) -> Part {
        Part::from_bits(val)
    }
}
impl From<Part> for u32 {
    #[inline(always)]
    fn from(val: Part) -> u32 {
        Part::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ram(pub u32);
impl Ram {
    #[doc = "16 kByte RAM"]
    pub const K16: Self = Self(0x10);
    #[doc = "32 kByte RAM"]
    pub const K32: Self = Self(0x20);
    #[doc = "64 kByte RAM"]
    pub const K64: Self = Self(0x40);
    #[doc = "128 kByte RAM"]
    pub const K128: Self = Self(0x80);
    #[doc = "256 kByte RAM"]
    pub const K256: Self = Self(0x0100);
    #[doc = "512 kByte RAM"]
    pub const K512: Self = Self(0x0200);
    #[doc = "Unspecified"]
    pub const UNSPECIFIED: Self = Self(0xffff_ffff);
}
impl Ram {
    pub const fn from_bits(val: u32) -> Ram {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Ram {
    #[inline(always)]
    fn from(val: u32) -> Ram {
        Ram::from_bits(val)
    }
}
impl From<Ram> for u32 {
    #[inline(always)]
    fn from(val: Ram) -> u32 {
        Ram::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Variant(pub u32);
impl Variant {
    #[doc = "CLAA"]
    pub const CLAA: Self = Self(0x434c_4141);
    #[doc = "QKAA"]
    pub const QKAA: Self = Self(0x514b_4141);
    #[doc = "Unspecified"]
    pub const UNSPECIFIED: Self = Self(0xffff_ffff);
}
impl Variant {
    pub const fn from_bits(val: u32) -> Variant {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Variant {
    #[inline(always)]
    fn from(val: u32) -> Variant {
        Variant::from_bits(val)
    }
}
impl From<Variant> for u32 {
    #[inline(always)]
    fn from(val: Variant) -> u32 {
        Variant::to_bits(val)
    }
}
