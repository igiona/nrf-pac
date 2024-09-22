#[doc = "Distributed programmable peripheral interconnect controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DppicNs {
    ptr: *mut u8,
}
unsafe impl Send for DppicNs {}
unsafe impl Sync for DppicNs {}
impl DppicNs {
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
    #[doc = "Subscribe configuration for tasks"]
    #[inline(always)]
    pub const fn subscribe_chg(self, n: usize) -> SubscribeChg {
        assert!(n < 6usize);
        unsafe { SubscribeChg::from_ptr(self.ptr.add(0x80usize + n * 8usize) as _) }
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
    #[doc = "Description collection: Channel group n Note: Writes to this register are ignored if either SUBSCRIBE_CHG\\[n\\].EN or SUBSCRIBE_CHG\\[n\\].DIS is enabled"]
    #[inline(always)]
    pub const fn chg(self, n: usize) -> crate::common::Reg<regs::Chg, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
    }
}
#[doc = "Subscribe configuration for tasks"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeChg {
    ptr: *mut u8,
}
unsafe impl Send for SubscribeChg {}
unsafe impl Sync for SubscribeChg {}
impl SubscribeChg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Subscribe configuration for task CHG\\[n\\].EN"]
    #[inline(always)]
    pub const fn en(self) -> crate::common::Reg<regs::SubscribeChgEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Subscribe configuration for task CHG\\[n\\].DIS"]
    #[inline(always)]
    pub const fn dis(self) -> crate::common::Reg<regs::SubscribeChgDis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
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
    pub const fn en(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Disable channel group n"]
    #[inline(always)]
    pub const fn dis(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
