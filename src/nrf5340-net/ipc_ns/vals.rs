#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsReceive {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsReceive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsReceive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsReceive {
    #[inline(always)]
    fn from(val: u8) -> EventsReceive {
        EventsReceive::from_bits(val)
    }
}
impl From<EventsReceive> for u8 {
    #[inline(always)]
    fn from(val: EventsReceive) -> u8 {
        EventsReceive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive0 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive0 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive0 {
        IntpendReceive0::from_bits(val)
    }
}
impl From<IntpendReceive0> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive0) -> u8 {
        IntpendReceive0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive1 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive1 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive1 {
        IntpendReceive1::from_bits(val)
    }
}
impl From<IntpendReceive1> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive1) -> u8 {
        IntpendReceive1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive10 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive10 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive10 {
        IntpendReceive10::from_bits(val)
    }
}
impl From<IntpendReceive10> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive10) -> u8 {
        IntpendReceive10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive11 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive11 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive11 {
        IntpendReceive11::from_bits(val)
    }
}
impl From<IntpendReceive11> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive11) -> u8 {
        IntpendReceive11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive12 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive12 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive12 {
        IntpendReceive12::from_bits(val)
    }
}
impl From<IntpendReceive12> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive12) -> u8 {
        IntpendReceive12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive13 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive13 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive13 {
        IntpendReceive13::from_bits(val)
    }
}
impl From<IntpendReceive13> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive13) -> u8 {
        IntpendReceive13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive14 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive14 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive14 {
        IntpendReceive14::from_bits(val)
    }
}
impl From<IntpendReceive14> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive14) -> u8 {
        IntpendReceive14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive15 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive15 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive15 {
        IntpendReceive15::from_bits(val)
    }
}
impl From<IntpendReceive15> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive15) -> u8 {
        IntpendReceive15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive2 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive2 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive2 {
        IntpendReceive2::from_bits(val)
    }
}
impl From<IntpendReceive2> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive2) -> u8 {
        IntpendReceive2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive3 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive3 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive3 {
        IntpendReceive3::from_bits(val)
    }
}
impl From<IntpendReceive3> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive3) -> u8 {
        IntpendReceive3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive4 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive4 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive4 {
        IntpendReceive4::from_bits(val)
    }
}
impl From<IntpendReceive4> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive4) -> u8 {
        IntpendReceive4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive5 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive5 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive5 {
        IntpendReceive5::from_bits(val)
    }
}
impl From<IntpendReceive5> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive5) -> u8 {
        IntpendReceive5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive6 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive6 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive6 {
        IntpendReceive6::from_bits(val)
    }
}
impl From<IntpendReceive6> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive6) -> u8 {
        IntpendReceive6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive7 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive7 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive7 {
        IntpendReceive7::from_bits(val)
    }
}
impl From<IntpendReceive7> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive7) -> u8 {
        IntpendReceive7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive8 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive8 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive8 {
        IntpendReceive8::from_bits(val)
    }
}
impl From<IntpendReceive8> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive8) -> u8 {
        IntpendReceive8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntpendReceive9 {
    #[doc = "Read: Not pending"]
    NOTPENDING = 0x0,
    #[doc = "Read: Pending"]
    PENDING = 0x01,
}
impl IntpendReceive9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntpendReceive9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntpendReceive9 {
    #[inline(always)]
    fn from(val: u8) -> IntpendReceive9 {
        IntpendReceive9::from_bits(val)
    }
}
impl From<IntpendReceive9> for u8 {
    #[inline(always)]
    fn from(val: IntpendReceive9) -> u8 {
        IntpendReceive9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PublishReceiveEn {
    #[doc = "Disable publishing"]
    DISABLED = 0x0,
    #[doc = "Enable publishing"]
    ENABLED = 0x01,
}
impl PublishReceiveEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PublishReceiveEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PublishReceiveEn {
    #[inline(always)]
    fn from(val: u8) -> PublishReceiveEn {
        PublishReceiveEn::from_bits(val)
    }
}
impl From<PublishReceiveEn> for u8 {
    #[inline(always)]
    fn from(val: PublishReceiveEn) -> u8 {
        PublishReceiveEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen0 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen0 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen0 {
        ReceiveCnfChen0::from_bits(val)
    }
}
impl From<ReceiveCnfChen0> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen0) -> u8 {
        ReceiveCnfChen0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen1 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen1 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen1 {
        ReceiveCnfChen1::from_bits(val)
    }
}
impl From<ReceiveCnfChen1> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen1) -> u8 {
        ReceiveCnfChen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen10 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen10 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen10 {
        ReceiveCnfChen10::from_bits(val)
    }
}
impl From<ReceiveCnfChen10> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen10) -> u8 {
        ReceiveCnfChen10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen11 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen11 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen11 {
        ReceiveCnfChen11::from_bits(val)
    }
}
impl From<ReceiveCnfChen11> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen11) -> u8 {
        ReceiveCnfChen11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen12 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen12 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen12 {
        ReceiveCnfChen12::from_bits(val)
    }
}
impl From<ReceiveCnfChen12> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen12) -> u8 {
        ReceiveCnfChen12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen13 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen13 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen13 {
        ReceiveCnfChen13::from_bits(val)
    }
}
impl From<ReceiveCnfChen13> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen13) -> u8 {
        ReceiveCnfChen13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen14 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen14 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen14 {
        ReceiveCnfChen14::from_bits(val)
    }
}
impl From<ReceiveCnfChen14> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen14) -> u8 {
        ReceiveCnfChen14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen15 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen15 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen15 {
        ReceiveCnfChen15::from_bits(val)
    }
}
impl From<ReceiveCnfChen15> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen15) -> u8 {
        ReceiveCnfChen15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen2 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen2 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen2 {
        ReceiveCnfChen2::from_bits(val)
    }
}
impl From<ReceiveCnfChen2> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen2) -> u8 {
        ReceiveCnfChen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen3 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen3 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen3 {
        ReceiveCnfChen3::from_bits(val)
    }
}
impl From<ReceiveCnfChen3> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen3) -> u8 {
        ReceiveCnfChen3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen4 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen4 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen4 {
        ReceiveCnfChen4::from_bits(val)
    }
}
impl From<ReceiveCnfChen4> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen4) -> u8 {
        ReceiveCnfChen4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen5 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen5 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen5 {
        ReceiveCnfChen5::from_bits(val)
    }
}
impl From<ReceiveCnfChen5> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen5) -> u8 {
        ReceiveCnfChen5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen6 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen6 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen6 {
        ReceiveCnfChen6::from_bits(val)
    }
}
impl From<ReceiveCnfChen6> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen6) -> u8 {
        ReceiveCnfChen6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen7 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen7 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen7 {
        ReceiveCnfChen7::from_bits(val)
    }
}
impl From<ReceiveCnfChen7> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen7) -> u8 {
        ReceiveCnfChen7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen8 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen8 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen8 {
        ReceiveCnfChen8::from_bits(val)
    }
}
impl From<ReceiveCnfChen8> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen8) -> u8 {
        ReceiveCnfChen8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReceiveCnfChen9 {
    #[doc = "Disable events"]
    DISABLE = 0x0,
    #[doc = "Enable events"]
    ENABLE = 0x01,
}
impl ReceiveCnfChen9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReceiveCnfChen9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReceiveCnfChen9 {
    #[inline(always)]
    fn from(val: u8) -> ReceiveCnfChen9 {
        ReceiveCnfChen9::from_bits(val)
    }
}
impl From<ReceiveCnfChen9> for u8 {
    #[inline(always)]
    fn from(val: ReceiveCnfChen9) -> u8 {
        ReceiveCnfChen9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen0 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen0 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen0 {
        SendCnfChen0::from_bits(val)
    }
}
impl From<SendCnfChen0> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen0) -> u8 {
        SendCnfChen0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen1 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen1 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen1 {
        SendCnfChen1::from_bits(val)
    }
}
impl From<SendCnfChen1> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen1) -> u8 {
        SendCnfChen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen10 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen10 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen10 {
        SendCnfChen10::from_bits(val)
    }
}
impl From<SendCnfChen10> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen10) -> u8 {
        SendCnfChen10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen11 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen11 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen11 {
        SendCnfChen11::from_bits(val)
    }
}
impl From<SendCnfChen11> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen11) -> u8 {
        SendCnfChen11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen12 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen12 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen12 {
        SendCnfChen12::from_bits(val)
    }
}
impl From<SendCnfChen12> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen12) -> u8 {
        SendCnfChen12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen13 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen13 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen13 {
        SendCnfChen13::from_bits(val)
    }
}
impl From<SendCnfChen13> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen13) -> u8 {
        SendCnfChen13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen14 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen14 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen14 {
        SendCnfChen14::from_bits(val)
    }
}
impl From<SendCnfChen14> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen14) -> u8 {
        SendCnfChen14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen15 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen15 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen15 {
        SendCnfChen15::from_bits(val)
    }
}
impl From<SendCnfChen15> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen15) -> u8 {
        SendCnfChen15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen2 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen2 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen2 {
        SendCnfChen2::from_bits(val)
    }
}
impl From<SendCnfChen2> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen2) -> u8 {
        SendCnfChen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen3 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen3 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen3 {
        SendCnfChen3::from_bits(val)
    }
}
impl From<SendCnfChen3> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen3) -> u8 {
        SendCnfChen3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen4 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen4 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen4 {
        SendCnfChen4::from_bits(val)
    }
}
impl From<SendCnfChen4> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen4) -> u8 {
        SendCnfChen4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen5 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen5 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen5 {
        SendCnfChen5::from_bits(val)
    }
}
impl From<SendCnfChen5> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen5) -> u8 {
        SendCnfChen5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen6 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen6 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen6 {
        SendCnfChen6::from_bits(val)
    }
}
impl From<SendCnfChen6> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen6) -> u8 {
        SendCnfChen6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen7 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen7 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen7 {
        SendCnfChen7::from_bits(val)
    }
}
impl From<SendCnfChen7> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen7) -> u8 {
        SendCnfChen7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen8 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen8 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen8 {
        SendCnfChen8::from_bits(val)
    }
}
impl From<SendCnfChen8> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen8) -> u8 {
        SendCnfChen8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SendCnfChen9 {
    #[doc = "Disable broadcast"]
    DISABLE = 0x0,
    #[doc = "Enable broadcast"]
    ENABLE = 0x01,
}
impl SendCnfChen9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SendCnfChen9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SendCnfChen9 {
    #[inline(always)]
    fn from(val: u8) -> SendCnfChen9 {
        SendCnfChen9::from_bits(val)
    }
}
impl From<SendCnfChen9> for u8 {
    #[inline(always)]
    fn from(val: SendCnfChen9) -> u8 {
        SendCnfChen9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubscribeSendEn {
    #[doc = "Disable subscription"]
    DISABLED = 0x0,
    #[doc = "Enable subscription"]
    ENABLED = 0x01,
}
impl SubscribeSendEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubscribeSendEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubscribeSendEn {
    #[inline(always)]
    fn from(val: u8) -> SubscribeSendEn {
        SubscribeSendEn::from_bits(val)
    }
}
impl From<SubscribeSendEn> for u8 {
    #[inline(always)]
    fn from(val: SubscribeSendEn) -> u8 {
        SubscribeSendEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksSend {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksSend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksSend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksSend {
    #[inline(always)]
    fn from(val: u8) -> TasksSend {
        TasksSend::from_bits(val)
    }
}
impl From<TasksSend> for u8 {
    #[inline(always)]
    fn from(val: TasksSend) -> u8 {
        TasksSend::to_bits(val)
    }
}
