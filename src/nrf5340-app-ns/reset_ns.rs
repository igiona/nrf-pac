#[doc = "ULP network core control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Network {
    ptr: *mut u8,
}
unsafe impl Send for Network {}
unsafe impl Sync for Network {}
impl Network {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Force network core off"]
    #[inline(always)]
    pub const fn forceoff(self) -> crate::common::Reg<regs::Forceoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Reset control 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetNs {
    ptr: *mut u8,
}
unsafe impl Send for ResetNs {}
unsafe impl Sync for ResetNs {}
impl ResetNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Reset reason"]
    #[inline(always)]
    pub const fn resetreas(self) -> crate::common::Reg<regs::Resetreas, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "ULP network core control"]
    #[inline(always)]
    pub const fn network(self) -> Network {
        unsafe { Network::from_ptr(self.ptr.add(0x0610usize) as _) }
    }
}
pub mod regs;
pub mod vals;
