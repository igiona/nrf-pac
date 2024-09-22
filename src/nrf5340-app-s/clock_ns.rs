#[doc = "Clock management 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClockNs {
    ptr: *mut u8,
}
unsafe impl Send for ClockNs {}
unsafe impl Sync for ClockNs {}
impl ClockNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start HFCLK128M/HFCLK64M source as selected in HFCLKSRC"]
    #[inline(always)]
    pub const fn tasks_hfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Stop HFCLK128M/HFCLK64M source"]
    #[inline(always)]
    pub const fn tasks_hfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Start LFCLK source as selected in LFCLKSRC"]
    #[inline(always)]
    pub const fn tasks_lfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Stop LFCLK source"]
    #[inline(always)]
    pub const fn tasks_lfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Start calibration of LFRC oscillator"]
    #[inline(always)]
    pub const fn tasks_cal(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Start HFCLKAUDIO source"]
    #[inline(always)]
    pub const fn tasks_hfclkaudiostart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Stop HFCLKAUDIO source"]
    #[inline(always)]
    pub const fn tasks_hfclkaudiostop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Start HFCLK192M source as selected in HFCLK192MSRC"]
    #[inline(always)]
    pub const fn tasks_hfclk192mstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Stop HFCLK192M source"]
    #[inline(always)]
    pub const fn tasks_hfclk192mstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Subscribe configuration for task HFCLKSTART"]
    #[inline(always)]
    pub const fn subscribe_hfclkstart(
        self,
    ) -> crate::common::Reg<regs::SubscribeHfclkstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Subscribe configuration for task HFCLKSTOP"]
    #[inline(always)]
    pub const fn subscribe_hfclkstop(
        self,
    ) -> crate::common::Reg<regs::SubscribeHfclkstop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Subscribe configuration for task LFCLKSTART"]
    #[inline(always)]
    pub const fn subscribe_lfclkstart(
        self,
    ) -> crate::common::Reg<regs::SubscribeLfclkstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Subscribe configuration for task LFCLKSTOP"]
    #[inline(always)]
    pub const fn subscribe_lfclkstop(
        self,
    ) -> crate::common::Reg<regs::SubscribeLfclkstop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Subscribe configuration for task CAL"]
    #[inline(always)]
    pub const fn subscribe_cal(self) -> crate::common::Reg<regs::SubscribeCal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Subscribe configuration for task HFCLKAUDIOSTART"]
    #[inline(always)]
    pub const fn subscribe_hfclkaudiostart(
        self,
    ) -> crate::common::Reg<regs::SubscribeHfclkaudiostart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Subscribe configuration for task HFCLKAUDIOSTOP"]
    #[inline(always)]
    pub const fn subscribe_hfclkaudiostop(
        self,
    ) -> crate::common::Reg<regs::SubscribeHfclkaudiostop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Subscribe configuration for task HFCLK192MSTART"]
    #[inline(always)]
    pub const fn subscribe_hfclk192mstart(
        self,
    ) -> crate::common::Reg<regs::SubscribeHfclk192mstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Subscribe configuration for task HFCLK192MSTOP"]
    #[inline(always)]
    pub const fn subscribe_hfclk192mstop(
        self,
    ) -> crate::common::Reg<regs::SubscribeHfclk192mstop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "HFCLK128M/HFCLK64M source started"]
    #[inline(always)]
    pub const fn events_hfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "LFCLK source started"]
    #[inline(always)]
    pub const fn events_lfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Calibration of LFRC oscillator complete event"]
    #[inline(always)]
    pub const fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "HFCLKAUDIO source started"]
    #[inline(always)]
    pub const fn events_hfclkaudiostarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "HFCLK192M source started"]
    #[inline(always)]
    pub const fn events_hfclk192mstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "Publish configuration for event HFCLKSTARTED"]
    #[inline(always)]
    pub const fn publish_hfclkstarted(
        self,
    ) -> crate::common::Reg<regs::PublishHfclkstarted, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Publish configuration for event LFCLKSTARTED"]
    #[inline(always)]
    pub const fn publish_lfclkstarted(
        self,
    ) -> crate::common::Reg<regs::PublishLfclkstarted, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Publish configuration for event DONE"]
    #[inline(always)]
    pub const fn publish_done(self) -> crate::common::Reg<regs::PublishDone, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
    }
    #[doc = "Publish configuration for event HFCLKAUDIOSTARTED"]
    #[inline(always)]
    pub const fn publish_hfclkaudiostarted(
        self,
    ) -> crate::common::Reg<regs::PublishHfclkaudiostarted, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[doc = "Publish configuration for event HFCLK192MSTARTED"]
    #[inline(always)]
    pub const fn publish_hfclk192mstarted(
        self,
    ) -> crate::common::Reg<regs::PublishHfclk192mstarted, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
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
    #[doc = "Pending interrupts"]
    #[inline(always)]
    pub const fn intpend(self) -> crate::common::Reg<regs::Intpend, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "Status indicating that HFCLKSTART task has been triggered"]
    #[inline(always)]
    pub const fn hfclkrun(self) -> crate::common::Reg<regs::Hfclkrun, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
    #[inline(always)]
    pub const fn hfclkstat(self) -> crate::common::Reg<regs::Hfclkstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "Status indicating that LFCLKSTART task has been triggered"]
    #[inline(always)]
    pub const fn lfclkrun(self) -> crate::common::Reg<regs::Lfclkrun, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance."]
    #[inline(always)]
    pub const fn lfclkstat(self) -> crate::common::Reg<regs::Lfclkstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    #[inline(always)]
    pub const fn lfclksrccopy(self) -> crate::common::Reg<regs::Lfclksrccopy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[doc = "Status indicating that HFCLKAUDIOSTART task has been triggered"]
    #[inline(always)]
    pub const fn hfclkaudiorun(self) -> crate::common::Reg<regs::Hfclkaudiorun, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0450usize) as _) }
    }
    #[doc = "Status indicating which HFCLKAUDIO source is running"]
    #[inline(always)]
    pub const fn hfclkaudiostat(
        self,
    ) -> crate::common::Reg<regs::Hfclkaudiostat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0454usize) as _) }
    }
    #[doc = "Status indicating that HFCLK192MSTART task has been triggered"]
    #[inline(always)]
    pub const fn hfclk192mrun(self) -> crate::common::Reg<regs::Hfclk192mrun, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0458usize) as _) }
    }
    #[doc = "Status indicating which HFCLK192M source is running"]
    #[inline(always)]
    pub const fn hfclk192mstat(self) -> crate::common::Reg<regs::Hfclk192mstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x045cusize) as _) }
    }
    #[doc = "Clock source for HFCLK128M/HFCLK64M"]
    #[inline(always)]
    pub const fn hfclksrc(self) -> crate::common::Reg<regs::Hfclksrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "Clock source for LFCLK"]
    #[inline(always)]
    pub const fn lfclksrc(self) -> crate::common::Reg<regs::Lfclksrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "HFCLK128M frequency configuration"]
    #[inline(always)]
    pub const fn hfclkctrl(self) -> crate::common::Reg<regs::Hfclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0558usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn hfclkaudio(self) -> Hfclkaudio {
        unsafe { Hfclkaudio::from_ptr(self.ptr.add(0x055cusize) as _) }
    }
    #[doc = "Automatic or manual control of HFCLK128M/HFCLK64M"]
    #[inline(always)]
    pub const fn hfclkalwaysrun(
        self,
    ) -> crate::common::Reg<regs::Hfclkalwaysrun, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0570usize) as _) }
    }
    #[doc = "Automatic or manual control of LFCLK"]
    #[inline(always)]
    pub const fn lfclkalwaysrun(
        self,
    ) -> crate::common::Reg<regs::Lfclkalwaysrun, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0574usize) as _) }
    }
    #[doc = "Automatic or manual control of HFCLKAUDIO"]
    #[inline(always)]
    pub const fn hfclkaudioalwaysrun(
        self,
    ) -> crate::common::Reg<regs::Hfclkaudioalwaysrun, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x057cusize) as _) }
    }
    #[doc = "Clock source for HFCLK192M"]
    #[inline(always)]
    pub const fn hfclk192msrc(self) -> crate::common::Reg<regs::Hfclk192msrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[doc = "Automatic or manual control of HFCLK192M"]
    #[inline(always)]
    pub const fn hfclk192malwaysrun(
        self,
    ) -> crate::common::Reg<regs::Hfclk192malwaysrun, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0584usize) as _) }
    }
    #[doc = "HFCLK192M frequency configuration"]
    #[inline(always)]
    pub const fn hfclk192mctrl(self) -> crate::common::Reg<regs::Hfclk192mctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b8usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfclkaudio {
    ptr: *mut u8,
}
unsafe impl Send for Hfclkaudio {}
unsafe impl Sync for Hfclkaudio {}
impl Hfclkaudio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Audio PLL frequency in 11.176 MHz - 11.402 MHz or 12.165 MHz - 12.411 MHz frequency bands"]
    #[inline(always)]
    pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
