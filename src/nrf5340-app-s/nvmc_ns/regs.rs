#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used."]
    #[inline(always)]
    pub const fn wen(&self) -> super::vals::ConfigWen {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::ConfigWen::from_bits(val as u8)
    }
    #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used."]
    #[inline(always)]
    pub fn set_wen(&mut self, val: super::vals::ConfigWen) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "Non-secure configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Configns(pub u32);
impl Configns {
    #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used."]
    #[inline(always)]
    pub const fn wen(&self) -> super::vals::ConfignsWen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::ConfignsWen::from_bits(val as u8)
    }
    #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used."]
    #[inline(always)]
    pub fn set_wen(&mut self, val: super::vals::ConfignsWen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Configns {
    #[inline(always)]
    fn default() -> Configns {
        Configns(0)
    }
}
#[doc = "Register for erasing all non-volatile user memory"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eraseall(pub u32);
impl Eraseall {
    #[doc = "Erase all non-volatile memory including UICR registers. Before the non-volatile memory can be erased, erasing must be enabled by setting CONFIG.WEN=Een."]
    #[inline(always)]
    pub const fn eraseall(&self) -> super::vals::Eraseall {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Eraseall::from_bits(val as u8)
    }
    #[doc = "Erase all non-volatile memory including UICR registers. Before the non-volatile memory can be erased, erasing must be enabled by setting CONFIG.WEN=Een."]
    #[inline(always)]
    pub fn set_eraseall(&mut self, val: super::vals::Eraseall) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Eraseall {
    #[inline(always)]
    fn default() -> Eraseall {
        Eraseall(0)
    }
}
#[doc = "Register for partial erase configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erasepagepartialcfg(pub u32);
impl Erasepagepartialcfg {
    #[doc = "Duration of the partial erase in milliseconds"]
    #[inline(always)]
    pub const fn duration(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Duration of the partial erase in milliseconds"]
    #[inline(always)]
    pub fn set_duration(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Erasepagepartialcfg {
    #[inline(always)]
    fn default() -> Erasepagepartialcfg {
        Erasepagepartialcfg(0)
    }
}
#[doc = "Ready flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ready(pub u32);
impl Ready {
    #[doc = "NVMC is ready or busy"]
    #[inline(always)]
    pub const fn ready(&self) -> super::vals::Ready {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ready::from_bits(val as u8)
    }
    #[doc = "NVMC is ready or busy"]
    #[inline(always)]
    pub fn set_ready(&mut self, val: super::vals::Ready) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ready {
    #[inline(always)]
    fn default() -> Ready {
        Ready(0)
    }
}
#[doc = "Ready flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Readynext(pub u32);
impl Readynext {
    #[doc = "NVMC can accept a new write operation"]
    #[inline(always)]
    pub const fn readynext(&self) -> super::vals::Readynext {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Readynext::from_bits(val as u8)
    }
    #[doc = "NVMC can accept a new write operation"]
    #[inline(always)]
    pub fn set_readynext(&mut self, val: super::vals::Readynext) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Readynext {
    #[inline(always)]
    fn default() -> Readynext {
        Readynext(0)
    }
}
#[doc = "Non-secure APPROTECT enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Writeuicrns(pub u32);
impl Writeuicrns {
    #[doc = "Allow non-secure code to set APPROTECT"]
    #[inline(always)]
    pub const fn set(&self) -> super::vals::Set {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Set::from_bits(val as u8)
    }
    #[doc = "Allow non-secure code to set APPROTECT"]
    #[inline(always)]
    pub fn set_set(&mut self, val: super::vals::Set) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Key to write in order to validate the write operation"]
    #[inline(always)]
    pub const fn key(&self) -> super::vals::Key {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        super::vals::Key::from_bits(val as u32)
    }
    #[doc = "Key to write in order to validate the write operation"]
    #[inline(always)]
    pub fn set_key(&mut self, val: super::vals::Key) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize))
            | (((val.to_bits() as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for Writeuicrns {
    #[inline(always)]
    fn default() -> Writeuicrns {
        Writeuicrns(0)
    }
}
