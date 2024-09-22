extern "C" {
    fn FPU();
    fn CACHE();
    fn SPU();
    fn CLOCK_POWER();
    fn SERIAL0();
    fn SERIAL1();
    fn SPIM4();
    fn SERIAL2();
    fn SERIAL3();
    fn GPIOTE0();
    fn SAADC();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn RTC0();
    fn RTC1();
    fn WDT0();
    fn WDT1();
    fn COMP_LPCOMP();
    fn EGU0();
    fn EGU1();
    fn EGU2();
    fn EGU3();
    fn EGU4();
    fn EGU5();
    fn PWM0();
    fn PWM1();
    fn PWM2();
    fn PWM3();
    fn PDM0();
    fn I2S0();
    fn IPC();
    fn QSPI();
    fn NFCT();
    fn GPIOTE1();
    fn QDEC0();
    fn QDEC1();
    fn USBD();
    fn USBREGULATOR();
    fn KMU();
    fn CRYPTOCELL();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 69] = [
    Vector { _handler: FPU },
    Vector { _handler: CACHE },
    Vector { _reserved: 0 },
    Vector { _handler: SPU },
    Vector { _reserved: 0 },
    Vector {
        _handler: CLOCK_POWER,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SERIAL0 },
    Vector { _handler: SERIAL1 },
    Vector { _handler: SPIM4 },
    Vector { _handler: SERIAL2 },
    Vector { _handler: SERIAL3 },
    Vector { _handler: GPIOTE0 },
    Vector { _handler: SAADC },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RTC0 },
    Vector { _handler: RTC1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: WDT0 },
    Vector { _handler: WDT1 },
    Vector {
        _handler: COMP_LPCOMP,
    },
    Vector { _handler: EGU0 },
    Vector { _handler: EGU1 },
    Vector { _handler: EGU2 },
    Vector { _handler: EGU3 },
    Vector { _handler: EGU4 },
    Vector { _handler: EGU5 },
    Vector { _handler: PWM0 },
    Vector { _handler: PWM1 },
    Vector { _handler: PWM2 },
    Vector { _handler: PWM3 },
    Vector { _reserved: 0 },
    Vector { _handler: PDM0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2S0 },
    Vector { _reserved: 0 },
    Vector { _handler: IPC },
    Vector { _handler: QSPI },
    Vector { _reserved: 0 },
    Vector { _handler: NFCT },
    Vector { _reserved: 0 },
    Vector { _handler: GPIOTE1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: QDEC0 },
    Vector { _handler: QDEC1 },
    Vector { _reserved: 0 },
    Vector { _handler: USBD },
    Vector {
        _handler: USBREGULATOR,
    },
    Vector { _reserved: 0 },
    Vector { _handler: KMU },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: CRYPTOCELL,
    },
];
