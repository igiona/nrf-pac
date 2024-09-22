#[doc = "Factory information configuration registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ficr {
    ptr: *mut u8,
}
unsafe impl Send for Ficr {}
unsafe impl Sync for Ficr {}
impl Ficr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Code memory page size"]
    #[inline(always)]
    pub const fn codepagesize(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Code memory size"]
    #[inline(always)]
    pub const fn codesize(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Description collection: Device identifier"]
    #[inline(always)]
    pub const fn deviceid(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Encryption root, word n"]
    #[inline(always)]
    pub const fn er(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Identity Root, word n"]
    #[inline(always)]
    pub const fn ir(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize + n * 4usize) as _) }
    }
    #[doc = "Device address type"]
    #[inline(always)]
    pub const fn deviceaddrtype(
        self,
    ) -> crate::common::Reg<regs::Deviceaddrtype, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Description collection: Device address n"]
    #[inline(always)]
    pub const fn deviceaddr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize + n * 4usize) as _) }
    }
    #[doc = "Device info"]
    #[inline(always)]
    pub const fn info(self) -> Info {
        unsafe { Info::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Description collection: Production test signature n"]
    #[inline(always)]
    pub const fn prodtest(self, n: usize) -> crate::common::Reg<regs::Prodtest, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0350usize + n * 4usize) as _) }
    }
    #[doc = "Registers storing factory TEMP module linearization coefficients"]
    #[inline(always)]
    pub const fn temp(self) -> Temp {
        unsafe { Temp::from_ptr(self.ptr.add(0x0404usize) as _) }
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
    #[doc = "Part code"]
    #[inline(always)]
    pub const fn part(self) -> crate::common::Reg<regs::Part, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Build code (hardware version and production configuration)"]
    #[inline(always)]
    pub const fn variant(self) -> crate::common::Reg<regs::Variant, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Package option"]
    #[inline(always)]
    pub const fn package(self) -> crate::common::Reg<regs::Package, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RAM variant"]
    #[inline(always)]
    pub const fn ram(self) -> crate::common::Reg<regs::Ram, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Flash variant"]
    #[inline(always)]
    pub const fn flash(self) -> crate::common::Reg<regs::Flash, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
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
    #[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST, and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader0(self) -> crate::common::Reg<regs::Tagheader0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST, and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader1(self) -> crate::common::Reg<regs::Tagheader1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST, and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader2(self) -> crate::common::Reg<regs::Tagheader2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST, and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader3(self) -> crate::common::Reg<regs::Tagheader3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "Registers storing factory TEMP module linearization coefficients"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Temp {
    ptr: *mut u8,
}
unsafe impl Send for Temp {}
unsafe impl Sync for Temp {}
impl Temp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Slope definition A0"]
    #[inline(always)]
    pub const fn a0(self) -> crate::common::Reg<regs::A0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Slope definition A1"]
    #[inline(always)]
    pub const fn a1(self) -> crate::common::Reg<regs::A1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Slope definition A2"]
    #[inline(always)]
    pub const fn a2(self) -> crate::common::Reg<regs::A2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Slope definition A3"]
    #[inline(always)]
    pub const fn a3(self) -> crate::common::Reg<regs::A3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Slope definition A4"]
    #[inline(always)]
    pub const fn a4(self) -> crate::common::Reg<regs::A4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Slope definition A5"]
    #[inline(always)]
    pub const fn a5(self) -> crate::common::Reg<regs::A5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Y-intercept B0"]
    #[inline(always)]
    pub const fn b0(self) -> crate::common::Reg<regs::B0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Y-intercept B1"]
    #[inline(always)]
    pub const fn b1(self) -> crate::common::Reg<regs::B1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Y-intercept B2"]
    #[inline(always)]
    pub const fn b2(self) -> crate::common::Reg<regs::B2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Y-intercept B3"]
    #[inline(always)]
    pub const fn b3(self) -> crate::common::Reg<regs::B3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Y-intercept B4"]
    #[inline(always)]
    pub const fn b4(self) -> crate::common::Reg<regs::B4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Y-intercept B5"]
    #[inline(always)]
    pub const fn b5(self) -> crate::common::Reg<regs::B5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Segment end T0"]
    #[inline(always)]
    pub const fn t0(self) -> crate::common::Reg<regs::T0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Segment end T1"]
    #[inline(always)]
    pub const fn t1(self) -> crate::common::Reg<regs::T1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Segment end T2"]
    #[inline(always)]
    pub const fn t2(self) -> crate::common::Reg<regs::T2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Segment end T3"]
    #[inline(always)]
    pub const fn t3(self) -> crate::common::Reg<regs::T3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Segment end T4"]
    #[inline(always)]
    pub const fn t4(self) -> crate::common::Reg<regs::T4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
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
