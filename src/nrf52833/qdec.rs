#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel {
    ptr: *mut u8,
}
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Pin select for LED signal"]
    #[inline(always)]
    pub const fn led(self) -> crate::common::Reg<regs::Led, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Pin select for A signal"]
    #[inline(always)]
    pub const fn a(self) -> crate::common::Reg<regs::A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Pin select for B signal"]
    #[inline(always)]
    pub const fn b(self) -> crate::common::Reg<regs::B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Quadrature Decoder"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdec {
    ptr: *mut u8,
}
unsafe impl Send for Qdec {}
unsafe impl Sync for Qdec {}
impl Qdec {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Task starting the quadrature decoder"]
    #[inline(always)]
    pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Task stopping the quadrature decoder"]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Read and clear ACC and ACCDBL"]
    #[inline(always)]
    pub const fn tasks_readclracc(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Read and clear ACC"]
    #[inline(always)]
    pub const fn tasks_rdclracc(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Read and clear ACCDBL"]
    #[inline(always)]
    pub const fn tasks_rdclrdbl(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Event being generated for every new sample value written to the SAMPLE register"]
    #[inline(always)]
    pub const fn events_samplerdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Non-null report ready"]
    #[inline(always)]
    pub const fn events_reportrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "ACC or ACCDBL register overflow"]
    #[inline(always)]
    pub const fn events_accof(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Double displacement(s) detected"]
    #[inline(always)]
    pub const fn events_dblrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "QDEC has been stopped"]
    #[inline(always)]
    pub const fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "Enable the quadrature decoder"]
    #[inline(always)]
    pub const fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "LED output pin polarity"]
    #[inline(always)]
    pub const fn ledpol(self) -> crate::common::Reg<regs::Ledpol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Sample period"]
    #[inline(always)]
    pub const fn sampleper(self) -> crate::common::Reg<regs::Sampleper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Motion sample value"]
    #[inline(always)]
    pub const fn sample(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
    #[inline(always)]
    pub const fn reportper(self) -> crate::common::Reg<regs::Reportper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Register accumulating the valid transitions"]
    #[inline(always)]
    pub const fn acc(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
    #[inline(always)]
    pub const fn accread(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn psel(self) -> Psel {
        unsafe { Psel::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "Enable input debounce filters"]
    #[inline(always)]
    pub const fn dbfen(self) -> crate::common::Reg<regs::Dbfen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
    }
    #[doc = "Time period the LED is switched ON prior to sampling"]
    #[inline(always)]
    pub const fn ledpre(self) -> crate::common::Reg<regs::Ledpre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "Register accumulating the number of detected double transitions"]
    #[inline(always)]
    pub const fn accdbl(self) -> crate::common::Reg<regs::Accdbl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
    }
    #[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
    #[inline(always)]
    pub const fn accdblread(self) -> crate::common::Reg<regs::Accdblread, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
    }
}
pub mod regs;
pub mod vals;
