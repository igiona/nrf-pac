#[doc = "ARM TrustZone CryptoCell register interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cryptocell {
    ptr: *mut u8,
}
unsafe impl Send for Cryptocell {}
unsafe impl Sync for Cryptocell {}
impl Cryptocell {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Enable CRYPTOCELL subsystem"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
}
pub mod regs;
pub mod vals;
