#[doc = "GPIO Tasks and Events"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpiote {
    ptr: *mut u8,
}
unsafe impl Send for Gpiote {}
unsafe impl Sync for Gpiote {}
impl Gpiote {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
    #[inline(always)]
    pub const fn tasks_out(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
    #[inline(always)]
    pub const fn tasks_set(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
    #[inline(always)]
    pub const fn tasks_clr(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Event generated from pin specified in CONFIG\\[n\\].PSEL"]
    #[inline(always)]
    pub const fn events_in(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    #[inline(always)]
    pub const fn events_port(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
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
    #[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\] tasks and IN\\[n\\] event"]
    #[inline(always)]
    pub const fn config(self, n: usize) -> crate::common::Reg<regs::Config, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
