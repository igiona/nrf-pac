#![doc = "Peripheral access API (generated using chiptool v0.1.0 (a349968 2024-08-11))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "5 - CLOCK_POWER"]
    CLOCK_POWER = 5,
    #[doc = "8 - RADIO"]
    RADIO = 8,
    #[doc = "9 - RNG"]
    RNG = 9,
    #[doc = "10 - GPIOTE"]
    GPIOTE = 10,
    #[doc = "11 - WDT"]
    WDT = 11,
    #[doc = "12 - TIMER0"]
    TIMER0 = 12,
    #[doc = "13 - ECB"]
    ECB = 13,
    #[doc = "14 - AAR_CCM"]
    AAR_CCM = 14,
    #[doc = "16 - TEMP"]
    TEMP = 16,
    #[doc = "17 - RTC0"]
    RTC0 = 17,
    #[doc = "18 - IPC"]
    IPC = 18,
    #[doc = "19 - SERIAL0"]
    SERIAL0 = 19,
    #[doc = "20 - EGU0"]
    EGU0 = 20,
    #[doc = "22 - RTC1"]
    RTC1 = 22,
    #[doc = "24 - TIMER1"]
    TIMER1 = 24,
    #[doc = "25 - TIMER2"]
    TIMER2 = 25,
    #[doc = "26 - SWI0"]
    SWI0 = 26,
    #[doc = "27 - SWI1"]
    SWI1 = 27,
    #[doc = "28 - SWI2"]
    SWI2 = 28,
    #[doc = "29 - SWI3"]
    SWI3 = 29,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "Factory Information Configuration Registers"]
pub const FICR_NS: ficr_ns::FicrNs = unsafe { ficr_ns::FicrNs::from_ptr(0x01ff_0000usize as _) };
#[doc = "User Information Configuration Registers"]
pub const UICR_NS: uicr_ns::UicrNs = unsafe { uicr_ns::UicrNs::from_ptr(0x01ff_8000usize as _) };
#[doc = "MUTEX 0"]
pub const APPMUTEX_NS: appmutex_ns::AppmutexNs =
    unsafe { appmutex_ns::AppmutexNs::from_ptr(0x4003_0000usize as _) };
#[doc = "Domain configuration management"]
pub const DCNF_NS: dcnf_ns::DcnfNs = unsafe { dcnf_ns::DcnfNs::from_ptr(0x4100_0000usize as _) };
#[doc = "Voltage request control"]
pub const VREQCTRL_NS: vreqctrl_ns::VreqctrlNs =
    unsafe { vreqctrl_ns::VreqctrlNs::from_ptr(0x4100_4000usize as _) };
#[doc = "Clock management"]
pub const CLOCK_NS: clock_ns::ClockNs =
    unsafe { clock_ns::ClockNs::from_ptr(0x4100_5000usize as _) };
#[doc = "Power control"]
pub const POWER_NS: power_ns::PowerNs =
    unsafe { power_ns::PowerNs::from_ptr(0x4100_5000usize as _) };
#[doc = "Reset control"]
pub const RESET_NS: reset_ns::ResetNs =
    unsafe { reset_ns::ResetNs::from_ptr(0x4100_5000usize as _) };
#[doc = "Control access port"]
pub const CTRLAP_NS: ctrlap_ns::CtrlapNs =
    unsafe { ctrlap_ns::CtrlapNs::from_ptr(0x4100_6000usize as _) };
#[doc = "2.4 GHz radio"]
pub const RADIO_NS: radio_ns::RadioNs =
    unsafe { radio_ns::RadioNs::from_ptr(0x4100_8000usize as _) };
#[doc = "Random Number Generator"]
pub const RNG_NS: rng_ns::RngNs = unsafe { rng_ns::RngNs::from_ptr(0x4100_9000usize as _) };
#[doc = "GPIO Tasks and Events"]
pub const GPIOTE_NS: gpiote_ns::GpioteNs =
    unsafe { gpiote_ns::GpioteNs::from_ptr(0x4100_a000usize as _) };
#[doc = "Watchdog Timer"]
pub const WDT_NS: wdt_ns::WdtNs = unsafe { wdt_ns::WdtNs::from_ptr(0x4100_b000usize as _) };
#[doc = "Timer/Counter 0"]
pub const TIMER0_NS: timer0_ns::Timer0ns =
    unsafe { timer0_ns::Timer0ns::from_ptr(0x4100_c000usize as _) };
#[doc = "AES ECB Mode Encryption"]
pub const ECB_NS: ecb_ns::EcbNs = unsafe { ecb_ns::EcbNs::from_ptr(0x4100_d000usize as _) };
#[doc = "Accelerated Address Resolver"]
pub const AAR_NS: aar_ns::AarNs = unsafe { aar_ns::AarNs::from_ptr(0x4100_e000usize as _) };
#[doc = "AES CCM mode encryption"]
pub const CCM_NS: ccm_ns::CcmNs = unsafe { ccm_ns::CcmNs::from_ptr(0x4100_e000usize as _) };
#[doc = "Distributed programmable peripheral interconnect controller"]
pub const DPPIC_NS: dppic_ns::DppicNs =
    unsafe { dppic_ns::DppicNs::from_ptr(0x4100_f000usize as _) };
#[doc = "Temperature Sensor"]
pub const TEMP_NS: temp_ns::TempNs = unsafe { temp_ns::TempNs::from_ptr(0x4101_0000usize as _) };
#[doc = "Real-time counter 0"]
pub const RTC0_NS: rtc0_ns::Rtc0ns = unsafe { rtc0_ns::Rtc0ns::from_ptr(0x4101_1000usize as _) };
#[doc = "Interprocessor communication"]
pub const IPC_NS: ipc_ns::IpcNs = unsafe { ipc_ns::IpcNs::from_ptr(0x4101_2000usize as _) };
#[doc = "Serial Peripheral Interface Master with EasyDMA"]
pub const SPIM0_NS: spim0_ns::Spim0ns =
    unsafe { spim0_ns::Spim0ns::from_ptr(0x4101_3000usize as _) };
#[doc = "SPI Slave"]
pub const SPIS0_NS: spis0_ns::Spis0ns =
    unsafe { spis0_ns::Spis0ns::from_ptr(0x4101_3000usize as _) };
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA"]
pub const TWIM0_NS: twim0_ns::Twim0ns =
    unsafe { twim0_ns::Twim0ns::from_ptr(0x4101_3000usize as _) };
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA"]
pub const TWIS0_NS: twis0_ns::Twis0ns =
    unsafe { twis0_ns::Twis0ns::from_ptr(0x4101_3000usize as _) };
#[doc = "UART with EasyDMA"]
pub const UARTE0_NS: uarte0_ns::Uarte0ns =
    unsafe { uarte0_ns::Uarte0ns::from_ptr(0x4101_3000usize as _) };
#[doc = "Event generator unit"]
pub const EGU0_NS: egu0_ns::Egu0ns = unsafe { egu0_ns::Egu0ns::from_ptr(0x4101_4000usize as _) };
#[doc = "Real-time counter 1"]
pub const RTC1_NS: rtc0_ns::Rtc0ns = unsafe { rtc0_ns::Rtc0ns::from_ptr(0x4101_6000usize as _) };
#[doc = "Timer/Counter 1"]
pub const TIMER1_NS: timer0_ns::Timer0ns =
    unsafe { timer0_ns::Timer0ns::from_ptr(0x4101_8000usize as _) };
#[doc = "Timer/Counter 2"]
pub const TIMER2_NS: timer0_ns::Timer0ns =
    unsafe { timer0_ns::Timer0ns::from_ptr(0x4101_9000usize as _) };
#[doc = "Software interrupt 0"]
pub const SWI0_NS: swi0_ns::Swi0ns = unsafe { swi0_ns::Swi0ns::from_ptr(0x4101_a000usize as _) };
#[doc = "Software interrupt 1"]
pub const SWI1_NS: swi0_ns::Swi0ns = unsafe { swi0_ns::Swi0ns::from_ptr(0x4101_b000usize as _) };
#[doc = "Software interrupt 2"]
pub const SWI2_NS: swi0_ns::Swi0ns = unsafe { swi0_ns::Swi0ns::from_ptr(0x4101_c000usize as _) };
#[doc = "Software interrupt 3"]
pub const SWI3_NS: swi0_ns::Swi0ns = unsafe { swi0_ns::Swi0ns::from_ptr(0x4101_d000usize as _) };
#[doc = "Access control lists"]
pub const ACL_NS: acl_ns::AclNs = unsafe { acl_ns::AclNs::from_ptr(0x4108_0000usize as _) };
#[doc = "Non-volatile memory controller"]
pub const NVMC_NS: nvmc_ns::NvmcNs = unsafe { nvmc_ns::NvmcNs::from_ptr(0x4108_0000usize as _) };
#[doc = "Volatile Memory controller"]
pub const VMC_NS: vmc_ns::VmcNs = unsafe { vmc_ns::VmcNs::from_ptr(0x4108_1000usize as _) };
#[doc = "GPIO Port 0"]
pub const P0_NS: p0_ns::P0ns = unsafe { p0_ns::P0ns::from_ptr(0x418c_0500usize as _) };
#[doc = "GPIO Port 1"]
pub const P1_NS: p0_ns::P0ns = unsafe { p0_ns::P0ns::from_ptr(0x418c_0800usize as _) };
#[doc = "MUTEX 1"]
pub const APPMUTEX_S: appmutex_ns::AppmutexNs =
    unsafe { appmutex_ns::AppmutexNs::from_ptr(0x5003_0000usize as _) };
#[doc = "Cross-Trigger Interface control. NOTE: this is not a separate peripheral, but describes CM33 functionality."]
pub const CTI_NS: cti_ns::CtiNs = unsafe { cti_ns::CtiNs::from_ptr(0xe004_2000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod aar_ns;
pub mod acl_ns;
pub mod appmutex_ns;
pub mod ccm_ns;
pub mod clock_ns;
pub mod common;
pub mod cti_ns;
pub mod ctrlap_ns;
pub mod dcnf_ns;
pub mod dppic_ns;
pub mod ecb_ns;
pub mod egu0_ns;
pub mod ficr_ns;
pub mod gpiote_ns;
pub mod ipc_ns;
pub mod nvmc_ns;
pub mod p0_ns;
pub mod power_ns;
pub mod radio_ns;
pub mod reset_ns;
pub mod rng_ns;
pub mod rtc0_ns;
pub mod spim0_ns;
pub mod spis0_ns;
pub mod swi0_ns;
pub mod temp_ns;
pub mod timer0_ns;
pub mod twim0_ns;
pub mod twis0_ns;
pub mod uarte0_ns;
pub mod uicr_ns;
pub mod vmc_ns;
pub mod vreqctrl_ns;
pub mod wdt_ns;
