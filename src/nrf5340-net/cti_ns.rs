#[doc = "Cross-Trigger Interface control. NOTE: this is not a separate peripheral, but describes CM33 functionality."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtiNs {
    ptr: *mut u8,
}
unsafe impl Send for CtiNs {}
unsafe impl Sync for CtiNs {}
impl CtiNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CTI Control register"]
    #[inline(always)]
    pub const fn cticontrol(self) -> crate::common::Reg<regs::Cticontrol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CTI Interrupt Acknowledge register"]
    #[inline(always)]
    pub const fn ctiintack(self) -> crate::common::Reg<regs::Ctiintack, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "CTI Application Trigger Set register"]
    #[inline(always)]
    pub const fn ctiappset(self) -> crate::common::Reg<regs::Ctiappset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "CTI Application Trigger Clear register"]
    #[inline(always)]
    pub const fn ctiappclear(self) -> crate::common::Reg<regs::Ctiappclear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "CTI Application Pulse register"]
    #[inline(always)]
    pub const fn ctiapppulse(self) -> crate::common::Reg<regs::Ctiapppulse, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Description collection: CTI Trigger input"]
    #[inline(always)]
    pub const fn ctiinen(self, n: usize) -> crate::common::Reg<regs::Ctiinen, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: CTI Trigger output"]
    #[inline(always)]
    pub const fn ctiouten(self, n: usize) -> crate::common::Reg<regs::Ctiouten, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "CTI Trigger In Status register"]
    #[inline(always)]
    pub const fn ctitriginstatus(
        self,
    ) -> crate::common::Reg<regs::Ctitriginstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "CTI Trigger Out Status register"]
    #[inline(always)]
    pub const fn ctitrigoutstatus(
        self,
    ) -> crate::common::Reg<regs::Ctitrigoutstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "CTI Channel In Status register"]
    #[inline(always)]
    pub const fn ctichinstatus(self) -> crate::common::Reg<regs::Ctichinstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "Enable CTI Channel Gate register"]
    #[inline(always)]
    pub const fn ctigate(self) -> crate::common::Reg<regs::Ctigate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "Device Architecture register"]
    #[inline(always)]
    pub const fn devarch(self) -> crate::common::Reg<regs::Devarch, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fbcusize) as _) }
    }
    #[doc = "Device Configuration register"]
    #[inline(always)]
    pub const fn devid(self) -> crate::common::Reg<regs::Devid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fc8usize) as _) }
    }
    #[doc = "Device Type Identifier register"]
    #[inline(always)]
    pub const fn devtype(self) -> crate::common::Reg<regs::Devtype, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fccusize) as _) }
    }
    #[doc = "Peripheral ID4 Register"]
    #[inline(always)]
    pub const fn pidr4(self) -> crate::common::Reg<regs::Pidr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd0usize) as _) }
    }
    #[doc = "Peripheral ID5 register"]
    #[inline(always)]
    pub const fn pidr5(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd4usize) as _) }
    }
    #[doc = "Peripheral ID6 register"]
    #[inline(always)]
    pub const fn pidr6(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd8usize) as _) }
    }
    #[doc = "Peripheral ID7 register"]
    #[inline(always)]
    pub const fn pidr7(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fdcusize) as _) }
    }
    #[doc = "Peripheral ID0 Register"]
    #[inline(always)]
    pub const fn pidr0(self) -> crate::common::Reg<regs::Pidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe0usize) as _) }
    }
    #[doc = "Peripheral ID1 Register"]
    #[inline(always)]
    pub const fn pidr1(self) -> crate::common::Reg<regs::Pidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe4usize) as _) }
    }
    #[doc = "Peripheral ID2 Register"]
    #[inline(always)]
    pub const fn pidr2(self) -> crate::common::Reg<regs::Pidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe8usize) as _) }
    }
    #[doc = "Peripheral ID3 Register"]
    #[inline(always)]
    pub const fn pidr3(self) -> crate::common::Reg<regs::Pidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fecusize) as _) }
    }
    #[doc = "Component ID0 Register"]
    #[inline(always)]
    pub const fn cidr0(self) -> crate::common::Reg<regs::Cidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff0usize) as _) }
    }
    #[doc = "Component ID1 Register"]
    #[inline(always)]
    pub const fn cidr1(self) -> crate::common::Reg<regs::Cidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff4usize) as _) }
    }
    #[doc = "Component ID2 Register"]
    #[inline(always)]
    pub const fn cidr2(self) -> crate::common::Reg<regs::Cidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff8usize) as _) }
    }
    #[doc = "Component ID3 Register"]
    #[inline(always)]
    pub const fn cidr3(self) -> crate::common::Reg<regs::Cidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
