#[doc = "CACHEINFO"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheinfoS {
    ptr: *mut u8,
}
unsafe impl Send for CacheinfoS {}
unsafe impl Sync for CacheinfoS {}
impl CacheinfoS {
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
    pub const fn set(self, n: usize) -> Set {
        assert!(n < 256usize);
        unsafe { Set::from_ptr(self.ptr.add(0x0usize + n * 8usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Set {
    ptr: *mut u8,
}
unsafe impl Send for Set {}
unsafe impl Sync for Set {}
impl Set {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description collection: Cache information for SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn way(self, n: usize) -> crate::common::Reg<regs::Way, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
