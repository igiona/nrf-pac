#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Brequest {
    #[doc = "Standard request GET_STATUS"]
    STD_GET_STATUS = 0x0,
    #[doc = "Standard request CLEAR_FEATURE"]
    STD_CLEAR_FEATURE = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Standard request SET_FEATURE"]
    STD_SET_FEATURE = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Standard request SET_ADDRESS"]
    STD_SET_ADDRESS = 0x05,
    #[doc = "Standard request GET_DESCRIPTOR"]
    STD_GET_DESCRIPTOR = 0x06,
    #[doc = "Standard request SET_DESCRIPTOR"]
    STD_SET_DESCRIPTOR = 0x07,
    #[doc = "Standard request GET_CONFIGURATION"]
    STD_GET_CONFIGURATION = 0x08,
    #[doc = "Standard request SET_CONFIGURATION"]
    STD_SET_CONFIGURATION = 0x09,
    #[doc = "Standard request GET_INTERFACE"]
    STD_GET_INTERFACE = 0x0a,
    #[doc = "Standard request SET_INTERFACE"]
    STD_SET_INTERFACE = 0x0b,
    #[doc = "Standard request SYNCH_FRAME"]
    STD_SYNCH_FRAME = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    _RESERVED_ff = 0xff,
}
impl Brequest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brequest {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brequest {
    #[inline(always)]
    fn from(val: u8) -> Brequest {
        Brequest::from_bits(val)
    }
}
impl From<Brequest> for u8 {
    #[inline(always)]
    fn from(val: Brequest) -> u8 {
        Brequest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Direction {
    #[doc = "Host-to-device"]
    HOSTTODEVICE = 0x0,
    #[doc = "Device-to-host"]
    DEVICETOHOST = 0x01,
}
impl Direction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Direction {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Direction {
    #[inline(always)]
    fn from(val: u8) -> Direction {
        Direction::from_bits(val)
    }
}
impl From<Direction> for u8 {
    #[inline(always)]
    fn from(val: Direction) -> u8 {
        Direction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DtoggleIo {
    #[doc = "Selects OUT endpoint"]
    OUT = 0x0,
    #[doc = "Selects IN endpoint"]
    IN = 0x01,
}
impl DtoggleIo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtoggleIo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtoggleIo {
    #[inline(always)]
    fn from(val: u8) -> DtoggleIo {
        DtoggleIo::from_bits(val)
    }
}
impl From<DtoggleIo> for u8 {
    #[inline(always)]
    fn from(val: DtoggleIo) -> u8 {
        DtoggleIo::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EpinGetstatus(pub u16);
impl EpinGetstatus {
    #[doc = "Endpoint is not halted"]
    pub const NOTHALTED: Self = Self(0x0);
    #[doc = "Endpoint is halted"]
    pub const HALTED: Self = Self(0x01);
}
impl EpinGetstatus {
    pub const fn from_bits(val: u16) -> EpinGetstatus {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for EpinGetstatus {
    #[inline(always)]
    fn from(val: u16) -> EpinGetstatus {
        EpinGetstatus::from_bits(val)
    }
}
impl From<EpinGetstatus> for u16 {
    #[inline(always)]
    fn from(val: EpinGetstatus) -> u16 {
        EpinGetstatus::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EpoutGetstatus(pub u16);
impl EpoutGetstatus {
    #[doc = "Endpoint is not halted"]
    pub const NOTHALTED: Self = Self(0x0);
    #[doc = "Endpoint is halted"]
    pub const HALTED: Self = Self(0x01);
}
impl EpoutGetstatus {
    pub const fn from_bits(val: u16) -> EpoutGetstatus {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for EpoutGetstatus {
    #[inline(always)]
    fn from(val: u16) -> EpoutGetstatus {
        EpoutGetstatus::from_bits(val)
    }
}
impl From<EpoutGetstatus> for u16 {
    #[inline(always)]
    fn from(val: EpoutGetstatus) -> u16 {
        EpoutGetstatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EpstallIo {
    #[doc = "Selects OUT endpoint"]
    OUT = 0x0,
    #[doc = "Selects IN endpoint"]
    IN = 0x01,
}
impl EpstallIo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EpstallIo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EpstallIo {
    #[inline(always)]
    fn from(val: u8) -> EpstallIo {
        EpstallIo::from_bits(val)
    }
}
impl From<EpstallIo> for u8 {
    #[inline(always)]
    fn from(val: EpstallIo) -> u8 {
        EpstallIo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEndepin {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEndepin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEndepin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEndepin {
    #[inline(always)]
    fn from(val: u8) -> EventsEndepin {
        EventsEndepin::from_bits(val)
    }
}
impl From<EventsEndepin> for u8 {
    #[inline(always)]
    fn from(val: EventsEndepin) -> u8 {
        EventsEndepin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEndepout {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEndepout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEndepout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEndepout {
    #[inline(always)]
    fn from(val: u8) -> EventsEndepout {
        EventsEndepout::from_bits(val)
    }
}
impl From<EventsEndepout> for u8 {
    #[inline(always)]
    fn from(val: EventsEndepout) -> u8 {
        EventsEndepout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEndisoin {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEndisoin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEndisoin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEndisoin {
    #[inline(always)]
    fn from(val: u8) -> EventsEndisoin {
        EventsEndisoin::from_bits(val)
    }
}
impl From<EventsEndisoin> for u8 {
    #[inline(always)]
    fn from(val: EventsEndisoin) -> u8 {
        EventsEndisoin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEndisoout {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEndisoout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEndisoout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEndisoout {
    #[inline(always)]
    fn from(val: u8) -> EventsEndisoout {
        EventsEndisoout::from_bits(val)
    }
}
impl From<EventsEndisoout> for u8 {
    #[inline(always)]
    fn from(val: EventsEndisoout) -> u8 {
        EventsEndisoout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEp0datadone {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEp0datadone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEp0datadone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEp0datadone {
    #[inline(always)]
    fn from(val: u8) -> EventsEp0datadone {
        EventsEp0datadone::from_bits(val)
    }
}
impl From<EventsEp0datadone> for u8 {
    #[inline(always)]
    fn from(val: EventsEp0datadone) -> u8 {
        EventsEp0datadone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEp0setup {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEp0setup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEp0setup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEp0setup {
    #[inline(always)]
    fn from(val: u8) -> EventsEp0setup {
        EventsEp0setup::from_bits(val)
    }
}
impl From<EventsEp0setup> for u8 {
    #[inline(always)]
    fn from(val: EventsEp0setup) -> u8 {
        EventsEp0setup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsEpdata {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsEpdata {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsEpdata {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsEpdata {
    #[inline(always)]
    fn from(val: u8) -> EventsEpdata {
        EventsEpdata::from_bits(val)
    }
}
impl From<EventsEpdata> for u8 {
    #[inline(always)]
    fn from(val: EventsEpdata) -> u8 {
        EventsEpdata::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsSof {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsSof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsSof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsSof {
    #[inline(always)]
    fn from(val: u8) -> EventsSof {
        EventsSof::from_bits(val)
    }
}
impl From<EventsSof> for u8 {
    #[inline(always)]
    fn from(val: EventsSof) -> u8 {
        EventsSof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsStarted {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsStarted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsStarted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsStarted {
    #[inline(always)]
    fn from(val: u8) -> EventsStarted {
        EventsStarted::from_bits(val)
    }
}
impl From<EventsStarted> for u8 {
    #[inline(always)]
    fn from(val: EventsStarted) -> u8 {
        EventsStarted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsUsbevent {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsUsbevent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsUsbevent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsUsbevent {
    #[inline(always)]
    fn from(val: u8) -> EventsUsbevent {
        EventsUsbevent::from_bits(val)
    }
}
impl From<EventsUsbevent> for u8 {
    #[inline(always)]
    fn from(val: EventsUsbevent) -> u8 {
        EventsUsbevent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EventsUsbreset {
    #[doc = "Event not generated"]
    NOTGENERATED = 0x0,
    #[doc = "Event generated"]
    GENERATED = 0x01,
}
impl EventsUsbreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventsUsbreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventsUsbreset {
    #[inline(always)]
    fn from(val: u8) -> EventsUsbreset {
        EventsUsbreset::from_bits(val)
    }
}
impl From<EventsUsbreset> for u8 {
    #[inline(always)]
    fn from(val: EventsUsbreset) -> u8 {
        EventsUsbreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Isooutcrc {
    #[doc = "No error detected"]
    NOTDETECTED = 0x0,
    #[doc = "Error detected"]
    DETECTED = 0x01,
}
impl Isooutcrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isooutcrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isooutcrc {
    #[inline(always)]
    fn from(val: u8) -> Isooutcrc {
        Isooutcrc::from_bits(val)
    }
}
impl From<Isooutcrc> for u8 {
    #[inline(always)]
    fn from(val: Isooutcrc) -> u8 {
        Isooutcrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lowpower {
    #[doc = "Software must write this value to exit low power mode and before performing a remote wake-up"]
    FORCENORMAL = 0x0,
    #[doc = "Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    LOWPOWER = 0x01,
}
impl Lowpower {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lowpower {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lowpower {
    #[inline(always)]
    fn from(val: u8) -> Lowpower {
        Lowpower::from_bits(val)
    }
}
impl From<Lowpower> for u8 {
    #[inline(always)]
    fn from(val: Lowpower) -> u8 {
        Lowpower::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ready {
    #[doc = "USBEVENT was not issued due to USBD peripheral ready"]
    NOTDETECTED = 0x0,
    #[doc = "USBD peripheral is ready"]
    READY = 0x01,
}
impl Ready {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ready {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ready {
    #[inline(always)]
    fn from(val: u8) -> Ready {
        Ready::from_bits(val)
    }
}
impl From<Ready> for u8 {
    #[inline(always)]
    fn from(val: Ready) -> u8 {
        Ready::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Recipient(pub u8);
impl Recipient {
    #[doc = "Device"]
    pub const DEVICE: Self = Self(0x0);
    #[doc = "Interface"]
    pub const INTERFACE: Self = Self(0x01);
    #[doc = "Endpoint"]
    pub const ENDPOINT: Self = Self(0x02);
    #[doc = "Other"]
    pub const OTHER: Self = Self(0x03);
}
impl Recipient {
    pub const fn from_bits(val: u8) -> Recipient {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Recipient {
    #[inline(always)]
    fn from(val: u8) -> Recipient {
        Recipient::from_bits(val)
    }
}
impl From<Recipient> for u8 {
    #[inline(always)]
    fn from(val: Recipient) -> u8 {
        Recipient::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Response {
    #[doc = "Endpoint does not respond in that case"]
    NORESP = 0x0,
    #[doc = "Endpoint responds with a zero-length data packet in that case"]
    ZERODATA = 0x01,
}
impl Response {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Response {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Response {
    #[inline(always)]
    fn from(val: u8) -> Response {
        Response::from_bits(val)
    }
}
impl From<Response> for u8 {
    #[inline(always)]
    fn from(val: Response) -> u8 {
        Response::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resume {
    #[doc = "Resume not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Resume detected"]
    DETECTED = 0x01,
}
impl Resume {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resume {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resume {
    #[inline(always)]
    fn from(val: u8) -> Resume {
        Resume::from_bits(val)
    }
}
impl From<Resume> for u8 {
    #[inline(always)]
    fn from(val: Resume) -> u8 {
        Resume::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Split(pub u16);
impl Split {
    #[doc = "Full buffer dedicated to either ISO IN or OUT"]
    pub const ONEDIR: Self = Self(0x0);
    #[doc = "Lower half for IN, upper half for OUT"]
    pub const HALFIN: Self = Self(0x80);
}
impl Split {
    pub const fn from_bits(val: u16) -> Split {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Split {
    #[inline(always)]
    fn from(val: u16) -> Split {
        Split::from_bits(val)
    }
}
impl From<Split> for u16 {
    #[inline(always)]
    fn from(val: Split) -> u16 {
        Split::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Stall {
    #[doc = "Don't stall selected endpoint"]
    UNSTALL = 0x0,
    #[doc = "Stall selected endpoint"]
    STALL = 0x01,
}
impl Stall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stall {
    #[inline(always)]
    fn from(val: u8) -> Stall {
        Stall::from_bits(val)
    }
}
impl From<Stall> for u8 {
    #[inline(always)]
    fn from(val: Stall) -> u8 {
        Stall::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct State(pub u8);
impl State {
    #[doc = "D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    pub const RESUME: Self = Self(0x01);
    #[doc = "D+ forced high, D- forced low (J state)"]
    pub const J: Self = Self(0x02);
    #[doc = "D+ forced low, D- forced high (K state)"]
    pub const K: Self = Self(0x04);
}
impl State {
    pub const fn from_bits(val: u8) -> State {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for State {
    #[inline(always)]
    fn from(val: u8) -> State {
        State::from_bits(val)
    }
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(val: State) -> u8 {
        State::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Suspend {
    #[doc = "Suspend not detected"]
    NOTDETECTED = 0x0,
    #[doc = "Suspend detected"]
    DETECTED = 0x01,
}
impl Suspend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Suspend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Suspend {
    #[inline(always)]
    fn from(val: u8) -> Suspend {
        Suspend::from_bits(val)
    }
}
impl From<Suspend> for u8 {
    #[inline(always)]
    fn from(val: Suspend) -> u8 {
        Suspend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksDpdmdrive {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksDpdmdrive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksDpdmdrive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksDpdmdrive {
    #[inline(always)]
    fn from(val: u8) -> TasksDpdmdrive {
        TasksDpdmdrive::from_bits(val)
    }
}
impl From<TasksDpdmdrive> for u8 {
    #[inline(always)]
    fn from(val: TasksDpdmdrive) -> u8 {
        TasksDpdmdrive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksDpdmnodrive {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksDpdmnodrive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksDpdmnodrive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksDpdmnodrive {
    #[inline(always)]
    fn from(val: u8) -> TasksDpdmnodrive {
        TasksDpdmnodrive::from_bits(val)
    }
}
impl From<TasksDpdmnodrive> for u8 {
    #[inline(always)]
    fn from(val: TasksDpdmnodrive) -> u8 {
        TasksDpdmnodrive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksEp0rcvout {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksEp0rcvout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksEp0rcvout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksEp0rcvout {
    #[inline(always)]
    fn from(val: u8) -> TasksEp0rcvout {
        TasksEp0rcvout::from_bits(val)
    }
}
impl From<TasksEp0rcvout> for u8 {
    #[inline(always)]
    fn from(val: TasksEp0rcvout) -> u8 {
        TasksEp0rcvout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksEp0stall {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksEp0stall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksEp0stall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksEp0stall {
    #[inline(always)]
    fn from(val: u8) -> TasksEp0stall {
        TasksEp0stall::from_bits(val)
    }
}
impl From<TasksEp0stall> for u8 {
    #[inline(always)]
    fn from(val: TasksEp0stall) -> u8 {
        TasksEp0stall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksEp0status {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksEp0status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksEp0status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksEp0status {
    #[inline(always)]
    fn from(val: u8) -> TasksEp0status {
        TasksEp0status::from_bits(val)
    }
}
impl From<TasksEp0status> for u8 {
    #[inline(always)]
    fn from(val: TasksEp0status) -> u8 {
        TasksEp0status::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStartepin {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStartepin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStartepin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStartepin {
    #[inline(always)]
    fn from(val: u8) -> TasksStartepin {
        TasksStartepin::from_bits(val)
    }
}
impl From<TasksStartepin> for u8 {
    #[inline(always)]
    fn from(val: TasksStartepin) -> u8 {
        TasksStartepin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStartepout {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStartepout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStartepout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStartepout {
    #[inline(always)]
    fn from(val: u8) -> TasksStartepout {
        TasksStartepout::from_bits(val)
    }
}
impl From<TasksStartepout> for u8 {
    #[inline(always)]
    fn from(val: TasksStartepout) -> u8 {
        TasksStartepout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStartisoin {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStartisoin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStartisoin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStartisoin {
    #[inline(always)]
    fn from(val: u8) -> TasksStartisoin {
        TasksStartisoin::from_bits(val)
    }
}
impl From<TasksStartisoin> for u8 {
    #[inline(always)]
    fn from(val: TasksStartisoin) -> u8 {
        TasksStartisoin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TasksStartisoout {
    _RESERVED_0 = 0x0,
    #[doc = "Trigger task"]
    TRIGGER = 0x01,
}
impl TasksStartisoout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TasksStartisoout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TasksStartisoout {
    #[inline(always)]
    fn from(val: u8) -> TasksStartisoout {
        TasksStartisoout::from_bits(val)
    }
}
impl From<TasksStartisoout> for u8 {
    #[inline(always)]
    fn from(val: TasksStartisoout) -> u8 {
        TasksStartisoout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    #[doc = "Standard"]
    STANDARD = 0x0,
    #[doc = "Class"]
    CLASS = 0x01,
    #[doc = "Vendor"]
    VENDOR = 0x02,
    _RESERVED_3 = 0x03,
}
impl Type {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Type {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Type {
    #[inline(always)]
    fn from(val: u8) -> Type {
        Type::from_bits(val)
    }
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(val: Type) -> u8 {
        Type::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usbwuallowed {
    #[doc = "Wake up not allowed"]
    NOTALLOWED = 0x0,
    #[doc = "Wake up allowed"]
    ALLOWED = 0x01,
}
impl Usbwuallowed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbwuallowed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbwuallowed {
    #[inline(always)]
    fn from(val: u8) -> Usbwuallowed {
        Usbwuallowed::from_bits(val)
    }
}
impl From<Usbwuallowed> for u8 {
    #[inline(always)]
    fn from(val: Usbwuallowed) -> u8 {
        Usbwuallowed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    #[doc = "No action on data toggle when writing the register with this value"]
    NOP = 0x0,
    #[doc = "Data toggle is DATA0 on endpoint set by EP and IO"]
    DATA0 = 0x01,
    #[doc = "Data toggle is DATA1 on endpoint set by EP and IO"]
    DATA1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Value {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Value {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Value {
    #[inline(always)]
    fn from(val: u8) -> Value {
        Value::from_bits(val)
    }
}
impl From<Value> for u8 {
    #[inline(always)]
    fn from(val: Value) -> u8 {
        Value::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Zero {
    #[doc = "No zero-length data received, use value in SIZE"]
    NORMAL = 0x0,
    #[doc = "Zero-length data received, ignore value in SIZE"]
    ZERODATA = 0x01,
}
impl Zero {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zero {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zero {
    #[inline(always)]
    fn from(val: u8) -> Zero {
        Zero::from_bits(val)
    }
}
impl From<Zero> for u8 {
    #[inline(always)]
    fn from(val: Zero) -> u8 {
        Zero::to_bits(val)
    }
}