#[doc = "Accelerated Address Resolver"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AarNs {
    ptr: *mut u8,
}
unsafe impl Send for AarNs {}
unsafe impl Sync for AarNs {}
impl AarNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start resolving addresses based on IRKs specified in the IRK data structure"]
    #[inline(always)]
    pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Stop resolving addresses"]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Address resolution procedure complete"]
    #[inline(always)]
    pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Address resolved"]
    #[inline(always)]
    pub const fn events_resolved(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Address not resolved"]
    #[inline(always)]
    pub const fn events_notresolved(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Publish configuration for event END"]
    #[inline(always)]
    pub const fn publish_end(self) -> crate::common::Reg<regs::PublishEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Publish configuration for event RESOLVED"]
    #[inline(always)]
    pub const fn publish_resolved(
        self,
    ) -> crate::common::Reg<regs::PublishResolved, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Publish configuration for event NOTRESOLVED"]
    #[inline(always)]
    pub const fn publish_notresolved(
        self,
    ) -> crate::common::Reg<regs::PublishNotresolved, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
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
    #[doc = "Resolution status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Enable AAR"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Number of IRKs"]
    #[inline(always)]
    pub const fn nirk(self) -> crate::common::Reg<regs::Nirk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Pointer to IRK data structure"]
    #[inline(always)]
    pub const fn irkptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Pointer to the resolvable address"]
    #[inline(always)]
    pub const fn addrptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Pointer to data area used for temporary storage"]
    #[inline(always)]
    pub const fn scratchptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
}
pub mod regs;
pub mod vals;
