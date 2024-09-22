#[doc = "Interprocessor communication"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpcNs {
    ptr: *mut u8,
}
unsafe impl Send for IpcNs {}
unsafe impl Sync for IpcNs {}
impl IpcNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description collection: Trigger events on IPC channel enabled in SEND_CNF\\[n\\]"]
    #[inline(always)]
    pub const fn tasks_send(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_send(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SubscribeSend, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
    #[inline(always)]
    pub const fn events_receive(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]"]
    #[inline(always)]
    pub const fn publish_receive(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PublishReceive, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize + n * 4usize) as _) }
    }
    #[doc = "Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
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
    #[doc = "Pending interrupts"]
    #[inline(always)]
    pub const fn intpend(self) -> crate::common::Reg<regs::Intpend, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]"]
    #[inline(always)]
    pub const fn send_cnf(self, n: usize) -> crate::common::Reg<regs::SendCnf, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]"]
    #[inline(always)]
    pub const fn receive_cnf(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ReceiveCnf, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0590usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: General purpose memory"]
    #[inline(always)]
    pub const fn gpmem(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0610usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
