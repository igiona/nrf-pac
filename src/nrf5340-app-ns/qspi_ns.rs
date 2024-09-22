#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaEnc {
    ptr: *mut u8,
}
unsafe impl Send for DmaEnc {}
unsafe impl Sync for DmaEnc {}
impl DmaEnc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bits 31:0 of DMA AES KEY"]
    #[inline(always)]
    pub const fn key0(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Bits 63:32 of DMA AES KEY"]
    #[inline(always)]
    pub const fn key1(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Bits 95:64 of DMA AES KEY"]
    #[inline(always)]
    pub const fn key2(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Bits 127:96 of DMA AES KEY"]
    #[inline(always)]
    pub const fn key3(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Bits 31:0 of DMA NONCE"]
    #[inline(always)]
    pub const fn nonce0(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Bits 63:32 of DMA NONCE"]
    #[inline(always)]
    pub const fn nonce1(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Bits 95:64 of DMA NONCE"]
    #[inline(always)]
    pub const fn nonce2(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Enable stream cipher for EasyDMA"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::DmaEncEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erase {
    ptr: *mut u8,
}
unsafe impl Send for Erase {}
unsafe impl Sync for Erase {}
impl Erase {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start address of flash block to be erased"]
    #[inline(always)]
    pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Size of block to be erased."]
    #[inline(always)]
    pub const fn len(self) -> crate::common::Reg<regs::Len, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
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
    #[doc = "Pin select for serial clock SCK"]
    #[inline(always)]
    pub const fn sck(self) -> crate::common::Reg<regs::Sck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Pin select for chip select signal CSN."]
    #[inline(always)]
    pub const fn csn(self) -> crate::common::Reg<regs::Csn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Pin select for serial data MOSI/IO0."]
    #[inline(always)]
    pub const fn io0(self) -> crate::common::Reg<regs::Io0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Pin select for serial data MISO/IO1."]
    #[inline(always)]
    pub const fn io1(self) -> crate::common::Reg<regs::Io1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Pin select for serial data WP/IO2."]
    #[inline(always)]
    pub const fn io2(self) -> crate::common::Reg<regs::Io2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Pin select for serial data HOLD/IO3."]
    #[inline(always)]
    pub const fn io3(self) -> crate::common::Reg<regs::Io3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
#[doc = "External flash interface 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspiNs {
    ptr: *mut u8,
}
unsafe impl Send for QspiNs {}
unsafe impl Sync for QspiNs {}
impl QspiNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Activate QSPI interface"]
    #[inline(always)]
    pub const fn tasks_activate(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Start transfer from external flash memory to internal RAM"]
    #[inline(always)]
    pub const fn tasks_readstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Start transfer from internal RAM to external flash memory"]
    #[inline(always)]
    pub const fn tasks_writestart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Start external flash memory erase operation"]
    #[inline(always)]
    pub const fn tasks_erasestart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Deactivate QSPI interface"]
    #[inline(always)]
    pub const fn tasks_deactivate(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Subscribe configuration for task ACTIVATE"]
    #[inline(always)]
    pub const fn subscribe_activate(
        self,
    ) -> crate::common::Reg<regs::SubscribeActivate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Subscribe configuration for task READSTART"]
    #[inline(always)]
    pub const fn subscribe_readstart(
        self,
    ) -> crate::common::Reg<regs::SubscribeReadstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Subscribe configuration for task WRITESTART"]
    #[inline(always)]
    pub const fn subscribe_writestart(
        self,
    ) -> crate::common::Reg<regs::SubscribeWritestart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Subscribe configuration for task ERASESTART"]
    #[inline(always)]
    pub const fn subscribe_erasestart(
        self,
    ) -> crate::common::Reg<regs::SubscribeErasestart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Subscribe configuration for task DEACTIVATE"]
    #[inline(always)]
    pub const fn subscribe_deactivate(
        self,
    ) -> crate::common::Reg<regs::SubscribeDeactivate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "QSPI peripheral is ready. This event will be generated as a response to all QSPI tasks except DEACTIVATE."]
    #[inline(always)]
    pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Publish configuration for event READY"]
    #[inline(always)]
    pub const fn publish_ready(self) -> crate::common::Reg<regs::PublishReady, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
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
    #[doc = "Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::QspiNsEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn read(self) -> Read {
        unsafe { Read::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn write(self) -> Write {
        unsafe { Write::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn erase(self) -> Erase {
        unsafe { Erase::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn psel(self) -> Psel {
        unsafe { Psel::from_ptr(self.ptr.add(0x0524usize) as _) }
    }
    #[doc = "Address offset into the external memory for Execute in Place operation."]
    #[inline(always)]
    pub const fn xipoffset(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "Interface configuration."]
    #[inline(always)]
    pub const fn ifconfig0(self) -> crate::common::Reg<regs::Ifconfig0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
    }
    #[doc = "Enable Execute in Place operation."]
    #[inline(always)]
    pub const fn xipen(self) -> crate::common::Reg<regs::Xipen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn xip_enc(self) -> XipEnc {
        unsafe { XipEnc::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn dma_enc(self) -> DmaEnc {
        unsafe { DmaEnc::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[doc = "Interface configuration."]
    #[inline(always)]
    pub const fn ifconfig1(self) -> crate::common::Reg<regs::Ifconfig1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[doc = "Set the duration required to enter/exit deep power-down mode (DPM)."]
    #[inline(always)]
    pub const fn dpmdur(self) -> crate::common::Reg<regs::Dpmdur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0614usize) as _) }
    }
    #[doc = "Extended address configuration."]
    #[inline(always)]
    pub const fn addrconf(self) -> crate::common::Reg<regs::Addrconf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0624usize) as _) }
    }
    #[doc = "Custom instruction configuration register."]
    #[inline(always)]
    pub const fn cinstrconf(self) -> crate::common::Reg<regs::Cinstrconf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0634usize) as _) }
    }
    #[doc = "Custom instruction data register 0."]
    #[inline(always)]
    pub const fn cinstrdat0(self) -> crate::common::Reg<regs::Cinstrdat0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0638usize) as _) }
    }
    #[doc = "Custom instruction data register 1."]
    #[inline(always)]
    pub const fn cinstrdat1(self) -> crate::common::Reg<regs::Cinstrdat1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x063cusize) as _) }
    }
    #[doc = "SPI interface timing."]
    #[inline(always)]
    pub const fn iftiming(self) -> crate::common::Reg<regs::Iftiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Read {
    ptr: *mut u8,
}
unsafe impl Send for Read {}
unsafe impl Sync for Read {}
impl Read {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash memory source address"]
    #[inline(always)]
    pub const fn src(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RAM destination address"]
    #[inline(always)]
    pub const fn dst(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Read transfer length"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::ReadCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Write {
    ptr: *mut u8,
}
unsafe impl Send for Write {}
unsafe impl Sync for Write {}
impl Write {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash destination address"]
    #[inline(always)]
    pub const fn dst(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RAM source address"]
    #[inline(always)]
    pub const fn src(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Write transfer length"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::WriteCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XipEnc {
    ptr: *mut u8,
}
unsafe impl Send for XipEnc {}
unsafe impl Sync for XipEnc {}
impl XipEnc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bits 31:0 of XIP AES KEY"]
    #[inline(always)]
    pub const fn key0(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Bits 63:32 of XIP AES KEY"]
    #[inline(always)]
    pub const fn key1(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Bits 95:64 of XIP AES KEY"]
    #[inline(always)]
    pub const fn key2(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Bits 127:96 of XIP AES KEY"]
    #[inline(always)]
    pub const fn key3(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Bits 31:0 of XIP NONCE"]
    #[inline(always)]
    pub const fn nonce0(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Bits 63:32 of XIP NONCE"]
    #[inline(always)]
    pub const fn nonce1(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Bits 95:64 of XIP NONCE"]
    #[inline(always)]
    pub const fn nonce2(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Enable stream cipher for XIP"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::XipEncEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
