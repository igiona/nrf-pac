#[doc = "Pin select for A signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A(pub u32);
impl A {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub fn set_port(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::Aconnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Aconnect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Aconnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for A {
    #[inline(always)]
    fn default() -> A {
        A(0)
    }
}
#[doc = "Register accumulating the number of detected double transitions"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accdbl(pub u32);
impl Accdbl {
    #[doc = "Register accumulating the number of detected double or illegal transitions. ( SAMPLE = 2 )."]
    #[inline(always)]
    pub const fn accdbl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Register accumulating the number of detected double or illegal transitions. ( SAMPLE = 2 )."]
    #[inline(always)]
    pub fn set_accdbl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Accdbl {
    #[inline(always)]
    fn default() -> Accdbl {
        Accdbl(0)
    }
}
#[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accdblread(pub u32);
impl Accdblread {
    #[doc = "Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
    #[inline(always)]
    pub const fn accdblread(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
    #[inline(always)]
    pub fn set_accdblread(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Accdblread {
    #[inline(always)]
    fn default() -> Accdblread {
        Accdblread(0)
    }
}
#[doc = "Pin select for B signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B(pub u32);
impl B {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub fn set_port(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::Bconnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Bconnect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Bconnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for B {
    #[inline(always)]
    fn default() -> B {
        B(0)
    }
}
#[doc = "Enable input debounce filters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbfen(pub u32);
impl Dbfen {
    #[doc = "Enable input debounce filters"]
    #[inline(always)]
    pub const fn dbfen(&self) -> super::vals::Dbfen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dbfen::from_bits(val as u8)
    }
    #[doc = "Enable input debounce filters"]
    #[inline(always)]
    pub fn set_dbfen(&mut self, val: super::vals::Dbfen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Dbfen {
    #[inline(always)]
    fn default() -> Dbfen {
        Dbfen(0)
    }
}
#[doc = "Enable the quadrature decoder"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable the quadrature decoder"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enable or disable the quadrature decoder"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Enable {
    #[inline(always)]
    fn default() -> Enable {
        Enable(0)
    }
}
#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to enable interrupt for event SAMPLERDY"]
    #[inline(always)]
    pub const fn samplerdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event SAMPLERDY"]
    #[inline(always)]
    pub fn set_samplerdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to enable interrupt for event REPORTRDY"]
    #[inline(always)]
    pub const fn reportrdy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event REPORTRDY"]
    #[inline(always)]
    pub fn set_reportrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable interrupt for event ACCOF"]
    #[inline(always)]
    pub const fn accof(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event ACCOF"]
    #[inline(always)]
    pub fn set_accof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to enable interrupt for event DBLRDY"]
    #[inline(always)]
    pub const fn dblrdy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event DBLRDY"]
    #[inline(always)]
    pub fn set_dblrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub const fn stopped(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn set_stopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Pin select for LED signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Led(pub u32);
impl Led {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub fn set_port(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::LedConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::LedConnect::from_bits(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::LedConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Led {
    #[inline(always)]
    fn default() -> Led {
        Led(0)
    }
}
#[doc = "LED output pin polarity"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ledpol(pub u32);
impl Ledpol {
    #[doc = "LED output pin polarity"]
    #[inline(always)]
    pub const fn ledpol(&self) -> super::vals::Ledpol {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ledpol::from_bits(val as u8)
    }
    #[doc = "LED output pin polarity"]
    #[inline(always)]
    pub fn set_ledpol(&mut self, val: super::vals::Ledpol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ledpol {
    #[inline(always)]
    fn default() -> Ledpol {
        Ledpol(0)
    }
}
#[doc = "Time period the LED is switched ON prior to sampling"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ledpre(pub u32);
impl Ledpre {
    #[doc = "Period in us the LED is switched on prior to sampling"]
    #[inline(always)]
    pub const fn ledpre(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Period in us the LED is switched on prior to sampling"]
    #[inline(always)]
    pub fn set_ledpre(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Ledpre {
    #[inline(always)]
    fn default() -> Ledpre {
        Ledpre(0)
    }
}
#[doc = "Publish configuration for event ACCOF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishAccof(pub u32);
impl PublishAccof {
    #[doc = "DPPI channel that event ACCOF will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event ACCOF will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishAccofEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishAccofEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishAccofEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishAccof {
    #[inline(always)]
    fn default() -> PublishAccof {
        PublishAccof(0)
    }
}
#[doc = "Publish configuration for event DBLRDY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishDblrdy(pub u32);
impl PublishDblrdy {
    #[doc = "DPPI channel that event DBLRDY will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event DBLRDY will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishDblrdyEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishDblrdyEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishDblrdyEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishDblrdy {
    #[inline(always)]
    fn default() -> PublishDblrdy {
        PublishDblrdy(0)
    }
}
#[doc = "Publish configuration for event REPORTRDY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishReportrdy(pub u32);
impl PublishReportrdy {
    #[doc = "DPPI channel that event REPORTRDY will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event REPORTRDY will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishReportrdyEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishReportrdyEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishReportrdyEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishReportrdy {
    #[inline(always)]
    fn default() -> PublishReportrdy {
        PublishReportrdy(0)
    }
}
#[doc = "Publish configuration for event SAMPLERDY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishSamplerdy(pub u32);
impl PublishSamplerdy {
    #[doc = "DPPI channel that event SAMPLERDY will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event SAMPLERDY will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishSamplerdyEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishSamplerdyEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishSamplerdyEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishSamplerdy {
    #[inline(always)]
    fn default() -> PublishSamplerdy {
        PublishSamplerdy(0)
    }
}
#[doc = "Publish configuration for event STOPPED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PublishStopped(pub u32);
impl PublishStopped {
    #[doc = "DPPI channel that event STOPPED will publish to."]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that event STOPPED will publish to."]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::PublishStoppedEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PublishStoppedEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::PublishStoppedEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PublishStopped {
    #[inline(always)]
    fn default() -> PublishStopped {
        PublishStopped(0)
    }
}
#[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reportper(pub u32);
impl Reportper {
    #[doc = "Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated."]
    #[inline(always)]
    pub const fn reportper(&self) -> super::vals::Reportper {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Reportper::from_bits(val as u8)
    }
    #[doc = "Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated."]
    #[inline(always)]
    pub fn set_reportper(&mut self, val: super::vals::Reportper) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Reportper {
    #[inline(always)]
    fn default() -> Reportper {
        Reportper(0)
    }
}
#[doc = "Sample period"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sampleper(pub u32);
impl Sampleper {
    #[doc = "Sample period. The SAMPLE register will be updated for every new sample"]
    #[inline(always)]
    pub const fn sampleper(&self) -> super::vals::Sampleper {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Sampleper::from_bits(val as u8)
    }
    #[doc = "Sample period. The SAMPLE register will be updated for every new sample"]
    #[inline(always)]
    pub fn set_sampleper(&mut self, val: super::vals::Sampleper) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Sampleper {
    #[inline(always)]
    fn default() -> Sampleper {
        Sampleper(0)
    }
}
#[doc = "Shortcuts between local events and tasks"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between event REPORTRDY and task READCLRACC"]
    #[inline(always)]
    pub const fn reportrdy_readclracc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event REPORTRDY and task READCLRACC"]
    #[inline(always)]
    pub fn set_reportrdy_readclracc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shortcut between event SAMPLERDY and task STOP"]
    #[inline(always)]
    pub const fn samplerdy_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event SAMPLERDY and task STOP"]
    #[inline(always)]
    pub fn set_samplerdy_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Shortcut between event REPORTRDY and task RDCLRACC"]
    #[inline(always)]
    pub const fn reportrdy_rdclracc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event REPORTRDY and task RDCLRACC"]
    #[inline(always)]
    pub fn set_reportrdy_rdclracc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Shortcut between event REPORTRDY and task STOP"]
    #[inline(always)]
    pub const fn reportrdy_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event REPORTRDY and task STOP"]
    #[inline(always)]
    pub fn set_reportrdy_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Shortcut between event DBLRDY and task RDCLRDBL"]
    #[inline(always)]
    pub const fn dblrdy_rdclrdbl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event DBLRDY and task RDCLRDBL"]
    #[inline(always)]
    pub fn set_dblrdy_rdclrdbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Shortcut between event DBLRDY and task STOP"]
    #[inline(always)]
    pub const fn dblrdy_stop(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event DBLRDY and task STOP"]
    #[inline(always)]
    pub fn set_dblrdy_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Shortcut between event SAMPLERDY and task READCLRACC"]
    #[inline(always)]
    pub const fn samplerdy_readclracc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between event SAMPLERDY and task READCLRACC"]
    #[inline(always)]
    pub fn set_samplerdy_readclracc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Subscribe configuration for task RDCLRACC"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeRdclracc(pub u32);
impl SubscribeRdclracc {
    #[doc = "DPPI channel that task RDCLRACC will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task RDCLRACC will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeRdclraccEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeRdclraccEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeRdclraccEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeRdclracc {
    #[inline(always)]
    fn default() -> SubscribeRdclracc {
        SubscribeRdclracc(0)
    }
}
#[doc = "Subscribe configuration for task RDCLRDBL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeRdclrdbl(pub u32);
impl SubscribeRdclrdbl {
    #[doc = "DPPI channel that task RDCLRDBL will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task RDCLRDBL will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeRdclrdblEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeRdclrdblEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeRdclrdblEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeRdclrdbl {
    #[inline(always)]
    fn default() -> SubscribeRdclrdbl {
        SubscribeRdclrdbl(0)
    }
}
#[doc = "Subscribe configuration for task READCLRACC"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeReadclracc(pub u32);
impl SubscribeReadclracc {
    #[doc = "DPPI channel that task READCLRACC will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task READCLRACC will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeReadclraccEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeReadclraccEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeReadclraccEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeReadclracc {
    #[inline(always)]
    fn default() -> SubscribeReadclracc {
        SubscribeReadclracc(0)
    }
}
#[doc = "Subscribe configuration for task START"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeStart(pub u32);
impl SubscribeStart {
    #[doc = "DPPI channel that task START will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task START will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeStartEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeStartEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeStartEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeStart {
    #[inline(always)]
    fn default() -> SubscribeStart {
        SubscribeStart(0)
    }
}
#[doc = "Subscribe configuration for task STOP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SubscribeStop(pub u32);
impl SubscribeStop {
    #[doc = "DPPI channel that task STOP will subscribe to"]
    #[inline(always)]
    pub const fn chidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DPPI channel that task STOP will subscribe to"]
    #[inline(always)]
    pub fn set_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn en(&self) -> super::vals::SubscribeStopEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SubscribeStopEn::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::SubscribeStopEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SubscribeStop {
    #[inline(always)]
    fn default() -> SubscribeStop {
        SubscribeStop(0)
    }
}