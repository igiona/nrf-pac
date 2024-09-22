#[doc = "Cache"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheS {
    ptr: *mut u8,
}
unsafe impl Send for CacheS {}
unsafe impl Sync for CacheS {}
impl CacheS {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn profiling(self, n: usize) -> Profiling {
        assert!(n < 2usize);
        unsafe { Profiling::from_ptr(self.ptr.add(0x0400usize + n * 32usize) as _) }
    }
    #[doc = "Enable cache."]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Invalidate the cache."]
    #[inline(always)]
    pub const fn invalidate(self) -> crate::common::Reg<regs::Invalidate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Erase the cache."]
    #[inline(always)]
    pub const fn erase(self) -> crate::common::Reg<regs::Erase, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Enable the profiling counters."]
    #[inline(always)]
    pub const fn profilingenable(
        self,
    ) -> crate::common::Reg<regs::Profilingenable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "Clear the profiling counters."]
    #[inline(always)]
    pub const fn profilingclear(
        self,
    ) -> crate::common::Reg<regs::Profilingclear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Cache mode. Switching from Cache to Ram mode causes the RAM to be cleared. Switching from RAM to Cache mode causes the cache to be invalidated."]
    #[inline(always)]
    pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "Lock debug mode."]
    #[inline(always)]
    pub const fn debuglock(self) -> crate::common::Reg<regs::Debuglock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "Cache erase status."]
    #[inline(always)]
    pub const fn erasestatus(self) -> crate::common::Reg<regs::Erasestatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode."]
    #[inline(always)]
    pub const fn writelock(self) -> crate::common::Reg<regs::Writelock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Profiling {
    ptr: *mut u8,
}
unsafe impl Send for Profiling {}
unsafe impl Sync for Profiling {}
impl Profiling {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    #[inline(always)]
    pub const fn ihit(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    #[inline(always)]
    pub const fn imiss(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    #[inline(always)]
    pub const fn dhit(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Description cluster: Data fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    #[inline(always)]
    pub const fn dmiss(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
