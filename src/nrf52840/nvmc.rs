#[doc = "Non Volatile Memory Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvmc {
    ptr: *mut u8,
}
unsafe impl Send for Nvmc {}
unsafe impl Sync for Nvmc {}
impl Nvmc {
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
    #[doc = "Register for erasing a page in code area"]
    #[inline(always)]
    pub const fn erasepage(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
    #[inline(always)]
    pub const fn erasepcr1(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Register for erasing all non-volatile user memory"]
    #[inline(always)]
    pub const fn eraseall(self) -> crate::common::Reg<regs::Eraseall, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
    #[inline(always)]
    pub const fn erasepcr0(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Register for erasing user information configuration registers"]
    #[inline(always)]
    pub const fn eraseuicr(self) -> crate::common::Reg<regs::Eraseuicr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "Register for partial erase of a page in code area"]
    #[inline(always)]
    pub const fn erasepagepartial(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "Register for partial erase configuration"]
    #[inline(always)]
    pub const fn erasepagepartialcfg(
        self,
    ) -> crate::common::Reg<regs::Erasepagepartialcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "I-code cache configuration register"]
    #[inline(always)]
    pub const fn icachecnf(self) -> crate::common::Reg<regs::Icachecnf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "I-code cache hit counter"]
    #[inline(always)]
    pub const fn ihit(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
    }
    #[doc = "I-code cache miss counter"]
    #[inline(always)]
    pub const fn imiss(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
