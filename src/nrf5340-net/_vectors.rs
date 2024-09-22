extern "C" {
    fn CLOCK_POWER();
    fn RADIO();
    fn RNG();
    fn GPIOTE();
    fn WDT();
    fn TIMER0();
    fn ECB();
    fn AAR_CCM();
    fn TEMP();
    fn RTC0();
    fn IPC();
    fn SERIAL0();
    fn EGU0();
    fn RTC1();
    fn TIMER1();
    fn TIMER2();
    fn SWI0();
    fn SWI1();
    fn SWI2();
    fn SWI3();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 30] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: CLOCK_POWER,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RADIO },
    Vector { _handler: RNG },
    Vector { _handler: GPIOTE },
    Vector { _handler: WDT },
    Vector { _handler: TIMER0 },
    Vector { _handler: ECB },
    Vector { _handler: AAR_CCM },
    Vector { _reserved: 0 },
    Vector { _handler: TEMP },
    Vector { _handler: RTC0 },
    Vector { _handler: IPC },
    Vector { _handler: SERIAL0 },
    Vector { _handler: EGU0 },
    Vector { _reserved: 0 },
    Vector { _handler: RTC1 },
    Vector { _reserved: 0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: SWI0 },
    Vector { _handler: SWI1 },
    Vector { _handler: SWI2 },
    Vector { _handler: SWI3 },
];
