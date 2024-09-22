#[doc = "Real time counter 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc0 {
    ptr: *mut u8,
}
unsafe impl Send for Rtc0 {}
unsafe impl Sync for Rtc0 {}
impl Rtc0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start RTC COUNTER"]
    #[inline(always)]
    pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Stop RTC COUNTER"]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Clear RTC COUNTER"]
    #[inline(always)]
    pub const fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Set COUNTER to 0xFFFFF0"]
    #[inline(always)]
    pub const fn tasks_trigovrflw(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Event on COUNTER increment"]
    #[inline(always)]
    pub const fn events_tick(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Event on COUNTER overflow"]
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
    #[doc = "Current COUNTER value"]
    #[inline(always)]
    pub const fn counter(self) -> crate::common::Reg<regs::Counter, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is stopped."]
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
