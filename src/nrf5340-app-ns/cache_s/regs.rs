#[doc = "Lock debug mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debuglock(pub u32);
impl Debuglock {
    #[doc = "Lock debug mode"]
    #[inline(always)]
    pub const fn debuglock(&self) -> super::vals::Debuglock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Debuglock::from_bits(val as u8)
    }
    #[doc = "Lock debug mode"]
    #[inline(always)]
    pub fn set_debuglock(&mut self, val: super::vals::Debuglock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Debuglock {
    #[inline(always)]
    fn default() -> Debuglock {
        Debuglock(0)
    }
}
#[doc = "Enable cache."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable cache"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::EnableEnable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EnableEnable::from_bits(val as u8)
    }
    #[doc = "Enable cache"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::EnableEnable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Enable {
    #[inline(always)]
    fn default() -> Enable {
        Enable(0)
    }
}
#[doc = "Erase the cache."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erase(pub u32);
impl Erase {
    #[doc = "Erase the cache"]
    #[inline(always)]
    pub const fn erase(&self) -> super::vals::Erase {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Erase::from_bits(val as u8)
    }
    #[doc = "Erase the cache"]
    #[inline(always)]
    pub fn set_erase(&mut self, val: super::vals::Erase) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Erase {
    #[inline(always)]
    fn default() -> Erase {
        Erase(0)
    }
}
#[doc = "Cache erase status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erasestatus(pub u32);
impl Erasestatus {
    #[doc = "Cache erase status"]
    #[inline(always)]
    pub const fn erasestatus(&self) -> super::vals::Erasestatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Erasestatus::from_bits(val as u8)
    }
    #[doc = "Cache erase status"]
    #[inline(always)]
    pub fn set_erasestatus(&mut self, val: super::vals::Erasestatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Erasestatus {
    #[inline(always)]
    fn default() -> Erasestatus {
        Erasestatus(0)
    }
}
#[doc = "Invalidate the cache."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Invalidate(pub u32);
impl Invalidate {
    #[doc = "Invalidate the cache"]
    #[inline(always)]
    pub const fn invalidate(&self) -> super::vals::Invalidate {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Invalidate::from_bits(val as u8)
    }
    #[doc = "Invalidate the cache"]
    #[inline(always)]
    pub fn set_invalidate(&mut self, val: super::vals::Invalidate) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Invalidate {
    #[inline(always)]
    fn default() -> Invalidate {
        Invalidate(0)
    }
}
#[doc = "Cache mode. Switching from Cache to Ram mode causes the RAM to be cleared. Switching from RAM to Cache mode causes the cache to be invalidated."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "Cache mode"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Cache mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "Clear the profiling counters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Profilingclear(pub u32);
impl Profilingclear {
    #[doc = "Clearing the profiling counters"]
    #[inline(always)]
    pub const fn clear(&self) -> super::vals::Clear {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Clearing the profiling counters"]
    #[inline(always)]
    pub fn set_clear(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Profilingclear {
    #[inline(always)]
    fn default() -> Profilingclear {
        Profilingclear(0)
    }
}
#[doc = "Enable the profiling counters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Profilingenable(pub u32);
impl Profilingenable {
    #[doc = "Enable the profiling counters"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::ProfilingenableEnable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ProfilingenableEnable::from_bits(val as u8)
    }
    #[doc = "Enable the profiling counters"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::ProfilingenableEnable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Profilingenable {
    #[inline(always)]
    fn default() -> Profilingenable {
        Profilingenable(0)
    }
}
#[doc = "Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Writelock(pub u32);
impl Writelock {
    #[doc = "Lock cache updates"]
    #[inline(always)]
    pub const fn writelock(&self) -> super::vals::Writelock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Writelock::from_bits(val as u8)
    }
    #[doc = "Lock cache updates"]
    #[inline(always)]
    pub fn set_writelock(&mut self, val: super::vals::Writelock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Writelock {
    #[inline(always)]
    fn default() -> Writelock {
        Writelock(0)
    }
}
