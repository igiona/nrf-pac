#[doc = "User information configuration registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uicr {
    ptr: *mut u8,
}
unsafe impl Send for Uicr {}
unsafe impl Sync for Uicr {}
impl Uicr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description collection: Reserved for Nordic firmware design"]
    #[inline(always)]
    pub const fn nrffw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 13usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Reserved for Nordic hardware design"]
    #[inline(always)]
    pub const fn nrfhw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Reserved for customer"]
    #[inline(always)]
    pub const fn customer(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
    #[inline(always)]
    pub const fn pselreset(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Pselreset, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "Access port protection"]
    #[inline(always)]
    pub const fn approtect(self) -> crate::common::Reg<regs::Approtect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    #[inline(always)]
    pub const fn nfcpins(self) -> crate::common::Reg<regs::Nfcpins, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "Processor debug control"]
    #[inline(always)]
    pub const fn debugctrl(self) -> crate::common::Reg<regs::Debugctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - V_VDDH-VDD."]
    #[inline(always)]
    pub const fn regout0(self) -> crate::common::Reg<regs::Regout0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
}
pub mod regs;
pub mod vals;
