#[doc = "Power control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Power {
    ptr: *mut u8,
}
unsafe impl Send for Power {}
unsafe impl Sync for Power {}
impl Power {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Enable Constant Latency mode"]
    #[inline(always)]
    pub const fn tasks_constlat(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Enable Low-power mode (variable latency)"]
    #[inline(always)]
    pub const fn tasks_lowpwr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "Power failure warning"]
    #[inline(always)]
    pub const fn events_pofwarn(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "CPU entered WFI/WFE sleep"]
    #[inline(always)]
    pub const fn events_sleepenter(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "CPU exited WFI/WFE sleep"]
    #[inline(always)]
    pub const fn events_sleepexit(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "Voltage supply detected on VBUS"]
    #[inline(always)]
    pub const fn events_usbdetected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "Voltage supply removed from VBUS"]
    #[inline(always)]
    pub const fn events_usbremoved(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "USB 3.3 V supply ready"]
    #[inline(always)]
    pub const fn events_usbpwrrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
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
    #[doc = "Reset reason"]
    #[inline(always)]
    pub const fn resetreas(self) -> crate::common::Reg<regs::Resetreas, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Deprecated register - RAM status register"]
    #[inline(always)]
    pub const fn ramstatus(self) -> crate::common::Reg<regs::Ramstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0428usize) as _) }
    }
    #[doc = "USB supply status"]
    #[inline(always)]
    pub const fn usbregstatus(self) -> crate::common::Reg<regs::Usbregstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0438usize) as _) }
    }
    #[doc = "System OFF register"]
    #[inline(always)]
    pub const fn systemoff(self) -> crate::common::Reg<regs::Systemoff, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Power-fail comparator configuration"]
    #[inline(always)]
    pub const fn pofcon(self) -> crate::common::Reg<regs::Pofcon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "General purpose retention register"]
    #[inline(always)]
    pub const fn gpregret(self) -> crate::common::Reg<regs::Gpregret, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "General purpose retention register"]
    #[inline(always)]
    pub const fn gpregret2(self) -> crate::common::Reg<regs::Gpregret2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[doc = "Enable DC/DC converter for REG1 stage"]
    #[inline(always)]
    pub const fn dcdcen(self) -> crate::common::Reg<regs::Dcdcen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0578usize) as _) }
    }
    #[doc = "Enable DC/DC converter for REG0 stage"]
    #[inline(always)]
    pub const fn dcdcen0(self) -> crate::common::Reg<regs::Dcdcen0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[doc = "Main supply status"]
    #[inline(always)]
    pub const fn mainregstatus(self) -> crate::common::Reg<regs::Mainregstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn ram(self, n: usize) -> Ram {
        assert!(n < 9usize);
        unsafe { Ram::from_ptr(self.ptr.add(0x0900usize + n * 16usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram {
    ptr: *mut u8,
}
unsafe impl Send for Ram {}
unsafe impl Sync for Ram {}
impl Ram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: RAMn power control register"]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: RAMn power control set register"]
    #[inline(always)]
    pub const fn powerset(self) -> crate::common::Reg<regs::Powerset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description cluster: RAMn power control clear register"]
    #[inline(always)]
    pub const fn powerclr(self) -> crate::common::Reg<regs::Powerclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs;
pub mod vals;
