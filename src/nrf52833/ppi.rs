#[doc = "PPI Channel"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch {
    ptr: *mut u8,
}
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Channel n event endpoint"]
    #[inline(always)]
    pub const fn eep(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Channel n task endpoint"]
    #[inline(always)]
    pub const fn tep(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Fork"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fork {
    ptr: *mut u8,
}
unsafe impl Send for Fork {}
unsafe impl Sync for Fork {}
impl Fork {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Channel n task endpoint"]
    #[inline(always)]
    pub const fn tep(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "Programmable Peripheral Interconnect"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppi {
    ptr: *mut u8,
}
unsafe impl Send for Ppi {}
unsafe impl Sync for Ppi {}
impl Ppi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel group tasks"]
    #[inline(always)]
    pub const fn tasks_chg(self, n: usize) -> TasksChg {
        assert!(n < 6usize);
        unsafe { TasksChg::from_ptr(self.ptr.add(0x0usize + n * 8usize) as _) }
    }
    #[doc = "Channel enable register"]
    #[inline(always)]
    pub const fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Channel enable set register"]
    #[inline(always)]
    pub const fn chenset(self) -> crate::common::Reg<regs::Chenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Channel enable clear register"]
    #[inline(always)]
    pub const fn chenclr(self) -> crate::common::Reg<regs::Chenclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "PPI Channel"]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 20usize);
        unsafe { Ch::from_ptr(self.ptr.add(0x0510usize + n * 8usize) as _) }
    }
    #[doc = "Description collection: Channel group n"]
    #[inline(always)]
    pub const fn chg(self, n: usize) -> crate::common::Reg<regs::Chg, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
    }
    #[doc = "Fork"]
    #[inline(always)]
    pub const fn fork(self, n: usize) -> Fork {
        assert!(n < 32usize);
        unsafe { Fork::from_ptr(self.ptr.add(0x0910usize + n * 4usize) as _) }
    }
}
#[doc = "Channel group tasks"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TasksChg {
    ptr: *mut u8,
}
unsafe impl Send for TasksChg {}
unsafe impl Sync for TasksChg {}
impl TasksChg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Enable channel group n"]
    #[inline(always)]
    pub const fn en(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Disable channel group n"]
    #[inline(always)]
    pub const fn dis(self) -> crate::common::Reg<regs::Dis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs;
