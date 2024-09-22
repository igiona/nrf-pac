#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config {
    ptr: *mut u8,
}
unsafe impl Send for Config {}
unsafe impl Sync for Config {}
impl Config {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE\\[0-3\\]) will be pushed by KMU. Note that this address must match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read."]
    #[inline(always)]
    pub const fn dest(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Define permissions for the key slot. Bits 0-15 and 16-31 can only be written when equal to 0xFFFF."]
    #[inline(always)]
    pub const fn perm(self) -> crate::common::Reg<regs::Perm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key {
    ptr: *mut u8,
}
unsafe impl Send for Key {}
unsafe impl Sync for Key {}
impl Key {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description collection: Define bits \\[31+o*32:0+o*32\\] of value assigned to KMU key slot."]
    #[inline(always)]
    pub const fn value(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyslot {
    ptr: *mut u8,
}
unsafe impl Send for Keyslot {}
unsafe impl Sync for Keyslot {}
impl Keyslot {
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
    pub const fn config(self, n: usize) -> Config {
        assert!(n < 128usize);
        unsafe { Config::from_ptr(self.ptr.add(0x0usize + n * 8usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn key(self, n: usize) -> Key {
        assert!(n < 128usize);
        unsafe { Key::from_ptr(self.ptr.add(0x0400usize + n * 16usize) as _) }
    }
}
#[doc = "User Information Configuration Registers User information configuration registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UicrS {
    ptr: *mut u8,
}
unsafe impl Send for UicrS {}
unsafe impl Sync for UicrS {}
impl UicrS {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Access port protection"]
    #[inline(always)]
    pub const fn approtect(self) -> crate::common::Reg<regs::Approtect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Output voltage from the high voltage (VREGH) regulator stage. The maximum output voltage from this stage is given as VDDH - VREGHDROP."]
    #[inline(always)]
    pub const fn vreghvout(self) -> crate::common::Reg<regs::Vreghvout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "HFXO startup counter"]
    #[inline(always)]
    pub const fn hfxocnt(self) -> crate::common::Reg<regs::Hfxocnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Secure access port protection"]
    #[inline(always)]
    pub const fn secureapprotect(
        self,
    ) -> crate::common::Reg<regs::Secureapprotect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Erase protection"]
    #[inline(always)]
    pub const fn eraseprotect(self) -> crate::common::Reg<regs::Eraseprotect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SW-DP Target instance"]
    #[inline(always)]
    pub const fn tinstance(self) -> crate::common::Reg<regs::Tinstance, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    #[inline(always)]
    pub const fn nfcpins(self) -> crate::common::Reg<regs::Nfcpins, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Description collection: One time programmable memory"]
    #[inline(always)]
    pub const fn otp(self, n: usize) -> crate::common::Reg<regs::Otp, crate::common::RW> {
        assert!(n < 192usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn keyslot(self) -> Keyslot {
        unsafe { Keyslot::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
}
pub mod regs;
pub mod vals;
