#[doc = "AES ECB Mode Encryption"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcbNs {
    ptr: *mut u8,
}
unsafe impl Send for EcbNs {}
unsafe impl Sync for EcbNs {}
impl EcbNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start ECB block encrypt"]
    #[inline(always)]
    pub const fn tasks_startecb(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Abort a possible executing ECB operation"]
    #[inline(always)]
    pub const fn tasks_stopecb(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Subscribe configuration for task STARTECB"]
    #[inline(always)]
    pub const fn subscribe_startecb(
        self,
    ) -> crate::common::Reg<regs::SubscribeStartecb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Subscribe configuration for task STOPECB"]
    #[inline(always)]
    pub const fn subscribe_stopecb(
        self,
    ) -> crate::common::Reg<regs::SubscribeStopecb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "ECB block encrypt complete"]
    #[inline(always)]
    pub const fn events_endecb(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error"]
    #[inline(always)]
    pub const fn events_errorecb(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Publish configuration for event ENDECB"]
    #[inline(always)]
    pub const fn publish_endecb(
        self,
    ) -> crate::common::Reg<regs::PublishEndecb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Publish configuration for event ERRORECB"]
    #[inline(always)]
    pub const fn publish_errorecb(
        self,
    ) -> crate::common::Reg<regs::PublishErrorecb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
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
    #[doc = "ECB block encrypt memory pointers"]
    #[inline(always)]
    pub const fn ecbdataptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
}
pub mod regs;
pub mod vals;
