#[doc = "Clock control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clock {
    ptr: *mut u8,
}
unsafe impl Send for Clock {}
unsafe impl Sync for Clock {}
impl Clock {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start HFXO crystal oscillator"]
    #[inline(always)]
    pub const fn tasks_hfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Stop HFXO crystal oscillator"]
    #[inline(always)]
    pub const fn tasks_hfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Start LFCLK"]
    #[inline(always)]
    pub const fn tasks_lfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Stop LFCLK"]
    #[inline(always)]
    pub const fn tasks_lfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Start calibration of LFRC"]
    #[inline(always)]
    pub const fn tasks_cal(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Start calibration timer"]
    #[inline(always)]
    pub const fn tasks_ctstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Stop calibration timer"]
    #[inline(always)]
    pub const fn tasks_ctstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "HFXO crystal oscillator started"]
    #[inline(always)]
    pub const fn events_hfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "LFCLK started"]
    #[inline(always)]
    pub const fn events_lfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Calibration of LFRC completed"]
    #[inline(always)]
    pub const fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Calibration timer timeout"]
    #[inline(always)]
    pub const fn events_ctto(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Calibration timer has been started and is ready to process new tasks"]
    #[inline(always)]
    pub const fn events_ctstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "Calibration timer has been stopped and is ready to process new tasks"]
    #[inline(always)]
    pub const fn events_ctstopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
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
    #[doc = "Status indicating that HFCLKSTART task has been triggered"]
    #[inline(always)]
    pub const fn hfclkrun(self) -> crate::common::Reg<regs::Hfclkrun, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "HFCLK status"]
    #[inline(always)]
    pub const fn hfclkstat(self) -> crate::common::Reg<regs::Hfclkstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "Status indicating that LFCLKSTART task has been triggered"]
    #[inline(always)]
    pub const fn lfclkrun(self) -> crate::common::Reg<regs::Lfclkrun, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "LFCLK status"]
    #[inline(always)]
    pub const fn lfclkstat(self) -> crate::common::Reg<regs::Lfclkstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    #[inline(always)]
    pub const fn lfclksrccopy(self) -> crate::common::Reg<regs::Lfclksrccopy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[doc = "Clock source for the LFCLK"]
    #[inline(always)]
    pub const fn lfclksrc(self) -> crate::common::Reg<regs::Lfclksrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task."]
    #[inline(always)]
    pub const fn hfxodebounce(self) -> crate::common::Reg<regs::Hfxodebounce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
    }
    #[doc = "LFXO debounce time. The LFXO is started by triggering the TASKS_LFCLKSTART task when the LFCLKSRC register is configured for Xtal."]
    #[inline(always)]
    pub const fn lfxodebounce(self) -> crate::common::Reg<regs::Lfxodebounce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
    }
    #[doc = "Calibration timer interval"]
    #[inline(always)]
    pub const fn ctiv(self) -> crate::common::Reg<regs::Ctiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
    }
    #[doc = "Clocking options for the trace port debug interface"]
    #[inline(always)]
    pub const fn traceconfig(self) -> crate::common::Reg<regs::Traceconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x055cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
