#[doc = "Software interrupt 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swi0ns {
    ptr: *mut u8,
}
unsafe impl Send for Swi0ns {}
unsafe impl Sync for Swi0ns {}
impl Swi0ns {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Unused."]
    #[inline(always)]
    pub const fn unused(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
