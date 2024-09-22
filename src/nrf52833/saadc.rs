#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch {
    ptr: *mut u8,
}
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Input positive pin selection for CH\\[n\\]"]
    #[inline(always)]
    pub const fn pselp(self) -> crate::common::Reg<regs::Pselp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Input negative pin selection for CH\\[n\\]"]
    #[inline(always)]
    pub const fn pseln(self) -> crate::common::Reg<regs::Pseln, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description cluster: Input configuration for CH\\[n\\]"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Description cluster: High/low limits for event monitoring of a channel"]
    #[inline(always)]
    pub const fn limit(self) -> crate::common::Reg<regs::Limit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "Peripheral events."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EventsCh {
    ptr: *mut u8,
}
unsafe impl Send for EventsCh {}
unsafe impl Sync for EventsCh {}
impl EventsCh {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Last result is equal or above CH\\[n\\].LIMIT.HIGH"]
    #[inline(always)]
    pub const fn limith(self) -> crate::common::Reg<regs::Limith, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Last result is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    pub const fn limitl(self) -> crate::common::Reg<regs::Limitl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "RESULT EasyDMA channel"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Result {
    ptr: *mut u8,
}
unsafe impl Send for Result {}
unsafe impl Sync for Result {}
impl Result {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data pointer"]
    #[inline(always)]
    pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Maximum number of 16-bit samples to be written to output RAM buffer"]
    #[inline(always)]
    pub const fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Number of 16-bit samples written to output RAM buffer since the previous START task"]
    #[inline(always)]
    pub const fn amount(self) -> crate::common::Reg<regs::Amount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Successive approximation register (SAR) analog-to-digital converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Saadc {
    ptr: *mut u8,
}
unsafe impl Send for Saadc {}
unsafe impl Sync for Saadc {}
impl Saadc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Starts the SAADC and prepares the result buffer in RAM"]
    #[inline(always)]
    pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Takes one SAADC sample"]
    #[inline(always)]
    pub const fn tasks_sample(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Stops the SAADC and terminates all on-going conversions"]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Starts offset auto-calibration"]
    #[inline(always)]
    pub const fn tasks_calibrateoffset(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "The SAADC has started"]
    #[inline(always)]
    pub const fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "The SAADC has filled up the result buffer"]
    #[inline(always)]
    pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "A conversion task has been completed. Depending on the configuration, multiple conversions might be needed for a result to be transferred to RAM."]
    #[inline(always)]
    pub const fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Result ready for transfer to RAM"]
    #[inline(always)]
    pub const fn events_resultdone(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Calibration is complete"]
    #[inline(always)]
    pub const fn events_calibratedone(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "The SAADC has stopped"]
    #[inline(always)]
    pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "Peripheral events."]
    #[inline(always)]
    pub const fn events_ch(self, n: usize) -> EventsCh {
        assert!(n < 8usize);
        unsafe { EventsCh::from_ptr(self.ptr.add(0x0118usize + n * 8usize) as _) }
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
    #[doc = "Status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Enable or disable SAADC"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 8usize);
        unsafe { Ch::from_ptr(self.ptr.add(0x0510usize + n * 16usize) as _) }
    }
    #[doc = "Resolution configuration"]
    #[inline(always)]
    pub const fn resolution(self) -> crate::common::Reg<regs::Resolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f0usize) as _) }
    }
    #[doc = "Oversampling configuration. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
    #[inline(always)]
    pub const fn oversample(self) -> crate::common::Reg<regs::Oversample, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f4usize) as _) }
    }
    #[doc = "Controls normal or continuous sample rate"]
    #[inline(always)]
    pub const fn samplerate(self) -> crate::common::Reg<regs::Samplerate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f8usize) as _) }
    }
    #[doc = "RESULT EasyDMA channel"]
    #[inline(always)]
    pub const fn result(self) -> Result {
        unsafe { Result::from_ptr(self.ptr.add(0x062cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
