#[doc = "GPIO Port 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0ns {
    ptr: *mut u8,
}
unsafe impl Send for P0ns {}
unsafe impl Sync for P0ns {}
impl P0ns {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Write GPIO port"]
    #[inline(always)]
    pub const fn out(self) -> crate::common::Reg<regs::Out, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Set individual bits in GPIO port"]
    #[inline(always)]
    pub const fn outset(self) -> crate::common::Reg<regs::Outset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Clear individual bits in GPIO port"]
    #[inline(always)]
    pub const fn outclr(self) -> crate::common::Reg<regs::Outclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Read GPIO port"]
    #[inline(always)]
    pub const fn in_(self) -> crate::common::Reg<regs::In, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Direction of GPIO pins"]
    #[inline(always)]
    pub const fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "DIR set register"]
    #[inline(always)]
    pub const fn dirset(self) -> crate::common::Reg<regs::Dirset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "DIR clear register"]
    #[inline(always)]
    pub const fn dirclr(self) -> crate::common::Reg<regs::Dirclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
    #[inline(always)]
    pub const fn latch(self) -> crate::common::Reg<regs::Latch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Select between default DETECT signal behavior and LDETECT mode (For non-secure pin only)"]
    #[inline(always)]
    pub const fn detectmode(self) -> crate::common::Reg<regs::Detectmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Select between default DETECT signal behavior and LDETECT mode (For secure pin only)"]
    #[inline(always)]
    pub const fn detectmode_sec(
        self,
    ) -> crate::common::Reg<regs::DetectmodeSec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Description collection: Configuration of GPIO pins"]
    #[inline(always)]
    pub const fn pin_cnf(self, n: usize) -> crate::common::Reg<regs::PinCnf, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
