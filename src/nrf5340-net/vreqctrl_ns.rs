#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vregradio {
    ptr: *mut u8,
}
unsafe impl Send for Vregradio {}
unsafe impl Sync for Vregradio {}
impl Vregradio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready"]
    #[inline(always)]
    pub const fn vreqh(self) -> crate::common::Reg<regs::Vreqh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "High voltage on RADIO is ready"]
    #[inline(always)]
    pub const fn vreqhready(self) -> crate::common::Reg<regs::Vreqhready, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Voltage request control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VreqctrlNs {
    ptr: *mut u8,
}
unsafe impl Send for VreqctrlNs {}
unsafe impl Sync for VreqctrlNs {}
impl VreqctrlNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn vregradio(self) -> Vregradio {
        unsafe { Vregradio::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
}
pub mod regs;
pub mod vals;
