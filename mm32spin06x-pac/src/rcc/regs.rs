#[doc = "Advanced High Performance Bus Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbenr(pub u32);
impl Ahbenr {
    #[doc = "DMA clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA clock enable"]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SRAM interface clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM interface clock enable"]
    #[inline(always)]
    pub const fn set_sram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FLASH clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn flash(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FLASH clock enable"]
    #[inline(always)]
    pub const fn set_flash(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CRC clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn crc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CRC clock enable"]
    #[inline(always)]
    pub const fn set_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIOA clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpioa(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GPIOA clock enable"]
    #[inline(always)]
    pub const fn set_gpioa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPIOB clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpiob(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPIOB clock enable"]
    #[inline(always)]
    pub const fn set_gpiob(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "GPIOC clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpioc(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "GPIOC clock enable"]
    #[inline(always)]
    pub const fn set_gpioc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIOD clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpiod(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIOD clock enable"]
    #[inline(always)]
    pub const fn set_gpiod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "HWDIV clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hwdiv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "HWDIV clock enable"]
    #[inline(always)]
    pub const fn set_hwdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Ahbenr {
    #[inline(always)]
    fn default() -> Ahbenr {
        Ahbenr(0)
    }
}
impl core::fmt::Debug for Ahbenr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbenr")
            .field("dma", &self.dma())
            .field("sram", &self.sram())
            .field("flash", &self.flash())
            .field("crc", &self.crc())
            .field("gpioa", &self.gpioa())
            .field("gpiob", &self.gpiob())
            .field("gpioc", &self.gpioc())
            .field("gpiod", &self.gpiod())
            .field("hwdiv", &self.hwdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbenr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbenr {{ dma: {=bool:?}, sram: {=bool:?}, flash: {=bool:?}, crc: {=bool:?}, gpioa: {=bool:?}, gpiob: {=bool:?}, gpioc: {=bool:?}, gpiod: {=bool:?}, hwdiv: {=bool:?} }}",
            self.dma(),
            self.sram(),
            self.flash(),
            self.crc(),
            self.gpioa(),
            self.gpiob(),
            self.gpioc(),
            self.gpiod(),
            self.hwdiv()
        )
    }
}
#[doc = "Advanced High Performance Bus Reset Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrstr(pub u32);
impl Ahbrstr {
    #[doc = "GPIOA reset"]
    #[must_use]
    #[inline(always)]
    pub const fn gpioa(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GPIOA reset"]
    #[inline(always)]
    pub const fn set_gpioa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPIOB reset"]
    #[must_use]
    #[inline(always)]
    pub const fn gpiob(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPIOB reset"]
    #[inline(always)]
    pub const fn set_gpiob(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "GPIOC reset"]
    #[must_use]
    #[inline(always)]
    pub const fn gpioc(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "GPIOC reset"]
    #[inline(always)]
    pub const fn set_gpioc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIOD reset"]
    #[must_use]
    #[inline(always)]
    pub const fn gpiod(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIOD reset"]
    #[inline(always)]
    pub const fn set_gpiod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "HWDIV reset"]
    #[must_use]
    #[inline(always)]
    pub const fn hwdiv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "HWDIV reset"]
    #[inline(always)]
    pub const fn set_hwdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Ahbrstr {
    #[inline(always)]
    fn default() -> Ahbrstr {
        Ahbrstr(0)
    }
}
impl core::fmt::Debug for Ahbrstr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrstr")
            .field("gpioa", &self.gpioa())
            .field("gpiob", &self.gpiob())
            .field("gpioc", &self.gpioc())
            .field("gpiod", &self.gpiod())
            .field("hwdiv", &self.hwdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrstr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrstr {{ gpioa: {=bool:?}, gpiob: {=bool:?}, gpioc: {=bool:?}, gpiod: {=bool:?}, hwdiv: {=bool:?} }}",
            self.gpioa(),
            self.gpiob(),
            self.gpioc(),
            self.gpiod(),
            self.hwdiv()
        )
    }
}
#[doc = "Advanced Peripheral Bus 1 Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apb1enr(pub u32);
impl Apb1enr {
    #[doc = "TIM2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tim2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM2 clock enable"]
    #[inline(always)]
    pub const fn set_tim2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tim3(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM3 clock enable"]
    #[inline(always)]
    pub const fn set_tim3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Window watchdog clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Window watchdog clock enable"]
    #[inline(always)]
    pub const fn set_wwdg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SPI2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spi2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SPI2 clock enable"]
    #[inline(always)]
    pub const fn set_spi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UART2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uart2(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "UART2 clock enable"]
    #[inline(always)]
    pub const fn set_uart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "I2C1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn i2c1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "I2C1 clock enable"]
    #[inline(always)]
    pub const fn set_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "USB clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usb(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "USB clock enable"]
    #[inline(always)]
    pub const fn set_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "CAN 2C1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn can(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "CAN 2C1 clock enable"]
    #[inline(always)]
    pub const fn set_can(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "CSM clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn csm(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "CSM clock enable"]
    #[inline(always)]
    pub const fn set_csm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "CRS clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn crs(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "CRS clock enable"]
    #[inline(always)]
    pub const fn set_crs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Power interface clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Power interface clock enable"]
    #[inline(always)]
    pub const fn set_pwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Apb1enr {
    #[inline(always)]
    fn default() -> Apb1enr {
        Apb1enr(0)
    }
}
impl core::fmt::Debug for Apb1enr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apb1enr")
            .field("tim2", &self.tim2())
            .field("tim3", &self.tim3())
            .field("wwdg", &self.wwdg())
            .field("spi2", &self.spi2())
            .field("uart2", &self.uart2())
            .field("i2c1", &self.i2c1())
            .field("usb", &self.usb())
            .field("can", &self.can())
            .field("csm", &self.csm())
            .field("crs", &self.crs())
            .field("pwr", &self.pwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apb1enr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apb1enr {{ tim2: {=bool:?}, tim3: {=bool:?}, wwdg: {=bool:?}, spi2: {=bool:?}, uart2: {=bool:?}, i2c1: {=bool:?}, usb: {=bool:?}, can: {=bool:?}, csm: {=bool:?}, crs: {=bool:?}, pwr: {=bool:?} }}",
            self.tim2(),
            self.tim3(),
            self.wwdg(),
            self.spi2(),
            self.uart2(),
            self.i2c1(),
            self.usb(),
            self.can(),
            self.csm(),
            self.crs(),
            self.pwr()
        )
    }
}
#[doc = "Advanced Peripheral Bus 1 Reset Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apb1rstr(pub u32);
impl Apb1rstr {
    #[doc = "TIM2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tim2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM2 reset"]
    #[inline(always)]
    pub const fn set_tim2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tim3(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM3 reset"]
    #[inline(always)]
    pub const fn set_tim3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Window watchdog reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Window watchdog reset"]
    #[inline(always)]
    pub const fn set_wwdg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SPI2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn spi2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SPI2 reset"]
    #[inline(always)]
    pub const fn set_spi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UART2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn uart2(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "UART2 reset"]
    #[inline(always)]
    pub const fn set_uart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "I2C1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn i2c1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "I2C1 reset"]
    #[inline(always)]
    pub const fn set_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "USB reset"]
    #[must_use]
    #[inline(always)]
    pub const fn usb(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "USB reset"]
    #[inline(always)]
    pub const fn set_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "CAN reset"]
    #[must_use]
    #[inline(always)]
    pub const fn can(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "CAN reset"]
    #[inline(always)]
    pub const fn set_can(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "CSM reset"]
    #[must_use]
    #[inline(always)]
    pub const fn csm(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "CSM reset"]
    #[inline(always)]
    pub const fn set_csm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "CRS interface reset"]
    #[must_use]
    #[inline(always)]
    pub const fn crs(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "CRS interface reset"]
    #[inline(always)]
    pub const fn set_crs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Power interface reset"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Power interface reset"]
    #[inline(always)]
    pub const fn set_pwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Apb1rstr {
    #[inline(always)]
    fn default() -> Apb1rstr {
        Apb1rstr(0)
    }
}
impl core::fmt::Debug for Apb1rstr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apb1rstr")
            .field("tim2", &self.tim2())
            .field("tim3", &self.tim3())
            .field("wwdg", &self.wwdg())
            .field("spi2", &self.spi2())
            .field("uart2", &self.uart2())
            .field("i2c1", &self.i2c1())
            .field("usb", &self.usb())
            .field("can", &self.can())
            .field("csm", &self.csm())
            .field("crs", &self.crs())
            .field("pwr", &self.pwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apb1rstr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apb1rstr {{ tim2: {=bool:?}, tim3: {=bool:?}, wwdg: {=bool:?}, spi2: {=bool:?}, uart2: {=bool:?}, i2c1: {=bool:?}, usb: {=bool:?}, can: {=bool:?}, csm: {=bool:?}, crs: {=bool:?}, pwr: {=bool:?} }}",
            self.tim2(),
            self.tim3(),
            self.wwdg(),
            self.spi2(),
            self.uart2(),
            self.i2c1(),
            self.usb(),
            self.can(),
            self.csm(),
            self.crs(),
            self.pwr()
        )
    }
}
#[doc = "Advanced Peripheral Bus 2 Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apb2enr(pub u32);
impl Apb2enr {
    #[doc = "Syscfg configuration register enable"]
    #[must_use]
    #[inline(always)]
    pub const fn syscfg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Syscfg configuration register enable"]
    #[inline(always)]
    pub const fn set_syscfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC1 interface enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1 interface enable"]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "TIM1 Timer clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tim1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TIM1 Timer clock enable"]
    #[inline(always)]
    pub const fn set_tim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SPI1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spi1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SPI1 clock enable"]
    #[inline(always)]
    pub const fn set_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "UART1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uart1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UART1 clock enable"]
    #[inline(always)]
    pub const fn set_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "COMP Comparator enable"]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "COMP Comparator enable"]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "TIM14 timer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tim14(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TIM14 timer enable"]
    #[inline(always)]
    pub const fn set_tim14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TIM16 timer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tim16(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TIM16 timer enable"]
    #[inline(always)]
    pub const fn set_tim16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TIM17 timer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tim17(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TIM17 timer enable"]
    #[inline(always)]
    pub const fn set_tim17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DBGMCU enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgmcu(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DBGMCU enable"]
    #[inline(always)]
    pub const fn set_dbgmcu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Apb2enr {
    #[inline(always)]
    fn default() -> Apb2enr {
        Apb2enr(0)
    }
}
impl core::fmt::Debug for Apb2enr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apb2enr")
            .field("syscfg", &self.syscfg())
            .field("adc1", &self.adc1())
            .field("tim1", &self.tim1())
            .field("spi1", &self.spi1())
            .field("uart1", &self.uart1())
            .field("comp", &self.comp())
            .field("tim14", &self.tim14())
            .field("tim16", &self.tim16())
            .field("tim17", &self.tim17())
            .field("dbgmcu", &self.dbgmcu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apb2enr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apb2enr {{ syscfg: {=bool:?}, adc1: {=bool:?}, tim1: {=bool:?}, spi1: {=bool:?}, uart1: {=bool:?}, comp: {=bool:?}, tim14: {=bool:?}, tim16: {=bool:?}, tim17: {=bool:?}, dbgmcu: {=bool:?} }}",
            self.syscfg(),
            self.adc1(),
            self.tim1(),
            self.spi1(),
            self.uart1(),
            self.comp(),
            self.tim14(),
            self.tim16(),
            self.tim17(),
            self.dbgmcu()
        )
    }
}
#[doc = "Advanced Peripheral Bus 2 Reset Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apb2rstr(pub u32);
impl Apb2rstr {
    #[doc = "Syscfg configuration register reset"]
    #[must_use]
    #[inline(always)]
    pub const fn syscfg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Syscfg configuration register reset"]
    #[inline(always)]
    pub const fn set_syscfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC1 interface reset"]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1 interface reset"]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "TIM1 timer reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tim1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TIM1 timer reset"]
    #[inline(always)]
    pub const fn set_tim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SPI1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn spi1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SPI1 reset"]
    #[inline(always)]
    pub const fn set_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "UART1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn uart1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UART1 reset"]
    #[inline(always)]
    pub const fn set_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Comparator reset"]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator reset"]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "TIM14 timer reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tim14(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TIM14 timer reset"]
    #[inline(always)]
    pub const fn set_tim14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TIM16 timer reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tim16(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TIM16 timer reset"]
    #[inline(always)]
    pub const fn set_tim16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TIM17 timer reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tim17(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TIM17 timer reset"]
    #[inline(always)]
    pub const fn set_tim17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DBGMCU reset"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgmcu(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DBGMCU reset"]
    #[inline(always)]
    pub const fn set_dbgmcu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Apb2rstr {
    #[inline(always)]
    fn default() -> Apb2rstr {
        Apb2rstr(0)
    }
}
impl core::fmt::Debug for Apb2rstr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apb2rstr")
            .field("syscfg", &self.syscfg())
            .field("adc1", &self.adc1())
            .field("tim1", &self.tim1())
            .field("spi1", &self.spi1())
            .field("uart1", &self.uart1())
            .field("comp", &self.comp())
            .field("tim14", &self.tim14())
            .field("tim16", &self.tim16())
            .field("tim17", &self.tim17())
            .field("dbgmcu", &self.dbgmcu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apb2rstr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apb2rstr {{ syscfg: {=bool:?}, adc1: {=bool:?}, tim1: {=bool:?}, spi1: {=bool:?}, uart1: {=bool:?}, comp: {=bool:?}, tim14: {=bool:?}, tim16: {=bool:?}, tim17: {=bool:?}, dbgmcu: {=bool:?} }}",
            self.syscfg(),
            self.adc1(),
            self.tim1(),
            self.spi1(),
            self.uart1(),
            self.comp(),
            self.tim14(),
            self.tim16(),
            self.tim17(),
            self.dbgmcu()
        )
    }
}
#[doc = "Backup Domain Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bdcr(pub u32);
impl Bdcr {
    #[doc = "External low-speed oscillator enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lseon(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "External low-speed oscillator enable"]
    #[inline(always)]
    pub const fn set_lseon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "External low-speed oscillator ready"]
    #[must_use]
    #[inline(always)]
    pub const fn lserdy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "External low-speed oscillator ready"]
    #[inline(always)]
    pub const fn set_lserdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "External low-speed oscillator bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn lsebyp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External low-speed oscillator bypass"]
    #[inline(always)]
    pub const fn set_lsebyp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RTC clock source selection"]
    #[must_use]
    #[inline(always)]
    pub const fn rtcsel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "RTC clock source selection"]
    #[inline(always)]
    pub const fn set_rtcsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "RTC clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rtcen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "RTC clock enable"]
    #[inline(always)]
    pub const fn set_rtcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Backup domain software reset"]
    #[must_use]
    #[inline(always)]
    pub const fn bdrst(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Backup domain software reset"]
    #[inline(always)]
    pub const fn set_bdrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Bdcr {
    #[inline(always)]
    fn default() -> Bdcr {
        Bdcr(0)
    }
}
impl core::fmt::Debug for Bdcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bdcr")
            .field("lseon", &self.lseon())
            .field("lserdy", &self.lserdy())
            .field("lsebyp", &self.lsebyp())
            .field("rtcsel", &self.rtcsel())
            .field("rtcen", &self.rtcen())
            .field("bdrst", &self.bdrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bdcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bdcr {{ lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, rtcsel: {=u8:?}, rtcen: {=bool:?}, bdrst: {=bool:?} }}",
            self.lseon(),
            self.lserdy(),
            self.lsebyp(),
            self.rtcsel(),
            self.rtcen(),
            self.bdrst()
        )
    }
}
#[doc = "Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc = "System clock switch"]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "System clock switch"]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "System clock switch status"]
    #[must_use]
    #[inline(always)]
    pub const fn sws(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "System clock switch status"]
    #[inline(always)]
    pub const fn set_sws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "AHB Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn hpre(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Prescaler"]
    #[inline(always)]
    pub const fn set_hpre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "APB low-speed prescaler(APB1)"]
    #[must_use]
    #[inline(always)]
    pub const fn ppre1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "APB low-speed prescaler(APB1)"]
    #[inline(always)]
    pub const fn set_ppre1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "APB high-speed prescaler(APB2)"]
    #[must_use]
    #[inline(always)]
    pub const fn ppre2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[doc = "APB high-speed prescaler(APB2)"]
    #[inline(always)]
    pub const fn set_ppre2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
    #[doc = "USB 48M clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn clk48sel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB 48M clock selection"]
    #[inline(always)]
    pub const fn set_clk48sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PLL entry clock source"]
    #[must_use]
    #[inline(always)]
    pub const fn pllsrc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PLL entry clock source"]
    #[inline(always)]
    pub const fn set_pllsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "HSE divider for PLL entry"]
    #[must_use]
    #[inline(always)]
    pub const fn pllxtpre(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "HSE divider for PLL entry"]
    #[inline(always)]
    pub const fn set_pllxtpre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "USB prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn usbrpe(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "USB prescaler"]
    #[inline(always)]
    pub const fn set_usbrpe(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Microcontroller clock output"]
    #[must_use]
    #[inline(always)]
    pub const fn mco(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Microcontroller clock output"]
    #[inline(always)]
    pub const fn set_mco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "PLL configuration factor expansion"]
    #[must_use]
    #[inline(always)]
    pub const fn plldn(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "PLL configuration factor expansion"]
    #[inline(always)]
    pub const fn set_plldn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Cfgr {
    #[inline(always)]
    fn default() -> Cfgr {
        Cfgr(0)
    }
}
impl core::fmt::Debug for Cfgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgr")
            .field("sw", &self.sw())
            .field("sws", &self.sws())
            .field("hpre", &self.hpre())
            .field("ppre1", &self.ppre1())
            .field("ppre2", &self.ppre2())
            .field("clk48sel", &self.clk48sel())
            .field("pllsrc", &self.pllsrc())
            .field("pllxtpre", &self.pllxtpre())
            .field("usbrpe", &self.usbrpe())
            .field("mco", &self.mco())
            .field("plldn", &self.plldn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfgr {{ sw: {=u8:?}, sws: {=u8:?}, hpre: {=u8:?}, ppre1: {=u8:?}, ppre2: {=u8:?}, clk48sel: {=bool:?}, pllsrc: {=bool:?}, pllxtpre: {=bool:?}, usbrpe: {=u8:?}, mco: {=u8:?}, plldn: {=u8:?} }}",
            self.sw(),
            self.sws(),
            self.hpre(),
            self.ppre1(),
            self.ppre2(),
            self.clk48sel(),
            self.pllsrc(),
            self.pllxtpre(),
            self.usbrpe(),
            self.mco(),
            self.plldn()
        )
    }
}
#[doc = "Configure Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
    #[doc = "TIMADV clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn timadv_clksel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIMADV clock selection"]
    #[inline(always)]
    pub const fn set_timadv_clksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIMADV lock prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn timadv_pre(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "TIMADV lock prescaler"]
    #[inline(always)]
    pub const fn set_timadv_pre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
}
impl Default for Cfgr2 {
    #[inline(always)]
    fn default() -> Cfgr2 {
        Cfgr2(0)
    }
}
impl core::fmt::Debug for Cfgr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgr2")
            .field("timadv_clksel", &self.timadv_clksel())
            .field("timadv_pre", &self.timadv_pre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfgr2 {{ timadv_clksel: {=bool:?}, timadv_pre: {=u8:?} }}",
            self.timadv_clksel(),
            self.timadv_pre()
        )
    }
}
#[doc = "Clock Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cir(pub u32);
impl Cir {
    #[doc = "LSI ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn lsirdyf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LSI ready interrupt flag"]
    #[inline(always)]
    pub const fn set_lsirdyf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LSE ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn lserdyf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LSE ready interrupt flag"]
    #[inline(always)]
    pub const fn set_lserdyf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HSI ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hsirdyf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HSI ready interrupt flag"]
    #[inline(always)]
    pub const fn set_hsirdyf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HSE ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hserdyf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HSE ready interrupt flag"]
    #[inline(always)]
    pub const fn set_hserdyf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PLL ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pllrdyf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PLL ready interrupt flag"]
    #[inline(always)]
    pub const fn set_pllrdyf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Clock security system interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cssf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Clock security system interrupt flag"]
    #[inline(always)]
    pub const fn set_cssf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LSI ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lsirdyie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LSI ready interrupt enable"]
    #[inline(always)]
    pub const fn set_lsirdyie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LSE ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lserdyie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LSE ready interrupt enable"]
    #[inline(always)]
    pub const fn set_lserdyie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HSI ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hsirdyie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HSI ready interrupt enable"]
    #[inline(always)]
    pub const fn set_hsirdyie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "HSE ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hserdyie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "HSE ready interrupt enable"]
    #[inline(always)]
    pub const fn set_hserdyie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PLL ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pllrdyie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PLL ready interrupt enable"]
    #[inline(always)]
    pub const fn set_pllrdyie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LSI ready interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn lsirdyc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LSI ready interrupt clear"]
    #[inline(always)]
    pub const fn set_lsirdyc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "LSE ready interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn lserdyc(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "LSE ready interrupt clear"]
    #[inline(always)]
    pub const fn set_lserdyc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "HSI ready interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hsirdyc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "HSI ready interrupt clear"]
    #[inline(always)]
    pub const fn set_hsirdyc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "HSE ready interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hserdyc(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "HSE ready interrupt clear"]
    #[inline(always)]
    pub const fn set_hserdyc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PLL ready interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn pllrdyc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PLL ready interrupt clear"]
    #[inline(always)]
    pub const fn set_pllrdyc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Clock security system interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cssc(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Clock security system interrupt clear"]
    #[inline(always)]
    pub const fn set_cssc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Cir {
    #[inline(always)]
    fn default() -> Cir {
        Cir(0)
    }
}
impl core::fmt::Debug for Cir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cir")
            .field("lsirdyf", &self.lsirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("hserdyf", &self.hserdyf())
            .field("pllrdyf", &self.pllrdyf())
            .field("cssf", &self.cssf())
            .field("lsirdyie", &self.lsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .field("pllrdyie", &self.pllrdyie())
            .field("lsirdyc", &self.lsirdyc())
            .field("lserdyc", &self.lserdyc())
            .field("hsirdyc", &self.hsirdyc())
            .field("hserdyc", &self.hserdyc())
            .field("pllrdyc", &self.pllrdyc())
            .field("cssc", &self.cssc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cir {{ lsirdyf: {=bool:?}, lserdyf: {=bool:?}, hsirdyf: {=bool:?}, hserdyf: {=bool:?}, pllrdyf: {=bool:?}, cssf: {=bool:?}, lsirdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, pllrdyie: {=bool:?}, lsirdyc: {=bool:?}, lserdyc: {=bool:?}, hsirdyc: {=bool:?}, hserdyc: {=bool:?}, pllrdyc: {=bool:?}, cssc: {=bool:?} }}",
            self.lsirdyf(),
            self.lserdyf(),
            self.hsirdyf(),
            self.hserdyf(),
            self.pllrdyf(),
            self.cssf(),
            self.lsirdyie(),
            self.lserdyie(),
            self.hsirdyie(),
            self.hserdyie(),
            self.pllrdyie(),
            self.lsirdyc(),
            self.lserdyc(),
            self.hsirdyc(),
            self.hserdyc(),
            self.pllrdyc(),
            self.cssc()
        )
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Internal high-speed clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hsion(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Internal high-speed clock enable"]
    #[inline(always)]
    pub const fn set_hsion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Internal high-speed clock ready flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hsirdy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal high-speed clock ready flag"]
    #[inline(always)]
    pub const fn set_hsirdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Internal high-speed clock temperature enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hsiten(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Internal high-speed clock temperature enable"]
    #[inline(always)]
    pub const fn set_hsiten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "External high-speed clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hseon(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "External high-speed clock enable"]
    #[inline(always)]
    pub const fn set_hseon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "External high-speed clock ready flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hserdy(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "External high-speed clock ready flag"]
    #[inline(always)]
    pub const fn set_hserdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "External high-speed clock bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn hsebyp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "External high-speed clock bypass"]
    #[inline(always)]
    pub const fn set_hsebyp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Clock security system enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ccson(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Clock security system enable"]
    #[inline(always)]
    pub const fn set_ccson(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PLL divider factor"]
    #[must_use]
    #[inline(always)]
    pub const fn plldiv(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "PLL divider factor"]
    #[inline(always)]
    pub const fn set_plldiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "PLL enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pllon(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PLL enable"]
    #[inline(always)]
    pub const fn set_pllon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "PLL clock ready flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pllrdy(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "PLL clock ready flag"]
    #[inline(always)]
    pub const fn set_pllrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "PLL multiplication factor"]
    #[must_use]
    #[inline(always)]
    pub const fn pllmul(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "PLL multiplication factor"]
    #[inline(always)]
    pub const fn set_pllmul(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("hsion", &self.hsion())
            .field("hsirdy", &self.hsirdy())
            .field("hsiten", &self.hsiten())
            .field("hseon", &self.hseon())
            .field("hserdy", &self.hserdy())
            .field("hsebyp", &self.hsebyp())
            .field("ccson", &self.ccson())
            .field("plldiv", &self.plldiv())
            .field("pllon", &self.pllon())
            .field("pllrdy", &self.pllrdy())
            .field("pllmul", &self.pllmul())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ hsion: {=bool:?}, hsirdy: {=bool:?}, hsiten: {=bool:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsebyp: {=bool:?}, ccson: {=bool:?}, plldiv: {=u8:?}, pllon: {=bool:?}, pllrdy: {=bool:?}, pllmul: {=u8:?} }}",
            self.hsion(),
            self.hsirdy(),
            self.hsiten(),
            self.hseon(),
            self.hserdy(),
            self.hsebyp(),
            self.ccson(),
            self.plldiv(),
            self.pllon(),
            self.pllrdy(),
            self.pllmul()
        )
    }
}
#[doc = "Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Internal low-speed oscillator enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lsion(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Internal low-speed oscillator enable"]
    #[inline(always)]
    pub const fn set_lsion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Internal low-speed oscillator ready"]
    #[must_use]
    #[inline(always)]
    pub const fn lsirdy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal low-speed oscillator ready"]
    #[inline(always)]
    pub const fn set_lsirdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PVD reset enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pvdrsten(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PVD reset enable"]
    #[inline(always)]
    pub const fn set_pvdrsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CPU lockup reset enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lockupen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CPU lockup reset enable"]
    #[inline(always)]
    pub const fn set_lockupen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PVD reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pvdrstf(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PVD reset flag"]
    #[inline(always)]
    pub const fn set_pvdrstf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "CPU lockup reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn lockupf(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "CPU lockup reset flag"]
    #[inline(always)]
    pub const fn set_lockupf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Remove reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rmvf(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Remove reset flag"]
    #[inline(always)]
    pub const fn set_rmvf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "PIN reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pinrstf(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "PIN reset flag"]
    #[inline(always)]
    pub const fn set_pinrstf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "POR/PDR reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn porrstf(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "POR/PDR reset flag"]
    #[inline(always)]
    pub const fn set_porrstf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Software reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrstf(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Software reset flag"]
    #[inline(always)]
    pub const fn set_sftrstf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Independent watchdog reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdgrstf(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Independent watchdog reset flag"]
    #[inline(always)]
    pub const fn set_iwdgrstf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Window watchdog reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdgrstf(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Window watchdog reset flag"]
    #[inline(always)]
    pub const fn set_wwdgrstf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr")
            .field("lsion", &self.lsion())
            .field("lsirdy", &self.lsirdy())
            .field("pvdrsten", &self.pvdrsten())
            .field("lockupen", &self.lockupen())
            .field("pvdrstf", &self.pvdrstf())
            .field("lockupf", &self.lockupf())
            .field("rmvf", &self.rmvf())
            .field("pinrstf", &self.pinrstf())
            .field("porrstf", &self.porrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csr {{ lsion: {=bool:?}, lsirdy: {=bool:?}, pvdrsten: {=bool:?}, lockupen: {=bool:?}, pvdrstf: {=bool:?}, lockupf: {=bool:?}, rmvf: {=bool:?}, pinrstf: {=bool:?}, porrstf: {=bool:?}, sftrstf: {=bool:?}, iwdgrstf: {=bool:?}, wwdgrstf: {=bool:?} }}",
            self.lsion(),
            self.lsirdy(),
            self.pvdrsten(),
            self.lockupen(),
            self.pvdrstf(),
            self.lockupf(),
            self.rmvf(),
            self.pinrstf(),
            self.porrstf(),
            self.sftrstf(),
            self.iwdgrstf(),
            self.wwdgrstf()
        )
    }
}
#[doc = "Random number register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng(pub u32);
impl Rng {
    #[doc = "RNG enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rng_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RNG enable"]
    #[inline(always)]
    pub const fn set_rng_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RNG load seed"]
    #[must_use]
    #[inline(always)]
    pub const fn rng_ldsd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RNG load seed"]
    #[inline(always)]
    pub const fn set_rng_ldsd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RNG done"]
    #[must_use]
    #[inline(always)]
    pub const fn rng_done(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RNG done"]
    #[inline(always)]
    pub const fn set_rng_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RNG data"]
    #[must_use]
    #[inline(always)]
    pub const fn rng_data(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "RNG data"]
    #[inline(always)]
    pub const fn set_rng_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Rng {
    #[inline(always)]
    fn default() -> Rng {
        Rng(0)
    }
}
impl core::fmt::Debug for Rng {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rng")
            .field("rng_en", &self.rng_en())
            .field("rng_ldsd", &self.rng_ldsd())
            .field("rng_done", &self.rng_done())
            .field("rng_data", &self.rng_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rng {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rng {{ rng_en: {=bool:?}, rng_ldsd: {=bool:?}, rng_done: {=bool:?}, rng_data: {=u16:?} }}",
            self.rng_en(),
            self.rng_ldsd(),
            self.rng_done(),
            self.rng_data()
        )
    }
}
#[doc = "System Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg(pub u32);
impl Syscfg {
    #[doc = "Whether to check if the data in Flash is 0xFF when writing Flash"]
    #[must_use]
    #[inline(always)]
    pub const fn prog_check_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Whether to check if the data in Flash is 0xFF when writing Flash"]
    #[inline(always)]
    pub const fn set_prog_check_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The size of flash page erase"]
    #[must_use]
    #[inline(always)]
    pub const fn sector_1k_cfg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "The size of flash page erase"]
    #[inline(always)]
    pub const fn set_sector_1k_cfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Syscfg {
    #[inline(always)]
    fn default() -> Syscfg {
        Syscfg(0)
    }
}
impl core::fmt::Debug for Syscfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syscfg")
            .field("prog_check_en", &self.prog_check_en())
            .field("sector_1k_cfg", &self.sector_1k_cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syscfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Syscfg {{ prog_check_en: {=bool:?}, sector_1k_cfg: {=bool:?} }}",
            self.prog_check_en(),
            self.sector_1k_cfg()
        )
    }
}
