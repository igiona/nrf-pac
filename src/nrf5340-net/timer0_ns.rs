#[doc = "Timer/Counter 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0ns {
    ptr: *mut u8,
}
unsafe impl Send for Timer0ns {}
unsafe impl Sync for Timer0ns {}
impl Timer0ns {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start Timer"]
    #[inline(always)]
    pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Stop Timer"]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Increment Timer (Counter mode only)"]
    #[inline(always)]
    pub const fn tasks_count(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Clear time"]
    #[inline(always)]
    pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Deprecated register - Shut down timer"]
    #[inline(always)]
    pub const fn tasks_shutdown(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Description collection: Capture Timer value to CC\\[n\\] register"]
    #[inline(always)]
    pub const fn tasks_capture(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
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
    #[doc = "Subscribe configuration for task COUNT"]
    #[inline(always)]
    pub const fn subscribe_count(
        self,
    ) -> crate::common::Reg<regs::SubscribeCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Subscribe configuration for task CLEAR"]
    #[inline(always)]
    pub const fn subscribe_clear(
        self,
    ) -> crate::common::Reg<regs::SubscribeClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Deprecated register - Subscribe configuration for task SHUTDOWN"]
    #[inline(always)]
    pub const fn subscribe_shutdown(
        self,
    ) -> crate::common::Reg<regs::SubscribeShutdown, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_capture(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SubscribeCapture, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Compare event on CC\\[n\\] match"]
    #[inline(always)]
    pub const fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
    #[inline(always)]
    pub const fn publish_compare(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PublishCompare, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize + n * 4usize) as _) }
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
    #[doc = "Timer mode selection"]
    #[inline(always)]
    pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Configure the number of bits used by the TIMER"]
    #[inline(always)]
    pub const fn bitmode(self) -> crate::common::Reg<regs::Bitmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Timer prescaler register"]
    #[inline(always)]
    pub const fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Description collection: Capture/Compare register n"]
    #[inline(always)]
    pub const fn cc(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n"]
    #[inline(always)]
    pub const fn oneshoten(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Oneshoten, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
