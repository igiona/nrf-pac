#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApprotectPall(pub u32);
impl ApprotectPall {
    #[doc = "Protected"]
    pub const PROTECTED: Self = Self(0x0);
    #[doc = "Unprotected"]
    pub const UNPROTECTED: Self = Self(0x50fa_50fa);
}
impl ApprotectPall {
    pub const fn from_bits(val: u32) -> ApprotectPall {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for ApprotectPall {
    #[inline(always)]
    fn from(val: u32) -> ApprotectPall {
        ApprotectPall::from_bits(val)
    }
}
impl From<ApprotectPall> for u32 {
    #[inline(always)]
    fn from(val: ApprotectPall) -> u32 {
        ApprotectPall::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EraseprotectPall(pub u32);
impl EraseprotectPall {
    #[doc = "Protected"]
    pub const PROTECTED: Self = Self(0x0);
    #[doc = "Unprotected"]
    pub const UNPROTECTED: Self = Self(0xffff_ffff);
}
impl EraseprotectPall {
    pub const fn from_bits(val: u32) -> EraseprotectPall {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for EraseprotectPall {
    #[inline(always)]
    fn from(val: u32) -> EraseprotectPall {
        EraseprotectPall::from_bits(val)
    }
}
impl From<EraseprotectPall> for u32 {
    #[inline(always)]
    fn from(val: EraseprotectPall) -> u32 {
        EraseprotectPall::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hfxocnt(pub u8);
impl Hfxocnt {
    #[doc = "Min debounce time = (0*64 us + 0.5 us)"]
    pub const MINDEBOUNCETIME: Self = Self(0x0);
    #[doc = "Max debounce time = (254*64 us + 0.5 us)"]
    pub const MAXDEBOUNCETIME: Self = Self(0xfe);
    #[doc = "Default debounce time for erased UICR = 4*64 us + 0.5 us"]
    pub const DEFAULTDEBOUNCETIME: Self = Self(0xff);
}
impl Hfxocnt {
    pub const fn from_bits(val: u8) -> Hfxocnt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Hfxocnt {
    #[inline(always)]
    fn from(val: u8) -> Hfxocnt {
        Hfxocnt::from_bits(val)
    }
}
impl From<Hfxocnt> for u8 {
    #[inline(always)]
    fn from(val: Hfxocnt) -> u8 {
        Hfxocnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Protect {
    #[doc = "Operation as GPIO pins. Same protection as normal GPIO pins"]
    DISABLED = 0x0,
    #[doc = "Operation as NFC antenna pins. Configures the protection for NFC operation"]
    NFC = 0x01,
}
impl Protect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Protect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Protect {
    #[inline(always)]
    fn from(val: u8) -> Protect {
        Protect::from_bits(val)
    }
}
impl From<Protect> for u8 {
    #[inline(always)]
    fn from(val: Protect) -> u8 {
        Protect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Push {
    #[doc = "Disable pushing of key value registers over secure APB, but can be read if field READ is Enabled"]
    DISABLED = 0x0,
    #[doc = "Enable pushing of key value registers over secure APB. Register KEYSLOT.CONFIGn.DEST must contain a valid destination address!"]
    ENABLED = 0x01,
}
impl Push {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Push {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Push {
    #[inline(always)]
    fn from(val: u8) -> Push {
        Push::from_bits(val)
    }
}
impl From<Push> for u8 {
    #[inline(always)]
    fn from(val: Push) -> u8 {
        Push::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Read {
    #[doc = "Disable read from key value registers"]
    DISABLED = 0x0,
    #[doc = "Enable read from key value registers"]
    ENABLED = 0x01,
}
impl Read {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Read {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Read {
    #[inline(always)]
    fn from(val: u8) -> Read {
        Read::from_bits(val)
    }
}
impl From<Read> for u8 {
    #[inline(always)]
    fn from(val: Read) -> u8 {
        Read::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SecureapprotectPall(pub u32);
impl SecureapprotectPall {
    #[doc = "Protected"]
    pub const PROTECTED: Self = Self(0x0);
    #[doc = "Unprotected"]
    pub const UNPROTECTED: Self = Self(0x50fa_50fa);
}
impl SecureapprotectPall {
    pub const fn from_bits(val: u32) -> SecureapprotectPall {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for SecureapprotectPall {
    #[inline(always)]
    fn from(val: u32) -> SecureapprotectPall {
        SecureapprotectPall::from_bits(val)
    }
}
impl From<SecureapprotectPall> for u32 {
    #[inline(always)]
    fn from(val: SecureapprotectPall) -> u32 {
        SecureapprotectPall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum State {
    #[doc = "Key value registers can no longer be read or pushed"]
    REVOKED = 0x0,
    #[doc = "Key value registers are readable (if enabled) and can be pushed (if enabled)"]
    ACTIVE = 0x01,
}
impl State {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> State {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
pub enum Vreghvout {
    #[doc = "1.8 V"]
    _1V8 = 0x0,
    #[doc = "2.1 V"]
    _2V1 = 0x01,
    #[doc = "2.4 V"]
    _2V4 = 0x02,
    #[doc = "2.7 V"]
    _2V7 = 0x03,
    #[doc = "3.0 V"]
    _3V0 = 0x04,
    #[doc = "3.3 V"]
    _3V3 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Default voltage: 1.8 V"]
    DEFAULT = 0x07,
}
impl Vreghvout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vreghvout {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vreghvout {
    #[inline(always)]
    fn from(val: u8) -> Vreghvout {
        Vreghvout::from_bits(val)
    }
}
impl From<Vreghvout> for u8 {
    #[inline(always)]
    fn from(val: Vreghvout) -> u8 {
        Vreghvout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Write {
    #[doc = "Disable write to the key value registers"]
    DISABLED = 0x0,
    #[doc = "Enable write to the key value registers"]
    ENABLED = 0x01,
}
impl Write {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Write {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Write {
    #[inline(always)]
    fn from(val: u8) -> Write {
        Write::from_bits(val)
    }
}
impl From<Write> for u8 {
    #[inline(always)]
    fn from(val: Write) -> u8 {
        Write::to_bits(val)
    }
}
