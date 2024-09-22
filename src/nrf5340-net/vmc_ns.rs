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
    #[doc = "Description cluster: RAM\\[n\\] power control register"]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: RAM\\[n\\] power control set register"]
    #[inline(always)]
    pub const fn powerset(self) -> crate::common::Reg<regs::Powerset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description cluster: RAM\\[n\\] power control clear register"]
    #[inline(always)]
    pub const fn powerclr(self) -> crate::common::Reg<regs::Powerclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Volatile Memory controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VmcNs {
    ptr: *mut u8,
}
unsafe impl Send for VmcNs {}
unsafe impl Sync for VmcNs {}
impl VmcNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn ram(self, n: usize) -> Ram {
        assert!(n < 4usize);
        unsafe { Ram::from_ptr(self.ptr.add(0x0600usize + n * 16usize) as _) }
    }
}
pub mod regs;
pub mod vals;
