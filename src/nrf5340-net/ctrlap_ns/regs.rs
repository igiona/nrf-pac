#[doc = "This register locks the APPROTECT.DISABLE register from being written to until next reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApprotectLock(pub u32);
impl ApprotectLock {
    #[doc = "Lock the APPROTECT.DISABLE register from being written to until next reset"]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::ApprotectLockLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ApprotectLockLock::from_bits(val as u8)
    }
    #[doc = "Lock the APPROTECT.DISABLE register from being written to until next reset"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::ApprotectLockLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for ApprotectLock {
    #[inline(always)]
    fn default() -> ApprotectLock {
        ApprotectLock(0)
    }
}
#[doc = "This register locks the ERASEPROTECT.DISABLE register from being written until next reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EraseprotectLock(pub u32);
impl EraseprotectLock {
    #[doc = "Lock ERASEPROTECT.DISABLE register from being written until next reset"]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::EraseprotectLockLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EraseprotectLockLock::from_bits(val as u8)
    }
    #[doc = "Lock ERASEPROTECT.DISABLE register from being written until next reset"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::EraseprotectLockLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EraseprotectLock {
    #[inline(always)]
    fn default() -> EraseprotectLock {
        EraseprotectLock(0)
    }
}
#[doc = "This register shows a status that indicates if data sent from the debugger to the CPU has been read."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxstatus(pub u32);
impl Rxstatus {
    #[doc = "Status of data in register RXDATA"]
    #[inline(always)]
    pub const fn rxstatus(&self) -> super::vals::Rxstatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rxstatus::from_bits(val as u8)
    }
    #[doc = "Status of data in register RXDATA"]
    #[inline(always)]
    pub fn set_rxstatus(&mut self, val: super::vals::Rxstatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Rxstatus {
    #[inline(always)]
    fn default() -> Rxstatus {
        Rxstatus(0)
    }
}
#[doc = "Status bits for CTRL-AP peripheral."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Status bit for UICR part of access port protection at last reset."]
    #[inline(always)]
    pub const fn uicrapprotect(&self) -> super::vals::Uicrapprotect {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Uicrapprotect::from_bits(val as u8)
    }
    #[doc = "Status bit for UICR part of access port protection at last reset."]
    #[inline(always)]
    pub fn set_uicrapprotect(&mut self, val: super::vals::Uicrapprotect) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Status bit for device debug interface mode"]
    #[inline(always)]
    pub const fn dbgifacemode(&self) -> super::vals::Dbgifacemode {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dbgifacemode::from_bits(val as u8)
    }
    #[doc = "Status bit for device debug interface mode"]
    #[inline(always)]
    pub fn set_dbgifacemode(&mut self, val: super::vals::Dbgifacemode) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "This register shows a status that indicates if the data sent from the CPU to the debugger has been read."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txstatus(pub u32);
impl Txstatus {
    #[doc = "Status of data in register TXDATA"]
    #[inline(always)]
    pub const fn txstatus(&self) -> super::vals::Txstatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Txstatus::from_bits(val as u8)
    }
    #[doc = "Status of data in register TXDATA"]
    #[inline(always)]
    pub fn set_txstatus(&mut self, val: super::vals::Txstatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Txstatus {
    #[inline(always)]
    fn default() -> Txstatus {
        Txstatus(0)
    }
}
