unsafe extern "C" {
    fn WWDG_IWDG();
    fn PVD();
    fn PWM();
    fn FLASH();
    fn RCC();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn HWDIV();
    fn DMA1_CHANNEL1();
    fn DMA1_CHANNEL2_3();
    fn DMA1_CHANNEL4_5();
    fn ADC_COMP();
    fn TIM1_BRK_UP_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM14();
    fn TIM16();
    fn TIM17();
    fn I2C1();
    fn SPI1();
    fn SPI2();
    fn UART1();
    fn UART2();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 29] = [
    Vector {
        _handler: WWDG_IWDG,
    },
    Vector { _handler: PVD },
    Vector { _handler: PWM },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_15 },
    Vector { _handler: HWDIV },
    Vector {
        _handler: DMA1_CHANNEL1,
    },
    Vector {
        _handler: DMA1_CHANNEL2_3,
    },
    Vector {
        _handler: DMA1_CHANNEL4_5,
    },
    Vector { _handler: ADC_COMP },
    Vector {
        _handler: TIM1_BRK_UP_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM14 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: I2C1 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: UART1 },
    Vector { _handler: UART2 },
];
