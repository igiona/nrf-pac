#[doc = "Main supply status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mainregstatus(pub u32);
impl Mainregstatus {
    #[doc = "VREGH status"]
    #[inline(always)]
    pub const fn vregh(&self) -> super::vals::Vregh {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Vregh::from_bits(val as u8)
    }
    #[doc = "VREGH status"]
    #[inline(always)]
    pub fn set_vregh(&mut self, val: super::vals::Vregh) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Mainregstatus {
    #[inline(always)]
    fn default() -> Mainregstatus {
        Mainregstatus(0)
    }
}
#[doc = "Power-fail comparator configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pofcon(pub u32);
impl Pofcon {
    #[doc = "Enable or disable power-fail comparator"]
    #[inline(always)]
    pub const fn pof(&self) -> super::vals::Pof {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pof::from_bits(val as u8)
    }
    #[doc = "Enable or disable power-fail comparator"]
    #[inline(always)]
    pub fn set_pof(&mut self, val: super::vals::Pof) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Power-fail comparator threshold setting"]
    #[inline(always)]
    pub const fn threshold(&self) -> super::vals::Threshold {
        let val = (self.0 >> 1usize) & 0x0f;
        super::vals::Threshold::from_bits(val as u8)
    }
    #[doc = "Power-fail comparator threshold setting"]
    #[inline(always)]
    pub fn set_threshold(&mut self, val: super::vals::Threshold) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val.to_bits() as u32) & 0x0f) << 1usize);
    }
    #[doc = "Power-fail comparator threshold setting for voltage supply on VDDH"]
    #[inline(always)]
    pub const fn thresholdvddh(&self) -> super::vals::Thresholdvddh {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Thresholdvddh::from_bits(val as u8)
    }
    #[doc = "Power-fail comparator threshold setting for voltage supply on VDDH"]
    #[inline(always)]
    pub fn set_thresholdvddh(&mut self, val: super::vals::Thresholdvddh) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
}
impl Default for Pofcon {
    #[inline(always)]
    fn default() -> Pofcon {
        Pofcon(0)
    }
}
#[doc = "System OFF register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systemoff(pub u32);
impl Systemoff {
    #[doc = "Enable System OFF mode"]
    #[inline(always)]
    pub const fn systemoff(&self) -> super::vals::Systemoff {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Systemoff::from_bits(val as u8)
    }
    #[doc = "Enable System OFF mode"]
    #[inline(always)]
    pub fn set_systemoff(&mut self, val: super::vals::Systemoff) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Systemoff {
    #[inline(always)]
    fn default() -> Systemoff {
        Systemoff(0)
    }
}
#[doc = "DC/DC enable register for VREGH"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VreghDcdcen(pub u32);
impl VreghDcdcen {
    #[doc = "Enable or disable DC/DC converter"]
    #[inline(always)]
    pub const fn dcdcen(&self) -> super::vals::VreghDcdcenDcdcen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::VreghDcdcenDcdcen::from_bits(val as u8)
    }
    #[doc = "Enable or disable DC/DC converter"]
    #[inline(always)]
    pub fn set_dcdcen(&mut self, val: super::vals::VreghDcdcenDcdcen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for VreghDcdcen {
    #[inline(always)]
    fn default() -> VreghDcdcen {
        VreghDcdcen(0)
    }
}
#[doc = "DC/DC enable register for VREGMAIN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VregmainDcdcen(pub u32);
impl VregmainDcdcen {
    #[doc = "Enable or disable DC/DC converter"]
    #[inline(always)]
    pub const fn dcdcen(&self) -> super::vals::VregmainDcdcenDcdcen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::VregmainDcdcenDcdcen::from_bits(val as u8)
    }
    #[doc = "Enable or disable DC/DC converter"]
    #[inline(always)]
    pub fn set_dcdcen(&mut self, val: super::vals::VregmainDcdcenDcdcen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for VregmainDcdcen {
    #[inline(always)]
    fn default() -> VregmainDcdcen {
        VregmainDcdcen(0)
    }
}
#[doc = "DC/DC enable register for VREGRADIO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VregradioDcdcen(pub u32);
impl VregradioDcdcen {
    #[doc = "Enable or disable DC/DC converter"]
    #[inline(always)]
    pub const fn dcdcen(&self) -> super::vals::VregradioDcdcenDcdcen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::VregradioDcdcenDcdcen::from_bits(val as u8)
    }
    #[doc = "Enable or disable DC/DC converter"]
    #[inline(always)]
    pub fn set_dcdcen(&mut self, val: super::vals::VregradioDcdcenDcdcen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for VregradioDcdcen {
    #[inline(always)]
    fn default() -> VregradioDcdcen {
        VregradioDcdcen(0)
    }
}
