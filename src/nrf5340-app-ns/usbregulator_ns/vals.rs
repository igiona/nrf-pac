#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsUsbdetected {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsUsbdetected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsUsbdetected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsUsbdetected {
    #[inline(always)]
    fn from(val: u8) -> EventsUsbdetected {
        EventsUsbdetected::from_bits(val)
    }
}
impl From<EventsUsbdetected> for u8 {
    #[inline(always)]
    fn from(val: EventsUsbdetected) -> u8 {
        EventsUsbdetected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsUsbpwrrdy {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsUsbpwrrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsUsbpwrrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsUsbpwrrdy {
    #[inline(always)]
    fn from(val: u8) -> EventsUsbpwrrdy {
        EventsUsbpwrrdy::from_bits(val)
    }
}
impl From<EventsUsbpwrrdy> for u8 {
    #[inline(always)]
    fn from(val: EventsUsbpwrrdy) -> u8 {
        EventsUsbpwrrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsUsbremoved {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsUsbremoved {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsUsbremoved {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsUsbremoved {
    #[inline(always)]
    fn from(val: u8) -> EventsUsbremoved {
        EventsUsbremoved::from_bits(val)
    }
}
impl From<EventsUsbremoved> for u8 {
    #[inline(always)]
    fn from(val: EventsUsbremoved) -> u8 {
        EventsUsbremoved::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Outputrdy {
    #[doc = "USBREG output settling time not elapsed"]
    NOTREADY = 0x0,
    #[doc = "USBREG output settling time elapsed (same information as USBPWRRDY event)"]
    READY = 0x01,
}
impl Outputrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outputrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outputrdy {
    #[inline(always)]
    fn from(val: u8) -> Outputrdy {
        Outputrdy::from_bits(val)
    }
}
impl From<Outputrdy> for u8 {
    #[inline(always)]
    fn from(val: Outputrdy) -> u8 {
        Outputrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishUsbdetectedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishUsbdetectedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishUsbdetectedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishUsbdetectedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishUsbdetectedEn {
        PublishUsbdetectedEn::from_bits(val)
    }
}
impl From<PublishUsbdetectedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishUsbdetectedEn) -> u8 {
        PublishUsbdetectedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishUsbpwrrdyEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishUsbpwrrdyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishUsbpwrrdyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishUsbpwrrdyEn {
    #[inline(always)]
    fn from(val: u8) -> PublishUsbpwrrdyEn {
        PublishUsbpwrrdyEn::from_bits(val)
    }
}
impl From<PublishUsbpwrrdyEn> for u8 {
    #[inline(always)]
    fn from(val: PublishUsbpwrrdyEn) -> u8 {
        PublishUsbpwrrdyEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishUsbremovedEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishUsbremovedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishUsbremovedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishUsbremovedEn {
    #[inline(always)]
    fn from(val: u8) -> PublishUsbremovedEn {
        PublishUsbremovedEn::from_bits(val)
    }
}
impl From<PublishUsbremovedEn> for u8 {
    #[inline(always)]
    fn from(val: PublishUsbremovedEn) -> u8 {
        PublishUsbremovedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vbusdetect {
    #[doc = "VBUS voltage below valid threshold"]
    NOVBUS = 0x0,
    #[doc = "VBUS voltage above valid threshold"]
    VBUSPRESENT = 0x01,
}
impl Vbusdetect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbusdetect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbusdetect {
    #[inline(always)]
    fn from(val: u8) -> Vbusdetect {
        Vbusdetect::from_bits(val)
    }
}
impl From<Vbusdetect> for u8 {
    #[inline(always)]
    fn from(val: Vbusdetect) -> u8 {
        Vbusdetect::to_bits(val)
    }
}
