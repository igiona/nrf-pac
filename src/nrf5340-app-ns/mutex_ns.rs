#[doc = "MUTEX 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MutexNs {
    ptr: *mut u8,
}
unsafe impl Send for MutexNs {}
unsafe impl Sync for MutexNs {}
impl MutexNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description collection: Mutex register"]
    #[inline(always)]
    pub const fn mutex(self, n: usize) -> crate::common::Reg<regs::Mutex, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
