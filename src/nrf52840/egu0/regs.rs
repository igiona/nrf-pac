#[doc = "Enable or disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[0\\]"]
    #[inline(always)]
    pub const fn triggered0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[0\\]"]
    #[inline(always)]
    pub fn set_triggered0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[1\\]"]
    #[inline(always)]
    pub const fn triggered1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[1\\]"]
    #[inline(always)]
    pub fn set_triggered1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[2\\]"]
    #[inline(always)]
    pub const fn triggered2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[2\\]"]
    #[inline(always)]
    pub fn set_triggered2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[3\\]"]
    #[inline(always)]
    pub const fn triggered3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[3\\]"]
    #[inline(always)]
    pub fn set_triggered3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[4\\]"]
    #[inline(always)]
    pub const fn triggered4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[4\\]"]
    #[inline(always)]
    pub fn set_triggered4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[5\\]"]
    #[inline(always)]
    pub const fn triggered5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[5\\]"]
    #[inline(always)]
    pub fn set_triggered5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[6\\]"]
    #[inline(always)]
    pub const fn triggered6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[6\\]"]
    #[inline(always)]
    pub fn set_triggered6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[7\\]"]
    #[inline(always)]
    pub const fn triggered7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[7\\]"]
    #[inline(always)]
    pub fn set_triggered7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[8\\]"]
    #[inline(always)]
    pub const fn triggered8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[8\\]"]
    #[inline(always)]
    pub fn set_triggered8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[9\\]"]
    #[inline(always)]
    pub const fn triggered9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[9\\]"]
    #[inline(always)]
    pub fn set_triggered9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[10\\]"]
    #[inline(always)]
    pub const fn triggered10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[10\\]"]
    #[inline(always)]
    pub fn set_triggered10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[11\\]"]
    #[inline(always)]
    pub const fn triggered11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[11\\]"]
    #[inline(always)]
    pub fn set_triggered11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[12\\]"]
    #[inline(always)]
    pub const fn triggered12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[12\\]"]
    #[inline(always)]
    pub fn set_triggered12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[13\\]"]
    #[inline(always)]
    pub const fn triggered13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[13\\]"]
    #[inline(always)]
    pub fn set_triggered13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[14\\]"]
    #[inline(always)]
    pub const fn triggered14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[14\\]"]
    #[inline(always)]
    pub fn set_triggered14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[15\\]"]
    #[inline(always)]
    pub const fn triggered15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for event TRIGGERED\\[15\\]"]
    #[inline(always)]
    pub fn set_triggered15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
