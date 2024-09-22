#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub const fn receive0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn set_receive0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub const fn receive1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn set_receive1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub const fn receive2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn set_receive2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub const fn receive3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn set_receive3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub const fn receive4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn set_receive4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub const fn receive5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn set_receive5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub const fn receive6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn set_receive6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub const fn receive7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn set_receive7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub const fn receive8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub fn set_receive8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub const fn receive9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub fn set_receive9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub const fn receive10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub fn set_receive10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub const fn receive11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub fn set_receive11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub const fn receive12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub fn set_receive12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub const fn receive13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub fn set_receive13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub const fn receive14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub fn set_receive14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub const fn receive15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub fn set_receive15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Pending interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intpend(pub u32);
impl Intpend {
    #[doc = "Read pending status of interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub const fn receive0(&self) -> super::vals::IntpendReceive0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IntpendReceive0::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn set_receive0(&mut self, val: super::vals::IntpendReceive0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub const fn receive1(&self) -> super::vals::IntpendReceive1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IntpendReceive1::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn set_receive1(&mut self, val: super::vals::IntpendReceive1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub const fn receive2(&self) -> super::vals::IntpendReceive2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::IntpendReceive2::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn set_receive2(&mut self, val: super::vals::IntpendReceive2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub const fn receive3(&self) -> super::vals::IntpendReceive3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::IntpendReceive3::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn set_receive3(&mut self, val: super::vals::IntpendReceive3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub const fn receive4(&self) -> super::vals::IntpendReceive4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::IntpendReceive4::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn set_receive4(&mut self, val: super::vals::IntpendReceive4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub const fn receive5(&self) -> super::vals::IntpendReceive5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::IntpendReceive5::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn set_receive5(&mut self, val: super::vals::IntpendReceive5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub const fn receive6(&self) -> super::vals::IntpendReceive6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::IntpendReceive6::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn set_receive6(&mut self, val: super::vals::IntpendReceive6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub const fn receive7(&self) -> super::vals::IntpendReceive7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::IntpendReceive7::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn set_receive7(&mut self, val: super::vals::IntpendReceive7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub const fn receive8(&self) -> super::vals::IntpendReceive8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::IntpendReceive8::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub fn set_receive8(&mut self, val: super::vals::IntpendReceive8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub const fn receive9(&self) -> super::vals::IntpendReceive9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::IntpendReceive9::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub fn set_receive9(&mut self, val: super::vals::IntpendReceive9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub const fn receive10(&self) -> super::vals::IntpendReceive10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::IntpendReceive10::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub fn set_receive10(&mut self, val: super::vals::IntpendReceive10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub const fn receive11(&self) -> super::vals::IntpendReceive11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::IntpendReceive11::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub fn set_receive11(&mut self, val: super::vals::IntpendReceive11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub const fn receive12(&self) -> super::vals::IntpendReceive12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::IntpendReceive12::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub fn set_receive12(&mut self, val: super::vals::IntpendReceive12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub const fn receive13(&self) -> super::vals::IntpendReceive13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::IntpendReceive13::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub fn set_receive13(&mut self, val: super::vals::IntpendReceive13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub const fn receive14(&self) -> super::vals::IntpendReceive14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::IntpendReceive14::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub fn set_receive14(&mut self, val: super::vals::IntpendReceive14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub const fn receive15(&self) -> super::vals::IntpendReceive15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::IntpendReceive15::from_bits(val as u8)
    }
    #[doc = "Read pending status of interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub fn set_receive15(&mut self, val: super::vals::IntpendReceive15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Intpend {
    #[inline(always)]
    fn default() -> Intpend {
        Intpend(0)
    }
}
#[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishReceive(pub u32);
impl PublishReceive {
    #[doc = "DPPI channel that event RECEIVE\\[n\\] will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event RECEIVE\\[n\\] will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishReceiveEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishReceiveEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishReceiveEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishReceive {
    #[inline(always)]
    fn default() -> PublishReceive {
        PublishReceive(0)
    }
}
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReceiveCnf(pub u32);
impl ReceiveCnf {
    #[doc = "Enable subscription to IPC channel 0"]
    #[inline(always)]
    pub const fn chen0(&self) -> super::vals::ReceiveCnfChen0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ReceiveCnfChen0::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 0"]
    #[inline(always)]
    pub fn set_chen0(&mut self, val: super::vals::ReceiveCnfChen0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable subscription to IPC channel 1"]
    #[inline(always)]
    pub const fn chen1(&self) -> super::vals::ReceiveCnfChen1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ReceiveCnfChen1::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 1"]
    #[inline(always)]
    pub fn set_chen1(&mut self, val: super::vals::ReceiveCnfChen1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable subscription to IPC channel 2"]
    #[inline(always)]
    pub const fn chen2(&self) -> super::vals::ReceiveCnfChen2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ReceiveCnfChen2::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 2"]
    #[inline(always)]
    pub fn set_chen2(&mut self, val: super::vals::ReceiveCnfChen2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable subscription to IPC channel 3"]
    #[inline(always)]
    pub const fn chen3(&self) -> super::vals::ReceiveCnfChen3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ReceiveCnfChen3::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 3"]
    #[inline(always)]
    pub fn set_chen3(&mut self, val: super::vals::ReceiveCnfChen3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable subscription to IPC channel 4"]
    #[inline(always)]
    pub const fn chen4(&self) -> super::vals::ReceiveCnfChen4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ReceiveCnfChen4::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 4"]
    #[inline(always)]
    pub fn set_chen4(&mut self, val: super::vals::ReceiveCnfChen4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable subscription to IPC channel 5"]
    #[inline(always)]
    pub const fn chen5(&self) -> super::vals::ReceiveCnfChen5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ReceiveCnfChen5::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 5"]
    #[inline(always)]
    pub fn set_chen5(&mut self, val: super::vals::ReceiveCnfChen5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable subscription to IPC channel 6"]
    #[inline(always)]
    pub const fn chen6(&self) -> super::vals::ReceiveCnfChen6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ReceiveCnfChen6::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 6"]
    #[inline(always)]
    pub fn set_chen6(&mut self, val: super::vals::ReceiveCnfChen6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable subscription to IPC channel 7"]
    #[inline(always)]
    pub const fn chen7(&self) -> super::vals::ReceiveCnfChen7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ReceiveCnfChen7::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 7"]
    #[inline(always)]
    pub fn set_chen7(&mut self, val: super::vals::ReceiveCnfChen7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable subscription to IPC channel 8"]
    #[inline(always)]
    pub const fn chen8(&self) -> super::vals::ReceiveCnfChen8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ReceiveCnfChen8::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 8"]
    #[inline(always)]
    pub fn set_chen8(&mut self, val: super::vals::ReceiveCnfChen8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable subscription to IPC channel 9"]
    #[inline(always)]
    pub const fn chen9(&self) -> super::vals::ReceiveCnfChen9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::ReceiveCnfChen9::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 9"]
    #[inline(always)]
    pub fn set_chen9(&mut self, val: super::vals::ReceiveCnfChen9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable subscription to IPC channel 10"]
    #[inline(always)]
    pub const fn chen10(&self) -> super::vals::ReceiveCnfChen10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ReceiveCnfChen10::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 10"]
    #[inline(always)]
    pub fn set_chen10(&mut self, val: super::vals::ReceiveCnfChen10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable subscription to IPC channel 11"]
    #[inline(always)]
    pub const fn chen11(&self) -> super::vals::ReceiveCnfChen11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::ReceiveCnfChen11::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 11"]
    #[inline(always)]
    pub fn set_chen11(&mut self, val: super::vals::ReceiveCnfChen11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable subscription to IPC channel 12"]
    #[inline(always)]
    pub const fn chen12(&self) -> super::vals::ReceiveCnfChen12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::ReceiveCnfChen12::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 12"]
    #[inline(always)]
    pub fn set_chen12(&mut self, val: super::vals::ReceiveCnfChen12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable subscription to IPC channel 13"]
    #[inline(always)]
    pub const fn chen13(&self) -> super::vals::ReceiveCnfChen13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::ReceiveCnfChen13::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 13"]
    #[inline(always)]
    pub fn set_chen13(&mut self, val: super::vals::ReceiveCnfChen13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable subscription to IPC channel 14"]
    #[inline(always)]
    pub const fn chen14(&self) -> super::vals::ReceiveCnfChen14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::ReceiveCnfChen14::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 14"]
    #[inline(always)]
    pub fn set_chen14(&mut self, val: super::vals::ReceiveCnfChen14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable subscription to IPC channel 15"]
    #[inline(always)]
    pub const fn chen15(&self) -> super::vals::ReceiveCnfChen15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::ReceiveCnfChen15::from_bits(val as u8)
    }
    #[doc = "Enable subscription to IPC channel 15"]
    #[inline(always)]
    pub fn set_chen15(&mut self, val: super::vals::ReceiveCnfChen15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for ReceiveCnf {
    #[inline(always)]
    fn default() -> ReceiveCnf {
        ReceiveCnf(0)
    }
}
#[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SendCnf(pub u32);
impl SendCnf {
    #[doc = "Enable broadcasting on IPC channel 0"]
    #[inline(always)]
    pub const fn chen0(&self) -> super::vals::SendCnfChen0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SendCnfChen0::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 0"]
    #[inline(always)]
    pub fn set_chen0(&mut self, val: super::vals::SendCnfChen0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable broadcasting on IPC channel 1"]
    #[inline(always)]
    pub const fn chen1(&self) -> super::vals::SendCnfChen1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SendCnfChen1::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 1"]
    #[inline(always)]
    pub fn set_chen1(&mut self, val: super::vals::SendCnfChen1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable broadcasting on IPC channel 2"]
    #[inline(always)]
    pub const fn chen2(&self) -> super::vals::SendCnfChen2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SendCnfChen2::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 2"]
    #[inline(always)]
    pub fn set_chen2(&mut self, val: super::vals::SendCnfChen2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable broadcasting on IPC channel 3"]
    #[inline(always)]
    pub const fn chen3(&self) -> super::vals::SendCnfChen3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SendCnfChen3::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 3"]
    #[inline(always)]
    pub fn set_chen3(&mut self, val: super::vals::SendCnfChen3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable broadcasting on IPC channel 4"]
    #[inline(always)]
    pub const fn chen4(&self) -> super::vals::SendCnfChen4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SendCnfChen4::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 4"]
    #[inline(always)]
    pub fn set_chen4(&mut self, val: super::vals::SendCnfChen4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable broadcasting on IPC channel 5"]
    #[inline(always)]
    pub const fn chen5(&self) -> super::vals::SendCnfChen5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SendCnfChen5::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 5"]
    #[inline(always)]
    pub fn set_chen5(&mut self, val: super::vals::SendCnfChen5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable broadcasting on IPC channel 6"]
    #[inline(always)]
    pub const fn chen6(&self) -> super::vals::SendCnfChen6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SendCnfChen6::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 6"]
    #[inline(always)]
    pub fn set_chen6(&mut self, val: super::vals::SendCnfChen6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable broadcasting on IPC channel 7"]
    #[inline(always)]
    pub const fn chen7(&self) -> super::vals::SendCnfChen7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SendCnfChen7::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 7"]
    #[inline(always)]
    pub fn set_chen7(&mut self, val: super::vals::SendCnfChen7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable broadcasting on IPC channel 8"]
    #[inline(always)]
    pub const fn chen8(&self) -> super::vals::SendCnfChen8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::SendCnfChen8::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 8"]
    #[inline(always)]
    pub fn set_chen8(&mut self, val: super::vals::SendCnfChen8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable broadcasting on IPC channel 9"]
    #[inline(always)]
    pub const fn chen9(&self) -> super::vals::SendCnfChen9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::SendCnfChen9::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 9"]
    #[inline(always)]
    pub fn set_chen9(&mut self, val: super::vals::SendCnfChen9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable broadcasting on IPC channel 10"]
    #[inline(always)]
    pub const fn chen10(&self) -> super::vals::SendCnfChen10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::SendCnfChen10::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 10"]
    #[inline(always)]
    pub fn set_chen10(&mut self, val: super::vals::SendCnfChen10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable broadcasting on IPC channel 11"]
    #[inline(always)]
    pub const fn chen11(&self) -> super::vals::SendCnfChen11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SendCnfChen11::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 11"]
    #[inline(always)]
    pub fn set_chen11(&mut self, val: super::vals::SendCnfChen11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable broadcasting on IPC channel 12"]
    #[inline(always)]
    pub const fn chen12(&self) -> super::vals::SendCnfChen12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SendCnfChen12::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 12"]
    #[inline(always)]
    pub fn set_chen12(&mut self, val: super::vals::SendCnfChen12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable broadcasting on IPC channel 13"]
    #[inline(always)]
    pub const fn chen13(&self) -> super::vals::SendCnfChen13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SendCnfChen13::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 13"]
    #[inline(always)]
    pub fn set_chen13(&mut self, val: super::vals::SendCnfChen13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable broadcasting on IPC channel 14"]
    #[inline(always)]
    pub const fn chen14(&self) -> super::vals::SendCnfChen14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::SendCnfChen14::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 14"]
    #[inline(always)]
    pub fn set_chen14(&mut self, val: super::vals::SendCnfChen14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable broadcasting on IPC channel 15"]
    #[inline(always)]
    pub const fn chen15(&self) -> super::vals::SendCnfChen15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SendCnfChen15::from_bits(val as u8)
    }
    #[doc = "Enable broadcasting on IPC channel 15"]
    #[inline(always)]
    pub fn set_chen15(&mut self, val: super::vals::SendCnfChen15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for SendCnf {
    #[inline(always)]
    fn default() -> SendCnf {
        SendCnf(0)
    }
}
#[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeSend(pub u32);
impl SubscribeSend {
    #[doc = "DPPI channel that task SEND\\[n\\] will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task SEND\\[n\\] will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeSendEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeSendEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeSendEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeSend {
    #[inline(always)]
    fn default() -> SubscribeSend {
        SubscribeSend(0)
    }
}
