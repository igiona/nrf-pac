#[doc = "Reset control"]
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
}
pub mod regs;
pub mod vals;
