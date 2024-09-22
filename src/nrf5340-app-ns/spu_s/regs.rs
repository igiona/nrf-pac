#[doc = "Show implemented features for the current device"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap(pub u32);
impl Cap {
    #[doc = "Show Arm TrustZone status"]
    #[inline(always)]
    pub const fn tzm(&self) -> super::vals::Tzm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tzm::from_bits(val as u8)
    }
    #[doc = "Show Arm TrustZone status"]
    #[inline(always)]
    pub fn set_tzm(&mut self, val: super::vals::Tzm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Cap {
    #[inline(always)]
    fn default() -> Cap {
        Cap(0)
    }
}
#[doc = "Configure bits to lock down CPU features at runtime"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpulock(pub u32);
impl Cpulock {
    #[doc = "Write '1' to prevent updating the secure interrupt configuration until the next reset"]
    #[inline(always)]
    pub const fn locksvtaircr(&self) -> super::vals::Locksvtaircr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Locksvtaircr::from_bits(val as u8)
    }
    #[doc = "Write '1' to prevent updating the secure interrupt configuration until the next reset"]
    #[inline(always)]
    pub fn set_locksvtaircr(&mut self, val: super::vals::Locksvtaircr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to prevent updating the non-secure vector table base address until the next reset"]
    #[inline(always)]
    pub const fn locknsvtor(&self) -> super::vals::Locknsvtor {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Locknsvtor::from_bits(val as u8)
    }
    #[doc = "Write '1' to prevent updating the non-secure vector table base address until the next reset"]
    #[inline(always)]
    pub fn set_locknsvtor(&mut self, val: super::vals::Locknsvtor) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to prevent updating the secure MPU regions until the next reset"]
    #[inline(always)]
    pub const fn locksmpu(&self) -> super::vals::Locksmpu {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Locksmpu::from_bits(val as u8)
    }
    #[doc = "Write '1' to prevent updating the secure MPU regions until the next reset"]
    #[inline(always)]
    pub fn set_locksmpu(&mut self, val: super::vals::Locksmpu) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to prevent updating the Non-secure MPU regions until the next reset"]
    #[inline(always)]
    pub const fn locknsmpu(&self) -> super::vals::Locknsmpu {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Locknsmpu::from_bits(val as u8)
    }
    #[doc = "Write '1' to prevent updating the Non-secure MPU regions until the next reset"]
    #[inline(always)]
    pub fn set_locknsmpu(&mut self, val: super::vals::Locknsmpu) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to prevent updating the secure SAU regions until the next reset"]
    #[inline(always)]
    pub const fn locksau(&self) -> super::vals::Locksau {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Locksau::from_bits(val as u8)
    }
    #[doc = "Write '1' to prevent updating the secure SAU regions until the next reset"]
    #[inline(always)]
    pub fn set_locksau(&mut self, val: super::vals::Locksau) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Cpulock {
    #[inline(always)]
    fn default() -> Cpulock {
        Cpulock(0)
    }
}
#[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DppiLock(pub u32);
impl DppiLock {
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::DppiLockLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DppiLockLock::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::DppiLockLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for DppiLock {
    #[inline(always)]
    fn default() -> DppiLock {
        DppiLock(0)
    }
}
#[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DppiPerm(pub u32);
impl DppiPerm {
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel0(&self) -> super::vals::Channel0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Channel0::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel0(&mut self, val: super::vals::Channel0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel1(&self) -> super::vals::Channel1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Channel1::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel1(&mut self, val: super::vals::Channel1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel2(&self) -> super::vals::Channel2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Channel2::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel2(&mut self, val: super::vals::Channel2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel3(&self) -> super::vals::Channel3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Channel3::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel3(&mut self, val: super::vals::Channel3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel4(&self) -> super::vals::Channel4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Channel4::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel4(&mut self, val: super::vals::Channel4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel5(&self) -> super::vals::Channel5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Channel5::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel5(&mut self, val: super::vals::Channel5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel6(&self) -> super::vals::Channel6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Channel6::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel6(&mut self, val: super::vals::Channel6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel7(&self) -> super::vals::Channel7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Channel7::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel7(&mut self, val: super::vals::Channel7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel8(&self) -> super::vals::Channel8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Channel8::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel8(&mut self, val: super::vals::Channel8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel9(&self) -> super::vals::Channel9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Channel9::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel9(&mut self, val: super::vals::Channel9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel10(&self) -> super::vals::Channel10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Channel10::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel10(&mut self, val: super::vals::Channel10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel11(&self) -> super::vals::Channel11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Channel11::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel11(&mut self, val: super::vals::Channel11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel12(&self) -> super::vals::Channel12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Channel12::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel12(&mut self, val: super::vals::Channel12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel13(&self) -> super::vals::Channel13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Channel13::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel13(&mut self, val: super::vals::Channel13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel14(&self) -> super::vals::Channel14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Channel14::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel14(&mut self, val: super::vals::Channel14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel15(&self) -> super::vals::Channel15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Channel15::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel15(&mut self, val: super::vals::Channel15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel16(&self) -> super::vals::Channel16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Channel16::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel16(&mut self, val: super::vals::Channel16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel17(&self) -> super::vals::Channel17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Channel17::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel17(&mut self, val: super::vals::Channel17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel18(&self) -> super::vals::Channel18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Channel18::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel18(&mut self, val: super::vals::Channel18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel19(&self) -> super::vals::Channel19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Channel19::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel19(&mut self, val: super::vals::Channel19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel20(&self) -> super::vals::Channel20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Channel20::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel20(&mut self, val: super::vals::Channel20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel21(&self) -> super::vals::Channel21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Channel21::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel21(&mut self, val: super::vals::Channel21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel22(&self) -> super::vals::Channel22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Channel22::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel22(&mut self, val: super::vals::Channel22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel23(&self) -> super::vals::Channel23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Channel23::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel23(&mut self, val: super::vals::Channel23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel24(&self) -> super::vals::Channel24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Channel24::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel24(&mut self, val: super::vals::Channel24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel25(&self) -> super::vals::Channel25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Channel25::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel25(&mut self, val: super::vals::Channel25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel26(&self) -> super::vals::Channel26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Channel26::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel26(&mut self, val: super::vals::Channel26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel27(&self) -> super::vals::Channel27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Channel27::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel27(&mut self, val: super::vals::Channel27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel28(&self) -> super::vals::Channel28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Channel28::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel28(&mut self, val: super::vals::Channel28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel29(&self) -> super::vals::Channel29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Channel29::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel29(&mut self, val: super::vals::Channel29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel30(&self) -> super::vals::Channel30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Channel30::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel30(&mut self, val: super::vals::Channel30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub const fn channel31(&self) -> super::vals::Channel31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Channel31::from_bits(val as u8)
    }
    #[doc = "Select secure attribute"]
    #[inline(always)]
    pub fn set_channel31(&mut self, val: super::vals::Channel31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for DppiPerm {
    #[inline(always)]
    fn default() -> DppiPerm {
        DppiPerm(0)
    }
}
#[doc = "Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtdomainPerm(pub u32);
impl ExtdomainPerm {
    #[doc = "Define configuration capabilities for TrustZone Cortex-M secure attribute"]
    #[inline(always)]
    pub const fn securemapping(&self) -> super::vals::ExtdomainPermSecuremapping {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::ExtdomainPermSecuremapping::from_bits(val as u8)
    }
    #[doc = "Define configuration capabilities for TrustZone Cortex-M secure attribute"]
    #[inline(always)]
    pub fn set_securemapping(&mut self, val: super::vals::ExtdomainPermSecuremapping) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Peripheral security mapping"]
    #[inline(always)]
    pub const fn secattr(&self) -> super::vals::ExtdomainPermSecattr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ExtdomainPermSecattr::from_bits(val as u8)
    }
    #[doc = "Peripheral security mapping"]
    #[inline(always)]
    pub fn set_secattr(&mut self, val: super::vals::ExtdomainPermSecattr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::ExtdomainPermLock {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ExtdomainPermLock::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::ExtdomainPermLock) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for ExtdomainPerm {
    #[inline(always)]
    fn default() -> ExtdomainPerm {
        ExtdomainPerm(0)
    }
}
#[doc = "Description cluster: Define which flash region can contain the non-secure callable (NSC) region n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashnscRegion(pub u32);
impl FlashnscRegion {
    #[doc = "Region number"]
    #[inline(always)]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Region number"]
    #[inline(always)]
    pub fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::FlashnscRegionLock {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FlashnscRegionLock::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::FlashnscRegionLock) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for FlashnscRegion {
    #[inline(always)]
    fn default() -> FlashnscRegion {
        FlashnscRegion(0)
    }
}
#[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashnscSize(pub u32);
impl FlashnscSize {
    #[doc = "Size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub const fn size(&self) -> super::vals::FlashnscSizeSize {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::FlashnscSizeSize::from_bits(val as u8)
    }
    #[doc = "Size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub fn set_size(&mut self, val: super::vals::FlashnscSizeSize) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::FlashnscSizeLock {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FlashnscSizeLock::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::FlashnscSizeLock) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for FlashnscSize {
    #[inline(always)]
    fn default() -> FlashnscSize {
        FlashnscSize(0)
    }
}
#[doc = "Description cluster: Access permissions for flash region n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashregionPerm(pub u32);
impl FlashregionPerm {
    #[doc = "Configure instruction fetch permissions from flash region n"]
    #[inline(always)]
    pub const fn execute(&self) -> super::vals::FlashregionPermExecute {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlashregionPermExecute::from_bits(val as u8)
    }
    #[doc = "Configure instruction fetch permissions from flash region n"]
    #[inline(always)]
    pub fn set_execute(&mut self, val: super::vals::FlashregionPermExecute) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Configure write permission for flash region n"]
    #[inline(always)]
    pub const fn write(&self) -> super::vals::FlashregionPermWrite {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::FlashregionPermWrite::from_bits(val as u8)
    }
    #[doc = "Configure write permission for flash region n"]
    #[inline(always)]
    pub fn set_write(&mut self, val: super::vals::FlashregionPermWrite) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Configure read permissions for flash region n"]
    #[inline(always)]
    pub const fn read(&self) -> super::vals::FlashregionPermRead {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FlashregionPermRead::from_bits(val as u8)
    }
    #[doc = "Configure read permissions for flash region n"]
    #[inline(always)]
    pub fn set_read(&mut self, val: super::vals::FlashregionPermRead) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Security attribute for flash region n"]
    #[inline(always)]
    pub const fn secattr(&self) -> super::vals::FlashregionPermSecattr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::FlashregionPermSecattr::from_bits(val as u8)
    }
    #[doc = "Security attribute for flash region n"]
    #[inline(always)]
    pub fn set_secattr(&mut self, val: super::vals::FlashregionPermSecattr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::FlashregionPermLock {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FlashregionPermLock::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::FlashregionPermLock) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for FlashregionPerm {
    #[inline(always)]
    fn default() -> FlashregionPerm {
        FlashregionPerm(0)
    }
}
#[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioportLock(pub u32);
impl GpioportLock {
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::GpioportLockLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::GpioportLockLock::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::GpioportLockLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for GpioportLock {
    #[inline(always)]
    fn default() -> GpioportLock {
        GpioportLock(0)
    }
}
#[doc = "Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioportPerm(pub u32);
impl GpioportPerm {
    #[doc = "Select secure attribute attribute for PIN 0."]
    #[inline(always)]
    pub const fn pin0(&self) -> super::vals::Pin0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pin0::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 0."]
    #[inline(always)]
    pub fn set_pin0(&mut self, val: super::vals::Pin0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Select secure attribute attribute for PIN 1."]
    #[inline(always)]
    pub const fn pin1(&self) -> super::vals::Pin1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pin1::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 1."]
    #[inline(always)]
    pub fn set_pin1(&mut self, val: super::vals::Pin1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Select secure attribute attribute for PIN 2."]
    #[inline(always)]
    pub const fn pin2(&self) -> super::vals::Pin2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pin2::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 2."]
    #[inline(always)]
    pub fn set_pin2(&mut self, val: super::vals::Pin2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Select secure attribute attribute for PIN 3."]
    #[inline(always)]
    pub const fn pin3(&self) -> super::vals::Pin3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pin3::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 3."]
    #[inline(always)]
    pub fn set_pin3(&mut self, val: super::vals::Pin3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Select secure attribute attribute for PIN 4."]
    #[inline(always)]
    pub const fn pin4(&self) -> super::vals::Pin4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pin4::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 4."]
    #[inline(always)]
    pub fn set_pin4(&mut self, val: super::vals::Pin4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Select secure attribute attribute for PIN 5."]
    #[inline(always)]
    pub const fn pin5(&self) -> super::vals::Pin5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pin5::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 5."]
    #[inline(always)]
    pub fn set_pin5(&mut self, val: super::vals::Pin5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Select secure attribute attribute for PIN 6."]
    #[inline(always)]
    pub const fn pin6(&self) -> super::vals::Pin6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pin6::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 6."]
    #[inline(always)]
    pub fn set_pin6(&mut self, val: super::vals::Pin6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Select secure attribute attribute for PIN 7."]
    #[inline(always)]
    pub const fn pin7(&self) -> super::vals::Pin7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pin7::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 7."]
    #[inline(always)]
    pub fn set_pin7(&mut self, val: super::vals::Pin7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Select secure attribute attribute for PIN 8."]
    #[inline(always)]
    pub const fn pin8(&self) -> super::vals::Pin8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pin8::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 8."]
    #[inline(always)]
    pub fn set_pin8(&mut self, val: super::vals::Pin8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Select secure attribute attribute for PIN 9."]
    #[inline(always)]
    pub const fn pin9(&self) -> super::vals::Pin9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pin9::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 9."]
    #[inline(always)]
    pub fn set_pin9(&mut self, val: super::vals::Pin9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Select secure attribute attribute for PIN 10."]
    #[inline(always)]
    pub const fn pin10(&self) -> super::vals::Pin10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pin10::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 10."]
    #[inline(always)]
    pub fn set_pin10(&mut self, val: super::vals::Pin10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Select secure attribute attribute for PIN 11."]
    #[inline(always)]
    pub const fn pin11(&self) -> super::vals::Pin11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pin11::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 11."]
    #[inline(always)]
    pub fn set_pin11(&mut self, val: super::vals::Pin11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Select secure attribute attribute for PIN 12."]
    #[inline(always)]
    pub const fn pin12(&self) -> super::vals::Pin12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pin12::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 12."]
    #[inline(always)]
    pub fn set_pin12(&mut self, val: super::vals::Pin12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Select secure attribute attribute for PIN 13."]
    #[inline(always)]
    pub const fn pin13(&self) -> super::vals::Pin13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pin13::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 13."]
    #[inline(always)]
    pub fn set_pin13(&mut self, val: super::vals::Pin13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Select secure attribute attribute for PIN 14."]
    #[inline(always)]
    pub const fn pin14(&self) -> super::vals::Pin14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pin14::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 14."]
    #[inline(always)]
    pub fn set_pin14(&mut self, val: super::vals::Pin14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Select secure attribute attribute for PIN 15."]
    #[inline(always)]
    pub const fn pin15(&self) -> super::vals::Pin15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pin15::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 15."]
    #[inline(always)]
    pub fn set_pin15(&mut self, val: super::vals::Pin15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Select secure attribute attribute for PIN 16."]
    #[inline(always)]
    pub const fn pin16(&self) -> super::vals::Pin16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pin16::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 16."]
    #[inline(always)]
    pub fn set_pin16(&mut self, val: super::vals::Pin16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Select secure attribute attribute for PIN 17."]
    #[inline(always)]
    pub const fn pin17(&self) -> super::vals::Pin17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pin17::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 17."]
    #[inline(always)]
    pub fn set_pin17(&mut self, val: super::vals::Pin17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Select secure attribute attribute for PIN 18."]
    #[inline(always)]
    pub const fn pin18(&self) -> super::vals::Pin18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pin18::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 18."]
    #[inline(always)]
    pub fn set_pin18(&mut self, val: super::vals::Pin18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Select secure attribute attribute for PIN 19."]
    #[inline(always)]
    pub const fn pin19(&self) -> super::vals::Pin19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pin19::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 19."]
    #[inline(always)]
    pub fn set_pin19(&mut self, val: super::vals::Pin19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Select secure attribute attribute for PIN 20."]
    #[inline(always)]
    pub const fn pin20(&self) -> super::vals::Pin20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pin20::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 20."]
    #[inline(always)]
    pub fn set_pin20(&mut self, val: super::vals::Pin20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Select secure attribute attribute for PIN 21."]
    #[inline(always)]
    pub const fn pin21(&self) -> super::vals::Pin21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pin21::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 21."]
    #[inline(always)]
    pub fn set_pin21(&mut self, val: super::vals::Pin21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Select secure attribute attribute for PIN 22."]
    #[inline(always)]
    pub const fn pin22(&self) -> super::vals::Pin22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pin22::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 22."]
    #[inline(always)]
    pub fn set_pin22(&mut self, val: super::vals::Pin22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Select secure attribute attribute for PIN 23."]
    #[inline(always)]
    pub const fn pin23(&self) -> super::vals::Pin23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pin23::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 23."]
    #[inline(always)]
    pub fn set_pin23(&mut self, val: super::vals::Pin23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Select secure attribute attribute for PIN 24."]
    #[inline(always)]
    pub const fn pin24(&self) -> super::vals::Pin24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pin24::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 24."]
    #[inline(always)]
    pub fn set_pin24(&mut self, val: super::vals::Pin24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Select secure attribute attribute for PIN 25."]
    #[inline(always)]
    pub const fn pin25(&self) -> super::vals::Pin25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pin25::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 25."]
    #[inline(always)]
    pub fn set_pin25(&mut self, val: super::vals::Pin25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Select secure attribute attribute for PIN 26."]
    #[inline(always)]
    pub const fn pin26(&self) -> super::vals::Pin26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pin26::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 26."]
    #[inline(always)]
    pub fn set_pin26(&mut self, val: super::vals::Pin26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Select secure attribute attribute for PIN 27."]
    #[inline(always)]
    pub const fn pin27(&self) -> super::vals::Pin27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pin27::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 27."]
    #[inline(always)]
    pub fn set_pin27(&mut self, val: super::vals::Pin27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Select secure attribute attribute for PIN 28."]
    #[inline(always)]
    pub const fn pin28(&self) -> super::vals::Pin28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pin28::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 28."]
    #[inline(always)]
    pub fn set_pin28(&mut self, val: super::vals::Pin28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Select secure attribute attribute for PIN 29."]
    #[inline(always)]
    pub const fn pin29(&self) -> super::vals::Pin29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pin29::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 29."]
    #[inline(always)]
    pub fn set_pin29(&mut self, val: super::vals::Pin29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Select secure attribute attribute for PIN 30."]
    #[inline(always)]
    pub const fn pin30(&self) -> super::vals::Pin30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pin30::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 30."]
    #[inline(always)]
    pub fn set_pin30(&mut self, val: super::vals::Pin30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Select secure attribute attribute for PIN 31."]
    #[inline(always)]
    pub const fn pin31(&self) -> super::vals::Pin31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pin31::from_bits(val as u8)
    }
    #[doc = "Select secure attribute attribute for PIN 31."]
    #[inline(always)]
    pub fn set_pin31(&mut self, val: super::vals::Pin31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for GpioportPerm {
    #[inline(always)]
    fn default() -> GpioportPerm {
        GpioportPerm(0)
    }
}
#[doc = "Enable or disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Enable or disable interrupt for event RAMACCERR"]
    #[inline(always)]
    pub const fn ramaccerr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event RAMACCERR"]
    #[inline(always)]
    pub fn set_ramaccerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable interrupt for event FLASHACCERR"]
    #[inline(always)]
    pub const fn flashaccerr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event FLASHACCERR"]
    #[inline(always)]
    pub fn set_flashaccerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    pub const fn periphaccerr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    pub fn set_periphaccerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Description cluster: List capabilities and access permissions for the peripheral with ID n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PeriphidPerm(pub u32);
impl PeriphidPerm {
    #[doc = "Define configuration capabilities for Arm TrustZone Cortex-M secure attribute"]
    #[inline(always)]
    pub const fn securemapping(&self) -> super::vals::PeriphidPermSecuremapping {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PeriphidPermSecuremapping::from_bits(val as u8)
    }
    #[doc = "Define configuration capabilities for Arm TrustZone Cortex-M secure attribute"]
    #[inline(always)]
    pub fn set_securemapping(&mut self, val: super::vals::PeriphidPermSecuremapping) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Indicates if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself"]
    #[inline(always)]
    pub const fn dma(&self) -> super::vals::Dma {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Dma::from_bits(val as u8)
    }
    #[doc = "Indicates if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself"]
    #[inline(always)]
    pub fn set_dma(&mut self, val: super::vals::Dma) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Peripheral security mapping"]
    #[inline(always)]
    pub const fn secattr(&self) -> super::vals::PeriphidPermSecattr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PeriphidPermSecattr::from_bits(val as u8)
    }
    #[doc = "Peripheral security mapping"]
    #[inline(always)]
    pub fn set_secattr(&mut self, val: super::vals::PeriphidPermSecattr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Security attribution for the DMA transfer"]
    #[inline(always)]
    pub const fn dmasec(&self) -> super::vals::Dmasec {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmasec::from_bits(val as u8)
    }
    #[doc = "Security attribution for the DMA transfer"]
    #[inline(always)]
    pub fn set_dmasec(&mut self, val: super::vals::Dmasec) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::PeriphidPermLock {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PeriphidPermLock::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::PeriphidPermLock) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicate if a peripheral is present with ID n"]
    #[inline(always)]
    pub const fn present(&self) -> super::vals::Present {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Present::from_bits(val as u8)
    }
    #[doc = "Indicate if a peripheral is present with ID n"]
    #[inline(always)]
    pub fn set_present(&mut self, val: super::vals::Present) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PeriphidPerm {
    #[inline(always)]
    fn default() -> PeriphidPerm {
        PeriphidPerm(0)
    }
}
#[doc = "Publish configuration for event FLASHACCERR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishFlashaccerr(pub u32);
impl PublishFlashaccerr {
    #[doc = "DPPI channel that event FLASHACCERR will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event FLASHACCERR will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishFlashaccerrEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishFlashaccerrEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishFlashaccerrEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishFlashaccerr {
    #[inline(always)]
    fn default() -> PublishFlashaccerr {
        PublishFlashaccerr(0)
    }
}
#[doc = "Publish configuration for event PERIPHACCERR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishPeriphaccerr(pub u32);
impl PublishPeriphaccerr {
    #[doc = "DPPI channel that event PERIPHACCERR will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event PERIPHACCERR will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishPeriphaccerrEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishPeriphaccerrEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishPeriphaccerrEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishPeriphaccerr {
    #[inline(always)]
    fn default() -> PublishPeriphaccerr {
        PublishPeriphaccerr(0)
    }
}
#[doc = "Publish configuration for event RAMACCERR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishRamaccerr(pub u32);
impl PublishRamaccerr {
    #[doc = "DPPI channel that event RAMACCERR will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event RAMACCERR will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishRamaccerrEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishRamaccerrEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishRamaccerrEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishRamaccerr {
    #[inline(always)]
    fn default() -> PublishRamaccerr {
        PublishRamaccerr(0)
    }
}
#[doc = "Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamnscRegion(pub u32);
impl RamnscRegion {
    #[doc = "Region number"]
    #[inline(always)]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Region number"]
    #[inline(always)]
    pub fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::RamnscRegionLock {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RamnscRegionLock::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::RamnscRegionLock) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for RamnscRegion {
    #[inline(always)]
    fn default() -> RamnscRegion {
        RamnscRegion(0)
    }
}
#[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamnscSize(pub u32);
impl RamnscSize {
    #[doc = "Size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub const fn size(&self) -> super::vals::RamnscSizeSize {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::RamnscSizeSize::from_bits(val as u8)
    }
    #[doc = "Size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub fn set_size(&mut self, val: super::vals::RamnscSizeSize) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::RamnscSizeLock {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RamnscSizeLock::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::RamnscSizeLock) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for RamnscSize {
    #[inline(always)]
    fn default() -> RamnscSize {
        RamnscSize(0)
    }
}
#[doc = "Description cluster: Access permissions for RAM region n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamregionPerm(pub u32);
impl RamregionPerm {
    #[doc = "Configure instruction fetch permissions from RAM region n"]
    #[inline(always)]
    pub const fn execute(&self) -> super::vals::RamregionPermExecute {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RamregionPermExecute::from_bits(val as u8)
    }
    #[doc = "Configure instruction fetch permissions from RAM region n"]
    #[inline(always)]
    pub fn set_execute(&mut self, val: super::vals::RamregionPermExecute) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Configure write permission for RAM region n"]
    #[inline(always)]
    pub const fn write(&self) -> super::vals::RamregionPermWrite {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RamregionPermWrite::from_bits(val as u8)
    }
    #[doc = "Configure write permission for RAM region n"]
    #[inline(always)]
    pub fn set_write(&mut self, val: super::vals::RamregionPermWrite) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Configure read permissions for RAM region n"]
    #[inline(always)]
    pub const fn read(&self) -> super::vals::RamregionPermRead {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RamregionPermRead::from_bits(val as u8)
    }
    #[doc = "Configure read permissions for RAM region n"]
    #[inline(always)]
    pub fn set_read(&mut self, val: super::vals::RamregionPermRead) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Security attribute for RAM region n"]
    #[inline(always)]
    pub const fn secattr(&self) -> super::vals::RamregionPermSecattr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::RamregionPermSecattr::from_bits(val as u8)
    }
    #[doc = "Security attribute for RAM region n"]
    #[inline(always)]
    pub fn set_secattr(&mut self, val: super::vals::RamregionPermSecattr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::RamregionPermLock {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RamregionPermLock::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::RamregionPermLock) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for RamregionPerm {
    #[inline(always)]
    fn default() -> RamregionPerm {
        RamregionPerm(0)
    }
}
