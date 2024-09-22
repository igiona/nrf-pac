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
    #[doc = "Pin select for SCL"]
    #[inline(always)]
    pub const fn scl(self) -> crate::common::Reg<regs::Scl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Pin select for SDA"]
    #[inline(always)]
    pub const fn sda(self) -> crate::common::Reg<regs::Sda, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "I2C compatible Two-Wire Interface 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Twi0 {
    ptr: *mut u8,
}
unsafe impl Send for Twi0 {}
unsafe impl Sync for Twi0 {}
impl Twi0 {
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
    #[doc = "Stop TWI transaction"]
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
    #[doc = "TWI RXD byte received"]
    #[inline(always)]
    pub const fn events_rxdready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "TWI TXD byte sent"]
    #[inline(always)]
    pub const fn events_txdsent(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "TWI error"]
    #[inline(always)]
    pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "TWI byte boundary, generated before each byte that is sent or received"]
    #[inline(always)]
    pub const fn events_bb(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "TWI entered the suspended state"]
    #[inline(always)]
    pub const fn events_suspended(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c4usize) as _) }
    }
    #[doc = "Enable TWI"]
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
    pub const fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "TXD register"]
    #[inline(always)]
    pub const fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
    }
    #[doc = "Address used in the TWI transfer"]
    #[inline(always)]
    pub const fn address(self) -> crate::common::Reg<regs::Address, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
    }
}
pub mod regs;
pub mod vals;
