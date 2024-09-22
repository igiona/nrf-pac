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
    #[doc = "Pin select for RTS"]
    #[inline(always)]
    pub const fn rts(self) -> crate::common::Reg<regs::Rts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Pin select for TXD"]
    #[inline(always)]
    pub const fn txd(self) -> crate::common::Reg<regs::PselTxd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Pin select for CTS"]
    #[inline(always)]
    pub const fn cts(self) -> crate::common::Reg<regs::Cts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Pin select for RXD"]
    #[inline(always)]
    pub const fn rxd(self) -> crate::common::Reg<regs::PselRxd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart0 {
    ptr: *mut u8,
}
unsafe impl Send for Uart0 {}
unsafe impl Sync for Uart0 {}
impl Uart0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start UART receiver"]
    #[inline(always)]
    pub const fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Stop UART receiver"]
    #[inline(always)]
    pub const fn tasks_stoprx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Start UART transmitter"]
    #[inline(always)]
    pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Stop UART transmitter"]
    #[inline(always)]
    pub const fn tasks_stoptx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Suspend UART"]
    #[inline(always)]
    pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "CTS is activated (set low). Clear To Send."]
    #[inline(always)]
    pub const fn events_cts(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "CTS is deactivated (set high). Not Clear To Send."]
    #[inline(always)]
    pub const fn events_ncts(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Data received in RXD"]
    #[inline(always)]
    pub const fn events_rxdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Data sent from TXD"]
    #[inline(always)]
    pub const fn events_txdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "Error detected"]
    #[inline(always)]
    pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "Receiver timeout"]
    #[inline(always)]
    pub const fn events_rxto(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
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
    #[doc = "Error source"]
    #[inline(always)]
    pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[doc = "Enable UART"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn psel(self) -> Psel {
        unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "RXD register"]
    #[inline(always)]
    pub const fn rxd(self) -> crate::common::Reg<regs::Uart0rxd, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "TXD register"]
    #[inline(always)]
    pub const fn txd(self) -> crate::common::Reg<regs::Uart0txd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub const fn baudrate(self) -> crate::common::Reg<regs::Baudrate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
    }
    #[doc = "Configuration of parity and hardware flow control"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
