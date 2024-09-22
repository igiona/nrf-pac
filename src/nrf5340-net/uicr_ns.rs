#[doc = "User Information Configuration Registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UicrNs {
    ptr: *mut u8,
}
unsafe impl Send for UicrNs {}
unsafe impl Sync for UicrNs {}
impl UicrNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Access port protection"]
    #[inline(always)]
    pub const fn approtect(self) -> crate::common::Reg<regs::Approtect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Erase protection"]
    #[inline(always)]
    pub const fn eraseprotect(self) -> crate::common::Reg<regs::Eraseprotect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description collection: Reserved for Nordic firmware design"]
    #[inline(always)]
    pub const fn nrffw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Reserved for customer"]
    #[inline(always)]
    pub const fn customer(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
