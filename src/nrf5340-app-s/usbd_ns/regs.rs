#[doc = "SETUP data, byte 0, bmRequestType"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmrequesttype(pub u32);
impl Bmrequesttype {
    #[doc = "Data transfer type"]
    #[inline(always)]
    pub const fn recipient(&self) -> super::vals::Recipient {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Recipient::from_bits(val as u8)
    }
    #[doc = "Data transfer type"]
    #[inline(always)]
    pub fn set_recipient(&mut self, val: super::vals::Recipient) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Data transfer type"]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::Type {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Type::from_bits(val as u8)
    }
    #[doc = "Data transfer type"]
    #[inline(always)]
    pub fn set_type_(&mut self, val: super::vals::Type) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Data transfer direction"]
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::Direction {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Direction::from_bits(val as u8)
    }
    #[doc = "Data transfer direction"]
    #[inline(always)]
    pub fn set_direction(&mut self, val: super::vals::Direction) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Bmrequesttype {
    #[inline(always)]
    fn default() -> Bmrequesttype {
        Bmrequesttype(0)
    }
}
#[doc = "SETUP data, byte 1, bRequest"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brequest(pub u32);
impl Brequest {
    #[doc = "SETUP data, byte 1, bRequest. Values provided for standard requests only, user must implement class and vendor values."]
    #[inline(always)]
    pub const fn brequest(&self) -> super::vals::Brequest {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Brequest::from_bits(val as u8)
    }
    #[doc = "SETUP data, byte 1, bRequest. Values provided for standard requests only, user must implement class and vendor values."]
    #[inline(always)]
    pub fn set_brequest(&mut self, val: super::vals::Brequest) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Brequest {
    #[inline(always)]
    fn default() -> Brequest {
        Brequest(0)
    }
}
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpdmvalue(pub u32);
impl Dpdmvalue {
    #[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task"]
    #[inline(always)]
    pub const fn state(&self) -> super::vals::State {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::State::from_bits(val as u8)
    }
    #[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task"]
    #[inline(always)]
    pub fn set_state(&mut self, val: super::vals::State) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Dpdmvalue {
    #[inline(always)]
    fn default() -> Dpdmvalue {
        Dpdmvalue(0)
    }
}
#[doc = "Data toggle control and status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtoggle(pub u32);
impl Dtoggle {
    #[doc = "Select bulk endpoint number"]
    #[inline(always)]
    pub const fn ep(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Select bulk endpoint number"]
    #[inline(always)]
    pub fn set_ep(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Selects IN or OUT endpoint"]
    #[inline(always)]
    pub const fn io(&self) -> super::vals::DtoggleIo {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DtoggleIo::from_bits(val as u8)
    }
    #[doc = "Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn set_io(&mut self, val: super::vals::DtoggleIo) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Data toggle value"]
    #[inline(always)]
    pub const fn value(&self) -> super::vals::Value {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Value::from_bits(val as u8)
    }
    #[doc = "Data toggle value"]
    #[inline(always)]
    pub fn set_value(&mut self, val: super::vals::Value) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for Dtoggle {
    #[inline(always)]
    fn default() -> Dtoggle {
        Dtoggle(0)
    }
}
#[doc = "Enable USB"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable USB"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enable USB"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Enable {
    #[inline(always)]
    fn default() -> Enable {
        Enable(0)
    }
}
#[doc = "Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epdatastatus(pub u32);
impl Epdatastatus {
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin1(&self) -> super::vals::EpdatastatusEpin1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EpdatastatusEpin1::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin1(&mut self, val: super::vals::EpdatastatusEpin1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin2(&self) -> super::vals::EpdatastatusEpin2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::EpdatastatusEpin2::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin2(&mut self, val: super::vals::EpdatastatusEpin2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin3(&self) -> super::vals::EpdatastatusEpin3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::EpdatastatusEpin3::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin3(&mut self, val: super::vals::EpdatastatusEpin3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin4(&self) -> super::vals::EpdatastatusEpin4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EpdatastatusEpin4::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin4(&mut self, val: super::vals::EpdatastatusEpin4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin5(&self) -> super::vals::EpdatastatusEpin5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::EpdatastatusEpin5::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin5(&mut self, val: super::vals::EpdatastatusEpin5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin6(&self) -> super::vals::EpdatastatusEpin6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::EpdatastatusEpin6::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin6(&mut self, val: super::vals::EpdatastatusEpin6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin7(&self) -> super::vals::EpdatastatusEpin7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::EpdatastatusEpin7::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin7(&mut self, val: super::vals::EpdatastatusEpin7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout1(&self) -> super::vals::EpdatastatusEpout1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::EpdatastatusEpout1::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout1(&mut self, val: super::vals::EpdatastatusEpout1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout2(&self) -> super::vals::EpdatastatusEpout2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::EpdatastatusEpout2::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout2(&mut self, val: super::vals::EpdatastatusEpout2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout3(&self) -> super::vals::EpdatastatusEpout3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::EpdatastatusEpout3::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout3(&mut self, val: super::vals::EpdatastatusEpout3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout4(&self) -> super::vals::EpdatastatusEpout4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::EpdatastatusEpout4::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout4(&mut self, val: super::vals::EpdatastatusEpout4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout5(&self) -> super::vals::EpdatastatusEpout5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::EpdatastatusEpout5::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout5(&mut self, val: super::vals::EpdatastatusEpout5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout6(&self) -> super::vals::EpdatastatusEpout6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::EpdatastatusEpout6::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout6(&mut self, val: super::vals::EpdatastatusEpout6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout7(&self) -> super::vals::EpdatastatusEpout7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::EpdatastatusEpout7::from_bits(val as u8)
    }
    #[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout7(&mut self, val: super::vals::EpdatastatusEpout7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Epdatastatus {
    #[inline(always)]
    fn default() -> Epdatastatus {
        Epdatastatus(0)
    }
}
#[doc = "Description collection: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epin(pub u32);
impl Epin {
    #[doc = "IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub const fn getstatus(&self) -> super::vals::EpinGetstatus {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::EpinGetstatus::from_bits(val as u16)
    }
    #[doc = "IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub fn set_getstatus(&mut self, val: super::vals::EpinGetstatus) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Epin {
    #[inline(always)]
    fn default() -> Epin {
        Epin(0)
    }
}
#[doc = "Description cluster: Number of bytes transferred in the last transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpinAmount(pub u32);
impl EpinAmount {
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn set_amount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for EpinAmount {
    #[inline(always)]
    fn default() -> EpinAmount {
        EpinAmount(0)
    }
}
#[doc = "Description cluster: Maximum number of bytes to transfer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpinMaxcnt(pub u32);
impl EpinMaxcnt {
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn set_maxcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for EpinMaxcnt {
    #[inline(always)]
    fn default() -> EpinMaxcnt {
        EpinMaxcnt(0)
    }
}
#[doc = "Endpoint IN enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epinen(pub u32);
impl Epinen {
    #[doc = "Enable IN endpoint 0"]
    #[inline(always)]
    pub const fn in0(&self) -> super::vals::In0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::In0::from_bits(val as u8)
    }
    #[doc = "Enable IN endpoint 0"]
    #[inline(always)]
    pub fn set_in0(&mut self, val: super::vals::In0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable IN endpoint 1"]
    #[inline(always)]
    pub const fn in1(&self) -> super::vals::In1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::In1::from_bits(val as u8)
    }
    #[doc = "Enable IN endpoint 1"]
    #[inline(always)]
    pub fn set_in1(&mut self, val: super::vals::In1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable IN endpoint 2"]
    #[inline(always)]
    pub const fn in2(&self) -> super::vals::In2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::In2::from_bits(val as u8)
    }
    #[doc = "Enable IN endpoint 2"]
    #[inline(always)]
    pub fn set_in2(&mut self, val: super::vals::In2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable IN endpoint 3"]
    #[inline(always)]
    pub const fn in3(&self) -> super::vals::In3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::In3::from_bits(val as u8)
    }
    #[doc = "Enable IN endpoint 3"]
    #[inline(always)]
    pub fn set_in3(&mut self, val: super::vals::In3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable IN endpoint 4"]
    #[inline(always)]
    pub const fn in4(&self) -> super::vals::In4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::In4::from_bits(val as u8)
    }
    #[doc = "Enable IN endpoint 4"]
    #[inline(always)]
    pub fn set_in4(&mut self, val: super::vals::In4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable IN endpoint 5"]
    #[inline(always)]
    pub const fn in5(&self) -> super::vals::In5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::In5::from_bits(val as u8)
    }
    #[doc = "Enable IN endpoint 5"]
    #[inline(always)]
    pub fn set_in5(&mut self, val: super::vals::In5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable IN endpoint 6"]
    #[inline(always)]
    pub const fn in6(&self) -> super::vals::In6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::In6::from_bits(val as u8)
    }
    #[doc = "Enable IN endpoint 6"]
    #[inline(always)]
    pub fn set_in6(&mut self, val: super::vals::In6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable IN endpoint 7"]
    #[inline(always)]
    pub const fn in7(&self) -> super::vals::In7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::In7::from_bits(val as u8)
    }
    #[doc = "Enable IN endpoint 7"]
    #[inline(always)]
    pub fn set_in7(&mut self, val: super::vals::In7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable ISO IN endpoint"]
    #[inline(always)]
    pub const fn isoin(&self) -> super::vals::Isoin {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Isoin::from_bits(val as u8)
    }
    #[doc = "Enable ISO IN endpoint"]
    #[inline(always)]
    pub fn set_isoin(&mut self, val: super::vals::Isoin) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Epinen {
    #[inline(always)]
    fn default() -> Epinen {
        Epinen(0)
    }
}
#[doc = "Description cluster: Number of bytes transferred in the last transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpoutAmount(pub u32);
impl EpoutAmount {
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn set_amount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for EpoutAmount {
    #[inline(always)]
    fn default() -> EpoutAmount {
        EpoutAmount(0)
    }
}
#[doc = "Description cluster: Maximum number of bytes to transfer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpoutMaxcnt(pub u32);
impl EpoutMaxcnt {
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn set_maxcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for EpoutMaxcnt {
    #[inline(always)]
    fn default() -> EpoutMaxcnt {
        EpoutMaxcnt(0)
    }
}
#[doc = "Endpoint OUT enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epouten(pub u32);
impl Epouten {
    #[doc = "Enable OUT endpoint 0"]
    #[inline(always)]
    pub const fn out0(&self) -> super::vals::Out0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Out0::from_bits(val as u8)
    }
    #[doc = "Enable OUT endpoint 0"]
    #[inline(always)]
    pub fn set_out0(&mut self, val: super::vals::Out0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable OUT endpoint 1"]
    #[inline(always)]
    pub const fn out1(&self) -> super::vals::Out1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Out1::from_bits(val as u8)
    }
    #[doc = "Enable OUT endpoint 1"]
    #[inline(always)]
    pub fn set_out1(&mut self, val: super::vals::Out1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable OUT endpoint 2"]
    #[inline(always)]
    pub const fn out2(&self) -> super::vals::Out2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Out2::from_bits(val as u8)
    }
    #[doc = "Enable OUT endpoint 2"]
    #[inline(always)]
    pub fn set_out2(&mut self, val: super::vals::Out2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable OUT endpoint 3"]
    #[inline(always)]
    pub const fn out3(&self) -> super::vals::Out3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Out3::from_bits(val as u8)
    }
    #[doc = "Enable OUT endpoint 3"]
    #[inline(always)]
    pub fn set_out3(&mut self, val: super::vals::Out3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable OUT endpoint 4"]
    #[inline(always)]
    pub const fn out4(&self) -> super::vals::Out4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Out4::from_bits(val as u8)
    }
    #[doc = "Enable OUT endpoint 4"]
    #[inline(always)]
    pub fn set_out4(&mut self, val: super::vals::Out4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable OUT endpoint 5"]
    #[inline(always)]
    pub const fn out5(&self) -> super::vals::Out5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Out5::from_bits(val as u8)
    }
    #[doc = "Enable OUT endpoint 5"]
    #[inline(always)]
    pub fn set_out5(&mut self, val: super::vals::Out5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable OUT endpoint 6"]
    #[inline(always)]
    pub const fn out6(&self) -> super::vals::Out6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Out6::from_bits(val as u8)
    }
    #[doc = "Enable OUT endpoint 6"]
    #[inline(always)]
    pub fn set_out6(&mut self, val: super::vals::Out6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable OUT endpoint 7"]
    #[inline(always)]
    pub const fn out7(&self) -> super::vals::Out7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Out7::from_bits(val as u8)
    }
    #[doc = "Enable OUT endpoint 7"]
    #[inline(always)]
    pub fn set_out7(&mut self, val: super::vals::Out7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable ISO OUT endpoint 8"]
    #[inline(always)]
    pub const fn isoout(&self) -> super::vals::Isoout {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Isoout::from_bits(val as u8)
    }
    #[doc = "Enable ISO OUT endpoint 8"]
    #[inline(always)]
    pub fn set_isoout(&mut self, val: super::vals::Isoout) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Epouten {
    #[inline(always)]
    fn default() -> Epouten {
        Epouten(0)
    }
}
#[doc = "STALL endpoints"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epstall(pub u32);
impl Epstall {
    #[doc = "Select endpoint number"]
    #[inline(always)]
    pub const fn ep(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Select endpoint number"]
    #[inline(always)]
    pub fn set_ep(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Selects IN or OUT endpoint"]
    #[inline(always)]
    pub const fn io(&self) -> super::vals::EpstallIo {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::EpstallIo::from_bits(val as u8)
    }
    #[doc = "Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn set_io(&mut self, val: super::vals::EpstallIo) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall selected endpoint"]
    #[inline(always)]
    pub const fn stall(&self) -> super::vals::Stall {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Stall::from_bits(val as u8)
    }
    #[doc = "Stall selected endpoint"]
    #[inline(always)]
    pub fn set_stall(&mut self, val: super::vals::Stall) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Epstall {
    #[inline(always)]
    fn default() -> Epstall {
        Epstall(0)
    }
}
#[doc = "Provides information on which endpoint's EasyDMA registers have been captured"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epstatus(pub u32);
impl Epstatus {
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin0(&self) -> super::vals::Epin0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Epin0::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin0(&mut self, val: super::vals::Epin0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin1(&self) -> super::vals::EpstatusEpin1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EpstatusEpin1::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin1(&mut self, val: super::vals::EpstatusEpin1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin2(&self) -> super::vals::EpstatusEpin2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::EpstatusEpin2::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin2(&mut self, val: super::vals::EpstatusEpin2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin3(&self) -> super::vals::EpstatusEpin3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::EpstatusEpin3::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin3(&mut self, val: super::vals::EpstatusEpin3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin4(&self) -> super::vals::EpstatusEpin4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EpstatusEpin4::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin4(&mut self, val: super::vals::EpstatusEpin4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin5(&self) -> super::vals::EpstatusEpin5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::EpstatusEpin5::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin5(&mut self, val: super::vals::EpstatusEpin5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin6(&self) -> super::vals::EpstatusEpin6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::EpstatusEpin6::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin6(&mut self, val: super::vals::EpstatusEpin6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin7(&self) -> super::vals::EpstatusEpin7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::EpstatusEpin7::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin7(&mut self, val: super::vals::EpstatusEpin7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epin8(&self) -> super::vals::Epin8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Epin8::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epin8(&mut self, val: super::vals::Epin8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout0(&self) -> super::vals::Epout0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Epout0::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout0(&mut self, val: super::vals::Epout0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout1(&self) -> super::vals::EpstatusEpout1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::EpstatusEpout1::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout1(&mut self, val: super::vals::EpstatusEpout1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout2(&self) -> super::vals::EpstatusEpout2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::EpstatusEpout2::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout2(&mut self, val: super::vals::EpstatusEpout2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout3(&self) -> super::vals::EpstatusEpout3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::EpstatusEpout3::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout3(&mut self, val: super::vals::EpstatusEpout3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout4(&self) -> super::vals::EpstatusEpout4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::EpstatusEpout4::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout4(&mut self, val: super::vals::EpstatusEpout4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout5(&self) -> super::vals::EpstatusEpout5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::EpstatusEpout5::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout5(&mut self, val: super::vals::EpstatusEpout5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout6(&self) -> super::vals::EpstatusEpout6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::EpstatusEpout6::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout6(&mut self, val: super::vals::EpstatusEpout6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout7(&self) -> super::vals::EpstatusEpout7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::EpstatusEpout7::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout7(&mut self, val: super::vals::EpstatusEpout7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub const fn epout8(&self) -> super::vals::Epout8 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Epout8::from_bits(val as u8)
    }
    #[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn set_epout8(&mut self, val: super::vals::Epout8) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Epstatus {
    #[inline(always)]
    fn default() -> Epstatus {
        Epstatus(0)
    }
}
#[doc = "Details on what caused the USBEVENT event"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eventcause(pub u32);
impl Eventcause {
    #[doc = "CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
    #[inline(always)]
    pub const fn isooutcrc(&self) -> super::vals::Isooutcrc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Isooutcrc::from_bits(val as u8)
    }
    #[doc = "CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
    #[inline(always)]
    pub fn set_isooutcrc(&mut self, val: super::vals::Isooutcrc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
    #[inline(always)]
    pub const fn suspend(&self) -> super::vals::Suspend {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Suspend::from_bits(val as u8)
    }
    #[doc = "Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
    #[inline(always)]
    pub fn set_suspend(&mut self, val: super::vals::Suspend) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
    #[inline(always)]
    pub const fn resume(&self) -> super::vals::Resume {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Resume::from_bits(val as u8)
    }
    #[doc = "Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
    #[inline(always)]
    pub fn set_resume(&mut self, val: super::vals::Resume) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "USB MAC has been woken up and operational. Write '1' to clear."]
    #[inline(always)]
    pub const fn usbwuallowed(&self) -> super::vals::Usbwuallowed {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Usbwuallowed::from_bits(val as u8)
    }
    #[doc = "USB MAC has been woken up and operational. Write '1' to clear."]
    #[inline(always)]
    pub fn set_usbwuallowed(&mut self, val: super::vals::Usbwuallowed) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "USB device is ready for normal operation. Write '1' to clear."]
    #[inline(always)]
    pub const fn ready(&self) -> super::vals::Ready {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ready::from_bits(val as u8)
    }
    #[doc = "USB device is ready for normal operation. Write '1' to clear."]
    #[inline(always)]
    pub fn set_ready(&mut self, val: super::vals::Ready) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for Eventcause {
    #[inline(always)]
    fn default() -> Eventcause {
        Eventcause(0)
    }
}
#[doc = "Returns the current value of the start of frame counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Framecntr(pub u32);
impl Framecntr {
    #[doc = "Returns the current value of the start of frame counter"]
    #[inline(always)]
    pub const fn framecntr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Returns the current value of the start of frame counter"]
    #[inline(always)]
    pub fn set_framecntr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for Framecntr {
    #[inline(always)]
    fn default() -> Framecntr {
        Framecntr(0)
    }
}
#[doc = "Description collection: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HaltedEpout(pub u32);
impl HaltedEpout {
    #[doc = "OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub const fn getstatus(&self) -> super::vals::EpoutGetstatus {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::EpoutGetstatus::from_bits(val as u16)
    }
    #[doc = "OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub fn set_getstatus(&mut self, val: super::vals::EpoutGetstatus) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for HaltedEpout {
    #[inline(always)]
    fn default() -> HaltedEpout {
        HaltedEpout(0)
    }
}
#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to enable interrupt for event USBRESET"]
    #[inline(always)]
    pub const fn usbreset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event USBRESET"]
    #[inline(always)]
    pub fn set_usbreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    pub const fn started(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    pub fn set_started(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[0\\]"]
    #[inline(always)]
    pub const fn endepin0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[0\\]"]
    #[inline(always)]
    pub fn set_endepin0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[1\\]"]
    #[inline(always)]
    pub const fn endepin1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[1\\]"]
    #[inline(always)]
    pub fn set_endepin1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[2\\]"]
    #[inline(always)]
    pub const fn endepin2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[2\\]"]
    #[inline(always)]
    pub fn set_endepin2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[3\\]"]
    #[inline(always)]
    pub const fn endepin3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[3\\]"]
    #[inline(always)]
    pub fn set_endepin3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[4\\]"]
    #[inline(always)]
    pub const fn endepin4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[4\\]"]
    #[inline(always)]
    pub fn set_endepin4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[5\\]"]
    #[inline(always)]
    pub const fn endepin5(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[5\\]"]
    #[inline(always)]
    pub fn set_endepin5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[6\\]"]
    #[inline(always)]
    pub const fn endepin6(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[6\\]"]
    #[inline(always)]
    pub fn set_endepin6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[7\\]"]
    #[inline(always)]
    pub const fn endepin7(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPIN\\[7\\]"]
    #[inline(always)]
    pub fn set_endepin7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write '1' to enable interrupt for event EP0DATADONE"]
    #[inline(always)]
    pub const fn ep0datadone(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event EP0DATADONE"]
    #[inline(always)]
    pub fn set_ep0datadone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDISOIN"]
    #[inline(always)]
    pub const fn endisoin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDISOIN"]
    #[inline(always)]
    pub fn set_endisoin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[0\\]"]
    #[inline(always)]
    pub const fn endepout0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn set_endepout0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[1\\]"]
    #[inline(always)]
    pub const fn endepout1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[1\\]"]
    #[inline(always)]
    pub fn set_endepout1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[2\\]"]
    #[inline(always)]
    pub const fn endepout2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[2\\]"]
    #[inline(always)]
    pub fn set_endepout2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[3\\]"]
    #[inline(always)]
    pub const fn endepout3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[3\\]"]
    #[inline(always)]
    pub fn set_endepout3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[4\\]"]
    #[inline(always)]
    pub const fn endepout4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[4\\]"]
    #[inline(always)]
    pub fn set_endepout4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[5\\]"]
    #[inline(always)]
    pub const fn endepout5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[5\\]"]
    #[inline(always)]
    pub fn set_endepout5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[6\\]"]
    #[inline(always)]
    pub const fn endepout6(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[6\\]"]
    #[inline(always)]
    pub fn set_endepout6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[7\\]"]
    #[inline(always)]
    pub const fn endepout7(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[7\\]"]
    #[inline(always)]
    pub fn set_endepout7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write '1' to enable interrupt for event ENDISOOUT"]
    #[inline(always)]
    pub const fn endisoout(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ENDISOOUT"]
    #[inline(always)]
    pub fn set_endisoout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write '1' to enable interrupt for event SOF"]
    #[inline(always)]
    pub const fn sof(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event SOF"]
    #[inline(always)]
    pub fn set_sof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Write '1' to enable interrupt for event USBEVENT"]
    #[inline(always)]
    pub const fn usbevent(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event USBEVENT"]
    #[inline(always)]
    pub fn set_usbevent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Write '1' to enable interrupt for event EP0SETUP"]
    #[inline(always)]
    pub const fn ep0setup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event EP0SETUP"]
    #[inline(always)]
    pub fn set_ep0setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Write '1' to enable interrupt for event EPDATA"]
    #[inline(always)]
    pub const fn epdata(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event EPDATA"]
    #[inline(always)]
    pub fn set_epdata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Number of bytes transferred in the last transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IsoinAmount(pub u32);
impl IsoinAmount {
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn set_amount(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for IsoinAmount {
    #[inline(always)]
    fn default() -> IsoinAmount {
        IsoinAmount(0)
    }
}
#[doc = "Maximum number of bytes to transfer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IsoinMaxcnt(pub u32);
impl IsoinMaxcnt {
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn set_maxcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for IsoinMaxcnt {
    #[inline(always)]
    fn default() -> IsoinMaxcnt {
        IsoinMaxcnt(0)
    }
}
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isoinconfig(pub u32);
impl Isoinconfig {
    #[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    #[inline(always)]
    pub const fn response(&self) -> super::vals::Response {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Response::from_bits(val as u8)
    }
    #[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    #[inline(always)]
    pub fn set_response(&mut self, val: super::vals::Response) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Isoinconfig {
    #[inline(always)]
    fn default() -> Isoinconfig {
        Isoinconfig(0)
    }
}
#[doc = "Number of bytes received last on this ISO OUT data endpoint"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isoout(pub u32);
impl Isoout {
    #[doc = "Number of bytes received last on this ISO OUT data endpoint"]
    #[inline(always)]
    pub const fn size(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of bytes received last on this ISO OUT data endpoint"]
    #[inline(always)]
    pub fn set_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Zero-length data packet received"]
    #[inline(always)]
    pub const fn zero(&self) -> super::vals::Zero {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Zero::from_bits(val as u8)
    }
    #[doc = "Zero-length data packet received"]
    #[inline(always)]
    pub fn set_zero(&mut self, val: super::vals::Zero) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Isoout {
    #[inline(always)]
    fn default() -> Isoout {
        Isoout(0)
    }
}
#[doc = "Number of bytes transferred in the last transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IsooutAmount(pub u32);
impl IsooutAmount {
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn set_amount(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for IsooutAmount {
    #[inline(always)]
    fn default() -> IsooutAmount {
        IsooutAmount(0)
    }
}
#[doc = "Maximum number of bytes to transfer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IsooutMaxcnt(pub u32);
impl IsooutMaxcnt {
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn set_maxcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for IsooutMaxcnt {
    #[inline(always)]
    fn default() -> IsooutMaxcnt {
        IsooutMaxcnt(0)
    }
}
#[doc = "Controls the split of ISO buffers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isosplit(pub u32);
impl Isosplit {
    #[doc = "Controls the split of ISO buffers"]
    #[inline(always)]
    pub const fn split(&self) -> super::vals::Split {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Split::from_bits(val as u16)
    }
    #[doc = "Controls the split of ISO buffers"]
    #[inline(always)]
    pub fn set_split(&mut self, val: super::vals::Split) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Isosplit {
    #[inline(always)]
    fn default() -> Isosplit {
        Isosplit(0)
    }
}
#[doc = "Controls USBD peripheral low power mode during USB suspend"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lowpower(pub u32);
impl Lowpower {
    #[doc = "Controls USBD peripheral low-power mode during USB suspend"]
    #[inline(always)]
    pub const fn lowpower(&self) -> super::vals::Lowpower {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lowpower::from_bits(val as u8)
    }
    #[doc = "Controls USBD peripheral low-power mode during USB suspend"]
    #[inline(always)]
    pub fn set_lowpower(&mut self, val: super::vals::Lowpower) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lowpower {
    #[inline(always)]
    fn default() -> Lowpower {
        Lowpower(0)
    }
}
#[doc = "Description collection: Publish configuration for event ENDEPIN\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEndepin(pub u32);
impl PublishEndepin {
    #[doc = "DPPI channel that event ENDEPIN\\[n\\] will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event ENDEPIN\\[n\\] will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEndepinEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEndepinEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEndepinEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEndepin {
    #[inline(always)]
    fn default() -> PublishEndepin {
        PublishEndepin(0)
    }
}
#[doc = "Description collection: Publish configuration for event ENDEPOUT\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEndepout(pub u32);
impl PublishEndepout {
    #[doc = "DPPI channel that event ENDEPOUT\\[n\\] will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event ENDEPOUT\\[n\\] will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEndepoutEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEndepoutEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEndepoutEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEndepout {
    #[inline(always)]
    fn default() -> PublishEndepout {
        PublishEndepout(0)
    }
}
#[doc = "Publish configuration for event ENDISOIN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEndisoin(pub u32);
impl PublishEndisoin {
    #[doc = "DPPI channel that event ENDISOIN will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event ENDISOIN will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEndisoinEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEndisoinEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEndisoinEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEndisoin {
    #[inline(always)]
    fn default() -> PublishEndisoin {
        PublishEndisoin(0)
    }
}
#[doc = "Publish configuration for event ENDISOOUT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEndisoout(pub u32);
impl PublishEndisoout {
    #[doc = "DPPI channel that event ENDISOOUT will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event ENDISOOUT will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEndisooutEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEndisooutEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEndisooutEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEndisoout {
    #[inline(always)]
    fn default() -> PublishEndisoout {
        PublishEndisoout(0)
    }
}
#[doc = "Publish configuration for event EP0DATADONE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEp0datadone(pub u32);
impl PublishEp0datadone {
    #[doc = "DPPI channel that event EP0DATADONE will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event EP0DATADONE will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEp0datadoneEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEp0datadoneEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEp0datadoneEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEp0datadone {
    #[inline(always)]
    fn default() -> PublishEp0datadone {
        PublishEp0datadone(0)
    }
}
#[doc = "Publish configuration for event EP0SETUP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEp0setup(pub u32);
impl PublishEp0setup {
    #[doc = "DPPI channel that event EP0SETUP will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event EP0SETUP will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEp0setupEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEp0setupEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEp0setupEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEp0setup {
    #[inline(always)]
    fn default() -> PublishEp0setup {
        PublishEp0setup(0)
    }
}
#[doc = "Publish configuration for event EPDATA"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishEpdata(pub u32);
impl PublishEpdata {
    #[doc = "DPPI channel that event EPDATA will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event EPDATA will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishEpdataEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishEpdataEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishEpdataEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishEpdata {
    #[inline(always)]
    fn default() -> PublishEpdata {
        PublishEpdata(0)
    }
}
#[doc = "Publish configuration for event SOF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishSof(pub u32);
impl PublishSof {
    #[doc = "DPPI channel that event SOF will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event SOF will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishSofEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishSofEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishSofEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishSof {
    #[inline(always)]
    fn default() -> PublishSof {
        PublishSof(0)
    }
}
#[doc = "Publish configuration for event STARTED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishStarted(pub u32);
impl PublishStarted {
    #[doc = "DPPI channel that event STARTED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event STARTED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishStartedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishStartedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishStartedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishStarted {
    #[inline(always)]
    fn default() -> PublishStarted {
        PublishStarted(0)
    }
}
#[doc = "Publish configuration for event USBEVENT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishUsbevent(pub u32);
impl PublishUsbevent {
    #[doc = "DPPI channel that event USBEVENT will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event USBEVENT will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishUsbeventEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishUsbeventEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishUsbeventEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishUsbevent {
    #[inline(always)]
    fn default() -> PublishUsbevent {
        PublishUsbevent(0)
    }
}
#[doc = "Publish configuration for event USBRESET"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishUsbreset(pub u32);
impl PublishUsbreset {
    #[doc = "DPPI channel that event USBRESET will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event USBRESET will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishUsbresetEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishUsbresetEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishUsbresetEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishUsbreset {
    #[inline(always)]
    fn default() -> PublishUsbreset {
        PublishUsbreset(0)
    }
}
#[doc = "Shortcuts between local events and tasks"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]"]
    #[inline(always)]
    pub const fn ep0datadone_startepin0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event EP0DATADONE and task STARTEPIN\\[0\\]"]
    #[inline(always)]
    pub fn set_ep0datadone_startepin0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]"]
    #[inline(always)]
    pub const fn ep0datadone_startepout0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event EP0DATADONE and task STARTEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn set_ep0datadone_startepout0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Shortcut between event EP0DATADONE and task EP0STATUS"]
    #[inline(always)]
    pub const fn ep0datadone_ep0status(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event EP0DATADONE and task EP0STATUS"]
    #[inline(always)]
    pub fn set_ep0datadone_ep0status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Shortcut between event ENDEPOUT\\[0\\] and task EP0STATUS"]
    #[inline(always)]
    pub const fn endepout0_ep0status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event ENDEPOUT\\[0\\] and task EP0STATUS"]
    #[inline(always)]
    pub fn set_endepout0_ep0status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Shortcut between event ENDEPOUT\\[0\\] and task EP0RCVOUT"]
    #[inline(always)]
    pub const fn endepout0_ep0rcvout(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event ENDEPOUT\\[0\\] and task EP0RCVOUT"]
    #[inline(always)]
    pub fn set_endepout0_ep0rcvout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Description collection: Number of bytes received last in the data stage of this OUT endpoint"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SizeEpout(pub u32);
impl SizeEpout {
    #[doc = "Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for SizeEpout {
    #[inline(always)]
    fn default() -> SizeEpout {
        SizeEpout(0)
    }
}
#[doc = "Subscribe configuration for task DPDMDRIVE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeDpdmdrive(pub u32);
impl SubscribeDpdmdrive {
    #[doc = "DPPI channel that task DPDMDRIVE will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task DPDMDRIVE will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeDpdmdriveEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeDpdmdriveEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeDpdmdriveEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeDpdmdrive {
    #[inline(always)]
    fn default() -> SubscribeDpdmdrive {
        SubscribeDpdmdrive(0)
    }
}
#[doc = "Subscribe configuration for task DPDMNODRIVE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeDpdmnodrive(pub u32);
impl SubscribeDpdmnodrive {
    #[doc = "DPPI channel that task DPDMNODRIVE will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task DPDMNODRIVE will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeDpdmnodriveEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeDpdmnodriveEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeDpdmnodriveEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeDpdmnodrive {
    #[inline(always)]
    fn default() -> SubscribeDpdmnodrive {
        SubscribeDpdmnodrive(0)
    }
}
#[doc = "Subscribe configuration for task EP0RCVOUT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeEp0rcvout(pub u32);
impl SubscribeEp0rcvout {
    #[doc = "DPPI channel that task EP0RCVOUT will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task EP0RCVOUT will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeEp0rcvoutEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeEp0rcvoutEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeEp0rcvoutEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeEp0rcvout {
    #[inline(always)]
    fn default() -> SubscribeEp0rcvout {
        SubscribeEp0rcvout(0)
    }
}
#[doc = "Subscribe configuration for task EP0STALL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeEp0stall(pub u32);
impl SubscribeEp0stall {
    #[doc = "DPPI channel that task EP0STALL will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task EP0STALL will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeEp0stallEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeEp0stallEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeEp0stallEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeEp0stall {
    #[inline(always)]
    fn default() -> SubscribeEp0stall {
        SubscribeEp0stall(0)
    }
}
#[doc = "Subscribe configuration for task EP0STATUS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeEp0status(pub u32);
impl SubscribeEp0status {
    #[doc = "DPPI channel that task EP0STATUS will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task EP0STATUS will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeEp0statusEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeEp0statusEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeEp0statusEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeEp0status {
    #[inline(always)]
    fn default() -> SubscribeEp0status {
        SubscribeEp0status(0)
    }
}
#[doc = "Description collection: Subscribe configuration for task STARTEPIN\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeStartepin(pub u32);
impl SubscribeStartepin {
    #[doc = "DPPI channel that task STARTEPIN\\[n\\] will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task STARTEPIN\\[n\\] will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeStartepinEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeStartepinEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeStartepinEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeStartepin {
    #[inline(always)]
    fn default() -> SubscribeStartepin {
        SubscribeStartepin(0)
    }
}
#[doc = "Description collection: Subscribe configuration for task STARTEPOUT\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeStartepout(pub u32);
impl SubscribeStartepout {
    #[doc = "DPPI channel that task STARTEPOUT\\[n\\] will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task STARTEPOUT\\[n\\] will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeStartepoutEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeStartepoutEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeStartepoutEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeStartepout {
    #[inline(always)]
    fn default() -> SubscribeStartepout {
        SubscribeStartepout(0)
    }
}
#[doc = "Subscribe configuration for task STARTISOIN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeStartisoin(pub u32);
impl SubscribeStartisoin {
    #[doc = "DPPI channel that task STARTISOIN will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task STARTISOIN will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeStartisoinEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeStartisoinEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeStartisoinEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeStartisoin {
    #[inline(always)]
    fn default() -> SubscribeStartisoin {
        SubscribeStartisoin(0)
    }
}
#[doc = "Subscribe configuration for task STARTISOOUT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeStartisoout(pub u32);
impl SubscribeStartisoout {
    #[doc = "DPPI channel that task STARTISOOUT will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task STARTISOOUT will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeStartisooutEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeStartisooutEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeStartisooutEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeStartisoout {
    #[inline(always)]
    fn default() -> SubscribeStartisoout {
        SubscribeStartisoout(0)
    }
}
#[doc = "Device USB address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbaddr(pub u32);
impl Usbaddr {
    #[doc = "Device USB address"]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Device USB address"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Usbaddr {
    #[inline(always)]
    fn default() -> Usbaddr {
        Usbaddr(0)
    }
}
#[doc = "Control of the USB pull-up"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbpullup(pub u32);
impl Usbpullup {
    #[doc = "Control of the USB pull-up on the D+ line"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::Connect {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Connect::from_bits(val as u8)
    }
    #[doc = "Control of the USB pull-up on the D+ line"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Connect) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usbpullup {
    #[inline(always)]
    fn default() -> Usbpullup {
        Usbpullup(0)
    }
}
#[doc = "SETUP data, byte 5, MSB of wIndex"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Windexh(pub u32);
impl Windexh {
    #[doc = "SETUP data, byte 5, MSB of wIndex"]
    #[inline(always)]
    pub const fn windexh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SETUP data, byte 5, MSB of wIndex"]
    #[inline(always)]
    pub fn set_windexh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Windexh {
    #[inline(always)]
    fn default() -> Windexh {
        Windexh(0)
    }
}
#[doc = "SETUP data, byte 4, LSB of wIndex"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Windexl(pub u32);
impl Windexl {
    #[doc = "SETUP data, byte 4, LSB of wIndex"]
    #[inline(always)]
    pub const fn windexl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SETUP data, byte 4, LSB of wIndex"]
    #[inline(always)]
    pub fn set_windexl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Windexl {
    #[inline(always)]
    fn default() -> Windexl {
        Windexl(0)
    }
}
#[doc = "SETUP data, byte 7, MSB of wLength"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wlengthh(pub u32);
impl Wlengthh {
    #[doc = "SETUP data, byte 7, MSB of wLength"]
    #[inline(always)]
    pub const fn wlengthh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SETUP data, byte 7, MSB of wLength"]
    #[inline(always)]
    pub fn set_wlengthh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Wlengthh {
    #[inline(always)]
    fn default() -> Wlengthh {
        Wlengthh(0)
    }
}
#[doc = "SETUP data, byte 6, LSB of wLength"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wlengthl(pub u32);
impl Wlengthl {
    #[doc = "SETUP data, byte 6, LSB of wLength"]
    #[inline(always)]
    pub const fn wlengthl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SETUP data, byte 6, LSB of wLength"]
    #[inline(always)]
    pub fn set_wlengthl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Wlengthl {
    #[inline(always)]
    fn default() -> Wlengthl {
        Wlengthl(0)
    }
}
#[doc = "SETUP data, byte 3, MSB of wValue"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wvalueh(pub u32);
impl Wvalueh {
    #[doc = "SETUP data, byte 3, MSB of wValue"]
    #[inline(always)]
    pub const fn wvalueh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SETUP data, byte 3, MSB of wValue"]
    #[inline(always)]
    pub fn set_wvalueh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Wvalueh {
    #[inline(always)]
    fn default() -> Wvalueh {
        Wvalueh(0)
    }
}
#[doc = "SETUP data, byte 2, LSB of wValue"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wvaluel(pub u32);
impl Wvaluel {
    #[doc = "SETUP data, byte 2, LSB of wValue"]
    #[inline(always)]
    pub const fn wvaluel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SETUP data, byte 2, LSB of wValue"]
    #[inline(always)]
    pub fn set_wvaluel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Wvaluel {
    #[inline(always)]
    fn default() -> Wvaluel {
        Wvaluel(0)
    }
}
