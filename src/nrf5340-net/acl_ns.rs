#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acl {
    ptr: *mut u8,
}
unsafe impl Send for Acl {}
unsafe impl Sync for Acl {}
impl Acl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Start address of region to protect. The start address must be word-aligned."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Writing a '0' has no effect."]
    #[inline(always)]
    pub const fn size(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
    #[inline(always)]
    pub const fn perm(self) -> crate::common::Reg<regs::Perm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Access control lists"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AclNs {
    ptr: *mut u8,
}
unsafe impl Send for AclNs {}
unsafe impl Sync for AclNs {}
impl AclNs {
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
    pub const fn acl(self, n: usize) -> Acl {
        assert!(n < 8usize);
        unsafe { Acl::from_ptr(self.ptr.add(0x0800usize + n * 16usize) as _) }
    }
}
pub mod regs;
pub mod vals;
