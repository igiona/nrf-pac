#[doc = "Watchdog Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt {
    ptr: *mut u8,
}
unsafe impl Send for Wdt {}
unsafe impl Sync for Wdt {}
impl Wdt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start the watchdog"]
    #[inline(always)]
    pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Watchdog timeout"]
    #[inline(always)]
    pub const fn events_timeout(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "Run status"]
    #[inline(always)]
    pub const fn runstatus(self) -> crate::common::Reg<regs::Runstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Request status"]
    #[inline(always)]
    pub const fn reqstatus(self) -> crate::common::Reg<regs::Reqstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "Counter reload value"]
    #[inline(always)]
    pub const fn crv(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Enable register for reload request registers"]
    #[inline(always)]
    pub const fn rren(self) -> crate::common::Reg<regs::Rren, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "Description collection: Reload request n"]
    #[inline(always)]
    pub const fn rr(self, n: usize) -> crate::common::Reg<regs::Rr, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
