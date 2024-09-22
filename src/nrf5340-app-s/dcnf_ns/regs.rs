#[doc = "CPU ID of this subsystem"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuid(pub u32);
impl Cpuid {
    #[doc = "CPU ID"]
    #[inline(always)]
    pub const fn cpuid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "CPU ID"]
    #[inline(always)]
    pub fn set_cpuid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cpuid {
    #[inline(always)]
    fn default() -> Cpuid {
        Cpuid(0)
    }
}
#[doc = "Description cluster: Control access from master connected to AMLI master port EXTCODE\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtcodeProtect(pub u32);
impl ExtcodeProtect {
    #[doc = "Control access to slave 0 of master EXTCODE\\[n\\]"]
    #[inline(always)]
    pub const fn slave0(&self) -> super::vals::ExtcodeProtectSlave0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ExtcodeProtectSlave0::from_bits(val as u8)
    }
    #[doc = "Control access to slave 0 of master EXTCODE\\[n\\]"]
    #[inline(always)]
    pub fn set_slave0(&mut self, val: super::vals::ExtcodeProtectSlave0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for ExtcodeProtect {
    #[inline(always)]
    fn default() -> ExtcodeProtect {
        ExtcodeProtect(0)
    }
}
#[doc = "Description cluster: Control access for master connected to AMLI master port EXTPERI\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtperiProtect(pub u32);
impl ExtperiProtect {
    #[doc = "Control access to slave 0 of master EXTPERI\\[n\\]"]
    #[inline(always)]
    pub const fn slave0(&self) -> super::vals::ExtperiProtectSlave0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ExtperiProtectSlave0::from_bits(val as u8)
    }
    #[doc = "Control access to slave 0 of master EXTPERI\\[n\\]"]
    #[inline(always)]
    pub fn set_slave0(&mut self, val: super::vals::ExtperiProtectSlave0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for ExtperiProtect {
    #[inline(always)]
    fn default() -> ExtperiProtect {
        ExtperiProtect(0)
    }
}
#[doc = "Description cluster: Control access from master connected to AMLI master port EXTRAM\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtramProtect(pub u32);
impl ExtramProtect {
    #[doc = "Control access to slave 0 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub const fn slave0(&self) -> super::vals::ExtramProtectSlave0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ExtramProtectSlave0::from_bits(val as u8)
    }
    #[doc = "Control access to slave 0 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn set_slave0(&mut self, val: super::vals::ExtramProtectSlave0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Control access to slave 1 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub const fn slave1(&self) -> super::vals::Slave1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Slave1::from_bits(val as u8)
    }
    #[doc = "Control access to slave 1 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn set_slave1(&mut self, val: super::vals::Slave1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Control access to slave 2 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub const fn slave2(&self) -> super::vals::Slave2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Slave2::from_bits(val as u8)
    }
    #[doc = "Control access to slave 2 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn set_slave2(&mut self, val: super::vals::Slave2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Control access to slave 3 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub const fn slave3(&self) -> super::vals::Slave3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Slave3::from_bits(val as u8)
    }
    #[doc = "Control access to slave 3 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn set_slave3(&mut self, val: super::vals::Slave3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Control access to slave 4 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub const fn slave4(&self) -> super::vals::Slave4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Slave4::from_bits(val as u8)
    }
    #[doc = "Control access to slave 4 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn set_slave4(&mut self, val: super::vals::Slave4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Control access to slave 5 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub const fn slave5(&self) -> super::vals::Slave5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Slave5::from_bits(val as u8)
    }
    #[doc = "Control access to slave 5 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn set_slave5(&mut self, val: super::vals::Slave5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Control access to slave 6 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub const fn slave6(&self) -> super::vals::Slave6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Slave6::from_bits(val as u8)
    }
    #[doc = "Control access to slave 6 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn set_slave6(&mut self, val: super::vals::Slave6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Control access to slave 7 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub const fn slave7(&self) -> super::vals::Slave7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Slave7::from_bits(val as u8)
    }
    #[doc = "Control access to slave 7 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn set_slave7(&mut self, val: super::vals::Slave7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for ExtramProtect {
    #[inline(always)]
    fn default() -> ExtramProtect {
        ExtramProtect(0)
    }
}
