#[doc = "Factory Information Configuration Registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FicrNs {
    ptr: *mut u8,
}
unsafe impl Send for FicrNs {}
unsafe impl Sync for FicrNs {}
impl FicrNs {
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
    #[doc = "Description collection: Encryption Root, word n"]
    #[inline(always)]
    pub const fn er(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Identity Root, word n"]
    #[inline(always)]
    pub const fn ir(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0290usize + n * 4usize) as _) }
    }
    #[doc = "Device address type"]
    #[inline(always)]
    pub const fn deviceaddrtype(
        self,
    ) -> crate::common::Reg<regs::Deviceaddrtype, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a0usize) as _) }
    }
    #[doc = "Description collection: Device address n"]
    #[inline(always)]
    pub const fn deviceaddr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a4usize + n * 4usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn trimcnf(self, n: usize) -> Trimcnf {
        assert!(n < 32usize);
        unsafe { Trimcnf::from_ptr(self.ptr.add(0x0300usize + n * 8usize) as _) }
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
    #[doc = "Description cluster: Address"]
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
pub mod regs;
pub mod vals;
