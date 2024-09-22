#[doc = "Peripheral events."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EventsPregion {
    ptr: *mut u8,
}
unsafe impl Send for EventsPregion {}
unsafe impl Sync for EventsPregion {}
impl EventsPregion {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Write access to peripheral region n detected"]
    #[inline(always)]
    pub const fn wa(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Read access to peripheral region n detected"]
    #[inline(always)]
    pub const fn ra(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Peripheral events."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EventsRegion {
    ptr: *mut u8,
}
unsafe impl Send for EventsRegion {}
unsafe impl Sync for EventsRegion {}
impl EventsRegion {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Write access to region n detected"]
    #[inline(always)]
    pub const fn wa(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Read access to region n detected"]
    #[inline(always)]
    pub const fn ra(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Memory Watch Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwu {
    ptr: *mut u8,
}
unsafe impl Send for Mwu {}
unsafe impl Sync for Mwu {}
impl Mwu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Peripheral events."]
    #[inline(always)]
    pub const fn events_region(self, n: usize) -> EventsRegion {
        assert!(n < 4usize);
        unsafe { EventsRegion::from_ptr(self.ptr.add(0x0100usize + n * 8usize) as _) }
    }
    #[doc = "Peripheral events."]
    #[inline(always)]
    pub const fn events_pregion(self, n: usize) -> EventsPregion {
        assert!(n < 2usize);
        unsafe { EventsPregion::from_ptr(self.ptr.add(0x0160usize + n * 8usize) as _) }
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
    #[doc = "Enable or disable interrupt"]
    #[inline(always)]
    pub const fn nmien(self) -> crate::common::Reg<regs::Nmien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub const fn nmienset(self) -> crate::common::Reg<regs::Nmien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0324usize) as _) }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub const fn nmienclr(self) -> crate::common::Reg<regs::Nmien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0328usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn perregion(self, n: usize) -> Perregion {
        assert!(n < 2usize);
        unsafe { Perregion::from_ptr(self.ptr.add(0x0400usize + n * 8usize) as _) }
    }
    #[doc = "Enable/disable regions watch"]
    #[inline(always)]
    pub const fn regionen(self) -> crate::common::Reg<regs::Regionen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Enable regions watch"]
    #[inline(always)]
    pub const fn regionenset(self) -> crate::common::Reg<regs::Regionen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "Disable regions watch"]
    #[inline(always)]
    pub const fn regionenclr(self) -> crate::common::Reg<regs::Regionen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn region(self, n: usize) -> Region {
        assert!(n < 4usize);
        unsafe { Region::from_ptr(self.ptr.add(0x0600usize + n * 16usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn pregion(self, n: usize) -> Pregion {
        assert!(n < 2usize);
        unsafe { Pregion::from_ptr(self.ptr.add(0x06c0usize + n * 16usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perregion {
    ptr: *mut u8,
}
unsafe impl Send for Perregion {}
unsafe impl Sync for Perregion {}
impl Perregion {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Source of event/interrupt in region n, write access detected while corresponding subregion was enabled for watching"]
    #[inline(always)]
    pub const fn substatwa(self) -> crate::common::Reg<regs::Substatwa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Source of event/interrupt in region n, read access detected while corresponding subregion was enabled for watching"]
    #[inline(always)]
    pub const fn substatra(self) -> crate::common::Reg<regs::Substatra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pregion {
    ptr: *mut u8,
}
unsafe impl Send for Pregion {}
unsafe impl Sync for Pregion {}
impl Pregion {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Reserved for future use"]
    #[inline(always)]
    pub const fn start(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: Reserved for future use"]
    #[inline(always)]
    pub const fn end(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Description cluster: Subregions of region n"]
    #[inline(always)]
    pub const fn subs(self) -> crate::common::Reg<regs::Subs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region {
    ptr: *mut u8,
}
unsafe impl Send for Region {}
unsafe impl Sync for Region {}
impl Region {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description cluster: Start address for region n"]
    #[inline(always)]
    pub const fn start(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Description cluster: End address of region n"]
    #[inline(always)]
    pub const fn end(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs;
