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
    #[doc = "QIxx - 7x7 73-pin aQFN"]
    pub const QI: Self = Self(0x2004);
    #[doc = "QDxx - 5x5 40-pin QFN"]
    pub const QD: Self = Self(0x2007);
    #[doc = "CJxx - 3.175 x 3.175 WLCSP"]
    pub const CJ: Self = Self(0x2008);
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
    #[doc = "nRF52820"]
    pub const N52820: Self = Self(0x0005_2820);
    #[doc = "nRF52833"]
    pub const N52833: Self = Self(0x0005_2833);
    #[doc = "nRF52840"]
    pub const N52840: Self = Self(0x0005_2840);
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
pub struct Prodtest(pub u32);
impl Prodtest {
    #[doc = "Production tests done"]
    pub const DONE: Self = Self(0xbb42_319f);
    #[doc = "Production tests not done"]
    pub const NOTDONE: Self = Self(0xffff_ffff);
}
impl Prodtest {
    pub const fn from_bits(val: u32) -> Prodtest {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Prodtest {
    #[inline(always)]
    fn from(val: u32) -> Prodtest {
        Prodtest::from_bits(val)
    }
}
impl From<Prodtest> for u32 {
    #[inline(always)]
    fn from(val: Prodtest) -> u32 {
        Prodtest::to_bits(val)
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
    #[doc = "AAA0"]
    pub const AAA0: Self = Self(0x4141_4130);
    #[doc = "AAA1"]
    pub const AAA1: Self = Self(0x4141_4131);
    #[doc = "AAAA"]
    pub const AAAA: Self = Self(0x4141_4141);
    #[doc = "AAAB"]
    pub const AAAB: Self = Self(0x4141_4142);
    #[doc = "AAB0"]
    pub const AAB0: Self = Self(0x4141_4230);
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
