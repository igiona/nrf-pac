#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExtcodeProtectSlave0 {
    #[doc = "Access to slave is allowed"]
    ALLOWED = 0x0,
    #[doc = "Access to slave is blocked"]
    BLOCKED = 0x01,
}
impl ExtcodeProtectSlave0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtcodeProtectSlave0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtcodeProtectSlave0 {
    #[inline(always)]
    fn from(val: u8) -> ExtcodeProtectSlave0 {
        ExtcodeProtectSlave0::from_bits(val)
    }
}
impl From<ExtcodeProtectSlave0> for u8 {
    #[inline(always)]
    fn from(val: ExtcodeProtectSlave0) -> u8 {
        ExtcodeProtectSlave0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExtperiProtectSlave0 {
    #[doc = "Access to slave is allowed"]
    ALLOWED = 0x0,
    #[doc = "Access to slave is blocked"]
    BLOCKED = 0x01,
}
impl ExtperiProtectSlave0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtperiProtectSlave0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtperiProtectSlave0 {
    #[inline(always)]
    fn from(val: u8) -> ExtperiProtectSlave0 {
        ExtperiProtectSlave0::from_bits(val)
    }
}
impl From<ExtperiProtectSlave0> for u8 {
    #[inline(always)]
    fn from(val: ExtperiProtectSlave0) -> u8 {
        ExtperiProtectSlave0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExtramProtectSlave0 {
    #[doc = "Access to slave is allowed"]
    ALLOWED = 0x0,
    #[doc = "Access to slave is blocked"]
    BLOCKED = 0x01,
}
impl ExtramProtectSlave0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtramProtectSlave0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtramProtectSlave0 {
    #[inline(always)]
    fn from(val: u8) -> ExtramProtectSlave0 {
        ExtramProtectSlave0::from_bits(val)
    }
}
impl From<ExtramProtectSlave0> for u8 {
    #[inline(always)]
    fn from(val: ExtramProtectSlave0) -> u8 {
        ExtramProtectSlave0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Slave1 {
    #[doc = "Access to slave is allowed"]
    ALLOWED = 0x0,
    #[doc = "Access to slave is blocked"]
    BLOCKED = 0x01,
}
impl Slave1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slave1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slave1 {
    #[inline(always)]
    fn from(val: u8) -> Slave1 {
        Slave1::from_bits(val)
    }
}
impl From<Slave1> for u8 {
    #[inline(always)]
    fn from(val: Slave1) -> u8 {
        Slave1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Slave2 {
    #[doc = "Access to slave is allowed"]
    ALLOWED = 0x0,
    #[doc = "Access to slave is blocked"]
    BLOCKED = 0x01,
}
impl Slave2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slave2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slave2 {
    #[inline(always)]
    fn from(val: u8) -> Slave2 {
        Slave2::from_bits(val)
    }
}
impl From<Slave2> for u8 {
    #[inline(always)]
    fn from(val: Slave2) -> u8 {
        Slave2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Slave3 {
    #[doc = "Access to slave is allowed"]
    ALLOWED = 0x0,
    #[doc = "Access to slave is blocked"]
    BLOCKED = 0x01,
}
impl Slave3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slave3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slave3 {
    #[inline(always)]
    fn from(val: u8) -> Slave3 {
        Slave3::from_bits(val)
    }
}
impl From<Slave3> for u8 {
    #[inline(always)]
    fn from(val: Slave3) -> u8 {
        Slave3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Slave4 {
    #[doc = "Access to slave is allowed"]
    ALLOWED = 0x0,
    #[doc = "Access to slave is blocked"]
    BLOCKED = 0x01,
}
impl Slave4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slave4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slave4 {
    #[inline(always)]
    fn from(val: u8) -> Slave4 {
        Slave4::from_bits(val)
    }
}
impl From<Slave4> for u8 {
    #[inline(always)]
    fn from(val: Slave4) -> u8 {
        Slave4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Slave5 {
    #[doc = "Access to slave is allowed"]
    ALLOWED = 0x0,
    #[doc = "Access to slave is blocked"]
    BLOCKED = 0x01,
}
impl Slave5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slave5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slave5 {
    #[inline(always)]
    fn from(val: u8) -> Slave5 {
        Slave5::from_bits(val)
    }
}
impl From<Slave5> for u8 {
    #[inline(always)]
    fn from(val: Slave5) -> u8 {
        Slave5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Slave6 {
    #[doc = "Access to slave is allowed"]
    ALLOWED = 0x0,
    #[doc = "Access to slave is blocked"]
    BLOCKED = 0x01,
}
impl Slave6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slave6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slave6 {
    #[inline(always)]
    fn from(val: u8) -> Slave6 {
        Slave6::from_bits(val)
    }
}
impl From<Slave6> for u8 {
    #[inline(always)]
    fn from(val: Slave6) -> u8 {
        Slave6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Slave7 {
    #[doc = "Access to slave is allowed"]
    ALLOWED = 0x0,
    #[doc = "Access to slave is blocked"]
    BLOCKED = 0x01,
}
impl Slave7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slave7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slave7 {
    #[inline(always)]
    fn from(val: u8) -> Slave7 {
        Slave7::from_bits(val)
    }
}
impl From<Slave7> for u8 {
    #[inline(always)]
    fn from(val: Slave7) -> u8 {
        Slave7::to_bits(val)
    }
}
