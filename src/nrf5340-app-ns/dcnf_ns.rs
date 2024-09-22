#[doc = "Domain configuration management 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcnfNs {
    ptr: *mut u8,
}
unsafe impl Send for DcnfNs {}
unsafe impl Sync for DcnfNs {}
impl DcnfNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CPU ID of this subsystem"]
    #[inline(always)]
    pub const fn cpuid(self) -> crate::common::Reg<regs::Cpuid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn extperi(self, n: usize) -> Extperi {
        assert!(n < 1usize);
        unsafe { Extperi::from_ptr(self.ptr.add(0x0440usize + n * 4usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn extram(self, n: usize) -> Extram {
        assert!(n < 1usize);
        unsafe { Extram::from_ptr(self.ptr.add(0x0460usize + n * 4usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn extcode(self, n: usize) -> Extcode {
        assert!(n < 1usize);
        unsafe { Extcode::from_ptr(self.ptr.add(0x0480usize + n * 4usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extcode {
    ptr: *mut u8,
}
unsafe impl Send for Extcode {}
unsafe impl Sync for Extcode {}
impl Extcode {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Control access from master connected to AMLI master port EXTCODE\\[n\\]"]
    #[inline(always)]
    pub const fn protect(self) -> crate::common::Reg<regs::ExtcodeProtect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extperi {
    ptr: *mut u8,
}
unsafe impl Send for Extperi {}
unsafe impl Sync for Extperi {}
impl Extperi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Control access for master connected to AMLI master port EXTPERI\\[n\\]"]
    #[inline(always)]
    pub const fn protect(self) -> crate::common::Reg<regs::ExtperiProtect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extram {
    ptr: *mut u8,
}
unsafe impl Send for Extram {}
unsafe impl Sync for Extram {}
impl Extram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Control access from master connected to AMLI master port EXTRAM\\[n\\]"]
    #[inline(always)]
    pub const fn protect(self) -> crate::common::Reg<regs::ExtramProtect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
