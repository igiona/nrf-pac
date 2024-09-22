#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Framestatus {
    ptr: *mut u8,
}
unsafe impl Send for Framestatus {}
unsafe impl Sync for Framestatus {}
impl Framestatus {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Result of last incoming frame"]
    #[inline(always)]
    pub const fn rx(self) -> crate::common::Reg<regs::Rx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "NFC-A compatible radio"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nfct {
    ptr: *mut u8,
}
unsafe impl Send for Nfct {}
unsafe impl Sync for Nfct {}
impl Nfct {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
    #[inline(always)]
    pub const fn tasks_activate(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Disable NFCT peripheral"]
    #[inline(always)]
    pub const fn tasks_disable(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Enable NFC sense field mode, change state to sense mode"]
    #[inline(always)]
    pub const fn tasks_sense(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Start transmission of an outgoing frame, change state to transmit"]
    #[inline(always)]
    pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Initializes the EasyDMA for receive."]
    #[inline(always)]
    pub const fn tasks_enablerxdata(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Force state machine to IDLE state"]
    #[inline(always)]
    pub const fn tasks_goidle(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Force state machine to SLEEP_A state"]
    #[inline(always)]
    pub const fn tasks_gosleep(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "The NFCT peripheral is ready to receive and send frames"]
    #[inline(always)]
    pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Remote NFC field detected"]
    #[inline(always)]
    pub const fn events_fielddetected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Remote NFC field lost"]
    #[inline(always)]
    pub const fn events_fieldlost(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Marks the start of the first symbol of a transmitted frame"]
    #[inline(always)]
    pub const fn events_txframestart(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Marks the end of the last transmitted on-air symbol of a frame"]
    #[inline(always)]
    pub const fn events_txframeend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Marks the end of the first symbol of a received frame"]
    #[inline(always)]
    pub const fn events_rxframestart(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
    #[inline(always)]
    pub const fn events_rxframeend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
    #[inline(always)]
    pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
    #[inline(always)]
    pub const fn events_rxerror(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
    #[inline(always)]
    pub const fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
    #[inline(always)]
    pub const fn events_endtx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "Auto collision resolution process has started"]
    #[inline(always)]
    pub const fn events_autocolresstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "NFC auto collision resolution error reported."]
    #[inline(always)]
    pub const fn events_collision(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "NFC auto collision resolution successfully completed"]
    #[inline(always)]
    pub const fn events_selected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "EasyDMA is ready to receive or send frames."]
    #[inline(always)]
    pub const fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
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
    #[doc = "NFC Error Status register"]
    #[inline(always)]
    pub const fn errorstatus(self) -> crate::common::Reg<regs::Errorstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn framestatus(self) -> Framestatus {
        unsafe { Framestatus::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "NfcTag state register"]
    #[inline(always)]
    pub const fn nfctagstate(self) -> crate::common::Reg<regs::Nfctagstate, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "Sleep state during automatic collision resolution"]
    #[inline(always)]
    pub const fn sleepstate(self) -> crate::common::Reg<regs::Sleepstate, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "Indicates the presence or not of a valid field"]
    #[inline(always)]
    pub const fn fieldpresent(self) -> crate::common::Reg<regs::Fieldpresent, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x043cusize) as _) }
    }
    #[doc = "Minimum frame delay"]
    #[inline(always)]
    pub const fn framedelaymin(self) -> crate::common::Reg<regs::Framedelaymin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Maximum frame delay"]
    #[inline(always)]
    pub const fn framedelaymax(self) -> crate::common::Reg<regs::Framedelaymax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Configuration register for the Frame Delay Timer"]
    #[inline(always)]
    pub const fn framedelaymode(
        self,
    ) -> crate::common::Reg<regs::Framedelaymode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "Packet pointer for TXD and RXD data storage in Data RAM"]
    #[inline(always)]
    pub const fn packetptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each"]
    #[inline(always)]
    pub const fn maxlen(self) -> crate::common::Reg<regs::Maxlen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn txd(self) -> Txd {
        unsafe { Txd::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn rxd(self) -> Rxd {
        unsafe { Rxd::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[doc = "Enables the modulation output to a GPIO pin which can be connected to a second external antenna."]
    #[inline(always)]
    pub const fn modulationctrl(
        self,
    ) -> crate::common::Reg<regs::Modulationctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
    }
    #[doc = "Pin select for Modulation control."]
    #[inline(always)]
    pub const fn modulationpsel(
        self,
    ) -> crate::common::Reg<regs::Modulationpsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
    }
    #[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
    #[inline(always)]
    pub const fn nfcid1_last(self) -> crate::common::Reg<regs::Nfcid1last, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0590usize) as _) }
    }
    #[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
    #[inline(always)]
    pub const fn nfcid1_2nd_last(
        self,
    ) -> crate::common::Reg<regs::Nfcid12ndLast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0594usize) as _) }
    }
    #[doc = "Third last NFCID1 part (10 bytes ID)"]
    #[inline(always)]
    pub const fn nfcid1_3rd_last(
        self,
    ) -> crate::common::Reg<regs::Nfcid13rdLast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0598usize) as _) }
    }
    #[doc = "Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated."]
    #[inline(always)]
    pub const fn autocolresconfig(
        self,
    ) -> crate::common::Reg<regs::Autocolresconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x059cusize) as _) }
    }
    #[doc = "NFC-A SENS_RES auto-response settings"]
    #[inline(always)]
    pub const fn sensres(self) -> crate::common::Reg<regs::Sensres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
    }
    #[doc = "NFC-A SEL_RES auto-response settings"]
    #[inline(always)]
    pub const fn selres(self) -> crate::common::Reg<regs::Selres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a4usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxd {
    ptr: *mut u8,
}
unsafe impl Send for Rxd {}
unsafe impl Sync for Rxd {}
impl Rxd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration of incoming frames"]
    #[inline(always)]
    pub const fn frameconfig(self) -> crate::common::Reg<regs::RxdFrameconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Size of last incoming frame"]
    #[inline(always)]
    pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txd {
    ptr: *mut u8,
}
unsafe impl Send for Txd {}
unsafe impl Sync for Txd {}
impl Txd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration of outgoing frames"]
    #[inline(always)]
    pub const fn frameconfig(self) -> crate::common::Reg<regs::TxdFrameconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Size of outgoing frame"]
    #[inline(always)]
    pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
