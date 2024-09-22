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
    #[doc = "Description collection: Output pin select for PWM channel n"]
    #[inline(always)]
    pub const fn out(self, n: usize) -> crate::common::Reg<regs::Out, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
#[doc = "Pulse width modulation unit 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0ns {
    ptr: *mut u8,
}
unsafe impl Send for Pwm0ns {}
unsafe impl Sync for Pwm0ns {}
impl Pwm0ns {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
    #[inline(always)]
    pub const fn tasks_seqstart(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
    #[inline(always)]
    pub const fn tasks_nextstep(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(
        self,
    ) -> crate::common::Reg<regs::SubscribeStop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Description collection: Subscribe configuration for task SEQSTART\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_seqstart(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SubscribeSeqstart, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize + n * 4usize) as _) }
    }
    #[doc = "Subscribe configuration for task NEXTSTEP"]
    #[inline(always)]
    pub const fn subscribe_nextstep(
        self,
    ) -> crate::common::Reg<regs::SubscribeNextstep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Response to STOP task, emitted when PWM pulses are no longer generated"]
    #[inline(always)]
    pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Description collection: First PWM period started on sequence n"]
    #[inline(always)]
    pub const fn events_seqstarted(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
    #[inline(always)]
    pub const fn events_seqend(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize + n * 4usize) as _) }
    }
    #[doc = "Emitted at the end of each PWM period"]
    #[inline(always)]
    pub const fn events_pwmperiodend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
    #[inline(always)]
    pub const fn events_loopsdone(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "Publish configuration for event STOPPED"]
    #[inline(always)]
    pub const fn publish_stopped(
        self,
    ) -> crate::common::Reg<regs::PublishStopped, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Description collection: Publish configuration for event SEQSTARTED\\[n\\]"]
    #[inline(always)]
    pub const fn publish_seqstarted(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PublishSeqstarted, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Publish configuration for event SEQEND\\[n\\]"]
    #[inline(always)]
    pub const fn publish_seqend(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PublishSeqend, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize + n * 4usize) as _) }
    }
    #[doc = "Publish configuration for event PWMPERIODEND"]
    #[inline(always)]
    pub const fn publish_pwmperiodend(
        self,
    ) -> crate::common::Reg<regs::PublishPwmperiodend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[doc = "Publish configuration for event LOOPSDONE"]
    #[inline(always)]
    pub const fn publish_loopsdone(
        self,
    ) -> crate::common::Reg<regs::PublishLoopsdone, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
    }
    #[doc = "Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
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
    #[doc = "PWM module enable register"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Selects operating mode of the wave counter"]
    #[inline(always)]
    pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Value up to which the pulse generator counter counts"]
    #[inline(always)]
    pub const fn countertop(self) -> crate::common::Reg<regs::Countertop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Configuration for PWM_CLK"]
    #[inline(always)]
    pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "Configuration of the decoder"]
    #[inline(always)]
    pub const fn decoder(self) -> crate::common::Reg<regs::Decoder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Number of playbacks of a loop"]
    #[inline(always)]
    pub const fn loop_(self) -> crate::common::Reg<regs::Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn seq(self, n: usize) -> Seq {
        assert!(n < 2usize);
        unsafe { Seq::from_ptr(self.ptr.add(0x0520usize + n * 32usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn psel(self) -> Psel {
        unsafe { Psel::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seq {
    ptr: *mut u8,
}
unsafe impl Send for Seq {}
unsafe impl Sync for Seq {}
impl Seq {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Beginning address in RAM of this sequence"]
    #[inline(always)]
    pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Number of values (duty cycles) in this sequence"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description cluster: Number of additional PWM periods between samples loaded into compare register"]
    #[inline(always)]
    pub const fn refresh(self) -> crate::common::Reg<regs::Refresh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Description cluster: Time added after the sequence"]
    #[inline(always)]
    pub const fn enddelay(self) -> crate::common::Reg<regs::Enddelay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
