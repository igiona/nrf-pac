#[doc = "Domain configuration management"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcnfNs {
    ptr: *mut u8,
}
unsafe impl Send for DcnfNs {}
unsafe impl Sync for DcnfNs {}
impl DcnfNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CPU ID of this subsystem"]
    #[inline(always)]
    pub const fn cpuid(self) -> crate::common::Reg<regs::Cpuid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
}
pub mod regs;
