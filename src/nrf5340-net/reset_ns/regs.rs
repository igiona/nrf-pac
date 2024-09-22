#[doc = "Reset reason"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resetreas(pub u32);
impl Resetreas {
    #[doc = "Reset from pin reset detected"]
    #[inline(always)]
    pub const fn resetpin(&self) -> super::vals::Resetpin {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Resetpin::from_bits(val as u8)
    }
    #[doc = "Reset from pin reset detected"]
    #[inline(always)]
    pub fn set_resetpin(&mut self, val: super::vals::Resetpin) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Reset from application watchdog timer 0 detected"]
    #[inline(always)]
    pub const fn dog0(&self) -> super::vals::Dog0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dog0::from_bits(val as u8)
    }
    #[doc = "Reset from application watchdog timer 0 detected"]
    #[inline(always)]
    pub fn set_dog0(&mut self, val: super::vals::Dog0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Reset from application CTRL-AP detected"]
    #[inline(always)]
    pub const fn ctrlap(&self) -> super::vals::Ctrlap {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ctrlap::from_bits(val as u8)
    }
    #[doc = "Reset from application CTRL-AP detected"]
    #[inline(always)]
    pub fn set_ctrlap(&mut self, val: super::vals::Ctrlap) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Reset from application soft reset detected"]
    #[inline(always)]
    pub const fn sreq(&self) -> super::vals::Sreq {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sreq::from_bits(val as u8)
    }
    #[doc = "Reset from application soft reset detected"]
    #[inline(always)]
    pub fn set_sreq(&mut self, val: super::vals::Sreq) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset from application CPU lockup detected"]
    #[inline(always)]
    pub const fn lockup(&self) -> super::vals::Lockup {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Lockup::from_bits(val as u8)
    }
    #[doc = "Reset from application CPU lockup detected"]
    #[inline(always)]
    pub fn set_lockup(&mut self, val: super::vals::Lockup) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub const fn off(&self) -> super::vals::Off {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Off::from_bits(val as u8)
    }
    #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub fn set_off(&mut self, val: super::vals::Off) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub const fn lpcomp(&self) -> super::vals::Lpcomp {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Lpcomp::from_bits(val as u8)
    }
    #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn set_lpcomp(&mut self, val: super::vals::Lpcomp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
    #[inline(always)]
    pub const fn dif(&self) -> super::vals::Dif {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dif::from_bits(val as u8)
    }
    #[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
    #[inline(always)]
    pub fn set_dif(&mut self, val: super::vals::Dif) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Reset from network soft reset detected"]
    #[inline(always)]
    pub const fn lsreq(&self) -> super::vals::Lsreq {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Lsreq::from_bits(val as u8)
    }
    #[doc = "Reset from network soft reset detected"]
    #[inline(always)]
    pub fn set_lsreq(&mut self, val: super::vals::Lsreq) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Reset from network CPU lockup detected"]
    #[inline(always)]
    pub const fn llockup(&self) -> super::vals::Llockup {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Llockup::from_bits(val as u8)
    }
    #[doc = "Reset from network CPU lockup detected"]
    #[inline(always)]
    pub fn set_llockup(&mut self, val: super::vals::Llockup) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Reset from network watchdog timer detected"]
    #[inline(always)]
    pub const fn ldog(&self) -> super::vals::Ldog {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Ldog::from_bits(val as u8)
    }
    #[doc = "Reset from network watchdog timer detected"]
    #[inline(always)]
    pub fn set_ldog(&mut self, val: super::vals::Ldog) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Force-OFF reset from application core detected"]
    #[inline(always)]
    pub const fn mforceoff(&self) -> super::vals::Mforceoff {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mforceoff::from_bits(val as u8)
    }
    #[doc = "Force-OFF reset from application core detected"]
    #[inline(always)]
    pub fn set_mforceoff(&mut self, val: super::vals::Mforceoff) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    pub const fn nfc(&self) -> super::vals::Nfc {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Nfc::from_bits(val as u8)
    }
    #[doc = "Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    pub fn set_nfc(&mut self, val: super::vals::Nfc) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Reset from application watchdog timer 1 detected"]
    #[inline(always)]
    pub const fn dog1(&self) -> super::vals::Dog1 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dog1::from_bits(val as u8)
    }
    #[doc = "Reset from application watchdog timer 1 detected"]
    #[inline(always)]
    pub fn set_dog1(&mut self, val: super::vals::Dog1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
    #[inline(always)]
    pub const fn vbus(&self) -> super::vals::Vbus {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Vbus::from_bits(val as u8)
    }
    #[doc = "Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
    #[inline(always)]
    pub fn set_vbus(&mut self, val: super::vals::Vbus) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Reset from network CTRL-AP detected"]
    #[inline(always)]
    pub const fn lctrlap(&self) -> super::vals::Lctrlap {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Lctrlap::from_bits(val as u8)
    }
    #[doc = "Reset from network CTRL-AP detected"]
    #[inline(always)]
    pub fn set_lctrlap(&mut self, val: super::vals::Lctrlap) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Resetreas {
    #[inline(always)]
    fn default() -> Resetreas {
        Resetreas(0)
    }
}
