#![doc = "Peripheral access API (generated using chiptool v0.1.0 (a349968 2024-08-11))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - FPU"]
    FPU = 0,
    #[doc = "1 - CACHE"]
    CACHE = 1,
    #[doc = "3 - SPU"]
    SPU = 3,
    #[doc = "5 - CLOCK_POWER"]
    CLOCK_POWER = 5,
    #[doc = "8 - SERIAL0"]
    SERIAL0 = 8,
    #[doc = "9 - SERIAL1"]
    SERIAL1 = 9,
    #[doc = "10 - SPIM4"]
    SPIM4 = 10,
    #[doc = "11 - SERIAL2"]
    SERIAL2 = 11,
    #[doc = "12 - SERIAL3"]
    SERIAL3 = 12,
    #[doc = "13 - GPIOTE0"]
    GPIOTE0 = 13,
    #[doc = "14 - SAADC"]
    SAADC = 14,
    #[doc = "15 - TIMER0"]
    TIMER0 = 15,
    #[doc = "16 - TIMER1"]
    TIMER1 = 16,
    #[doc = "17 - TIMER2"]
    TIMER2 = 17,
    #[doc = "20 - RTC0"]
    RTC0 = 20,
    #[doc = "21 - RTC1"]
    RTC1 = 21,
    #[doc = "24 - WDT0"]
    WDT0 = 24,
    #[doc = "25 - WDT1"]
    WDT1 = 25,
    #[doc = "26 - COMP_LPCOMP"]
    COMP_LPCOMP = 26,
    #[doc = "27 - EGU0"]
    EGU0 = 27,
    #[doc = "28 - EGU1"]
    EGU1 = 28,
    #[doc = "29 - EGU2"]
    EGU2 = 29,
    #[doc = "30 - EGU3"]
    EGU3 = 30,
    #[doc = "31 - EGU4"]
    EGU4 = 31,
    #[doc = "32 - EGU5"]
    EGU5 = 32,
    #[doc = "33 - PWM0"]
    PWM0 = 33,
    #[doc = "34 - PWM1"]
    PWM1 = 34,
    #[doc = "35 - PWM2"]
    PWM2 = 35,
    #[doc = "36 - PWM3"]
    PWM3 = 36,
    #[doc = "38 - PDM0"]
    PDM0 = 38,
    #[doc = "40 - I2S0"]
    I2S0 = 40,
    #[doc = "42 - IPC"]
    IPC = 42,
    #[doc = "43 - QSPI"]
    QSPI = 43,
    #[doc = "45 - NFCT"]
    NFCT = 45,
    #[doc = "47 - GPIOTE1"]
    GPIOTE1 = 47,
    #[doc = "51 - QDEC0"]
    QDEC0 = 51,
    #[doc = "52 - QDEC1"]
    QDEC1 = 52,
    #[doc = "54 - USBD"]
    USBD = 54,
    #[doc = "55 - USBREGULATOR"]
    USBREGULATOR = 55,
    #[doc = "57 - KMU"]
    KMU = 57,
    #[doc = "68 - CRYPTOCELL"]
    CRYPTOCELL = 68,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "CACHEDATA"]
pub const CACHEDATA_S: cachedata_s::CachedataS =
    unsafe { cachedata_s::CachedataS::from_ptr(0x00f0_0000usize as _) };
#[doc = "CACHEINFO"]
pub const CACHEINFO_S: cacheinfo_s::CacheinfoS =
    unsafe { cacheinfo_s::CacheinfoS::from_ptr(0x00f0_8000usize as _) };
#[doc = "Factory Information Configuration Registers"]
pub const FICR_S: ficr_s::FicrS = unsafe { ficr_s::FicrS::from_ptr(0x00ff_0000usize as _) };
#[doc = "User Information Configuration Registers User information configuration registers"]
pub const UICR_S: uicr_s::UicrS = unsafe { uicr_s::UicrS::from_ptr(0x00ff_8000usize as _) };
#[doc = "Domain configuration management 0"]
pub const DCNF_NS: dcnf_ns::DcnfNs = unsafe { dcnf_ns::DcnfNs::from_ptr(0x4000_0000usize as _) };
#[doc = "FPU control peripheral 0"]
pub const FPU_NS: fpu_ns::FpuNs = unsafe { fpu_ns::FpuNs::from_ptr(0x4000_0000usize as _) };
#[doc = "Oscillator control 0"]
pub const OSCILLATORS_NS: oscillators_ns::OscillatorsNs =
    unsafe { oscillators_ns::OscillatorsNs::from_ptr(0x4000_4000usize as _) };
#[doc = "Voltage regulators 0"]
pub const REGULATORS_NS: regulators_ns::RegulatorsNs =
    unsafe { regulators_ns::RegulatorsNs::from_ptr(0x4000_4000usize as _) };
#[doc = "Clock management 0"]
pub const CLOCK_NS: clock_ns::ClockNs =
    unsafe { clock_ns::ClockNs::from_ptr(0x4000_5000usize as _) };
#[doc = "Power control 0"]
pub const POWER_NS: power_ns::PowerNs =
    unsafe { power_ns::PowerNs::from_ptr(0x4000_5000usize as _) };
#[doc = "Reset control 0"]
pub const RESET_NS: reset_ns::ResetNs =
    unsafe { reset_ns::ResetNs::from_ptr(0x4000_5000usize as _) };
#[doc = "Control access port 0"]
pub const CTRLAP_NS: ctrlap_ns::CtrlapNs =
    unsafe { ctrlap_ns::CtrlapNs::from_ptr(0x4000_6000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub const SPIM0_NS: spim0_ns::Spim0ns =
    unsafe { spim0_ns::Spim0ns::from_ptr(0x4000_8000usize as _) };
#[doc = "SPI Slave 0"]
pub const SPIS0_NS: spis0_ns::Spis0ns =
    unsafe { spis0_ns::Spis0ns::from_ptr(0x4000_8000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub const TWIM0_NS: twim0_ns::Twim0ns =
    unsafe { twim0_ns::Twim0ns::from_ptr(0x4000_8000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub const TWIS0_NS: twis0_ns::Twis0ns =
    unsafe { twis0_ns::Twis0ns::from_ptr(0x4000_8000usize as _) };
#[doc = "UART with EasyDMA 0"]
pub const UARTE0_NS: uarte0_ns::Uarte0ns =
    unsafe { uarte0_ns::Uarte0ns::from_ptr(0x4000_8000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 2"]
pub const SPIM1_NS: spim0_ns::Spim0ns =
    unsafe { spim0_ns::Spim0ns::from_ptr(0x4000_9000usize as _) };
#[doc = "SPI Slave 2"]
pub const SPIS1_NS: spis0_ns::Spis0ns =
    unsafe { spis0_ns::Spis0ns::from_ptr(0x4000_9000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 2"]
pub const TWIM1_NS: twim0_ns::Twim0ns =
    unsafe { twim0_ns::Twim0ns::from_ptr(0x4000_9000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 2"]
pub const TWIS1_NS: twis0_ns::Twis0ns =
    unsafe { twis0_ns::Twis0ns::from_ptr(0x4000_9000usize as _) };
#[doc = "UART with EasyDMA 2"]
pub const UARTE1_NS: uarte0_ns::Uarte0ns =
    unsafe { uarte0_ns::Uarte0ns::from_ptr(0x4000_9000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 4"]
pub const SPIM4_NS: spim0_ns::Spim0ns =
    unsafe { spim0_ns::Spim0ns::from_ptr(0x4000_a000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 6"]
pub const SPIM2_NS: spim0_ns::Spim0ns =
    unsafe { spim0_ns::Spim0ns::from_ptr(0x4000_b000usize as _) };
#[doc = "SPI Slave 4"]
pub const SPIS2_NS: spis0_ns::Spis0ns =
    unsafe { spis0_ns::Spis0ns::from_ptr(0x4000_b000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 4"]
pub const TWIM2_NS: twim0_ns::Twim0ns =
    unsafe { twim0_ns::Twim0ns::from_ptr(0x4000_b000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 4"]
pub const TWIS2_NS: twis0_ns::Twis0ns =
    unsafe { twis0_ns::Twis0ns::from_ptr(0x4000_b000usize as _) };
#[doc = "UART with EasyDMA 4"]
pub const UARTE2_NS: uarte0_ns::Uarte0ns =
    unsafe { uarte0_ns::Uarte0ns::from_ptr(0x4000_b000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 8"]
pub const SPIM3_NS: spim0_ns::Spim0ns =
    unsafe { spim0_ns::Spim0ns::from_ptr(0x4000_c000usize as _) };
#[doc = "SPI Slave 6"]
pub const SPIS3_NS: spis0_ns::Spis0ns =
    unsafe { spis0_ns::Spis0ns::from_ptr(0x4000_c000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 6"]
pub const TWIM3_NS: twim0_ns::Twim0ns =
    unsafe { twim0_ns::Twim0ns::from_ptr(0x4000_c000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 6"]
pub const TWIS3_NS: twis0_ns::Twis0ns =
    unsafe { twis0_ns::Twis0ns::from_ptr(0x4000_c000usize as _) };
#[doc = "UART with EasyDMA 6"]
pub const UARTE3_NS: uarte0_ns::Uarte0ns =
    unsafe { uarte0_ns::Uarte0ns::from_ptr(0x4000_c000usize as _) };
#[doc = "Analog to Digital Converter 0"]
pub const SAADC_NS: saadc_ns::SaadcNs =
    unsafe { saadc_ns::SaadcNs::from_ptr(0x4000_e000usize as _) };
#[doc = "Timer/Counter 0"]
pub const TIMER0_NS: timer0_ns::Timer0ns =
    unsafe { timer0_ns::Timer0ns::from_ptr(0x4000_f000usize as _) };
#[doc = "Timer/Counter 2"]
pub const TIMER1_NS: timer0_ns::Timer0ns =
    unsafe { timer0_ns::Timer0ns::from_ptr(0x4001_0000usize as _) };
#[doc = "Timer/Counter 4"]
pub const TIMER2_NS: timer0_ns::Timer0ns =
    unsafe { timer0_ns::Timer0ns::from_ptr(0x4001_1000usize as _) };
#[doc = "Real-time counter 0"]
pub const RTC0_NS: rtc0_ns::Rtc0ns = unsafe { rtc0_ns::Rtc0ns::from_ptr(0x4001_4000usize as _) };
#[doc = "Real-time counter 2"]
pub const RTC1_NS: rtc0_ns::Rtc0ns = unsafe { rtc0_ns::Rtc0ns::from_ptr(0x4001_5000usize as _) };
#[doc = "Distributed programmable peripheral interconnect controller 0"]
pub const DPPIC_NS: dppic_ns::DppicNs =
    unsafe { dppic_ns::DppicNs::from_ptr(0x4001_7000usize as _) };
#[doc = "Watchdog Timer 0"]
pub const WDT0_NS: wdt0_ns::Wdt0ns = unsafe { wdt0_ns::Wdt0ns::from_ptr(0x4001_8000usize as _) };
#[doc = "Watchdog Timer 2"]
pub const WDT1_NS: wdt0_ns::Wdt0ns = unsafe { wdt0_ns::Wdt0ns::from_ptr(0x4001_9000usize as _) };
#[doc = "Comparator 0"]
pub const COMP_NS: comp_ns::CompNs = unsafe { comp_ns::CompNs::from_ptr(0x4001_a000usize as _) };
#[doc = "Low-power comparator 0"]
pub const LPCOMP_NS: lpcomp_ns::LpcompNs =
    unsafe { lpcomp_ns::LpcompNs::from_ptr(0x4001_a000usize as _) };
#[doc = "Event generator unit 0"]
pub const EGU0_NS: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x4001_b000usize as _) };
#[doc = "Event generator unit 2"]
pub const EGU1_NS: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x4001_c000usize as _) };
#[doc = "Event generator unit 4"]
pub const EGU2_NS: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x4001_d000usize as _) };
#[doc = "Event generator unit 6"]
pub const EGU3_NS: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x4001_e000usize as _) };
#[doc = "Event generator unit 8"]
pub const EGU4_NS: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x4001_f000usize as _) };
#[doc = "Event generator unit 10"]
pub const EGU5_NS: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x4002_0000usize as _) };
#[doc = "Pulse width modulation unit 0"]
pub const PWM0_NS: pwm0_ns::Pwm0ns = unsafe { pwm0_ns::Pwm0ns::from_ptr(0x4002_1000usize as _) };
#[doc = "Pulse width modulation unit 2"]
pub const PWM1_NS: pwm0_ns::Pwm0ns = unsafe { pwm0_ns::Pwm0ns::from_ptr(0x4002_2000usize as _) };
#[doc = "Pulse width modulation unit 4"]
pub const PWM2_NS: pwm0_ns::Pwm0ns = unsafe { pwm0_ns::Pwm0ns::from_ptr(0x4002_3000usize as _) };
#[doc = "Pulse width modulation unit 6"]
pub const PWM3_NS: pwm0_ns::Pwm0ns = unsafe { pwm0_ns::Pwm0ns::from_ptr(0x4002_4000usize as _) };
#[doc = "Pulse Density Modulation (Digital Microphone) Interface 0"]
pub const PDM0_NS: pdm0_ns::Pdm0ns = unsafe { pdm0_ns::Pdm0ns::from_ptr(0x4002_6000usize as _) };
#[doc = "Inter-IC Sound 0"]
pub const I2S0_NS: i2s0_ns::I2s0ns = unsafe { i2s0_ns::I2s0ns::from_ptr(0x4002_8000usize as _) };
#[doc = "Interprocessor communication 0"]
pub const IPC_NS: ipc_ns::IpcNs = unsafe { ipc_ns::IpcNs::from_ptr(0x4002_a000usize as _) };
#[doc = "External flash interface 0"]
pub const QSPI_NS: qspi_ns::QspiNs = unsafe { qspi_ns::QspiNs::from_ptr(0x4002_b000usize as _) };
#[doc = "NFC-A compatible radio 0"]
pub const NFCT_NS: nfct_ns::NfctNs = unsafe { nfct_ns::NfctNs::from_ptr(0x4002_d000usize as _) };
#[doc = "GPIO Tasks and Events 1"]
pub const GPIOTE1_NS: gpiote0_s::Gpiote0s =
    unsafe { gpiote0_s::Gpiote0s::from_ptr(0x4002_f000usize as _) };
#[doc = "MUTEX 0"]
pub const MUTEX_NS: mutex_ns::MutexNs =
    unsafe { mutex_ns::MutexNs::from_ptr(0x4003_0000usize as _) };
#[doc = "Quadrature Decoder 0"]
pub const QDEC0_NS: qdec0_ns::Qdec0ns =
    unsafe { qdec0_ns::Qdec0ns::from_ptr(0x4003_3000usize as _) };
#[doc = "Quadrature Decoder 2"]
pub const QDEC1_NS: qdec0_ns::Qdec0ns =
    unsafe { qdec0_ns::Qdec0ns::from_ptr(0x4003_4000usize as _) };
#[doc = "Universal serial bus device 0"]
pub const USBD_NS: usbd_ns::UsbdNs = unsafe { usbd_ns::UsbdNs::from_ptr(0x4003_6000usize as _) };
#[doc = "USB Regulator 0"]
pub const USBREGULATOR_NS: usbregulator_ns::UsbregulatorNs =
    unsafe { usbregulator_ns::UsbregulatorNs::from_ptr(0x4003_7000usize as _) };
#[doc = "Key management unit 0"]
pub const KMU_NS: kmu_ns::KmuNs = unsafe { kmu_ns::KmuNs::from_ptr(0x4003_9000usize as _) };
#[doc = "Non-volatile memory controller 0"]
pub const NVMC_NS: nvmc_ns::NvmcNs = unsafe { nvmc_ns::NvmcNs::from_ptr(0x4003_9000usize as _) };
#[doc = "Volatile Memory controller 0"]
pub const VMC_NS: vmc_ns::VmcNs = unsafe { vmc_ns::VmcNs::from_ptr(0x4008_1000usize as _) };
#[doc = "GPIO Port 0"]
pub const P0_NS: p0_ns::P0ns = unsafe { p0_ns::P0ns::from_ptr(0x4084_2500usize as _) };
#[doc = "GPIO Port 1"]
pub const P1_NS: p0_ns::P0ns = unsafe { p0_ns::P0ns::from_ptr(0x4084_2800usize as _) };
#[doc = "Domain configuration management 1"]
pub const DCNF_S: dcnf_ns::DcnfNs = unsafe { dcnf_ns::DcnfNs::from_ptr(0x5000_0000usize as _) };
#[doc = "FPU control peripheral 1"]
pub const FPU_S: fpu_ns::FpuNs = unsafe { fpu_ns::FpuNs::from_ptr(0x5000_0000usize as _) };
#[doc = "Cache"]
pub const CACHE_S: cache_s::CacheS = unsafe { cache_s::CacheS::from_ptr(0x5000_1000usize as _) };
#[doc = "System protection unit"]
pub const SPU_S: spu_s::SpuS = unsafe { spu_s::SpuS::from_ptr(0x5000_3000usize as _) };
#[doc = "Oscillator control 1"]
pub const OSCILLATORS_S: oscillators_ns::OscillatorsNs =
    unsafe { oscillators_ns::OscillatorsNs::from_ptr(0x5000_4000usize as _) };
#[doc = "Voltage regulators 1"]
pub const REGULATORS_S: regulators_ns::RegulatorsNs =
    unsafe { regulators_ns::RegulatorsNs::from_ptr(0x5000_4000usize as _) };
#[doc = "Clock management 1"]
pub const CLOCK_S: clock_ns::ClockNs =
    unsafe { clock_ns::ClockNs::from_ptr(0x5000_5000usize as _) };
#[doc = "Power control 1"]
pub const POWER_S: power_ns::PowerNs =
    unsafe { power_ns::PowerNs::from_ptr(0x5000_5000usize as _) };
#[doc = "Reset control 1"]
pub const RESET_S: reset_ns::ResetNs =
    unsafe { reset_ns::ResetNs::from_ptr(0x5000_5000usize as _) };
#[doc = "Control access port 1"]
pub const CTRLAP_S: ctrlap_ns::CtrlapNs =
    unsafe { ctrlap_ns::CtrlapNs::from_ptr(0x5000_6000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 1"]
pub const SPIM0_S: spim0_ns::Spim0ns =
    unsafe { spim0_ns::Spim0ns::from_ptr(0x5000_8000usize as _) };
#[doc = "SPI Slave 1"]
pub const SPIS0_S: spis0_ns::Spis0ns =
    unsafe { spis0_ns::Spis0ns::from_ptr(0x5000_8000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 1"]
pub const TWIM0_S: twim0_ns::Twim0ns =
    unsafe { twim0_ns::Twim0ns::from_ptr(0x5000_8000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 1"]
pub const TWIS0_S: twis0_ns::Twis0ns =
    unsafe { twis0_ns::Twis0ns::from_ptr(0x5000_8000usize as _) };
#[doc = "UART with EasyDMA 1"]
pub const UARTE0_S: uarte0_ns::Uarte0ns =
    unsafe { uarte0_ns::Uarte0ns::from_ptr(0x5000_8000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 3"]
pub const SPIM1_S: spim0_ns::Spim0ns =
    unsafe { spim0_ns::Spim0ns::from_ptr(0x5000_9000usize as _) };
#[doc = "SPI Slave 3"]
pub const SPIS1_S: spis0_ns::Spis0ns =
    unsafe { spis0_ns::Spis0ns::from_ptr(0x5000_9000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 3"]
pub const TWIM1_S: twim0_ns::Twim0ns =
    unsafe { twim0_ns::Twim0ns::from_ptr(0x5000_9000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 3"]
pub const TWIS1_S: twis0_ns::Twis0ns =
    unsafe { twis0_ns::Twis0ns::from_ptr(0x5000_9000usize as _) };
#[doc = "UART with EasyDMA 3"]
pub const UARTE1_S: uarte0_ns::Uarte0ns =
    unsafe { uarte0_ns::Uarte0ns::from_ptr(0x5000_9000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 5"]
pub const SPIM4_S: spim0_ns::Spim0ns =
    unsafe { spim0_ns::Spim0ns::from_ptr(0x5000_a000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 7"]
pub const SPIM2_S: spim0_ns::Spim0ns =
    unsafe { spim0_ns::Spim0ns::from_ptr(0x5000_b000usize as _) };
#[doc = "SPI Slave 5"]
pub const SPIS2_S: spis0_ns::Spis0ns =
    unsafe { spis0_ns::Spis0ns::from_ptr(0x5000_b000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 5"]
pub const TWIM2_S: twim0_ns::Twim0ns =
    unsafe { twim0_ns::Twim0ns::from_ptr(0x5000_b000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 5"]
pub const TWIS2_S: twis0_ns::Twis0ns =
    unsafe { twis0_ns::Twis0ns::from_ptr(0x5000_b000usize as _) };
#[doc = "UART with EasyDMA 5"]
pub const UARTE2_S: uarte0_ns::Uarte0ns =
    unsafe { uarte0_ns::Uarte0ns::from_ptr(0x5000_b000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA 9"]
pub const SPIM3_S: spim0_ns::Spim0ns =
    unsafe { spim0_ns::Spim0ns::from_ptr(0x5000_c000usize as _) };
#[doc = "SPI Slave 7"]
pub const SPIS3_S: spis0_ns::Spis0ns =
    unsafe { spis0_ns::Spis0ns::from_ptr(0x5000_c000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 7"]
pub const TWIM3_S: twim0_ns::Twim0ns =
    unsafe { twim0_ns::Twim0ns::from_ptr(0x5000_c000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 7"]
pub const TWIS3_S: twis0_ns::Twis0ns =
    unsafe { twis0_ns::Twis0ns::from_ptr(0x5000_c000usize as _) };
#[doc = "UART with EasyDMA 7"]
pub const UARTE3_S: uarte0_ns::Uarte0ns =
    unsafe { uarte0_ns::Uarte0ns::from_ptr(0x5000_c000usize as _) };
#[doc = "GPIO Tasks and Events 0"]
pub const GPIOTE0_S: gpiote0_s::Gpiote0s =
    unsafe { gpiote0_s::Gpiote0s::from_ptr(0x5000_d000usize as _) };
#[doc = "Analog to Digital Converter 1"]
pub const SAADC_S: saadc_ns::SaadcNs =
    unsafe { saadc_ns::SaadcNs::from_ptr(0x5000_e000usize as _) };
#[doc = "Timer/Counter 1"]
pub const TIMER0_S: timer0_ns::Timer0ns =
    unsafe { timer0_ns::Timer0ns::from_ptr(0x5000_f000usize as _) };
#[doc = "Timer/Counter 3"]
pub const TIMER1_S: timer0_ns::Timer0ns =
    unsafe { timer0_ns::Timer0ns::from_ptr(0x5001_0000usize as _) };
#[doc = "Timer/Counter 5"]
pub const TIMER2_S: timer0_ns::Timer0ns =
    unsafe { timer0_ns::Timer0ns::from_ptr(0x5001_1000usize as _) };
#[doc = "Real-time counter 1"]
pub const RTC0_S: rtc0_ns::Rtc0ns = unsafe { rtc0_ns::Rtc0ns::from_ptr(0x5001_4000usize as _) };
#[doc = "Real-time counter 3"]
pub const RTC1_S: rtc0_ns::Rtc0ns = unsafe { rtc0_ns::Rtc0ns::from_ptr(0x5001_5000usize as _) };
#[doc = "Distributed programmable peripheral interconnect controller 1"]
pub const DPPIC_S: dppic_ns::DppicNs =
    unsafe { dppic_ns::DppicNs::from_ptr(0x5001_7000usize as _) };
#[doc = "Watchdog Timer 1"]
pub const WDT0_S: wdt0_ns::Wdt0ns = unsafe { wdt0_ns::Wdt0ns::from_ptr(0x5001_8000usize as _) };
#[doc = "Watchdog Timer 3"]
pub const WDT1_S: wdt0_ns::Wdt0ns = unsafe { wdt0_ns::Wdt0ns::from_ptr(0x5001_9000usize as _) };
#[doc = "Comparator 1"]
pub const COMP_S: comp_ns::CompNs = unsafe { comp_ns::CompNs::from_ptr(0x5001_a000usize as _) };
#[doc = "Low-power comparator 1"]
pub const LPCOMP_S: lpcomp_ns::LpcompNs =
    unsafe { lpcomp_ns::LpcompNs::from_ptr(0x5001_a000usize as _) };
#[doc = "Event generator unit 1"]
pub const EGU0_S: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x5001_b000usize as _) };
#[doc = "Event generator unit 3"]
pub const EGU1_S: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x5001_c000usize as _) };
#[doc = "Event generator unit 5"]
pub const EGU2_S: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x5001_d000usize as _) };
#[doc = "Event generator unit 7"]
pub const EGU3_S: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x5001_e000usize as _) };
#[doc = "Event generator unit 9"]
pub const EGU4_S: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x5001_f000usize as _) };
#[doc = "Event generator unit 11"]
pub const EGU5_S: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x5002_0000usize as _) };
#[doc = "Pulse width modulation unit 1"]
pub const PWM0_S: pwm0_ns::Pwm0ns = unsafe { pwm0_ns::Pwm0ns::from_ptr(0x5002_1000usize as _) };
#[doc = "Pulse width modulation unit 3"]
pub const PWM1_S: pwm0_ns::Pwm0ns = unsafe { pwm0_ns::Pwm0ns::from_ptr(0x5002_2000usize as _) };
#[doc = "Pulse width modulation unit 5"]
pub const PWM2_S: pwm0_ns::Pwm0ns = unsafe { pwm0_ns::Pwm0ns::from_ptr(0x5002_3000usize as _) };
#[doc = "Pulse width modulation unit 7"]
pub const PWM3_S: pwm0_ns::Pwm0ns = unsafe { pwm0_ns::Pwm0ns::from_ptr(0x5002_4000usize as _) };
#[doc = "Pulse Density Modulation (Digital Microphone) Interface 1"]
pub const PDM0_S: pdm0_ns::Pdm0ns = unsafe { pdm0_ns::Pdm0ns::from_ptr(0x5002_6000usize as _) };
#[doc = "Inter-IC Sound 1"]
pub const I2S0_S: i2s0_ns::I2s0ns = unsafe { i2s0_ns::I2s0ns::from_ptr(0x5002_8000usize as _) };
#[doc = "Interprocessor communication 1"]
pub const IPC_S: ipc_ns::IpcNs = unsafe { ipc_ns::IpcNs::from_ptr(0x5002_a000usize as _) };
#[doc = "External flash interface 1"]
pub const QSPI_S: qspi_ns::QspiNs = unsafe { qspi_ns::QspiNs::from_ptr(0x5002_b000usize as _) };
#[doc = "NFC-A compatible radio 1"]
pub const NFCT_S: nfct_ns::NfctNs = unsafe { nfct_ns::NfctNs::from_ptr(0x5002_d000usize as _) };
#[doc = "MUTEX 1"]
pub const MUTEX_S: mutex_ns::MutexNs =
    unsafe { mutex_ns::MutexNs::from_ptr(0x5003_0000usize as _) };
#[doc = "Quadrature Decoder 1"]
pub const QDEC0_S: qdec0_ns::Qdec0ns =
    unsafe { qdec0_ns::Qdec0ns::from_ptr(0x5003_3000usize as _) };
#[doc = "Quadrature Decoder 3"]
pub const QDEC1_S: qdec0_ns::Qdec0ns =
    unsafe { qdec0_ns::Qdec0ns::from_ptr(0x5003_4000usize as _) };
#[doc = "Universal serial bus device 1"]
pub const USBD_S: usbd_ns::UsbdNs = unsafe { usbd_ns::UsbdNs::from_ptr(0x5003_6000usize as _) };
#[doc = "USB Regulator 1"]
pub const USBREGULATOR_S: usbregulator_ns::UsbregulatorNs =
    unsafe { usbregulator_ns::UsbregulatorNs::from_ptr(0x5003_7000usize as _) };
#[doc = "Key management unit 1"]
pub const KMU_S: kmu_ns::KmuNs = unsafe { kmu_ns::KmuNs::from_ptr(0x5003_9000usize as _) };
#[doc = "Non-volatile memory controller 1"]
pub const NVMC_S: nvmc_ns::NvmcNs = unsafe { nvmc_ns::NvmcNs::from_ptr(0x5003_9000usize as _) };
#[doc = "Volatile Memory controller 1"]
pub const VMC_S: vmc_ns::VmcNs = unsafe { vmc_ns::VmcNs::from_ptr(0x5008_1000usize as _) };
#[doc = "GPIO Port 2"]
pub const P0_S: p0_ns::P0ns = unsafe { p0_ns::P0ns::from_ptr(0x5084_2500usize as _) };
#[doc = "GPIO Port 3"]
pub const P1_S: p0_ns::P0ns = unsafe { p0_ns::P0ns::from_ptr(0x5084_2800usize as _) };
#[doc = "ARM TrustZone CryptoCell register interface"]
pub const CRYPTOCELL_S: cryptocell_s::CryptocellS =
    unsafe { cryptocell_s::CryptocellS::from_ptr(0x5084_4000usize as _) };
#[doc = "Cross-Trigger Interface control. NOTE: this is not a separate peripheral, but describes CM33 functionality."]
pub const CTI_S: cti_s::CtiS = unsafe { cti_s::CtiS::from_ptr(0xe004_2000usize as _) };
#[doc = "Trace and debug control"]
pub const TAD_S: tad_s::TadS = unsafe { tad_s::TadS::from_ptr(0xe008_0000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod cache_s;
pub mod cachedata_s;
pub mod cacheinfo_s;
pub mod clock_ns;
pub mod common;
pub mod comp_ns;
pub mod cryptocell_s;
pub mod cti_s;
pub mod ctrlap_ns;
pub mod dcnf_ns;
pub mod dppic_ns;
pub mod egu0_ns;
pub mod ficr_s;
pub mod fpu_ns;
pub mod gpiote0_s;
pub mod i2s0_ns;
pub mod ipc_ns;
pub mod kmu_ns;
pub mod lpcomp_ns;
pub mod mutex_ns;
pub mod nfct_ns;
pub mod nvmc_ns;
pub mod oscillators_ns;
pub mod p0_ns;
pub mod pdm0_ns;
pub mod power_ns;
pub mod pwm0_ns;
pub mod qdec0_ns;
pub mod qspi_ns;
pub mod regulators_ns;
pub mod reset_ns;
pub mod rtc0_ns;
pub mod saadc_ns;
pub mod spim0_ns;
pub mod spis0_ns;
pub mod spu_s;
pub mod tad_s;
pub mod timer0_ns;
pub mod twim0_ns;
pub mod twis0_ns;
pub mod uarte0_ns;
pub mod uicr_s;
pub mod usbd_ns;
pub mod usbregulator_ns;
pub mod vmc_ns;
pub mod wdt0_ns;
