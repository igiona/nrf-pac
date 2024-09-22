#[doc = "Oscillator control 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscillatorsNs {
    ptr: *mut u8,
}
unsafe impl Send for OscillatorsNs {}
unsafe impl Sync for OscillatorsNs {}
impl OscillatorsNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Programmable capacitance of XC1 and XC2"]
    #[inline(always)]
    pub const fn xosc32mcaps(self) -> crate::common::Reg<regs::Xosc32mcaps, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c4usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn xosc32ki(self) -> Xosc32ki {
        unsafe { Xosc32ki::from_ptr(self.ptr.add(0x06c0usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xosc32ki {
    ptr: *mut u8,
}
unsafe impl Send for Xosc32ki {}
unsafe impl Sync for Xosc32ki {}
impl Xosc32ki {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub const fn bypass(self) -> crate::common::Reg<regs::Bypass, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Control usage of internal load capacitors"]
    #[inline(always)]
    pub const fn intcap(self) -> crate::common::Reg<regs::Intcap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs;
pub mod vals;
