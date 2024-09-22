#[doc = "Non-volatile memory controller 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NvmcNs {
    ptr: *mut u8,
}
unsafe impl Send for NvmcNs {}
unsafe impl Sync for NvmcNs {}
impl NvmcNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Ready flag"]
    #[inline(always)]
    pub const fn ready(self) -> crate::common::Reg<regs::Ready, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Ready flag"]
    #[inline(always)]
    pub const fn readynext(self) -> crate::common::Reg<regs::Readynext, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Register for erasing all non-volatile user memory"]
    #[inline(always)]
    pub const fn eraseall(self) -> crate::common::Reg<regs::Eraseall, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "Register for partial erase configuration"]
    #[inline(always)]
    pub const fn erasepagepartialcfg(
        self,
    ) -> crate::common::Reg<regs::Erasepagepartialcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "Non-secure configuration register"]
    #[inline(always)]
    pub const fn configns(self) -> crate::common::Reg<regs::Configns, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0584usize) as _) }
    }
    #[doc = "Non-secure APPROTECT enable register"]
    #[inline(always)]
    pub const fn writeuicrns(self) -> crate::common::Reg<regs::Writeuicrns, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
    }
}
pub mod regs;
pub mod vals;
