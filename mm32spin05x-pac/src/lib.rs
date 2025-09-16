#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (828b7b8 2025-09-01))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - Watchdog interrupt"]
    WWDG_IWDG = 0,
    #[doc = "1 - PVD through EXTI line detection interrupt"]
    PVD = 1,
    #[doc = "2 - Commutation and Input Interrupt"]
    PWM = 2,
    #[doc = "3 - Flash global interrupt"]
    FLASH = 3,
    #[doc = "4 - RCC global interrupt"]
    RCC = 4,
    #[doc = "5 - EXTI Line0_1 interrupt"]
    EXTI0_1 = 5,
    #[doc = "6 - EXTI Line2_3 interrupt"]
    EXTI2_3 = 6,
    #[doc = "7 - EXTI Line4_15 interrupt"]
    EXTI4_15 = 7,
    #[doc = "8 - HWDIV global interrupt"]
    HWDIV = 8,
    #[doc = "9 - DMA1 Channel1 global interrupt"]
    DMA1_CHANNEL1 = 9,
    #[doc = "10 - DMA1 Channel2_3 global interrupt"]
    DMA1_CHANNEL2_3 = 10,
    #[doc = "11 - DMA1 Channel4_5 global interrupt"]
    DMA1_CHANNEL4_5 = 11,
    #[doc = "12 - ADC and COMP global interrupt"]
    ADC_COMP = 12,
    #[doc = "13 - TIM1 break,Update,Trigger and Commutation interrupt"]
    TIM1_BRK_UP_TRG_COM = 13,
    #[doc = "14 - TIM1 Capture Compare interrupt"]
    TIM1_CC = 14,
    #[doc = "15 - TIM2 global interrupt"]
    TIM2 = 15,
    #[doc = "16 - TIM3 global interrupt"]
    TIM3 = 16,
    #[doc = "19 - TIM14 global interrupt"]
    TIM14 = 19,
    #[doc = "21 - TIM16 global interrupt"]
    TIM16 = 21,
    #[doc = "22 - TIM17 global interrupt"]
    TIM17 = 22,
    #[doc = "23 - I2C1 interrupt"]
    I2C1 = 23,
    #[doc = "25 - SPI1 global interrupt"]
    SPI1 = 25,
    #[doc = "26 - SPI2 global interrupt"]
    SPI2 = 26,
    #[doc = "27 - UART1 global interrupt"]
    UART1 = 27,
    #[doc = "28 - UART2 global interrupt"]
    UART2 = 28,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "DEVICE"]
pub const DEVICE: device::Device = unsafe { device::Device::from_ptr(0x1fff_f7e8usize as _) };
#[doc = "General purpose timer"]
pub const TIM2: tim2::Tim2 = unsafe { tim2::Tim2::from_ptr(0x4000_0000usize as _) };
#[doc = "General purpose timer"]
pub const TIM3: tim3::Tim3 = unsafe { tim3::Tim3::from_ptr(0x4000_0400usize as _) };
#[doc = "Window watchdog"]
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
#[doc = "Independent watchdog"]
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi1::Spi1 = unsafe { spi1::Spi1::from_ptr(0x4000_3800usize as _) };
pub const UART2: uart1::Uart1 = unsafe { uart1::Uart1::from_ptr(0x4000_4400usize as _) };
#[doc = "Inter integrated circuit"]
pub const I2C1: i2c1::I2c1 = unsafe { i2c1::I2c1::from_ptr(0x4000_5400usize as _) };
#[doc = "Power control"]
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
#[doc = "External interrupt/event controller"]
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0000usize as _) };
#[doc = "Analog to digital converter"]
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
#[doc = "Advanced timer"]
pub const TIM1: tim1::Tim1 = unsafe { tim1::Tim1::from_ptr(0x4001_2c00usize as _) };
#[doc = "Serial peripheral interface"]
pub const SPI1: spi1::Spi1 = unsafe { spi1::Spi1::from_ptr(0x4001_3000usize as _) };
#[doc = "Debug support"]
pub const DBG: dbg::Dbg = unsafe { dbg::Dbg::from_ptr(0x4001_3400usize as _) };
#[doc = "Universal asynchronous receiver transmitter"]
pub const UART1: uart1::Uart1 = unsafe { uart1::Uart1::from_ptr(0x4001_3800usize as _) };
#[doc = "comparator"]
pub const COMP: comp::Comp = unsafe { comp::Comp::from_ptr(0x4001_3c00usize as _) };
#[doc = "Basic purpose timer"]
pub const TIM14: tim14::Tim14 = unsafe { tim14::Tim14::from_ptr(0x4001_4000usize as _) };
#[doc = "Basic purpose timer"]
pub const TIM16: tim16::Tim16 = unsafe { tim16::Tim16::from_ptr(0x4001_4400usize as _) };
pub const TIM17: tim16::Tim16 = unsafe { tim16::Tim16::from_ptr(0x4001_4800usize as _) };
#[doc = "PWM Control"]
pub const PWM: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4001_6400usize as _) };
#[doc = "DMA1 controller"]
pub const DMA1: dma1::Dma1 = unsafe { dma1::Dma1::from_ptr(0x4002_0000usize as _) };
#[doc = "Reset and clock control"]
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
#[doc = "FLASH"]
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
#[doc = "CRC calculation unit"]
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
#[doc = "Hardware divider"]
pub const DIV: div::Div = unsafe { div::Div::from_ptr(0x4003_0000usize as _) };
#[doc = "General purpose I/O"]
pub const GPIOA: gpioa::Gpioa = unsafe { gpioa::Gpioa::from_ptr(0x4800_0000usize as _) };
pub const GPIOB: gpioa::Gpioa = unsafe { gpioa::Gpioa::from_ptr(0x4800_0400usize as _) };
pub const GPIOC: gpioa::Gpioa = unsafe { gpioa::Gpioa::from_ptr(0x4800_0800usize as _) };
pub const GPIOD: gpioa::Gpioa = unsafe { gpioa::Gpioa::from_ptr(0x4800_0c00usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub mod adc;
pub mod common;
pub mod comp;
pub mod crc;
pub mod dbg;
pub mod device;
pub mod div;
pub mod dma1;
pub mod exti;
pub mod flash;
pub mod gpioa;
pub mod i2c1;
pub mod iwdg;
pub mod pwm;
pub mod pwr;
pub mod rcc;
pub mod spi1;
pub mod tim1;
pub mod tim14;
pub mod tim16;
pub mod tim2;
pub mod tim3;
pub mod uart1;
pub mod wwdg;
