#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApprotectLockLock {
    #[doc = "Register APPROTECT.DISABLE is writeable"]
    UNLOCKED = 0x0,
    #[doc = "Register APPROTECT.DISABLE is read-only"]
    LOCKED = 0x01,
}
impl ApprotectLockLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApprotectLockLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApprotectLockLock {
    #[inline(always)]
    fn from(val: u8) -> ApprotectLockLock {
        ApprotectLockLock::from_bits(val)
    }
}
impl From<ApprotectLockLock> for u8 {
    #[inline(always)]
    fn from(val: ApprotectLockLock) -> u8 {
        ApprotectLockLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dbgifacemode {
    #[doc = "No debugger attached"]
    DISABLED = 0x0,
    #[doc = "Debugger is attached and device is in debug interface mode"]
    ENABLED = 0x01,
}
impl Dbgifacemode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgifacemode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgifacemode {
    #[inline(always)]
    fn from(val: u8) -> Dbgifacemode {
        Dbgifacemode::from_bits(val)
    }
}
impl From<Dbgifacemode> for u8 {
    #[inline(always)]
    fn from(val: Dbgifacemode) -> u8 {
        Dbgifacemode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EraseprotectLockLock {
    #[doc = "Register ERASEPROTECT.DISABLE is writeable"]
    UNLOCKED = 0x0,
    #[doc = "Register ERASEPROTECT.DISABLE is read-only"]
    LOCKED = 0x01,
}
impl EraseprotectLockLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EraseprotectLockLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EraseprotectLockLock {
    #[inline(always)]
    fn from(val: u8) -> EraseprotectLockLock {
        EraseprotectLockLock::from_bits(val)
    }
}
impl From<EraseprotectLockLock> for u8 {
    #[inline(always)]
    fn from(val: EraseprotectLockLock) -> u8 {
        EraseprotectLockLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rxstatus {
    #[doc = "No data pending in register RXDATA"]
    NODATAPENDING = 0x0,
    #[doc = "Data pending in register RXDATA"]
    DATAPENDING = 0x01,
}
impl Rxstatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxstatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxstatus {
    #[inline(always)]
    fn from(val: u8) -> Rxstatus {
        Rxstatus::from_bits(val)
    }
}
impl From<Rxstatus> for u8 {
    #[inline(always)]
    fn from(val: Rxstatus) -> u8 {
        Rxstatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SecureapprotectLockLock {
    #[doc = "Register SECUREAPPROTECT.DISABLE is writeable"]
    UNLOCKED = 0x0,
    #[doc = "Register SECUREAPPROTECT.DISABLE is read-only"]
    LOCKED = 0x01,
}
impl SecureapprotectLockLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecureapprotectLockLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecureapprotectLockLock {
    #[inline(always)]
    fn from(val: u8) -> SecureapprotectLockLock {
        SecureapprotectLockLock::from_bits(val)
    }
}
impl From<SecureapprotectLockLock> for u8 {
    #[inline(always)]
    fn from(val: SecureapprotectLockLock) -> u8 {
        SecureapprotectLockLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Txstatus {
    #[doc = "No data pending in register TXDATA"]
    NODATAPENDING = 0x0,
    #[doc = "Data pending in register TXDATA"]
    DATAPENDING = 0x01,
}
impl Txstatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txstatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txstatus {
    #[inline(always)]
    fn from(val: u8) -> Txstatus {
        Txstatus::from_bits(val)
    }
}
impl From<Txstatus> for u8 {
    #[inline(always)]
    fn from(val: Txstatus) -> u8 {
        Txstatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Uicrapprotect {
    #[doc = "APPROTECT was enabled in UICR"]
    ENABLED = 0x0,
    #[doc = "APPROTECT wasdisabled in UICR"]
    DISABLED = 0x01,
}
impl Uicrapprotect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uicrapprotect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uicrapprotect {
    #[inline(always)]
    fn from(val: u8) -> Uicrapprotect {
        Uicrapprotect::from_bits(val)
    }
}
impl From<Uicrapprotect> for u8 {
    #[inline(always)]
    fn from(val: Uicrapprotect) -> u8 {
        Uicrapprotect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Uicrsecureapprotect {
    #[doc = "SECUREAPPROTECT was enabled in UICR"]
    ENABLED = 0x0,
    #[doc = "SECUREAPPROTECT was disabled in UICR"]
    DISABLED = 0x01,
}
impl Uicrsecureapprotect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uicrsecureapprotect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uicrsecureapprotect {
    #[inline(always)]
    fn from(val: u8) -> Uicrsecureapprotect {
        Uicrsecureapprotect::from_bits(val)
    }
}
impl From<Uicrsecureapprotect> for u8 {
    #[inline(always)]
    fn from(val: Uicrsecureapprotect) -> u8 {
        Uicrsecureapprotect::to_bits(val)
    }
}
