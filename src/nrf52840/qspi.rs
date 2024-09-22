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
    #[doc = "Pin select for serial data IO2."]
    #[inline(always)]
    pub const fn io2(self) -> crate::common::Reg<regs::Io2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Pin select for serial data IO3."]
    #[inline(always)]
    pub const fn io3(self) -> crate::common::Reg<regs::Io3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
#[doc = "External flash interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi {
    ptr: *mut u8,
}
unsafe impl Send for Qspi {}
unsafe impl Sync for Qspi {}
impl Qspi {
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
    #[doc = "QSPI peripheral is ready. This event will be generated as a response to any QSPI task."]
    #[inline(always)]
    pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
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
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
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
pub mod regs;
pub mod vals;
