#[doc = "Component ID0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cidr0(pub u32);
impl Cidr0 {
    #[doc = "Preamble\\[0\\]. Contains bits\\[7:0\\] of the component identification code."]
    #[inline(always)]
    pub const fn prmbl_0(&self) -> super::vals::Prmbl0 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Prmbl0::from_bits(val as u8)
    }
    #[doc = "Preamble\\[0\\]. Contains bits\\[7:0\\] of the component identification code."]
    #[inline(always)]
    pub fn set_prmbl_0(&mut self, val: super::vals::Prmbl0) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Cidr0 {
    #[inline(always)]
    fn default() -> Cidr0 {
        Cidr0(0)
    }
}
#[doc = "Component ID1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cidr1(pub u32);
impl Cidr1 {
    #[doc = "Preamble\\[1\\]. Contains bits\\[11:8\\] of the component identification code."]
    #[inline(always)]
    pub const fn prmbl_1(&self) -> super::vals::Prmbl1 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Prmbl1::from_bits(val as u8)
    }
    #[doc = "Preamble\\[1\\]. Contains bits\\[11:8\\] of the component identification code."]
    #[inline(always)]
    pub fn set_prmbl_1(&mut self, val: super::vals::Prmbl1) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\] of the component identification code"]
    #[inline(always)]
    pub const fn class(&self) -> super::vals::Class {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Class::from_bits(val as u8)
    }
    #[doc = "Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\] of the component identification code"]
    #[inline(always)]
    pub fn set_class(&mut self, val: super::vals::Class) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
}
impl Default for Cidr1 {
    #[inline(always)]
    fn default() -> Cidr1 {
        Cidr1(0)
    }
}
#[doc = "Component ID2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cidr2(pub u32);
impl Cidr2 {
    #[doc = "Preamble\\[2\\]. Contains bits\\[23:16\\] of the component identification code."]
    #[inline(always)]
    pub const fn prmbl_2(&self) -> super::vals::Prmbl2 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Prmbl2::from_bits(val as u8)
    }
    #[doc = "Preamble\\[2\\]. Contains bits\\[23:16\\] of the component identification code."]
    #[inline(always)]
    pub fn set_prmbl_2(&mut self, val: super::vals::Prmbl2) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Cidr2 {
    #[inline(always)]
    fn default() -> Cidr2 {
        Cidr2(0)
    }
}
#[doc = "Component ID3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cidr3(pub u32);
impl Cidr3 {
    #[doc = "Preamble\\[3\\]. Contains bits\\[31:24\\] of the component identification code."]
    #[inline(always)]
    pub const fn prmbl_3(&self) -> super::vals::Prmbl3 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Prmbl3::from_bits(val as u8)
    }
    #[doc = "Preamble\\[3\\]. Contains bits\\[31:24\\] of the component identification code."]
    #[inline(always)]
    pub fn set_prmbl_3(&mut self, val: super::vals::Prmbl3) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Cidr3 {
    #[inline(always)]
    fn default() -> Cidr3 {
        Cidr3(0)
    }
}
#[doc = "CTI Application Trigger Clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctiappclear(pub u32);
impl Ctiappclear {
    #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub const fn appclear_0(&self) -> super::vals::Appclear0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Appclear0::from_bits(val as u8)
    }
    #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn set_appclear_0(&mut self, val: super::vals::Appclear0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub const fn appclear_1(&self) -> super::vals::Appclear1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Appclear1::from_bits(val as u8)
    }
    #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn set_appclear_1(&mut self, val: super::vals::Appclear1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub const fn appclear_2(&self) -> super::vals::Appclear2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Appclear2::from_bits(val as u8)
    }
    #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn set_appclear_2(&mut self, val: super::vals::Appclear2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub const fn appclear_3(&self) -> super::vals::Appclear3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Appclear3::from_bits(val as u8)
    }
    #[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn set_appclear_3(&mut self, val: super::vals::Appclear3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Ctiappclear {
    #[inline(always)]
    fn default() -> Ctiappclear {
        Ctiappclear(0)
    }
}
#[doc = "CTI Application Pulse register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctiapppulse(pub u32);
impl Ctiapppulse {
    #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub const fn appulse_0(&self) -> super::vals::Appulse0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Appulse0::from_bits(val as u8)
    }
    #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn set_appulse_0(&mut self, val: super::vals::Appulse0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub const fn appulse_1(&self) -> super::vals::Appulse1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Appulse1::from_bits(val as u8)
    }
    #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn set_appulse_1(&mut self, val: super::vals::Appulse1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub const fn appulse_2(&self) -> super::vals::Appulse2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Appulse2::from_bits(val as u8)
    }
    #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn set_appulse_2(&mut self, val: super::vals::Appulse2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub const fn appulse_3(&self) -> super::vals::Appulse3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Appulse3::from_bits(val as u8)
    }
    #[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn set_appulse_3(&mut self, val: super::vals::Appulse3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Ctiapppulse {
    #[inline(always)]
    fn default() -> Ctiapppulse {
        Ctiapppulse(0)
    }
}
#[doc = "CTI Application Trigger Set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctiappset(pub u32);
impl Ctiappset {
    #[doc = "Application trigger event for channel 0."]
    #[inline(always)]
    pub const fn appset_0(&self) -> super::vals::Appset0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Appset0::from_bits(val as u8)
    }
    #[doc = "Application trigger event for channel 0."]
    #[inline(always)]
    pub fn set_appset_0(&mut self, val: super::vals::Appset0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Application trigger event for channel 1."]
    #[inline(always)]
    pub const fn appset_1(&self) -> super::vals::Appset1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Appset1::from_bits(val as u8)
    }
    #[doc = "Application trigger event for channel 1."]
    #[inline(always)]
    pub fn set_appset_1(&mut self, val: super::vals::Appset1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Application trigger event for channel 2."]
    #[inline(always)]
    pub const fn appset_2(&self) -> super::vals::Appset2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Appset2::from_bits(val as u8)
    }
    #[doc = "Application trigger event for channel 2."]
    #[inline(always)]
    pub fn set_appset_2(&mut self, val: super::vals::Appset2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Application trigger event for channel 3."]
    #[inline(always)]
    pub const fn appset_3(&self) -> super::vals::Appset3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Appset3::from_bits(val as u8)
    }
    #[doc = "Application trigger event for channel 3."]
    #[inline(always)]
    pub fn set_appset_3(&mut self, val: super::vals::Appset3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Ctiappset {
    #[inline(always)]
    fn default() -> Ctiappset {
        Ctiappset(0)
    }
}
#[doc = "CTI Channel In Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctichinstatus(pub u32);
impl Ctichinstatus {
    #[doc = "Shows the status of the ctitrigin 0 input."]
    #[inline(always)]
    pub const fn ctichinstatus_0(&self) -> super::vals::Ctichinstatus0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ctichinstatus0::from_bits(val as u8)
    }
    #[doc = "Shows the status of the ctitrigin 0 input."]
    #[inline(always)]
    pub fn set_ctichinstatus_0(&mut self, val: super::vals::Ctichinstatus0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Shows the status of the ctitrigin 1 input."]
    #[inline(always)]
    pub const fn ctichinstatus_1(&self) -> super::vals::Ctichinstatus1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ctichinstatus1::from_bits(val as u8)
    }
    #[doc = "Shows the status of the ctitrigin 1 input."]
    #[inline(always)]
    pub fn set_ctichinstatus_1(&mut self, val: super::vals::Ctichinstatus1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Shows the status of the ctitrigin 2 input."]
    #[inline(always)]
    pub const fn ctichinstatus_2(&self) -> super::vals::Ctichinstatus2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ctichinstatus2::from_bits(val as u8)
    }
    #[doc = "Shows the status of the ctitrigin 2 input."]
    #[inline(always)]
    pub fn set_ctichinstatus_2(&mut self, val: super::vals::Ctichinstatus2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Shows the status of the ctitrigin 3 input."]
    #[inline(always)]
    pub const fn ctichinstatus_3(&self) -> super::vals::Ctichinstatus3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ctichinstatus3::from_bits(val as u8)
    }
    #[doc = "Shows the status of the ctitrigin 3 input."]
    #[inline(always)]
    pub fn set_ctichinstatus_3(&mut self, val: super::vals::Ctichinstatus3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Ctichinstatus {
    #[inline(always)]
    fn default() -> Ctichinstatus {
        Ctichinstatus(0)
    }
}
#[doc = "CTI Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cticontrol(pub u32);
impl Cticontrol {
    #[doc = "Enables or disables the CTI."]
    #[inline(always)]
    pub const fn glben(&self) -> super::vals::Glben {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Glben::from_bits(val as u8)
    }
    #[doc = "Enables or disables the CTI."]
    #[inline(always)]
    pub fn set_glben(&mut self, val: super::vals::Glben) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Cticontrol {
    #[inline(always)]
    fn default() -> Cticontrol {
        Cticontrol(0)
    }
}
#[doc = "Enable CTI Channel Gate register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctigate(pub u32);
impl Ctigate {
    #[doc = "Enable ctichout0."]
    #[inline(always)]
    pub const fn ctigateen_0(&self) -> super::vals::Ctigateen0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ctigateen0::from_bits(val as u8)
    }
    #[doc = "Enable ctichout0."]
    #[inline(always)]
    pub fn set_ctigateen_0(&mut self, val: super::vals::Ctigateen0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable ctichout1."]
    #[inline(always)]
    pub const fn ctigateen_1(&self) -> super::vals::Ctigateen1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ctigateen1::from_bits(val as u8)
    }
    #[doc = "Enable ctichout1."]
    #[inline(always)]
    pub fn set_ctigateen_1(&mut self, val: super::vals::Ctigateen1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable ctichout2."]
    #[inline(always)]
    pub const fn ctigateen_2(&self) -> super::vals::Ctigateen2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ctigateen2::from_bits(val as u8)
    }
    #[doc = "Enable ctichout2."]
    #[inline(always)]
    pub fn set_ctigateen_2(&mut self, val: super::vals::Ctigateen2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable ctichout3."]
    #[inline(always)]
    pub const fn ctigateen_3(&self) -> super::vals::Ctigateen3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ctigateen3::from_bits(val as u8)
    }
    #[doc = "Enable ctichout3."]
    #[inline(always)]
    pub fn set_ctigateen_3(&mut self, val: super::vals::Ctigateen3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Ctigate {
    #[inline(always)]
    fn default() -> Ctigate {
        Ctigate(0)
    }
}
#[doc = "Description collection: CTI Trigger input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctiinen(pub u32);
impl Ctiinen {
    #[doc = "Enables a cross trigger event to channel 0 when a ctitrigin input is activated."]
    #[inline(always)]
    pub const fn triginen_0(&self) -> super::vals::Triginen0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Triginen0::from_bits(val as u8)
    }
    #[doc = "Enables a cross trigger event to channel 0 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn set_triginen_0(&mut self, val: super::vals::Triginen0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables a cross trigger event to channel 1 when a ctitrigin input is activated."]
    #[inline(always)]
    pub const fn triginen_1(&self) -> super::vals::Triginen1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Triginen1::from_bits(val as u8)
    }
    #[doc = "Enables a cross trigger event to channel 1 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn set_triginen_1(&mut self, val: super::vals::Triginen1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables a cross trigger event to channel 2 when a ctitrigin input is activated."]
    #[inline(always)]
    pub const fn triginen_2(&self) -> super::vals::Triginen2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Triginen2::from_bits(val as u8)
    }
    #[doc = "Enables a cross trigger event to channel 2 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn set_triginen_2(&mut self, val: super::vals::Triginen2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables a cross trigger event to channel 3 when a ctitrigin input is activated."]
    #[inline(always)]
    pub const fn triginen_3(&self) -> super::vals::Triginen3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Triginen3::from_bits(val as u8)
    }
    #[doc = "Enables a cross trigger event to channel 3 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn set_triginen_3(&mut self, val: super::vals::Triginen3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Ctiinen {
    #[inline(always)]
    fn default() -> Ctiinen {
        Ctiinen(0)
    }
}
#[doc = "CTI Interrupt Acknowledge register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctiintack(pub u32);
impl Ctiintack {
    #[doc = "Processor debug request"]
    #[inline(always)]
    pub const fn debugreq(&self) -> super::vals::CtiintackDebugreq {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CtiintackDebugreq::from_bits(val as u8)
    }
    #[doc = "Processor debug request"]
    #[inline(always)]
    pub fn set_debugreq(&mut self, val: super::vals::CtiintackDebugreq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Processor Restart"]
    #[inline(always)]
    pub const fn cpurestart(&self) -> super::vals::CtiintackCpurestart {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::CtiintackCpurestart::from_bits(val as u8)
    }
    #[doc = "Processor Restart"]
    #[inline(always)]
    pub fn set_cpurestart(&mut self, val: super::vals::CtiintackCpurestart) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn unused0(&self) -> super::vals::CtiintackUnused0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CtiintackUnused0::from_bits(val as u8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_unused0(&mut self, val: super::vals::CtiintackUnused0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn unused1(&self) -> super::vals::CtiintackUnused1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::CtiintackUnused1::from_bits(val as u8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_unused1(&mut self, val: super::vals::CtiintackUnused1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ETM Event Input 0"]
    #[inline(always)]
    pub const fn etmevtin0(&self) -> super::vals::CtiintackEtmevtin0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CtiintackEtmevtin0::from_bits(val as u8)
    }
    #[doc = "ETM Event Input 0"]
    #[inline(always)]
    pub fn set_etmevtin0(&mut self, val: super::vals::CtiintackEtmevtin0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "ETM Event Input 1"]
    #[inline(always)]
    pub const fn etmevtin1(&self) -> super::vals::CtiintackEtmevtin1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::CtiintackEtmevtin1::from_bits(val as u8)
    }
    #[doc = "ETM Event Input 1"]
    #[inline(always)]
    pub fn set_etmevtin1(&mut self, val: super::vals::CtiintackEtmevtin1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "ETM Event Input 2"]
    #[inline(always)]
    pub const fn etmevtin2(&self) -> super::vals::CtiintackEtmevtin2 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::CtiintackEtmevtin2::from_bits(val as u8)
    }
    #[doc = "ETM Event Input 2"]
    #[inline(always)]
    pub fn set_etmevtin2(&mut self, val: super::vals::CtiintackEtmevtin2) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "ETM Event Input 3"]
    #[inline(always)]
    pub const fn etmevtin3(&self) -> super::vals::CtiintackEtmevtin3 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CtiintackEtmevtin3::from_bits(val as u8)
    }
    #[doc = "ETM Event Input 3"]
    #[inline(always)]
    pub fn set_etmevtin3(&mut self, val: super::vals::CtiintackEtmevtin3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Ctiintack {
    #[inline(always)]
    fn default() -> Ctiintack {
        Ctiintack(0)
    }
}
#[doc = "Description collection: CTI Trigger output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctiouten(pub u32);
impl Ctiouten {
    #[doc = "Enables a cross trigger event to ctitrigout when channel 0 is activated."]
    #[inline(always)]
    pub const fn trigouten_0(&self) -> super::vals::Trigouten0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Trigouten0::from_bits(val as u8)
    }
    #[doc = "Enables a cross trigger event to ctitrigout when channel 0 is activated."]
    #[inline(always)]
    pub fn set_trigouten_0(&mut self, val: super::vals::Trigouten0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables a cross trigger event to ctitrigout when channel 1 is activated."]
    #[inline(always)]
    pub const fn trigouten_1(&self) -> super::vals::Trigouten1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Trigouten1::from_bits(val as u8)
    }
    #[doc = "Enables a cross trigger event to ctitrigout when channel 1 is activated."]
    #[inline(always)]
    pub fn set_trigouten_1(&mut self, val: super::vals::Trigouten1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables a cross trigger event to ctitrigout when channel 2 is activated."]
    #[inline(always)]
    pub const fn trigouten_2(&self) -> super::vals::Trigouten2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Trigouten2::from_bits(val as u8)
    }
    #[doc = "Enables a cross trigger event to ctitrigout when channel 2 is activated."]
    #[inline(always)]
    pub fn set_trigouten_2(&mut self, val: super::vals::Trigouten2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables a cross trigger event to ctitrigout when channel 3 is activated."]
    #[inline(always)]
    pub const fn trigouten_3(&self) -> super::vals::Trigouten3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Trigouten3::from_bits(val as u8)
    }
    #[doc = "Enables a cross trigger event to ctitrigout when channel 3 is activated."]
    #[inline(always)]
    pub fn set_trigouten_3(&mut self, val: super::vals::Trigouten3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Ctiouten {
    #[inline(always)]
    fn default() -> Ctiouten {
        Ctiouten(0)
    }
}
#[doc = "CTI Trigger In Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctitriginstatus(pub u32);
impl Ctitriginstatus {
    #[doc = "Processor Halted"]
    #[inline(always)]
    pub const fn cpuhalted(&self) -> super::vals::Cpuhalted {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cpuhalted::from_bits(val as u8)
    }
    #[doc = "Processor Halted"]
    #[inline(always)]
    pub fn set_cpuhalted(&mut self, val: super::vals::Cpuhalted) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DWT Comparator Output 0"]
    #[inline(always)]
    pub const fn dwtcompout0(&self) -> super::vals::Dwtcompout0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dwtcompout0::from_bits(val as u8)
    }
    #[doc = "DWT Comparator Output 0"]
    #[inline(always)]
    pub fn set_dwtcompout0(&mut self, val: super::vals::Dwtcompout0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DWT Comparator Output 1"]
    #[inline(always)]
    pub const fn dwtcompout1(&self) -> super::vals::Dwtcompout1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dwtcompout1::from_bits(val as u8)
    }
    #[doc = "DWT Comparator Output 1"]
    #[inline(always)]
    pub fn set_dwtcompout1(&mut self, val: super::vals::Dwtcompout1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DWT Comparator Output 2"]
    #[inline(always)]
    pub const fn dwtcompout2(&self) -> super::vals::Dwtcompout2 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dwtcompout2::from_bits(val as u8)
    }
    #[doc = "DWT Comparator Output 2"]
    #[inline(always)]
    pub fn set_dwtcompout2(&mut self, val: super::vals::Dwtcompout2) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ETM Event Output 0"]
    #[inline(always)]
    pub const fn etmevtout0(&self) -> super::vals::Etmevtout0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Etmevtout0::from_bits(val as u8)
    }
    #[doc = "ETM Event Output 0"]
    #[inline(always)]
    pub fn set_etmevtout0(&mut self, val: super::vals::Etmevtout0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "ETM Event Output 1"]
    #[inline(always)]
    pub const fn etmevtout1(&self) -> super::vals::Etmevtout1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Etmevtout1::from_bits(val as u8)
    }
    #[doc = "ETM Event Output 1"]
    #[inline(always)]
    pub fn set_etmevtout1(&mut self, val: super::vals::Etmevtout1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn unused0(&self) -> super::vals::CtitriginstatusUnused0 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::CtitriginstatusUnused0::from_bits(val as u8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_unused0(&mut self, val: super::vals::CtitriginstatusUnused0) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn unused1(&self) -> super::vals::CtitriginstatusUnused1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CtitriginstatusUnused1::from_bits(val as u8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_unused1(&mut self, val: super::vals::CtitriginstatusUnused1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Ctitriginstatus {
    #[inline(always)]
    fn default() -> Ctitriginstatus {
        Ctitriginstatus(0)
    }
}
#[doc = "CTI Trigger Out Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctitrigoutstatus(pub u32);
impl Ctitrigoutstatus {
    #[doc = "Processor debug request"]
    #[inline(always)]
    pub const fn debugreq(&self) -> super::vals::CtitrigoutstatusDebugreq {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CtitrigoutstatusDebugreq::from_bits(val as u8)
    }
    #[doc = "Processor debug request"]
    #[inline(always)]
    pub fn set_debugreq(&mut self, val: super::vals::CtitrigoutstatusDebugreq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Processor Restart"]
    #[inline(always)]
    pub const fn cpurestart(&self) -> super::vals::CtitrigoutstatusCpurestart {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::CtitrigoutstatusCpurestart::from_bits(val as u8)
    }
    #[doc = "Processor Restart"]
    #[inline(always)]
    pub fn set_cpurestart(&mut self, val: super::vals::CtitrigoutstatusCpurestart) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn unused0(&self) -> super::vals::CtitrigoutstatusUnused0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CtitrigoutstatusUnused0::from_bits(val as u8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_unused0(&mut self, val: super::vals::CtitrigoutstatusUnused0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn unused1(&self) -> super::vals::CtitrigoutstatusUnused1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::CtitrigoutstatusUnused1::from_bits(val as u8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_unused1(&mut self, val: super::vals::CtitrigoutstatusUnused1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ETM Event Input 0"]
    #[inline(always)]
    pub const fn etmevtin0(&self) -> super::vals::CtitrigoutstatusEtmevtin0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CtitrigoutstatusEtmevtin0::from_bits(val as u8)
    }
    #[doc = "ETM Event Input 0"]
    #[inline(always)]
    pub fn set_etmevtin0(&mut self, val: super::vals::CtitrigoutstatusEtmevtin0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "ETM Event Input 1"]
    #[inline(always)]
    pub const fn etmevtin1(&self) -> super::vals::CtitrigoutstatusEtmevtin1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::CtitrigoutstatusEtmevtin1::from_bits(val as u8)
    }
    #[doc = "ETM Event Input 1"]
    #[inline(always)]
    pub fn set_etmevtin1(&mut self, val: super::vals::CtitrigoutstatusEtmevtin1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "ETM Event Input 2"]
    #[inline(always)]
    pub const fn etmevtin2(&self) -> super::vals::CtitrigoutstatusEtmevtin2 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::CtitrigoutstatusEtmevtin2::from_bits(val as u8)
    }
    #[doc = "ETM Event Input 2"]
    #[inline(always)]
    pub fn set_etmevtin2(&mut self, val: super::vals::CtitrigoutstatusEtmevtin2) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "ETM Event Input 3"]
    #[inline(always)]
    pub const fn etmevtin3(&self) -> super::vals::CtitrigoutstatusEtmevtin3 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CtitrigoutstatusEtmevtin3::from_bits(val as u8)
    }
    #[doc = "ETM Event Input 3"]
    #[inline(always)]
    pub fn set_etmevtin3(&mut self, val: super::vals::CtitrigoutstatusEtmevtin3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Ctitrigoutstatus {
    #[inline(always)]
    fn default() -> Ctitrigoutstatus {
        Ctitrigoutstatus(0)
    }
}
#[doc = "Device Architecture register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devarch(pub u32);
impl Devarch {
    #[doc = "Contains the CTI device architecture."]
    #[inline(always)]
    pub const fn architecture(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Contains the CTI device architecture."]
    #[inline(always)]
    pub fn set_architecture(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Devarch {
    #[inline(always)]
    fn default() -> Devarch {
        Devarch(0)
    }
}
#[doc = "Device Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devid(pub u32);
impl Devid {
    #[doc = "Indicates the number of multiplexers available on Trigger Inputs and Trigger Outputs that are using asicctl. The default value of 0b00000 indicates that no multiplexing is present."]
    #[inline(always)]
    pub const fn extmuxnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates the number of multiplexers available on Trigger Inputs and Trigger Outputs that are using asicctl. The default value of 0b00000 indicates that no multiplexing is present."]
    #[inline(always)]
    pub fn set_extmuxnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Number of ECT triggers available."]
    #[inline(always)]
    pub const fn numtrig(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of ECT triggers available."]
    #[inline(always)]
    pub fn set_numtrig(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of ECT channels available."]
    #[inline(always)]
    pub const fn numch(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of ECT channels available."]
    #[inline(always)]
    pub fn set_numch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Devid {
    #[inline(always)]
    fn default() -> Devid {
        Devid(0)
    }
}
#[doc = "Device Type Identifier register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devtype(pub u32);
impl Devtype {
    #[doc = "Major classification of the type of the debug component as specified in the Arm Architecture Specification for this debug and trace component."]
    #[inline(always)]
    pub const fn major(&self) -> super::vals::Major {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Major::from_bits(val as u8)
    }
    #[doc = "Major classification of the type of the debug component as specified in the Arm Architecture Specification for this debug and trace component."]
    #[inline(always)]
    pub fn set_major(&mut self, val: super::vals::Major) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Sub-classification of the type of the debug component as specified in the Arm Architecture Specification within the major classification as specified in the MAJOR field."]
    #[inline(always)]
    pub const fn sub(&self) -> super::vals::Sub {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Sub::from_bits(val as u8)
    }
    #[doc = "Sub-classification of the type of the debug component as specified in the Arm Architecture Specification within the major classification as specified in the MAJOR field."]
    #[inline(always)]
    pub fn set_sub(&mut self, val: super::vals::Sub) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
}
impl Default for Devtype {
    #[inline(always)]
    fn default() -> Devtype {
        Devtype(0)
    }
}
#[doc = "Peripheral ID0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr0(pub u32);
impl Pidr0 {
    #[doc = "Bits\\[7:0\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
    #[inline(always)]
    pub const fn part_0(&self) -> super::vals::Part0 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Part0::from_bits(val as u8)
    }
    #[doc = "Bits\\[7:0\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
    #[inline(always)]
    pub fn set_part_0(&mut self, val: super::vals::Part0) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Pidr0 {
    #[inline(always)]
    fn default() -> Pidr0 {
        Pidr0(0)
    }
}
#[doc = "Peripheral ID1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr1(pub u32);
impl Pidr1 {
    #[doc = "Bits\\[11:8\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
    #[inline(always)]
    pub const fn part_1(&self) -> super::vals::Part1 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Part1::from_bits(val as u8)
    }
    #[doc = "Bits\\[11:8\\] of the 12-bit part number of the component. The designer of the component assigns this part number."]
    #[inline(always)]
    pub fn set_part_1(&mut self, val: super::vals::Part1) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub const fn des_0(&self) -> super::vals::Des0 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Des0::from_bits(val as u8)
    }
    #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn set_des_0(&mut self, val: super::vals::Des0) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
}
impl Default for Pidr1 {
    #[inline(always)]
    fn default() -> Pidr1 {
        Pidr1(0)
    }
}
#[doc = "Peripheral ID2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr2(pub u32);
impl Pidr2 {
    #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub const fn des_1(&self) -> super::vals::Des1 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Des1::from_bits(val as u8)
    }
    #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn set_des_1(&mut self, val: super::vals::Des1) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Always 1. Indicates that the JEDEC-assigned designer ID is used."]
    #[inline(always)]
    pub const fn jedec(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Always 1. Indicates that the JEDEC-assigned designer ID is used."]
    #[inline(always)]
    pub fn set_jedec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Peripheral revision"]
    #[inline(always)]
    pub const fn revision(&self) -> super::vals::Revision {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Revision::from_bits(val as u8)
    }
    #[doc = "Peripheral revision"]
    #[inline(always)]
    pub fn set_revision(&mut self, val: super::vals::Revision) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
}
impl Default for Pidr2 {
    #[inline(always)]
    fn default() -> Pidr2 {
        Pidr2(0)
    }
}
#[doc = "Peripheral ID3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr3(pub u32);
impl Pidr3 {
    #[doc = "Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component."]
    #[inline(always)]
    pub const fn cmod(&self) -> super::vals::Cmod {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Cmod::from_bits(val as u8)
    }
    #[doc = "Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component."]
    #[inline(always)]
    pub fn set_cmod(&mut self, val: super::vals::Cmod) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. Arm recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000."]
    #[inline(always)]
    pub const fn revand(&self) -> super::vals::Revand {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Revand::from_bits(val as u8)
    }
    #[doc = "Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. Arm recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000."]
    #[inline(always)]
    pub fn set_revand(&mut self, val: super::vals::Revand) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
}
impl Default for Pidr3 {
    #[inline(always)]
    fn default() -> Pidr3 {
        Pidr3(0)
    }
}
#[doc = "Peripheral ID4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr4(pub u32);
impl Pidr4 {
    #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub const fn des_2(&self) -> super::vals::Des2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Des2::from_bits(val as u8)
    }
    #[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn set_des_2(&mut self, val: super::vals::Des2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Always 0b0000. Indicates that the device only occupies 4KB of memory."]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Always 0b0000. Indicates that the device only occupies 4KB of memory."]
    #[inline(always)]
    pub fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Pidr4 {
    #[inline(always)]
    fn default() -> Pidr4 {
        Pidr4(0)
    }
}
