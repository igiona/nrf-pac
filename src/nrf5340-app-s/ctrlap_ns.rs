#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Approtect {
    ptr: *mut u8,
}
unsafe impl Send for Approtect {}
unsafe impl Sync for Approtect {}
impl Approtect {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "This register locks the APPROTECT.DISABLE register from being written to until next reset."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::ApprotectLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "This register disables the APPROTECT register and enables debug access to non-secure mode."]
    #[inline(always)]
    pub const fn disable(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Control access port 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlapNs {
    ptr: *mut u8,
}
unsafe impl Send for CtrlapNs {}
unsafe impl Sync for CtrlapNs {}
impl CtrlapNs {
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
    pub const fn mailbox(self) -> Mailbox {
        unsafe { Mailbox::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn eraseprotect(self) -> Eraseprotect {
        unsafe { Eraseprotect::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn approtect(self) -> Approtect {
        unsafe { Approtect::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn secureapprotect(self) -> Secureapprotect {
        unsafe { Secureapprotect::from_ptr(self.ptr.add(0x0548usize) as _) }
    }
    #[doc = "Status bits for CTRL-AP peripheral."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eraseprotect {
    ptr: *mut u8,
}
unsafe impl Send for Eraseprotect {}
unsafe impl Sync for Eraseprotect {}
impl Eraseprotect {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "This register locks the ERASEPROTECT.DISABLE register from being written until next reset."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::EraseprotectLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "This register disables the ERASEPROTECT register and performs an ERASEALL operation."]
    #[inline(always)]
    pub const fn disable(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mailbox {
    ptr: *mut u8,
}
unsafe impl Send for Mailbox {}
unsafe impl Sync for Mailbox {}
impl Mailbox {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data sent from the debugger to the CPU."]
    #[inline(always)]
    pub const fn rxdata(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "This register shows a status that indicates if data sent from the debugger to the CPU has been read."]
    #[inline(always)]
    pub const fn rxstatus(self) -> crate::common::Reg<regs::Rxstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Data sent from the CPU to the debugger."]
    #[inline(always)]
    pub const fn txdata(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "This register shows a status that indicates if the data sent from the CPU to the debugger has been read."]
    #[inline(always)]
    pub const fn txstatus(self) -> crate::common::Reg<regs::Txstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Secureapprotect {
    ptr: *mut u8,
}
unsafe impl Send for Secureapprotect {}
unsafe impl Sync for Secureapprotect {}
impl Secureapprotect {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "This register locks the SECUREAPPROTECT.DISABLE register from being written until next reset."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::SecureapprotectLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "This register disables the SECUREAPPROTECT register and enables debug access to secure mode."]
    #[inline(always)]
    pub const fn disable(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
