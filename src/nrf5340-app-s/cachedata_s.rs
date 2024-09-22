#[doc = "CACHEDATA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CachedataS {
    ptr: *mut u8,
}
unsafe impl Send for CachedataS {}
unsafe impl Sync for CachedataS {}
impl CachedataS {
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
        unsafe { Set::from_ptr(self.ptr.add(0x0usize + n * 32usize) as _) }
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
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn way(self, n: usize) -> Way {
        assert!(n < 2usize);
        unsafe { Way::from_ptr(self.ptr.add(0x0usize + n * 16usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Way {
    ptr: *mut u8,
}
unsafe impl Send for Way {}
unsafe impl Sync for Way {}
impl Way {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Cache data bits \\[31:0\\] of SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn data0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Cache data bits \\[63:32\\] of SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn data1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description cluster: Cache data bits \\[95:64\\] of SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn data2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Description cluster: Cache data bits \\[127:96\\] of SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn data3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
