#[doc = "Temperature Sensor"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TempNs {
    ptr: *mut u8,
}
unsafe impl Send for TempNs {}
unsafe impl Sync for TempNs {}
impl TempNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start temperature measurement"]
    #[inline(always)]
    pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Stop temperature measurement"]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Subscribe configuration for task START"]
    #[inline(always)]
    pub const fn subscribe_start(
        self,
    ) -> crate::common::Reg<regs::SubscribeStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(
        self,
    ) -> crate::common::Reg<regs::SubscribeStop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Temperature measurement complete, data ready"]
    #[inline(always)]
    pub const fn events_datardy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Publish configuration for event DATARDY"]
    #[inline(always)]
    pub const fn publish_datardy(
        self,
    ) -> crate::common::Reg<regs::PublishDatardy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
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
    #[doc = "Temperature in degC (0.25deg steps)"]
    #[inline(always)]
    pub const fn temp(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Slope of first piecewise linear function"]
    #[inline(always)]
    pub const fn a0(self) -> crate::common::Reg<regs::A0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[doc = "Slope of second piecewise linear function"]
    #[inline(always)]
    pub const fn a1(self) -> crate::common::Reg<regs::A1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
    }
    #[doc = "Slope of third piecewise linear function"]
    #[inline(always)]
    pub const fn a2(self) -> crate::common::Reg<regs::A2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
    }
    #[doc = "Slope of fourth piecewise linear function"]
    #[inline(always)]
    pub const fn a3(self) -> crate::common::Reg<regs::A3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
    }
    #[doc = "Slope of fifth piecewise linear function"]
    #[inline(always)]
    pub const fn a4(self) -> crate::common::Reg<regs::A4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
    }
    #[doc = "Slope of sixth piecewise linear function"]
    #[inline(always)]
    pub const fn a5(self) -> crate::common::Reg<regs::A5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
    }
    #[doc = "y-intercept of first piecewise linear function"]
    #[inline(always)]
    pub const fn b0(self) -> crate::common::Reg<regs::B0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "y-intercept of second piecewise linear function"]
    #[inline(always)]
    pub const fn b1(self) -> crate::common::Reg<regs::B1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
    }
    #[doc = "y-intercept of third piecewise linear function"]
    #[inline(always)]
    pub const fn b2(self) -> crate::common::Reg<regs::B2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
    }
    #[doc = "y-intercept of fourth piecewise linear function"]
    #[inline(always)]
    pub const fn b3(self) -> crate::common::Reg<regs::B3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
    }
    #[doc = "y-intercept of fifth piecewise linear function"]
    #[inline(always)]
    pub const fn b4(self) -> crate::common::Reg<regs::B4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
    }
    #[doc = "y-intercept of sixth piecewise linear function"]
    #[inline(always)]
    pub const fn b5(self) -> crate::common::Reg<regs::B5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
    }
    #[doc = "Endpoint of first piecewise linear function"]
    #[inline(always)]
    pub const fn t0(self) -> crate::common::Reg<regs::T0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
    #[doc = "Endpoint of second piecewise linear function"]
    #[inline(always)]
    pub const fn t1(self) -> crate::common::Reg<regs::T1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0564usize) as _) }
    }
    #[doc = "Endpoint of third piecewise linear function"]
    #[inline(always)]
    pub const fn t2(self) -> crate::common::Reg<regs::T2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0568usize) as _) }
    }
    #[doc = "Endpoint of fourth piecewise linear function"]
    #[inline(always)]
    pub const fn t3(self) -> crate::common::Reg<regs::T3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
    }
    #[doc = "Endpoint of fifth piecewise linear function"]
    #[inline(always)]
    pub const fn t4(self) -> crate::common::Reg<regs::T4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0570usize) as _) }
    }
}
pub mod regs;
pub mod vals;
