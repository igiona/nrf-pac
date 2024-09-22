#[doc = "Access Port Protection"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Approtect {
    ptr: *mut u8,
}
unsafe impl Send for Approtect {}
unsafe impl Sync for Approtect {}
impl Approtect {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Software force enable APPROTECT mechanism until next reset."]
    #[inline(always)]
    pub const fn forceprotect(self) -> crate::common::Reg<regs::Forceprotect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
    }
    #[doc = "Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub const fn disable(self) -> crate::common::Reg<regs::Disable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0558usize) as _) }
    }
}
pub mod regs;
pub mod vals;
