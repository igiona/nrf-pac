#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub const fn keyslot_pushed(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub fn set_keyslot_pushed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub const fn keyslot_revoked(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub fn set_keyslot_revoked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub const fn keyslot_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub fn set_keyslot_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Pending interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intpend(pub u32);
impl Intpend {
    #[doc = "Read pending status of interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub const fn keyslot_pushed(&self) -> super::vals::IntpendKeyslotPushed {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IntpendKeyslotPushed::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub fn set_keyslot_pushed(&mut self, val: super::vals::IntpendKeyslotPushed) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Read pending status of interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub const fn keyslot_revoked(&self) -> super::vals::IntpendKeyslotRevoked {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IntpendKeyslotRevoked::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub fn set_keyslot_revoked(&mut self, val: super::vals::IntpendKeyslotRevoked) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Read pending status of interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub const fn keyslot_error(&self) -> super::vals::IntpendKeyslotError {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::IntpendKeyslotError::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub fn set_keyslot_error(&mut self, val: super::vals::IntpendKeyslotError) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Intpend {
    #[inline(always)]
    fn default() -> Intpend {
        Intpend(0)
    }
}
#[doc = "Select key slot to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Selectkeyslot(pub u32);
impl Selectkeyslot {
    #[doc = "Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\] and UICR-&gt;KEYSLOT.CONFIG\\[N\\] corresponds to KMU key slot ID=N+1."]
    #[inline(always)]
    pub const fn id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started. NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use. NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\] and UICR-&gt;KEYSLOT.CONFIG\\[N\\] corresponds to KMU key slot ID=N+1."]
    #[inline(always)]
    pub fn set_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Selectkeyslot {
    #[inline(always)]
    fn default() -> Selectkeyslot {
        Selectkeyslot(0)
    }
}
#[doc = "Status bits for KMU operation"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Key slot ID successfully selected by the KMU"]
    #[inline(always)]
    pub const fn selected(&self) -> super::vals::Selected {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Selected::from_bits(val as u8)
    }
    #[doc = "Key slot ID successfully selected by the KMU"]
    #[inline(always)]
    pub fn set_selected(&mut self, val: super::vals::Selected) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Violation status"]
    #[inline(always)]
    pub const fn blocked(&self) -> super::vals::Blocked {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Blocked::from_bits(val as u8)
    }
    #[doc = "Violation status"]
    #[inline(always)]
    pub fn set_blocked(&mut self, val: super::vals::Blocked) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
