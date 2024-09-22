#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for event REGION0WA"]
    #[inline(always)]
    pub const fn region0wa(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event REGION0WA"]
    #[inline(always)]
    pub fn set_region0wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for event REGION0RA"]
    #[inline(always)]
    pub const fn region0ra(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event REGION0RA"]
    #[inline(always)]
    pub fn set_region0ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for event REGION1WA"]
    #[inline(always)]
    pub const fn region1wa(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event REGION1WA"]
    #[inline(always)]
    pub fn set_region1wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to disable interrupt for event REGION1RA"]
    #[inline(always)]
    pub const fn region1ra(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event REGION1RA"]
    #[inline(always)]
    pub fn set_region1ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to disable interrupt for event REGION2WA"]
    #[inline(always)]
    pub const fn region2wa(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event REGION2WA"]
    #[inline(always)]
    pub fn set_region2wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to disable interrupt for event REGION2RA"]
    #[inline(always)]
    pub const fn region2ra(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event REGION2RA"]
    #[inline(always)]
    pub fn set_region2ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to disable interrupt for event REGION3WA"]
    #[inline(always)]
    pub const fn region3wa(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event REGION3WA"]
    #[inline(always)]
    pub fn set_region3wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to disable interrupt for event REGION3RA"]
    #[inline(always)]
    pub const fn region3ra(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event REGION3RA"]
    #[inline(always)]
    pub fn set_region3ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to disable interrupt for event PREGION0WA"]
    #[inline(always)]
    pub const fn pregion0wa(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event PREGION0WA"]
    #[inline(always)]
    pub fn set_pregion0wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Write '1' to disable interrupt for event PREGION0RA"]
    #[inline(always)]
    pub const fn pregion0ra(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event PREGION0RA"]
    #[inline(always)]
    pub fn set_pregion0ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Write '1' to disable interrupt for event PREGION1WA"]
    #[inline(always)]
    pub const fn pregion1wa(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event PREGION1WA"]
    #[inline(always)]
    pub fn set_pregion1wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Write '1' to disable interrupt for event PREGION1RA"]
    #[inline(always)]
    pub const fn pregion1ra(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for event PREGION1RA"]
    #[inline(always)]
    pub fn set_pregion1ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmien(pub u32);
impl Nmien {
    #[doc = "Write '1' to enable interrupt for event REGION0WA"]
    #[inline(always)]
    pub const fn region0wa(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event REGION0WA"]
    #[inline(always)]
    pub fn set_region0wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to enable interrupt for event REGION0RA"]
    #[inline(always)]
    pub const fn region0ra(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event REGION0RA"]
    #[inline(always)]
    pub fn set_region0ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable interrupt for event REGION1WA"]
    #[inline(always)]
    pub const fn region1wa(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event REGION1WA"]
    #[inline(always)]
    pub fn set_region1wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to enable interrupt for event REGION1RA"]
    #[inline(always)]
    pub const fn region1ra(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event REGION1RA"]
    #[inline(always)]
    pub fn set_region1ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to enable interrupt for event REGION2WA"]
    #[inline(always)]
    pub const fn region2wa(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event REGION2WA"]
    #[inline(always)]
    pub fn set_region2wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to enable interrupt for event REGION2RA"]
    #[inline(always)]
    pub const fn region2ra(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event REGION2RA"]
    #[inline(always)]
    pub fn set_region2ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to enable interrupt for event REGION3WA"]
    #[inline(always)]
    pub const fn region3wa(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event REGION3WA"]
    #[inline(always)]
    pub fn set_region3wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to enable interrupt for event REGION3RA"]
    #[inline(always)]
    pub const fn region3ra(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event REGION3RA"]
    #[inline(always)]
    pub fn set_region3ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to enable interrupt for event PREGION0WA"]
    #[inline(always)]
    pub const fn pregion0wa(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event PREGION0WA"]
    #[inline(always)]
    pub fn set_pregion0wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Write '1' to enable interrupt for event PREGION0RA"]
    #[inline(always)]
    pub const fn pregion0ra(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event PREGION0RA"]
    #[inline(always)]
    pub fn set_pregion0ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Write '1' to enable interrupt for event PREGION1WA"]
    #[inline(always)]
    pub const fn pregion1wa(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event PREGION1WA"]
    #[inline(always)]
    pub fn set_pregion1wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Write '1' to enable interrupt for event PREGION1RA"]
    #[inline(always)]
    pub const fn pregion1ra(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event PREGION1RA"]
    #[inline(always)]
    pub fn set_pregion1ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Nmien {
    #[inline(always)]
    fn default() -> Nmien {
        Nmien(0)
    }
}
#[doc = "Disable regions watch"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Regionen(pub u32);
impl Regionen {
    #[doc = "Disable write access watch in region\\[0\\]"]
    #[inline(always)]
    pub const fn rgn_wa(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Disable write access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn set_rgn_wa(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Disable read access watch in region\\[0\\]"]
    #[inline(always)]
    pub const fn rgn_ra(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 1usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Disable read access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn set_rgn_ra(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 1usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Disable write access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub const fn prgn_wa(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 24usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Disable write access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn set_prgn_wa(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 24usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Disable read access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub const fn prgn_ra(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 25usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Disable read access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn set_prgn_ra(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 25usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Regionen {
    #[inline(always)]
    fn default() -> Regionen {
        Regionen(0)
    }
}
#[doc = "Description cluster: Subregions of region n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Subs(pub u32);
impl Subs {
    #[doc = "Include or exclude subregion 0 in region"]
    #[inline(always)]
    pub const fn sr(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Include or exclude subregion 0 in region"]
    #[inline(always)]
    pub fn set_sr(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Subs {
    #[inline(always)]
    fn default() -> Subs {
        Subs(0)
    }
}
#[doc = "Description cluster: Source of event/interrupt in region n, read access detected while corresponding subregion was enabled for watching"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Substatra(pub u32);
impl Substatra {
    #[doc = "Subregion 0 in region n (write '1' to clear)"]
    #[inline(always)]
    pub const fn sr(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Subregion 0 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn set_sr(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Substatra {
    #[inline(always)]
    fn default() -> Substatra {
        Substatra(0)
    }
}
#[doc = "Description cluster: Source of event/interrupt in region n, write access detected while corresponding subregion was enabled for watching"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Substatwa(pub u32);
impl Substatwa {
    #[doc = "Subregion 0 in region n (write '1' to clear)"]
    #[inline(always)]
    pub const fn sr(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Subregion 0 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn set_sr(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Substatwa {
    #[inline(always)]
    fn default() -> Substatwa {
        Substatwa(0)
    }
}
