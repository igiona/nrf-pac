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
    #[doc = "Description cluster: High/low limits for event monitoring a channel"]
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
    #[doc = "Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
    #[inline(always)]
    pub const fn limith(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    pub const fn limitl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Publish configuration for events"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishCh {
    ptr: *mut u8,
}
unsafe impl Send for PublishCh {}
unsafe impl Sync for PublishCh {}
impl PublishCh {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Publish configuration for event CH\\[n\\].LIMITH"]
    #[inline(always)]
    pub const fn limith(self) -> crate::common::Reg<regs::PublishChLimith, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Publish configuration for event CH\\[n\\].LIMITL"]
    #[inline(always)]
    pub const fn limitl(self) -> crate::common::Reg<regs::PublishChLimitl, crate::common::RW> {
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
    #[doc = "Maximum number of buffer words to transfer"]
    #[inline(always)]
    pub const fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Number of buffer words transferred since last START"]
    #[inline(always)]
    pub const fn amount(self) -> crate::common::Reg<regs::Amount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Analog to Digital Converter 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SaadcNs {
    ptr: *mut u8,
}
unsafe impl Send for SaadcNs {}
unsafe impl Sync for SaadcNs {}
impl SaadcNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start the ADC and prepare the result buffer in RAM"]
    #[inline(always)]
    pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Take one ADC sample, if scan is enabled all channels are sampled"]
    #[inline(always)]
    pub const fn tasks_sample(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Stop the ADC and terminate any ongoing conversion"]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Starts offset auto-calibration"]
    #[inline(always)]
    pub const fn tasks_calibrateoffset(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Subscribe configuration for task START"]
    #[inline(always)]
    pub const fn subscribe_start(
        self,
    ) -> crate::common::Reg<regs::SubscribeStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Subscribe configuration for task SAMPLE"]
    #[inline(always)]
    pub const fn subscribe_sample(
        self,
    ) -> crate::common::Reg<regs::SubscribeSample, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(
        self,
    ) -> crate::common::Reg<regs::SubscribeStop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Subscribe configuration for task CALIBRATEOFFSET"]
    #[inline(always)]
    pub const fn subscribe_calibrateoffset(
        self,
    ) -> crate::common::Reg<regs::SubscribeCalibrateoffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "The ADC has started"]
    #[inline(always)]
    pub const fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "The ADC has filled up the Result buffer"]
    #[inline(always)]
    pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
    #[inline(always)]
    pub const fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "A result is ready to get transferred to RAM"]
    #[inline(always)]
    pub const fn events_resultdone(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Calibration is complete"]
    #[inline(always)]
    pub const fn events_calibratedone(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "The ADC has stopped"]
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
    #[doc = "Publish configuration for event STARTED"]
    #[inline(always)]
    pub const fn publish_started(
        self,
    ) -> crate::common::Reg<regs::PublishStarted, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Publish configuration for event END"]
    #[inline(always)]
    pub const fn publish_end(self) -> crate::common::Reg<regs::PublishEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Publish configuration for event DONE"]
    #[inline(always)]
    pub const fn publish_done(self) -> crate::common::Reg<regs::PublishDone, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "Publish configuration for event RESULTDONE"]
    #[inline(always)]
    pub const fn publish_resultdone(
        self,
    ) -> crate::common::Reg<regs::PublishResultdone, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[doc = "Publish configuration for event CALIBRATEDONE"]
    #[inline(always)]
    pub const fn publish_calibratedone(
        self,
    ) -> crate::common::Reg<regs::PublishCalibratedone, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "Publish configuration for event STOPPED"]
    #[inline(always)]
    pub const fn publish_stopped(
        self,
    ) -> crate::common::Reg<regs::PublishStopped, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[doc = "Publish configuration for events"]
    #[inline(always)]
    pub const fn publish_ch(self, n: usize) -> PublishCh {
        assert!(n < 8usize);
        unsafe { PublishCh::from_ptr(self.ptr.add(0x0198usize + n * 8usize) as _) }
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
    #[doc = "Enable or disable ADC"]
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
    #[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
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
