#[doc = "Voltage regulators 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegulatorsNs {
    ptr: *mut u8,
}
unsafe impl Send for RegulatorsNs {}
unsafe impl Sync for RegulatorsNs {}
impl RegulatorsNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Main supply status"]
    #[inline(always)]
    pub const fn mainregstatus(self) -> crate::common::Reg<regs::Mainregstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0428usize) as _) }
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
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn vregmain(self) -> Vregmain {
        unsafe { Vregmain::from_ptr(self.ptr.add(0x0704usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn vregradio(self) -> Vregradio {
        unsafe { Vregradio::from_ptr(self.ptr.add(0x0900usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn vregh(self) -> Vregh {
        unsafe { Vregh::from_ptr(self.ptr.add(0x0b00usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vregh {
    ptr: *mut u8,
}
unsafe impl Send for Vregh {}
unsafe impl Sync for Vregh {}
impl Vregh {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DC/DC enable register for VREGH"]
    #[inline(always)]
    pub const fn dcdcen(self) -> crate::common::Reg<regs::VreghDcdcen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vregmain {
    ptr: *mut u8,
}
unsafe impl Send for Vregmain {}
unsafe impl Sync for Vregmain {}
impl Vregmain {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DC/DC enable register for VREGMAIN"]
    #[inline(always)]
    pub const fn dcdcen(self) -> crate::common::Reg<regs::VregmainDcdcen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vregradio {
    ptr: *mut u8,
}
unsafe impl Send for Vregradio {}
unsafe impl Sync for Vregradio {}
impl Vregradio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DC/DC enable register for VREGRADIO"]
    #[inline(always)]
    pub const fn dcdcen(self) -> crate::common::Reg<regs::VregradioDcdcen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
