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
    #[doc = "Pin select for SCL signal"]
    #[inline(always)]
    pub const fn scl(self) -> crate::common::Reg<regs::Scl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Pin select for SDA signal"]
    #[inline(always)]
    pub const fn sda(self) -> crate::common::Reg<regs::Sda, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "RXD EasyDMA channel"]
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
    #[doc = "Data pointer"]
    #[inline(always)]
    pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Maximum number of bytes in receive buffer"]
    #[inline(always)]
    pub const fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "EasyDMA list type"]
    #[inline(always)]
    pub const fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Twim0 {
    ptr: *mut u8,
}
unsafe impl Send for Twim0 {}
unsafe impl Sync for Twim0 {}
impl Twim0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start TWI receive sequence"]
    #[inline(always)]
    pub const fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Start TWI transmit sequence"]
    #[inline(always)]
    pub const fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Suspend TWI transaction"]
    #[inline(always)]
    pub const fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Resume TWI transaction"]
    #[inline(always)]
    pub const fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "TWI stopped"]
    #[inline(always)]
    pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "TWI error"]
    #[inline(always)]
    pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "SUSPEND task has been issued, TWI traffic is now suspended."]
    #[inline(always)]
    pub const fn events_suspended(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "Receive sequence started"]
    #[inline(always)]
    pub const fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "Transmit sequence started"]
    #[inline(always)]
    pub const fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "Byte boundary, starting to receive the last byte"]
    #[inline(always)]
    pub const fn events_lastrx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "Byte boundary, starting to transmit the last byte"]
    #[inline(always)]
    pub const fn events_lasttx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
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
    #[doc = "Error source"]
    #[inline(always)]
    pub const fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c4usize) as _) }
    }
    #[doc = "Enable TWIM"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn psel(self) -> Psel {
        unsafe { Psel::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
    }
    #[doc = "RXD EasyDMA channel"]
    #[inline(always)]
    pub const fn rxd(self) -> Rxd {
        unsafe { Rxd::from_ptr(self.ptr.add(0x0534usize) as _) }
    }
    #[doc = "TXD EasyDMA channel"]
    #[inline(always)]
    pub const fn txd(self) -> Txd {
        unsafe { Txd::from_ptr(self.ptr.add(0x0544usize) as _) }
    }
    #[doc = "Address used in the TWI transfer"]
    #[inline(always)]
    pub const fn address(self) -> crate::common::Reg<regs::Address, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
    }
}
#[doc = "TXD EasyDMA channel"]
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
    #[doc = "Data pointer"]
    #[inline(always)]
    pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Maximum number of bytes in transmit buffer"]
    #[inline(always)]
    pub const fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "EasyDMA list type"]
    #[inline(always)]
    pub const fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
