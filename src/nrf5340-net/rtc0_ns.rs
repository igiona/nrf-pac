#[doc = "Real-time counter 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc0ns {
    ptr: *mut u8,
}
unsafe impl Send for Rtc0ns {}
unsafe impl Sync for Rtc0ns {}
impl Rtc0ns {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start RTC counter"]
    #[inline(always)]
    pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Stop RTC counter"]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Clear RTC counter"]
    #[inline(always)]
    pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Set counter to 0xFFFFF0"]
    #[inline(always)]
    pub const fn tasks_trigovrflw(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Description collection: Capture RTC counter to CC\\[n\\] register"]
    #[inline(always)]
    pub const fn tasks_capture(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
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
    #[doc = "Subscribe configuration for task CLEAR"]
    #[inline(always)]
    pub const fn subscribe_clear(
        self,
    ) -> crate::common::Reg<regs::SubscribeClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Subscribe configuration for task TRIGOVRFLW"]
    #[inline(always)]
    pub const fn subscribe_trigovrflw(
        self,
    ) -> crate::common::Reg<regs::SubscribeTrigovrflw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_capture(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SubscribeCapture, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "Event on counter increment"]
    #[inline(always)]
    pub const fn events_tick(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Event on counter overflow"]
    #[inline(always)]
    pub const fn events_ovrflw(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Description collection: Compare event on CC\\[n\\] match"]
    #[inline(always)]
    pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
    }
    #[doc = "Publish configuration for event TICK"]
    #[inline(always)]
    pub const fn publish_tick(self) -> crate::common::Reg<regs::PublishTick, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Publish configuration for event OVRFLW"]
    #[inline(always)]
    pub const fn publish_ovrflw(
        self,
    ) -> crate::common::Reg<regs::PublishOvrflw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
    #[inline(always)]
    pub const fn publish_compare(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PublishCompare, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize + n * 4usize) as _) }
    }
    #[doc = "Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
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
    #[doc = "Enable or disable event routing"]
    #[inline(always)]
    pub const fn evten(self) -> crate::common::Reg<regs::Evten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
    }
    #[doc = "Enable event routing"]
    #[inline(always)]
    pub const fn evtenset(self) -> crate::common::Reg<regs::Evtenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
    }
    #[doc = "Disable event routing"]
    #[inline(always)]
    pub const fn evtenclr(self) -> crate::common::Reg<regs::Evtenclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0348usize) as _) }
    }
    #[doc = "Current counter value"]
    #[inline(always)]
    pub const fn counter(self) -> crate::common::Reg<regs::Counter, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "12-bit prescaler for counter frequency (32768 / (PRESCALER + 1)). Must be written when RTC is stopped."]
    #[inline(always)]
    pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Description collection: Compare register n"]
    #[inline(always)]
    pub const fn cc(self, n: usize) -> crate::common::Reg<regs::Cc, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
