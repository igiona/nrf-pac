#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epin {
    ptr: *mut u8,
}
unsafe impl Send for Epin {}
unsafe impl Sync for Epin {}
impl Epin {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Data pointer"]
    #[inline(always)]
    pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Maximum number of bytes to transfer"]
    #[inline(always)]
    pub const fn maxcnt(self) -> crate::common::Reg<regs::EpinMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description cluster: Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(self) -> crate::common::Reg<regs::EpinAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epout {
    ptr: *mut u8,
}
unsafe impl Send for Epout {}
unsafe impl Sync for Epout {}
impl Epout {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Data pointer"]
    #[inline(always)]
    pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Maximum number of bytes to transfer"]
    #[inline(always)]
    pub const fn maxcnt(self) -> crate::common::Reg<regs::EpoutMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description cluster: Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(self) -> crate::common::Reg<regs::EpoutAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Halted {
    ptr: *mut u8,
}
unsafe impl Send for Halted {}
unsafe impl Sync for Halted {}
impl Halted {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description collection: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub const fn epin(self, n: usize) -> crate::common::Reg<regs::Epin, crate::common::R> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub const fn epout(self, n: usize) -> crate::common::Reg<regs::HaltedEpout, crate::common::R> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize + n * 4usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isoin {
    ptr: *mut u8,
}
unsafe impl Send for Isoin {}
unsafe impl Sync for Isoin {}
impl Isoin {
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
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub const fn maxcnt(self) -> crate::common::Reg<regs::IsoinMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(self) -> crate::common::Reg<regs::IsoinAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isoout {
    ptr: *mut u8,
}
unsafe impl Send for Isoout {}
unsafe impl Sync for Isoout {}
impl Isoout {
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
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub const fn maxcnt(self) -> crate::common::Reg<regs::IsooutMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(self) -> crate::common::Reg<regs::IsooutAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Size {
    ptr: *mut u8,
}
unsafe impl Send for Size {}
unsafe impl Sync for Size {}
impl Size {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description collection: Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub const fn epout(self, n: usize) -> crate::common::Reg<regs::SizeEpout, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Number of bytes received last on this ISO OUT data endpoint"]
    #[inline(always)]
    pub const fn isoout(self) -> crate::common::Reg<regs::Isoout, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
#[doc = "Universal serial bus device 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbdNs {
    ptr: *mut u8,
}
unsafe impl Send for UsbdNs {}
unsafe impl Sync for UsbdNs {}
impl UsbdNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description collection: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
    #[inline(always)]
    pub const fn tasks_startepin(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
    #[inline(always)]
    pub const fn tasks_startisoin(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Description collection: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
    #[inline(always)]
    pub const fn tasks_startepout(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize + n * 4usize) as _) }
    }
    #[doc = "Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
    #[inline(always)]
    pub const fn tasks_startisoout(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Allows OUT data stage on control endpoint 0"]
    #[inline(always)]
    pub const fn tasks_ep0rcvout(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Allows status stage on control endpoint 0"]
    #[inline(always)]
    pub const fn tasks_ep0status(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Stalls data and status stage on control endpoint 0"]
    #[inline(always)]
    pub const fn tasks_ep0stall(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
    #[inline(always)]
    pub const fn tasks_dpdmdrive(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Stops forcing D+ and D- lines into any state (USB engine takes control)"]
    #[inline(always)]
    pub const fn tasks_dpdmnodrive(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Description collection: Subscribe configuration for task STARTEPIN\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_startepin(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SubscribeStartepin, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize + n * 4usize) as _) }
    }
    #[doc = "Subscribe configuration for task STARTISOIN"]
    #[inline(always)]
    pub const fn subscribe_startisoin(
        self,
    ) -> crate::common::Reg<regs::SubscribeStartisoin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Description collection: Subscribe configuration for task STARTEPOUT\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_startepout(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SubscribeStartepout, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize + n * 4usize) as _) }
    }
    #[doc = "Subscribe configuration for task STARTISOOUT"]
    #[inline(always)]
    pub const fn subscribe_startisoout(
        self,
    ) -> crate::common::Reg<regs::SubscribeStartisoout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "Subscribe configuration for task EP0RCVOUT"]
    #[inline(always)]
    pub const fn subscribe_ep0rcvout(
        self,
    ) -> crate::common::Reg<regs::SubscribeEp0rcvout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "Subscribe configuration for task EP0STATUS"]
    #[inline(always)]
    pub const fn subscribe_ep0status(
        self,
    ) -> crate::common::Reg<regs::SubscribeEp0status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "Subscribe configuration for task EP0STALL"]
    #[inline(always)]
    pub const fn subscribe_ep0stall(
        self,
    ) -> crate::common::Reg<regs::SubscribeEp0stall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "Subscribe configuration for task DPDMDRIVE"]
    #[inline(always)]
    pub const fn subscribe_dpdmdrive(
        self,
    ) -> crate::common::Reg<regs::SubscribeDpdmdrive, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "Subscribe configuration for task DPDMNODRIVE"]
    #[inline(always)]
    pub const fn subscribe_dpdmnodrive(
        self,
    ) -> crate::common::Reg<regs::SubscribeDpdmnodrive, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "Signals that a USB reset condition has been detected on USB lines"]
    #[inline(always)]
    pub const fn events_usbreset(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Confirms that the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT, or EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register"]
    #[inline(always)]
    pub const fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Description collection: The whole EPIN\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub const fn events_endepin(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize + n * 4usize) as _) }
    }
    #[doc = "An acknowledged data transfer has taken place on the control endpoint"]
    #[inline(always)]
    pub const fn events_ep0datadone(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "The whole ISOIN buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub const fn events_endisoin(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "Description collection: The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub const fn events_endepout(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize + n * 4usize) as _) }
    }
    #[doc = "The whole ISOOUT buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub const fn events_endisoout(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "Signals that a SOF (start of frame) condition has been detected on USB lines"]
    #[inline(always)]
    pub const fn events_sof(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
    #[inline(always)]
    pub const fn events_usbevent(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "A valid SETUP token has been received (and acknowledged) on the control endpoint"]
    #[inline(always)]
    pub const fn events_ep0setup(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
    #[inline(always)]
    pub const fn events_epdata(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "Publish configuration for event USBRESET"]
    #[inline(always)]
    pub const fn publish_usbreset(
        self,
    ) -> crate::common::Reg<regs::PublishUsbreset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Publish configuration for event STARTED"]
    #[inline(always)]
    pub const fn publish_started(
        self,
    ) -> crate::common::Reg<regs::PublishStarted, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Description collection: Publish configuration for event ENDEPIN\\[n\\]"]
    #[inline(always)]
    pub const fn publish_endepin(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PublishEndepin, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize + n * 4usize) as _) }
    }
    #[doc = "Publish configuration for event EP0DATADONE"]
    #[inline(always)]
    pub const fn publish_ep0datadone(
        self,
    ) -> crate::common::Reg<regs::PublishEp0datadone, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[doc = "Publish configuration for event ENDISOIN"]
    #[inline(always)]
    pub const fn publish_endisoin(
        self,
    ) -> crate::common::Reg<regs::PublishEndisoin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
    #[doc = "Description collection: Publish configuration for event ENDEPOUT\\[n\\]"]
    #[inline(always)]
    pub const fn publish_endepout(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PublishEndepout, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize + n * 4usize) as _) }
    }
    #[doc = "Publish configuration for event ENDISOOUT"]
    #[inline(always)]
    pub const fn publish_endisoout(
        self,
    ) -> crate::common::Reg<regs::PublishEndisoout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[doc = "Publish configuration for event SOF"]
    #[inline(always)]
    pub const fn publish_sof(self) -> crate::common::Reg<regs::PublishSof, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d4usize) as _) }
    }
    #[doc = "Publish configuration for event USBEVENT"]
    #[inline(always)]
    pub const fn publish_usbevent(
        self,
    ) -> crate::common::Reg<regs::PublishUsbevent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d8usize) as _) }
    }
    #[doc = "Publish configuration for event EP0SETUP"]
    #[inline(always)]
    pub const fn publish_ep0setup(
        self,
    ) -> crate::common::Reg<regs::PublishEp0setup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01dcusize) as _) }
    }
    #[doc = "Publish configuration for event EPDATA"]
    #[inline(always)]
    pub const fn publish_epdata(
        self,
    ) -> crate::common::Reg<regs::PublishEpdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
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
    #[doc = "Details on what caused the USBEVENT event"]
    #[inline(always)]
    pub const fn eventcause(self) -> crate::common::Reg<regs::Eventcause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn halted(self) -> Halted {
        unsafe { Halted::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "Provides information on which endpoint's EasyDMA registers have been captured"]
    #[inline(always)]
    pub const fn epstatus(self) -> crate::common::Reg<regs::Epstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0468usize) as _) }
    }
    #[doc = "Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
    #[inline(always)]
    pub const fn epdatastatus(self) -> crate::common::Reg<regs::Epdatastatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x046cusize) as _) }
    }
    #[doc = "Device USB address"]
    #[inline(always)]
    pub const fn usbaddr(self) -> crate::common::Reg<regs::Usbaddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0470usize) as _) }
    }
    #[doc = "SETUP data, byte 0, bmRequestType"]
    #[inline(always)]
    pub const fn bmrequesttype(self) -> crate::common::Reg<regs::Bmrequesttype, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[doc = "SETUP data, byte 1, bRequest"]
    #[inline(always)]
    pub const fn brequest(self) -> crate::common::Reg<regs::Brequest, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0484usize) as _) }
    }
    #[doc = "SETUP data, byte 2, LSB of wValue"]
    #[inline(always)]
    pub const fn wvaluel(self) -> crate::common::Reg<regs::Wvaluel, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0488usize) as _) }
    }
    #[doc = "SETUP data, byte 3, MSB of wValue"]
    #[inline(always)]
    pub const fn wvalueh(self) -> crate::common::Reg<regs::Wvalueh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x048cusize) as _) }
    }
    #[doc = "SETUP data, byte 4, LSB of wIndex"]
    #[inline(always)]
    pub const fn windexl(self) -> crate::common::Reg<regs::Windexl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0490usize) as _) }
    }
    #[doc = "SETUP data, byte 5, MSB of wIndex"]
    #[inline(always)]
    pub const fn windexh(self) -> crate::common::Reg<regs::Windexh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0494usize) as _) }
    }
    #[doc = "SETUP data, byte 6, LSB of wLength"]
    #[inline(always)]
    pub const fn wlengthl(self) -> crate::common::Reg<regs::Wlengthl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0498usize) as _) }
    }
    #[doc = "SETUP data, byte 7, MSB of wLength"]
    #[inline(always)]
    pub const fn wlengthh(self) -> crate::common::Reg<regs::Wlengthh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x049cusize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn size(self) -> Size {
        unsafe { Size::from_ptr(self.ptr.add(0x04a0usize) as _) }
    }
    #[doc = "Enable USB"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Control of the USB pull-up"]
    #[inline(always)]
    pub const fn usbpullup(self) -> crate::common::Reg<regs::Usbpullup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
    #[inline(always)]
    pub const fn dpdmvalue(self) -> crate::common::Reg<regs::Dpdmvalue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Data toggle control and status"]
    #[inline(always)]
    pub const fn dtoggle(self) -> crate::common::Reg<regs::Dtoggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "Endpoint IN enable"]
    #[inline(always)]
    pub const fn epinen(self) -> crate::common::Reg<regs::Epinen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Endpoint OUT enable"]
    #[inline(always)]
    pub const fn epouten(self) -> crate::common::Reg<regs::Epouten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "STALL endpoints"]
    #[inline(always)]
    pub const fn epstall(self) -> crate::common::Reg<regs::Epstall, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "Controls the split of ISO buffers"]
    #[inline(always)]
    pub const fn isosplit(self) -> crate::common::Reg<regs::Isosplit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "Returns the current value of the start of frame counter"]
    #[inline(always)]
    pub const fn framecntr(self) -> crate::common::Reg<regs::Framecntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[doc = "Controls USBD peripheral low power mode during USB suspend"]
    #[inline(always)]
    pub const fn lowpower(self) -> crate::common::Reg<regs::Lowpower, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
    }
    #[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    #[inline(always)]
    pub const fn isoinconfig(self) -> crate::common::Reg<regs::Isoinconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn epin(self, n: usize) -> Epin {
        assert!(n < 8usize);
        unsafe { Epin::from_ptr(self.ptr.add(0x0600usize + n * 20usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn isoin(self) -> Isoin {
        unsafe { Isoin::from_ptr(self.ptr.add(0x06a0usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn epout(self, n: usize) -> Epout {
        assert!(n < 8usize);
        unsafe { Epout::from_ptr(self.ptr.add(0x0700usize + n * 20usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn isoout(self) -> Isoout {
        unsafe { Isoout::from_ptr(self.ptr.add(0x07a0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
