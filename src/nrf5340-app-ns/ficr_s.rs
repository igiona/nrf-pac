#[doc = "Factory Information Configuration Registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FicrS {
    ptr: *mut u8,
}
unsafe impl Send for FicrS {}
unsafe impl Sync for FicrS {}
impl FicrS {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Device info"]
    #[inline(always)]
    pub const fn info(self) -> Info {
        unsafe { Info::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn trimcnf(self, n: usize) -> Trimcnf {
        assert!(n < 32usize);
        unsafe { Trimcnf::from_ptr(self.ptr.add(0x0300usize + n * 8usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn nfc(self) -> Nfc {
        unsafe { Nfc::from_ptr(self.ptr.add(0x0450usize) as _) }
    }
    #[doc = "NIST800-90B RNG calibration data"]
    #[inline(always)]
    pub const fn trng90b(self) -> Trng90b {
        unsafe { Trng90b::from_ptr(self.ptr.add(0x0c00usize) as _) }
    }
    #[doc = "XOSC32M capacitor selection trim values"]
    #[inline(always)]
    pub const fn xosc32mtrim(self) -> crate::common::Reg<regs::Xosc32mtrim, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c20usize) as _) }
    }
}
#[doc = "Device info"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Info {
    ptr: *mut u8,
}
unsafe impl Send for Info {}
unsafe impl Sync for Info {}
impl Info {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration identifier"]
    #[inline(always)]
    pub const fn configid(self) -> crate::common::Reg<regs::Configid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description collection: Device identifier"]
    #[inline(always)]
    pub const fn deviceid(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "Part code"]
    #[inline(always)]
    pub const fn part(self) -> crate::common::Reg<regs::Part, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Part Variant, Hardware version and Production configuration"]
    #[inline(always)]
    pub const fn variant(self) -> crate::common::Reg<regs::Variant, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Package option"]
    #[inline(always)]
    pub const fn package(self) -> crate::common::Reg<regs::Package, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "RAM variant"]
    #[inline(always)]
    pub const fn ram(self) -> crate::common::Reg<regs::Ram, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Flash variant"]
    #[inline(always)]
    pub const fn flash(self) -> crate::common::Reg<regs::Flash, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Code memory page size in bytes"]
    #[inline(always)]
    pub const fn codepagesize(self) -> crate::common::Reg<regs::Codepagesize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Code memory size"]
    #[inline(always)]
    pub const fn codesize(self) -> crate::common::Reg<regs::Codesize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Device type"]
    #[inline(always)]
    pub const fn devicetype(self) -> crate::common::Reg<regs::Devicetype, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nfc {
    ptr: *mut u8,
}
unsafe impl Send for Nfc {}
unsafe impl Sync for Nfc {}
impl Nfc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader0(self) -> crate::common::Reg<regs::Tagheader0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader1(self) -> crate::common::Reg<regs::Tagheader1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader2(self) -> crate::common::Reg<regs::Tagheader2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader3(self) -> crate::common::Reg<regs::Tagheader3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trimcnf {
    ptr: *mut u8,
}
unsafe impl Send for Trimcnf {}
unsafe impl Sync for Trimcnf {}
impl Trimcnf {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Address of the PAR register which will be written"]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Data"]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "NIST800-90B RNG calibration data"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trng90b {
    ptr: *mut u8,
}
unsafe impl Send for Trng90b {}
unsafe impl Sync for Trng90b {}
impl Trng90b {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Amount of bytes for the required entropy bits"]
    #[inline(always)]
    pub const fn bytes(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Repetition counter cutoff"]
    #[inline(always)]
    pub const fn rccutoff(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Adaptive proportion cutoff"]
    #[inline(always)]
    pub const fn apcutoff(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Amount of bytes for the startup tests"]
    #[inline(always)]
    pub const fn startup(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Sample count for ring oscillator 1"]
    #[inline(always)]
    pub const fn rosc1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Sample count for ring oscillator 2"]
    #[inline(always)]
    pub const fn rosc2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Sample count for ring oscillator 3"]
    #[inline(always)]
    pub const fn rosc3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Sample count for ring oscillator 4"]
    #[inline(always)]
    pub const fn rosc4(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
