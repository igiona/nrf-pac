#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel {
    ptr: *mut u8,
}
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Pin configuration for TRACECLK"]
    #[inline(always)]
    pub const fn traceclk(self) -> crate::common::Reg<regs::Traceclk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Pin configuration for TRACEDATA\\[0\\]"]
    #[inline(always)]
    pub const fn tracedata0(self) -> crate::common::Reg<regs::Tracedata0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Pin configuration for TRACEDATA\\[1\\]"]
    #[inline(always)]
    pub const fn tracedata1(self) -> crate::common::Reg<regs::Tracedata1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Pin configuration for TRACEDATA\\[2\\]"]
    #[inline(always)]
    pub const fn tracedata2(self) -> crate::common::Reg<regs::Tracedata2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Pin configuration for TRACEDATA\\[3\\]"]
    #[inline(always)]
    pub const fn tracedata3(self) -> crate::common::Reg<regs::Tracedata3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
#[doc = "Trace and debug control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TadS {
    ptr: *mut u8,
}
unsafe impl Send for TadS {}
unsafe impl Sync for TadS {}
impl TadS {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start all trace and debug clocks."]
    #[inline(always)]
    pub const fn clockstart(self) -> crate::common::Reg<regs::Clockstart, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Stop all trace and debug clocks."]
    #[inline(always)]
    pub const fn clockstop(self) -> crate::common::Reg<regs::Clockstop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Enable debug domain and aquire selected GPIOs"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn psel(self) -> Psel {
        unsafe { Psel::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Clocking options for the Trace Port debug interface Reset behavior is the same as debug components"]
    #[inline(always)]
    pub const fn traceportspeed(
        self,
    ) -> crate::common::Reg<regs::Traceportspeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
}
pub mod regs;
pub mod vals;
