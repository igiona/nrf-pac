#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config {
    ptr: *mut u8,
}
unsafe impl Send for Config {}
unsafe impl Sync for Config {}
impl Config {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "I2S mode"]
    #[inline(always)]
    pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Reception (RX) enable"]
    #[inline(always)]
    pub const fn rxen(self) -> crate::common::Reg<regs::Rxen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Transmission (TX) enable"]
    #[inline(always)]
    pub const fn txen(self) -> crate::common::Reg<regs::Txen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Master clock generator enable"]
    #[inline(always)]
    pub const fn mcken(self) -> crate::common::Reg<regs::Mcken, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "I2S clock generator control"]
    #[inline(always)]
    pub const fn mckfreq(self) -> crate::common::Reg<regs::Mckfreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "MCK / LRCK ratio"]
    #[inline(always)]
    pub const fn ratio(self) -> crate::common::Reg<regs::Ratio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Sample width"]
    #[inline(always)]
    pub const fn swidth(self) -> crate::common::Reg<regs::Swidth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Alignment of sample within a frame"]
    #[inline(always)]
    pub const fn align(self) -> crate::common::Reg<regs::Align, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Frame format"]
    #[inline(always)]
    pub const fn format(self) -> crate::common::Reg<regs::Format, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Enable channels"]
    #[inline(always)]
    pub const fn channels(self) -> crate::common::Reg<regs::Channels, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Clock source selection for the I2S module"]
    #[inline(always)]
    pub const fn clkconfig(self) -> crate::common::Reg<regs::Clkconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
#[doc = "Inter-IC Sound 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2s0ns {
    ptr: *mut u8,
}
unsafe impl Send for I2s0ns {}
unsafe impl Sync for I2s0ns {}
impl I2s0ns {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled"]
    #[inline(always)]
    pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Stops I2S transfer and MCK generator. Triggering this task will cause the event STOPPED to be generated."]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Subscribe configuration for task START"]
    #[inline(always)]
    pub const fn subscribe_start(
        self,
    ) -> crate::common::Reg<regs::SubscribeStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(
        self,
    ) -> crate::common::Reg<regs::SubscribeStop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words received on the SDIN pin."]
    #[inline(always)]
    pub const fn events_rxptrupd(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "I2S transfer stopped."]
    #[inline(always)]
    pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    #[inline(always)]
    pub const fn events_txptrupd(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "Frame start event, generated on the active edge of LRCK"]
    #[inline(always)]
    pub const fn events_framestart(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "Publish configuration for event RXPTRUPD"]
    #[inline(always)]
    pub const fn publish_rxptrupd(
        self,
    ) -> crate::common::Reg<regs::PublishRxptrupd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Publish configuration for event STOPPED"]
    #[inline(always)]
    pub const fn publish_stopped(
        self,
    ) -> crate::common::Reg<regs::PublishStopped, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "Publish configuration for event TXPTRUPD"]
    #[inline(always)]
    pub const fn publish_txptrupd(
        self,
    ) -> crate::common::Reg<regs::PublishTxptrupd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[doc = "Publish configuration for event FRAMESTART"]
    #[inline(always)]
    pub const fn publish_framestart(
        self,
    ) -> crate::common::Reg<regs::PublishFramestart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
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
    #[doc = "Enable I2S module"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn config(self) -> Config {
        unsafe { Config::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn rxd(self) -> Rxd {
        unsafe { Rxd::from_ptr(self.ptr.add(0x0538usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn txd(self) -> Txd {
        unsafe { Txd::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn rxtxd(self) -> Rxtxd {
        unsafe { Rxtxd::from_ptr(self.ptr.add(0x0550usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn psel(self) -> Psel {
        unsafe { Psel::from_ptr(self.ptr.add(0x0560usize) as _) }
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
    #[doc = "Pin select for MCK signal"]
    #[inline(always)]
    pub const fn mck(self) -> crate::common::Reg<regs::Mck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Pin select for SCK signal"]
    #[inline(always)]
    pub const fn sck(self) -> crate::common::Reg<regs::Sck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Pin select for LRCK signal"]
    #[inline(always)]
    pub const fn lrck(self) -> crate::common::Reg<regs::Lrck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Pin select for SDIN signal"]
    #[inline(always)]
    pub const fn sdin(self) -> crate::common::Reg<regs::Sdin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Pin select for SDOUT signal"]
    #[inline(always)]
    pub const fn sdout(self) -> crate::common::Reg<regs::Sdout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
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
    #[doc = "Receive buffer RAM start address."]
    #[inline(always)]
    pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxtxd {
    ptr: *mut u8,
}
unsafe impl Send for Rxtxd {}
unsafe impl Sync for Rxtxd {}
impl Rxtxd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Size of RXD and TXD buffers"]
    #[inline(always)]
    pub const fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
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
    #[doc = "Transmit buffer RAM start address"]
    #[inline(always)]
    pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
