#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to enable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub const fn usbdetected(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub fn set_usbdetected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to enable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub const fn usbremoved(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub fn set_usbremoved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable interrupt for event USBPWRRDY"]
    #[inline(always)]
    pub const fn usbpwrrdy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event USBPWRRDY"]
    #[inline(always)]
    pub fn set_usbpwrrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Publish configuration for event USBDETECTED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishUsbdetected(pub u32);
impl PublishUsbdetected {
    #[doc = "DPPI channel that event USBDETECTED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event USBDETECTED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishUsbdetectedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishUsbdetectedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishUsbdetectedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishUsbdetected {
    #[inline(always)]
    fn default() -> PublishUsbdetected {
        PublishUsbdetected(0)
    }
}
#[doc = "Publish configuration for event USBPWRRDY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishUsbpwrrdy(pub u32);
impl PublishUsbpwrrdy {
    #[doc = "DPPI channel that event USBPWRRDY will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event USBPWRRDY will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishUsbpwrrdyEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishUsbpwrrdyEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishUsbpwrrdyEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishUsbpwrrdy {
    #[inline(always)]
    fn default() -> PublishUsbpwrrdy {
        PublishUsbpwrrdy(0)
    }
}
#[doc = "Publish configuration for event USBREMOVED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishUsbremoved(pub u32);
impl PublishUsbremoved {
    #[doc = "DPPI channel that event USBREMOVED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event USBREMOVED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishUsbremovedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishUsbremovedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishUsbremovedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishUsbremoved {
    #[inline(always)]
    fn default() -> PublishUsbremoved {
        PublishUsbremoved(0)
    }
}
#[doc = "USB supply status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbregstatus(pub u32);
impl Usbregstatus {
    #[doc = "VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)"]
    #[inline(always)]
    pub const fn vbusdetect(&self) -> super::vals::Vbusdetect {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Vbusdetect::from_bits(val as u8)
    }
    #[doc = "VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)"]
    #[inline(always)]
    pub fn set_vbusdetect(&mut self, val: super::vals::Vbusdetect) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB supply output settling time elapsed"]
    #[inline(always)]
    pub const fn outputrdy(&self) -> super::vals::Outputrdy {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Outputrdy::from_bits(val as u8)
    }
    #[doc = "USB supply output settling time elapsed"]
    #[inline(always)]
    pub fn set_outputrdy(&mut self, val: super::vals::Outputrdy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Usbregstatus {
    #[inline(always)]
    fn default() -> Usbregstatus {
        Usbregstatus(0)
    }
}
