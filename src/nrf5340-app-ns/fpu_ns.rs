#[doc = "FPU control peripheral 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpuNs {
    ptr: *mut u8,
}
unsafe impl Send for FpuNs {}
unsafe impl Sync for FpuNs {}
impl FpuNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_invalidoperation(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_dividebyzero(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_overflow(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_underflow(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_inexact(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
    #[inline(always)]
    pub const fn events_denormalinput(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
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
}
pub mod regs;
pub mod vals;
