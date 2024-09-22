#[doc = "USB Regulator 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbregulatorNs {
    ptr: *mut u8,
}
unsafe impl Send for UsbregulatorNs {}
unsafe impl Sync for UsbregulatorNs {}
impl UsbregulatorNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Voltage supply detected on VBUS"]
    #[inline(always)]
    pub const fn events_usbdetected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Voltage supply removed from VBUS"]
    #[inline(always)]
    pub const fn events_usbremoved(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "USB 3.3 V supply ready"]
    #[inline(always)]
    pub const fn events_usbpwrrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Publish configuration for event USBDETECTED"]
    #[inline(always)]
    pub const fn publish_usbdetected(
        self,
    ) -> crate::common::Reg<regs::PublishUsbdetected, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Publish configuration for event USBREMOVED"]
    #[inline(always)]
    pub const fn publish_usbremoved(
        self,
    ) -> crate::common::Reg<regs::PublishUsbremoved, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Publish configuration for event USBPWRRDY"]
    #[inline(always)]
    pub const fn publish_usbpwrrdy(
        self,
    ) -> crate::common::Reg<regs::PublishUsbpwrrdy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
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
    #[doc = "USB supply status"]
    #[inline(always)]
    pub const fn usbregstatus(self) -> crate::common::Reg<regs::Usbregstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
}
pub mod regs;
pub mod vals;
