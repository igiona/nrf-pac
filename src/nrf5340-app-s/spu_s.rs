#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dppi {
    ptr: *mut u8,
}
unsafe impl Send for Dppi {}
unsafe impl Sync for Dppi {}
impl Dppi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels"]
    #[inline(always)]
    pub const fn perm(self) -> crate::common::Reg<regs::DppiPerm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::DppiLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extdomain {
    ptr: *mut u8,
}
unsafe impl Send for Extdomain {}
unsafe impl Sync for Extdomain {}
impl Extdomain {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
    #[inline(always)]
    pub const fn perm(self) -> crate::common::Reg<regs::ExtdomainPerm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashnsc {
    ptr: *mut u8,
}
unsafe impl Send for Flashnsc {}
unsafe impl Sync for Flashnsc {}
impl Flashnsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Define which flash region can contain the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub const fn region(self) -> crate::common::Reg<regs::FlashnscRegion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub const fn size(self) -> crate::common::Reg<regs::FlashnscSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashregion {
    ptr: *mut u8,
}
unsafe impl Send for Flashregion {}
unsafe impl Sync for Flashregion {}
impl Flashregion {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Access permissions for flash region n"]
    #[inline(always)]
    pub const fn perm(self) -> crate::common::Reg<regs::FlashregionPerm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpioport {
    ptr: *mut u8,
}
unsafe impl Send for Gpioport {}
unsafe impl Sync for Gpioport {}
impl Gpioport {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n"]
    #[inline(always)]
    pub const fn perm(self) -> crate::common::Reg<regs::GpioportPerm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::GpioportLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Periphid {
    ptr: *mut u8,
}
unsafe impl Send for Periphid {}
unsafe impl Sync for Periphid {}
impl Periphid {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: List capabilities and access permissions for the peripheral with ID n"]
    #[inline(always)]
    pub const fn perm(self) -> crate::common::Reg<regs::PeriphidPerm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ramnsc {
    ptr: *mut u8,
}
unsafe impl Send for Ramnsc {}
unsafe impl Sync for Ramnsc {}
impl Ramnsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub const fn region(self) -> crate::common::Reg<regs::RamnscRegion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub const fn size(self) -> crate::common::Reg<regs::RamnscSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ramregion {
    ptr: *mut u8,
}
unsafe impl Send for Ramregion {}
unsafe impl Sync for Ramregion {}
impl Ramregion {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Access permissions for RAM region n"]
    #[inline(always)]
    pub const fn perm(self) -> crate::common::Reg<regs::RamregionPerm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "System protection unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpuS {
    ptr: *mut u8,
}
unsafe impl Send for SpuS {}
unsafe impl Sync for SpuS {}
impl SpuS {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "A security violation has been detected for the RAM memory space"]
    #[inline(always)]
    pub const fn events_ramaccerr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "A security violation has been detected for the flash memory space"]
    #[inline(always)]
    pub const fn events_flashaccerr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "A security violation has been detected on one or several peripherals"]
    #[inline(always)]
    pub const fn events_periphaccerr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Publish configuration for event RAMACCERR"]
    #[inline(always)]
    pub const fn publish_ramaccerr(
        self,
    ) -> crate::common::Reg<regs::PublishRamaccerr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Publish configuration for event FLASHACCERR"]
    #[inline(always)]
    pub const fn publish_flashaccerr(
        self,
    ) -> crate::common::Reg<regs::PublishFlashaccerr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Publish configuration for event PERIPHACCERR"]
    #[inline(always)]
    pub const fn publish_periphaccerr(
        self,
    ) -> crate::common::Reg<regs::PublishPeriphaccerr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "Show implemented features for the current device"]
    #[inline(always)]
    pub const fn cap(self) -> crate::common::Reg<regs::Cap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Configure bits to lock down CPU features at runtime"]
    #[inline(always)]
    pub const fn cpulock(self) -> crate::common::Reg<regs::Cpulock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn extdomain(self, n: usize) -> Extdomain {
        assert!(n < 1usize);
        unsafe { Extdomain::from_ptr(self.ptr.add(0x0440usize + n * 4usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn dppi(self, n: usize) -> Dppi {
        assert!(n < 1usize);
        unsafe { Dppi::from_ptr(self.ptr.add(0x0480usize + n * 8usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn gpioport(self, n: usize) -> Gpioport {
        assert!(n < 2usize);
        unsafe { Gpioport::from_ptr(self.ptr.add(0x04c0usize + n * 8usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn flashnsc(self, n: usize) -> Flashnsc {
        assert!(n < 2usize);
        unsafe { Flashnsc::from_ptr(self.ptr.add(0x0500usize + n * 8usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn ramnsc(self, n: usize) -> Ramnsc {
        assert!(n < 2usize);
        unsafe { Ramnsc::from_ptr(self.ptr.add(0x0540usize + n * 8usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn flashregion(self, n: usize) -> Flashregion {
        assert!(n < 64usize);
        unsafe { Flashregion::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn ramregion(self, n: usize) -> Ramregion {
        assert!(n < 64usize);
        unsafe { Ramregion::from_ptr(self.ptr.add(0x0700usize + n * 4usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn periphid(self, n: usize) -> Periphid {
        assert!(n < 256usize);
        unsafe { Periphid::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
