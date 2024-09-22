#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
    #[inline(always)]
    pub const fn wen(&self) -> super::vals::Wen {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Wen::from_bits(val as u8)
    }
    #[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
    #[inline(always)]
    pub fn set_wen(&mut self, val: super::vals::Wen) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
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
#[doc = "I-code cache configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icachecnf(pub u32);
impl Icachecnf {
    #[doc = "Cache enable"]
    #[inline(always)]
    pub const fn cacheen(&self) -> super::vals::Cacheen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cacheen::from_bits(val as u8)
    }
    #[doc = "Cache enable"]
    #[inline(always)]
    pub fn set_cacheen(&mut self, val: super::vals::Cacheen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Cache profiling enable"]
    #[inline(always)]
    pub const fn cacheprofen(&self) -> super::vals::Cacheprofen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cacheprofen::from_bits(val as u8)
    }
    #[doc = "Cache profiling enable"]
    #[inline(always)]
    pub fn set_cacheprofen(&mut self, val: super::vals::Cacheprofen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Icachecnf {
    #[inline(always)]
    fn default() -> Icachecnf {
        Icachecnf(0)
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
