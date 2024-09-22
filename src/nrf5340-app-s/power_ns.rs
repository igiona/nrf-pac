#[doc = "Power control 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PowerNs {
    ptr: *mut u8,
}
unsafe impl Send for PowerNs {}
unsafe impl Sync for PowerNs {}
impl PowerNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Enable Constant Latency mode"]
    #[inline(always)]
    pub const fn tasks_constlat(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Enable Low-Power mode (variable latency)"]
    #[inline(always)]
    pub const fn tasks_lowpwr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "Subscribe configuration for task CONSTLAT"]
    #[inline(always)]
    pub const fn subscribe_constlat(
        self,
    ) -> crate::common::Reg<regs::SubscribeConstlat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "Subscribe configuration for task LOWPWR"]
    #[inline(always)]
    pub const fn subscribe_lowpwr(
        self,
    ) -> crate::common::Reg<regs::SubscribeLowpwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "Power failure warning"]
    #[inline(always)]
    pub const fn events_pofwarn(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "CPU entered WFI/WFE sleep"]
    #[inline(always)]
    pub const fn events_sleepenter(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "CPU exited WFI/WFE sleep"]
    #[inline(always)]
    pub const fn events_sleepexit(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "Publish configuration for event POFWARN"]
    #[inline(always)]
    pub const fn publish_pofwarn(
        self,
    ) -> crate::common::Reg<regs::PublishPofwarn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "Publish configuration for event SLEEPENTER"]
    #[inline(always)]
    pub const fn publish_sleepenter(
        self,
    ) -> crate::common::Reg<regs::PublishSleepenter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[doc = "Publish configuration for event SLEEPEXIT"]
    #[inline(always)]
    pub const fn publish_sleepexit(
        self,
    ) -> crate::common::Reg<regs::PublishSleepexit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
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
    #[doc = "Description collection: General purpose retention register"]
    #[inline(always)]
    pub const fn gpregret(self, n: usize) -> crate::common::Reg<regs::Gpregret, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
