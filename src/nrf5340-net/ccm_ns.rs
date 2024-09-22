#[doc = "AES CCM mode encryption"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcmNs {
    ptr: *mut u8,
}
unsafe impl Send for CcmNs {}
unsafe impl Sync for CcmNs {}
impl CcmNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Start generation of keystream. This operation will stop by itself when completed."]
    #[inline(always)]
    pub const fn tasks_ksgen(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
    #[inline(always)]
    pub const fn tasks_crypt(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Stop encryption/decryption"]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
    #[inline(always)]
    pub const fn tasks_rateoverride(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Subscribe configuration for task KSGEN"]
    #[inline(always)]
    pub const fn subscribe_ksgen(
        self,
    ) -> crate::common::Reg<regs::SubscribeKsgen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Subscribe configuration for task CRYPT"]
    #[inline(always)]
    pub const fn subscribe_crypt(
        self,
    ) -> crate::common::Reg<regs::SubscribeCrypt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(
        self,
    ) -> crate::common::Reg<regs::SubscribeStop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Subscribe configuration for task RATEOVERRIDE"]
    #[inline(always)]
    pub const fn subscribe_rateoverride(
        self,
    ) -> crate::common::Reg<regs::SubscribeRateoverride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Keystream generation complete"]
    #[inline(always)]
    pub const fn events_endksgen(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Encrypt/decrypt complete"]
    #[inline(always)]
    pub const fn events_endcrypt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Deprecated register - CCM error event"]
    #[inline(always)]
    pub const fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Publish configuration for event ENDKSGEN"]
    #[inline(always)]
    pub const fn publish_endksgen(
        self,
    ) -> crate::common::Reg<regs::PublishEndksgen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Publish configuration for event ENDCRYPT"]
    #[inline(always)]
    pub const fn publish_endcrypt(
        self,
    ) -> crate::common::Reg<regs::PublishEndcrypt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Deprecated register - Publish configuration for event ERROR"]
    #[inline(always)]
    pub const fn publish_error(self) -> crate::common::Reg<regs::PublishError, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
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
    #[doc = "MIC check result"]
    #[inline(always)]
    pub const fn micstatus(self) -> crate::common::Reg<regs::Micstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Operation mode"]
    #[inline(always)]
    pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Pointer to data structure holding the AES key and the NONCE vector"]
    #[inline(always)]
    pub const fn cnfptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Input pointer"]
    #[inline(always)]
    pub const fn inptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "Output pointer"]
    #[inline(always)]
    pub const fn outptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Pointer to data area used for temporary storage"]
    #[inline(always)]
    pub const fn scratchptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "Length of keystream generated when MODE.LENGTH = Extended"]
    #[inline(always)]
    pub const fn maxpacketsize(self) -> crate::common::Reg<regs::Maxpacketsize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "Data rate override setting."]
    #[inline(always)]
    pub const fn rateoverride(self) -> crate::common::Reg<regs::Rateoverride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "Header (S0) mask."]
    #[inline(always)]
    pub const fn headermask(self) -> crate::common::Reg<regs::Headermask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
}
pub mod regs;
pub mod vals;
