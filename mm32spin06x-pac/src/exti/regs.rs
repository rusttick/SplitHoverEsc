#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc = "EXTI_Memory Remap Config"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "EXTI_Memory Remap Config"]
    #[inline(always)]
    pub const fn set_mem_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "PA11 DMA remap"]
    #[must_use]
    #[inline(always)]
    pub const fn pa11_rermp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PA11 DMA remap"]
    #[inline(always)]
    pub const fn set_pa11_rermp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PA12 DMA remap"]
    #[must_use]
    #[inline(always)]
    pub const fn pa12_rermp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PA12 DMA remap"]
    #[inline(always)]
    pub const fn set_pa12_rermp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ADC DMA remap"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_dma_rmp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ADC DMA remap"]
    #[inline(always)]
    pub const fn set_adc_dma_rmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "UART1 TX DMA remap"]
    #[must_use]
    #[inline(always)]
    pub const fn uart1_tx_dma_rmp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "UART1 TX DMA remap"]
    #[inline(always)]
    pub const fn set_uart1_tx_dma_rmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "UART1 RX DMA remap"]
    #[must_use]
    #[inline(always)]
    pub const fn uart1_rx_dma_rmp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "UART1 RX DMA remap"]
    #[inline(always)]
    pub const fn set_uart1_rx_dma_rmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Timer 16 DMA remap"]
    #[must_use]
    #[inline(always)]
    pub const fn tim16_dma_rmp(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 16 DMA remap"]
    #[inline(always)]
    pub const fn set_tim16_dma_rmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Timer 17 DMA remap"]
    #[must_use]
    #[inline(always)]
    pub const fn tim17_dma_rmp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timer 17 DMA remap"]
    #[inline(always)]
    pub const fn set_tim17_dma_rmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CSMCH2 DMA remap"]
    #[must_use]
    #[inline(always)]
    pub const fn csmch2_dma_rmp(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CSMCH2 DMA remap"]
    #[inline(always)]
    pub const fn set_csmch2_dma_rmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CSMCH1 DMA remap"]
    #[must_use]
    #[inline(always)]
    pub const fn csmch1_dma_rmp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CSMCH1 DMA remap"]
    #[inline(always)]
    pub const fn set_csmch1_dma_rmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PA11 PA12 remap"]
    #[must_use]
    #[inline(always)]
    pub const fn pa11_pa12_rmp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PA11 PA12 remap"]
    #[inline(always)]
    pub const fn set_pa11_pa12_rmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            .field("mem_mode", &self.mem_mode())
            .field("pa11_rermp", &self.pa11_rermp())
            .field("pa12_rermp", &self.pa12_rermp())
            .field("adc_dma_rmp", &self.adc_dma_rmp())
            .field("uart1_tx_dma_rmp", &self.uart1_tx_dma_rmp())
            .field("uart1_rx_dma_rmp", &self.uart1_rx_dma_rmp())
            .field("tim16_dma_rmp", &self.tim16_dma_rmp())
            .field("tim17_dma_rmp", &self.tim17_dma_rmp())
            .field("csmch2_dma_rmp", &self.csmch2_dma_rmp())
            .field("csmch1_dma_rmp", &self.csmch1_dma_rmp())
            .field("pa11_pa12_rmp", &self.pa11_pa12_rmp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfgr {{ mem_mode: {=u8:?}, pa11_rermp: {=bool:?}, pa12_rermp: {=bool:?}, adc_dma_rmp: {=bool:?}, uart1_tx_dma_rmp: {=bool:?}, uart1_rx_dma_rmp: {=bool:?}, tim16_dma_rmp: {=bool:?}, tim17_dma_rmp: {=bool:?}, csmch2_dma_rmp: {=bool:?}, csmch1_dma_rmp: {=bool:?}, pa11_pa12_rmp: {=bool:?} }}",
            self.mem_mode(),
            self.pa11_rermp(),
            self.pa12_rermp(),
            self.adc_dma_rmp(),
            self.uart1_tx_dma_rmp(),
            self.uart1_rx_dma_rmp(),
            self.tim16_dma_rmp(),
            self.tim17_dma_rmp(),
            self.csmch2_dma_rmp(),
            self.csmch1_dma_rmp(),
            self.pa11_pa12_rmp()
        )
    }
}
#[doc = "External interrupt configuration register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "EXTI 0 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 0 configuration"]
    #[inline(always)]
    pub const fn set_exti0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "EXTI 1 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 1 configuration"]
    #[inline(always)]
    pub const fn set_exti1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "EXTI 2 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 2 configuration"]
    #[inline(always)]
    pub const fn set_exti2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "EXTI 3 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 3 configuration"]
    #[inline(always)]
    pub const fn set_exti3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
impl core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr1")
            .field("exti0", &self.exti0())
            .field("exti1", &self.exti1())
            .field("exti2", &self.exti2())
            .field("exti3", &self.exti3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr1 {{ exti0: {=u8:?}, exti1: {=u8:?}, exti2: {=u8:?}, exti3: {=u8:?} }}",
            self.exti0(),
            self.exti1(),
            self.exti2(),
            self.exti3()
        )
    }
}
#[doc = "External interrupt configuration register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc = "EXTI 4 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 4 configuration"]
    #[inline(always)]
    pub const fn set_exti4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "EXTI 5 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti5(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 5 configuration"]
    #[inline(always)]
    pub const fn set_exti5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "EXTI 6 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 6 configuration"]
    #[inline(always)]
    pub const fn set_exti6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "EXTI 7 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti7(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 7 configuration"]
    #[inline(always)]
    pub const fn set_exti7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        Cr2(0)
    }
}
impl core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr2")
            .field("exti4", &self.exti4())
            .field("exti5", &self.exti5())
            .field("exti6", &self.exti6())
            .field("exti7", &self.exti7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr2 {{ exti4: {=u8:?}, exti5: {=u8:?}, exti6: {=u8:?}, exti7: {=u8:?} }}",
            self.exti4(),
            self.exti5(),
            self.exti6(),
            self.exti7()
        )
    }
}
#[doc = "External interrupt configuration register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr3(pub u32);
impl Cr3 {
    #[doc = "EXTI 8 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 8 configuration"]
    #[inline(always)]
    pub const fn set_exti8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "EXTI 9 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 9 configuration"]
    #[inline(always)]
    pub const fn set_exti9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "EXTI 10 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 10 configuration"]
    #[inline(always)]
    pub const fn set_exti10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "EXTI 11 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 11 configuration"]
    #[inline(always)]
    pub const fn set_exti11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Cr3 {
    #[inline(always)]
    fn default() -> Cr3 {
        Cr3(0)
    }
}
impl core::fmt::Debug for Cr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr3")
            .field("exti8", &self.exti8())
            .field("exti9", &self.exti9())
            .field("exti10", &self.exti10())
            .field("exti11", &self.exti11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr3 {{ exti8: {=u8:?}, exti9: {=u8:?}, exti10: {=u8:?}, exti11: {=u8:?} }}",
            self.exti8(),
            self.exti9(),
            self.exti10(),
            self.exti11()
        )
    }
}
#[doc = "External interrupt configuration register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr4(pub u32);
impl Cr4 {
    #[doc = "EXTI 12 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 12 configuration"]
    #[inline(always)]
    pub const fn set_exti12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "EXTI 13 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti13(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 13 configuration"]
    #[inline(always)]
    pub const fn set_exti13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "EXTI 14 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti14(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 14 configuration"]
    #[inline(always)]
    pub const fn set_exti14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "EXTI 15 configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn exti15(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "EXTI 15 configuration"]
    #[inline(always)]
    pub const fn set_exti15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Cr4 {
    #[inline(always)]
    fn default() -> Cr4 {
        Cr4(0)
    }
}
impl core::fmt::Debug for Cr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr4")
            .field("exti12", &self.exti12())
            .field("exti13", &self.exti13())
            .field("exti14", &self.exti14())
            .field("exti15", &self.exti15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr4 {{ exti12: {=u8:?}, exti13: {=u8:?}, exti14: {=u8:?}, exti15: {=u8:?} }}",
            self.exti12(),
            self.exti13(),
            self.exti14(),
            self.exti15()
        )
    }
}
#[doc = "Event mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emr(pub u32);
impl Emr {
    #[doc = "Event Mask on line 0"]
    #[must_use]
    #[inline(always)]
    pub const fn emr0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 0"]
    #[inline(always)]
    pub const fn set_emr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Event Mask on line 1"]
    #[must_use]
    #[inline(always)]
    pub const fn emr1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 1"]
    #[inline(always)]
    pub const fn set_emr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event Mask on line 2"]
    #[must_use]
    #[inline(always)]
    pub const fn emr2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 2"]
    #[inline(always)]
    pub const fn set_emr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Event Mask on line 3"]
    #[must_use]
    #[inline(always)]
    pub const fn emr3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 3"]
    #[inline(always)]
    pub const fn set_emr3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Event Mask on line 4"]
    #[must_use]
    #[inline(always)]
    pub const fn emr4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 4"]
    #[inline(always)]
    pub const fn set_emr4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Event Mask on line 5"]
    #[must_use]
    #[inline(always)]
    pub const fn emr5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 5"]
    #[inline(always)]
    pub const fn set_emr5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Event Mask on line 6"]
    #[must_use]
    #[inline(always)]
    pub const fn emr6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 6"]
    #[inline(always)]
    pub const fn set_emr6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Event Mask on line 7"]
    #[must_use]
    #[inline(always)]
    pub const fn emr7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 7"]
    #[inline(always)]
    pub const fn set_emr7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Event Mask on line 8"]
    #[must_use]
    #[inline(always)]
    pub const fn emr8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 8"]
    #[inline(always)]
    pub const fn set_emr8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Event Mask on line 9"]
    #[must_use]
    #[inline(always)]
    pub const fn emr9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 9"]
    #[inline(always)]
    pub const fn set_emr9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Event Mask on line 10"]
    #[must_use]
    #[inline(always)]
    pub const fn emr10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 10"]
    #[inline(always)]
    pub const fn set_emr10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Event Mask on line 11"]
    #[must_use]
    #[inline(always)]
    pub const fn emr11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 11"]
    #[inline(always)]
    pub const fn set_emr11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Event Mask on line 12"]
    #[must_use]
    #[inline(always)]
    pub const fn emr12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 12"]
    #[inline(always)]
    pub const fn set_emr12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Event Mask on line 13"]
    #[must_use]
    #[inline(always)]
    pub const fn emr13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 13"]
    #[inline(always)]
    pub const fn set_emr13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Event Mask on line 14"]
    #[must_use]
    #[inline(always)]
    pub const fn emr14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 14"]
    #[inline(always)]
    pub const fn set_emr14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Event Mask on line 15"]
    #[must_use]
    #[inline(always)]
    pub const fn emr15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 15"]
    #[inline(always)]
    pub const fn set_emr15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Event Mask on line 16"]
    #[must_use]
    #[inline(always)]
    pub const fn emr16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 16"]
    #[inline(always)]
    pub const fn set_emr16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Event Mask on line 17"]
    #[must_use]
    #[inline(always)]
    pub const fn emr17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 17"]
    #[inline(always)]
    pub const fn set_emr17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Event Mask on line 18"]
    #[must_use]
    #[inline(always)]
    pub const fn emr18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 18"]
    #[inline(always)]
    pub const fn set_emr18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Event Mask on line 19"]
    #[must_use]
    #[inline(always)]
    pub const fn emr19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 19"]
    #[inline(always)]
    pub const fn set_emr19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Event Mask on line 20"]
    #[must_use]
    #[inline(always)]
    pub const fn emr20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 20"]
    #[inline(always)]
    pub const fn set_emr20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Event Mask on line 24"]
    #[must_use]
    #[inline(always)]
    pub const fn emr24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 24"]
    #[inline(always)]
    pub const fn set_emr24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Emr {
    #[inline(always)]
    fn default() -> Emr {
        Emr(0)
    }
}
impl core::fmt::Debug for Emr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emr")
            .field("emr0", &self.emr0())
            .field("emr1", &self.emr1())
            .field("emr2", &self.emr2())
            .field("emr3", &self.emr3())
            .field("emr4", &self.emr4())
            .field("emr5", &self.emr5())
            .field("emr6", &self.emr6())
            .field("emr7", &self.emr7())
            .field("emr8", &self.emr8())
            .field("emr9", &self.emr9())
            .field("emr10", &self.emr10())
            .field("emr11", &self.emr11())
            .field("emr12", &self.emr12())
            .field("emr13", &self.emr13())
            .field("emr14", &self.emr14())
            .field("emr15", &self.emr15())
            .field("emr16", &self.emr16())
            .field("emr17", &self.emr17())
            .field("emr18", &self.emr18())
            .field("emr19", &self.emr19())
            .field("emr20", &self.emr20())
            .field("emr24", &self.emr24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Emr {{ emr0: {=bool:?}, emr1: {=bool:?}, emr2: {=bool:?}, emr3: {=bool:?}, emr4: {=bool:?}, emr5: {=bool:?}, emr6: {=bool:?}, emr7: {=bool:?}, emr8: {=bool:?}, emr9: {=bool:?}, emr10: {=bool:?}, emr11: {=bool:?}, emr12: {=bool:?}, emr13: {=bool:?}, emr14: {=bool:?}, emr15: {=bool:?}, emr16: {=bool:?}, emr17: {=bool:?}, emr18: {=bool:?}, emr19: {=bool:?}, emr20: {=bool:?}, emr24: {=bool:?} }}",
            self.emr0(),
            self.emr1(),
            self.emr2(),
            self.emr3(),
            self.emr4(),
            self.emr5(),
            self.emr6(),
            self.emr7(),
            self.emr8(),
            self.emr9(),
            self.emr10(),
            self.emr11(),
            self.emr12(),
            self.emr13(),
            self.emr14(),
            self.emr15(),
            self.emr16(),
            self.emr17(),
            self.emr18(),
            self.emr19(),
            self.emr20(),
            self.emr24()
        )
    }
}
#[doc = "Falling trigger selection register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftsr(pub u32);
impl Ftsr {
    #[doc = "Falling trigger event configuration bit of line 0"]
    #[must_use]
    #[inline(always)]
    pub const fn tr0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 0"]
    #[inline(always)]
    pub const fn set_tr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Falling trigger event configuration bit of line 1"]
    #[must_use]
    #[inline(always)]
    pub const fn tr1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 1"]
    #[inline(always)]
    pub const fn set_tr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Falling trigger event configuration bit of line 2"]
    #[must_use]
    #[inline(always)]
    pub const fn tr2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 2"]
    #[inline(always)]
    pub const fn set_tr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Falling trigger event configuration bit of line 3"]
    #[must_use]
    #[inline(always)]
    pub const fn tr3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 3"]
    #[inline(always)]
    pub const fn set_tr3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Falling trigger event configuration bit of line 4"]
    #[must_use]
    #[inline(always)]
    pub const fn tr4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 4"]
    #[inline(always)]
    pub const fn set_tr4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Falling trigger event configuration bit of line 5"]
    #[must_use]
    #[inline(always)]
    pub const fn tr5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 5"]
    #[inline(always)]
    pub const fn set_tr5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Falling trigger event configuration bit of line 6"]
    #[must_use]
    #[inline(always)]
    pub const fn tr6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 6"]
    #[inline(always)]
    pub const fn set_tr6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Falling trigger event configuration bit of line 7"]
    #[must_use]
    #[inline(always)]
    pub const fn tr7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 7"]
    #[inline(always)]
    pub const fn set_tr7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Falling trigger event configuration bit of line 8"]
    #[must_use]
    #[inline(always)]
    pub const fn tr8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 8"]
    #[inline(always)]
    pub const fn set_tr8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Falling trigger event configuration bit of line 9"]
    #[must_use]
    #[inline(always)]
    pub const fn tr9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 9"]
    #[inline(always)]
    pub const fn set_tr9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Falling trigger event configuration bit of line 10"]
    #[must_use]
    #[inline(always)]
    pub const fn tr10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 10"]
    #[inline(always)]
    pub const fn set_tr10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Falling trigger event configuration bit of line 11"]
    #[must_use]
    #[inline(always)]
    pub const fn tr11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 11"]
    #[inline(always)]
    pub const fn set_tr11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Falling trigger event configuration bit of line 12"]
    #[must_use]
    #[inline(always)]
    pub const fn tr12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 12"]
    #[inline(always)]
    pub const fn set_tr12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Falling trigger event configuration bit of line 13"]
    #[must_use]
    #[inline(always)]
    pub const fn tr13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 13"]
    #[inline(always)]
    pub const fn set_tr13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Falling trigger event configuration bit of line 14"]
    #[must_use]
    #[inline(always)]
    pub const fn tr14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 14"]
    #[inline(always)]
    pub const fn set_tr14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Falling trigger event configuration bit of line 15"]
    #[must_use]
    #[inline(always)]
    pub const fn tr15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 15"]
    #[inline(always)]
    pub const fn set_tr15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Falling trigger event configuration bit of line 16"]
    #[must_use]
    #[inline(always)]
    pub const fn tr16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 16"]
    #[inline(always)]
    pub const fn set_tr16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Falling trigger event configuration bit of line 17"]
    #[must_use]
    #[inline(always)]
    pub const fn tr17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 17"]
    #[inline(always)]
    pub const fn set_tr17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Falling trigger event configuration bit of line 18"]
    #[must_use]
    #[inline(always)]
    pub const fn tr18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 18"]
    #[inline(always)]
    pub const fn set_tr18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Falling trigger event configuration bit of line 19"]
    #[must_use]
    #[inline(always)]
    pub const fn tr19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 19"]
    #[inline(always)]
    pub const fn set_tr19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Falling trigger event configuration bit of line 20"]
    #[must_use]
    #[inline(always)]
    pub const fn tr20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 20"]
    #[inline(always)]
    pub const fn set_tr20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Falling trigger event configuration bit of line 24"]
    #[must_use]
    #[inline(always)]
    pub const fn tr24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 24"]
    #[inline(always)]
    pub const fn set_tr24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Ftsr {
    #[inline(always)]
    fn default() -> Ftsr {
        Ftsr(0)
    }
}
impl core::fmt::Debug for Ftsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftsr")
            .field("tr0", &self.tr0())
            .field("tr1", &self.tr1())
            .field("tr2", &self.tr2())
            .field("tr3", &self.tr3())
            .field("tr4", &self.tr4())
            .field("tr5", &self.tr5())
            .field("tr6", &self.tr6())
            .field("tr7", &self.tr7())
            .field("tr8", &self.tr8())
            .field("tr9", &self.tr9())
            .field("tr10", &self.tr10())
            .field("tr11", &self.tr11())
            .field("tr12", &self.tr12())
            .field("tr13", &self.tr13())
            .field("tr14", &self.tr14())
            .field("tr15", &self.tr15())
            .field("tr16", &self.tr16())
            .field("tr17", &self.tr17())
            .field("tr18", &self.tr18())
            .field("tr19", &self.tr19())
            .field("tr20", &self.tr20())
            .field("tr24", &self.tr24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ftsr {{ tr0: {=bool:?}, tr1: {=bool:?}, tr2: {=bool:?}, tr3: {=bool:?}, tr4: {=bool:?}, tr5: {=bool:?}, tr6: {=bool:?}, tr7: {=bool:?}, tr8: {=bool:?}, tr9: {=bool:?}, tr10: {=bool:?}, tr11: {=bool:?}, tr12: {=bool:?}, tr13: {=bool:?}, tr14: {=bool:?}, tr15: {=bool:?}, tr16: {=bool:?}, tr17: {=bool:?}, tr18: {=bool:?}, tr19: {=bool:?}, tr20: {=bool:?}, tr24: {=bool:?} }}",
            self.tr0(),
            self.tr1(),
            self.tr2(),
            self.tr3(),
            self.tr4(),
            self.tr5(),
            self.tr6(),
            self.tr7(),
            self.tr8(),
            self.tr9(),
            self.tr10(),
            self.tr11(),
            self.tr12(),
            self.tr13(),
            self.tr14(),
            self.tr15(),
            self.tr16(),
            self.tr17(),
            self.tr18(),
            self.tr19(),
            self.tr20(),
            self.tr24()
        )
    }
}
#[doc = "Interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc = "Interrupt Mask on line 0"]
    #[must_use]
    #[inline(always)]
    pub const fn imr0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 0"]
    #[inline(always)]
    pub const fn set_imr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Mask on line 1"]
    #[must_use]
    #[inline(always)]
    pub const fn imr1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 1"]
    #[inline(always)]
    pub const fn set_imr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Mask on line 2"]
    #[must_use]
    #[inline(always)]
    pub const fn imr2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 2"]
    #[inline(always)]
    pub const fn set_imr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Mask on line 3"]
    #[must_use]
    #[inline(always)]
    pub const fn imr3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 3"]
    #[inline(always)]
    pub const fn set_imr3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Mask on line 4"]
    #[must_use]
    #[inline(always)]
    pub const fn imr4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 4"]
    #[inline(always)]
    pub const fn set_imr4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Mask on line 5"]
    #[must_use]
    #[inline(always)]
    pub const fn imr5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 5"]
    #[inline(always)]
    pub const fn set_imr5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Mask on line 6"]
    #[must_use]
    #[inline(always)]
    pub const fn imr6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 6"]
    #[inline(always)]
    pub const fn set_imr6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Mask on line 7"]
    #[must_use]
    #[inline(always)]
    pub const fn imr7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 7"]
    #[inline(always)]
    pub const fn set_imr7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt Mask on line 8"]
    #[must_use]
    #[inline(always)]
    pub const fn imr8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 8"]
    #[inline(always)]
    pub const fn set_imr8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Mask on line 9"]
    #[must_use]
    #[inline(always)]
    pub const fn imr9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 9"]
    #[inline(always)]
    pub const fn set_imr9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Mask on line 10"]
    #[must_use]
    #[inline(always)]
    pub const fn imr10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 10"]
    #[inline(always)]
    pub const fn set_imr10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt Mask on line 11"]
    #[must_use]
    #[inline(always)]
    pub const fn imr11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 11"]
    #[inline(always)]
    pub const fn set_imr11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt Mask on line 12"]
    #[must_use]
    #[inline(always)]
    pub const fn imr12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 12"]
    #[inline(always)]
    pub const fn set_imr12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Mask on line 13"]
    #[must_use]
    #[inline(always)]
    pub const fn imr13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 13"]
    #[inline(always)]
    pub const fn set_imr13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt Mask on line 14"]
    #[must_use]
    #[inline(always)]
    pub const fn imr14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 14"]
    #[inline(always)]
    pub const fn set_imr14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt Mask on line 15"]
    #[must_use]
    #[inline(always)]
    pub const fn imr15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 15"]
    #[inline(always)]
    pub const fn set_imr15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Mask on line 16"]
    #[must_use]
    #[inline(always)]
    pub const fn imr16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 16"]
    #[inline(always)]
    pub const fn set_imr16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt Mask on line 17"]
    #[must_use]
    #[inline(always)]
    pub const fn imr17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 17"]
    #[inline(always)]
    pub const fn set_imr17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt Mask on line 18"]
    #[must_use]
    #[inline(always)]
    pub const fn imr18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 18"]
    #[inline(always)]
    pub const fn set_imr18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt Mask on line 19"]
    #[must_use]
    #[inline(always)]
    pub const fn imr19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 19"]
    #[inline(always)]
    pub const fn set_imr19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt Mask on line 20"]
    #[must_use]
    #[inline(always)]
    pub const fn imr20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 20"]
    #[inline(always)]
    pub const fn set_imr20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt Mask on line 24"]
    #[must_use]
    #[inline(always)]
    pub const fn imr24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 24"]
    #[inline(always)]
    pub const fn set_imr24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Imr {
    #[inline(always)]
    fn default() -> Imr {
        Imr(0)
    }
}
impl core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr")
            .field("imr0", &self.imr0())
            .field("imr1", &self.imr1())
            .field("imr2", &self.imr2())
            .field("imr3", &self.imr3())
            .field("imr4", &self.imr4())
            .field("imr5", &self.imr5())
            .field("imr6", &self.imr6())
            .field("imr7", &self.imr7())
            .field("imr8", &self.imr8())
            .field("imr9", &self.imr9())
            .field("imr10", &self.imr10())
            .field("imr11", &self.imr11())
            .field("imr12", &self.imr12())
            .field("imr13", &self.imr13())
            .field("imr14", &self.imr14())
            .field("imr15", &self.imr15())
            .field("imr16", &self.imr16())
            .field("imr17", &self.imr17())
            .field("imr18", &self.imr18())
            .field("imr19", &self.imr19())
            .field("imr20", &self.imr20())
            .field("imr24", &self.imr24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Imr {{ imr0: {=bool:?}, imr1: {=bool:?}, imr2: {=bool:?}, imr3: {=bool:?}, imr4: {=bool:?}, imr5: {=bool:?}, imr6: {=bool:?}, imr7: {=bool:?}, imr8: {=bool:?}, imr9: {=bool:?}, imr10: {=bool:?}, imr11: {=bool:?}, imr12: {=bool:?}, imr13: {=bool:?}, imr14: {=bool:?}, imr15: {=bool:?}, imr16: {=bool:?}, imr17: {=bool:?}, imr18: {=bool:?}, imr19: {=bool:?}, imr20: {=bool:?}, imr24: {=bool:?} }}",
            self.imr0(),
            self.imr1(),
            self.imr2(),
            self.imr3(),
            self.imr4(),
            self.imr5(),
            self.imr6(),
            self.imr7(),
            self.imr8(),
            self.imr9(),
            self.imr10(),
            self.imr11(),
            self.imr12(),
            self.imr13(),
            self.imr14(),
            self.imr15(),
            self.imr16(),
            self.imr17(),
            self.imr18(),
            self.imr19(),
            self.imr20(),
            self.imr24()
        )
    }
}
#[doc = "PAD configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padhys(pub u32);
impl Padhys {
    #[doc = "I2C1 mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn i2c1_mode_sel(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "I2C1 mode selection"]
    #[inline(always)]
    pub const fn set_i2c1_mode_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Padhys {
    #[inline(always)]
    fn default() -> Padhys {
        Padhys(0)
    }
}
impl core::fmt::Debug for Padhys {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padhys")
            .field("i2c1_mode_sel", &self.i2c1_mode_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padhys {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Padhys {{ i2c1_mode_sel: {=bool:?} }}",
            self.i2c1_mode_sel()
        )
    }
}
#[doc = "Pending register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pr(pub u32);
impl Pr {
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pending bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pr24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit"]
    #[inline(always)]
    pub const fn set_pr24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Pr {
    #[inline(always)]
    fn default() -> Pr {
        Pr(0)
    }
}
impl core::fmt::Debug for Pr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pr")
            .field("pr0", &self.pr0())
            .field("pr1", &self.pr1())
            .field("pr2", &self.pr2())
            .field("pr3", &self.pr3())
            .field("pr4", &self.pr4())
            .field("pr5", &self.pr5())
            .field("pr6", &self.pr6())
            .field("pr7", &self.pr7())
            .field("pr8", &self.pr8())
            .field("pr9", &self.pr9())
            .field("pr10", &self.pr10())
            .field("pr11", &self.pr11())
            .field("pr12", &self.pr12())
            .field("pr13", &self.pr13())
            .field("pr14", &self.pr14())
            .field("pr15", &self.pr15())
            .field("pr16", &self.pr16())
            .field("pr17", &self.pr17())
            .field("pr18", &self.pr18())
            .field("pr19", &self.pr19())
            .field("pr20", &self.pr20())
            .field("pr24", &self.pr24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pr {{ pr0: {=bool:?}, pr1: {=bool:?}, pr2: {=bool:?}, pr3: {=bool:?}, pr4: {=bool:?}, pr5: {=bool:?}, pr6: {=bool:?}, pr7: {=bool:?}, pr8: {=bool:?}, pr9: {=bool:?}, pr10: {=bool:?}, pr11: {=bool:?}, pr12: {=bool:?}, pr13: {=bool:?}, pr14: {=bool:?}, pr15: {=bool:?}, pr16: {=bool:?}, pr17: {=bool:?}, pr18: {=bool:?}, pr19: {=bool:?}, pr20: {=bool:?}, pr24: {=bool:?} }}",
            self.pr0(),
            self.pr1(),
            self.pr2(),
            self.pr3(),
            self.pr4(),
            self.pr5(),
            self.pr6(),
            self.pr7(),
            self.pr8(),
            self.pr9(),
            self.pr10(),
            self.pr11(),
            self.pr12(),
            self.pr13(),
            self.pr14(),
            self.pr15(),
            self.pr16(),
            self.pr17(),
            self.pr18(),
            self.pr19(),
            self.pr20(),
            self.pr24()
        )
    }
}
#[doc = "Rising trigger selection register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtsr(pub u32);
impl Rtsr {
    #[doc = "Rising trigger event configuration bit of line 0"]
    #[must_use]
    #[inline(always)]
    pub const fn tr0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 0"]
    #[inline(always)]
    pub const fn set_tr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rising trigger event configuration bit of line 1"]
    #[must_use]
    #[inline(always)]
    pub const fn tr1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 1"]
    #[inline(always)]
    pub const fn set_tr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rising trigger event configuration bit of line 2"]
    #[must_use]
    #[inline(always)]
    pub const fn tr2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 2"]
    #[inline(always)]
    pub const fn set_tr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rising trigger event configuration bit of line 3"]
    #[must_use]
    #[inline(always)]
    pub const fn tr3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 3"]
    #[inline(always)]
    pub const fn set_tr3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rising trigger event configuration bit of line 4"]
    #[must_use]
    #[inline(always)]
    pub const fn tr4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 4"]
    #[inline(always)]
    pub const fn set_tr4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rising trigger event configuration bit of line 5"]
    #[must_use]
    #[inline(always)]
    pub const fn tr5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 5"]
    #[inline(always)]
    pub const fn set_tr5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rising trigger event configuration bit of line 6"]
    #[must_use]
    #[inline(always)]
    pub const fn tr6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 6"]
    #[inline(always)]
    pub const fn set_tr6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rising trigger event configuration bit of line 7"]
    #[must_use]
    #[inline(always)]
    pub const fn tr7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 7"]
    #[inline(always)]
    pub const fn set_tr7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Rising trigger event configuration bit of line 8"]
    #[must_use]
    #[inline(always)]
    pub const fn tr8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 8"]
    #[inline(always)]
    pub const fn set_tr8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Rising trigger event configuration bit of line 9"]
    #[must_use]
    #[inline(always)]
    pub const fn tr9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 9"]
    #[inline(always)]
    pub const fn set_tr9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Rising trigger event configuration bit of line 10"]
    #[must_use]
    #[inline(always)]
    pub const fn tr10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 10"]
    #[inline(always)]
    pub const fn set_tr10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Rising trigger event configuration bit of line 11"]
    #[must_use]
    #[inline(always)]
    pub const fn tr11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 11"]
    #[inline(always)]
    pub const fn set_tr11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Rising trigger event configuration bit of line 12"]
    #[must_use]
    #[inline(always)]
    pub const fn tr12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 12"]
    #[inline(always)]
    pub const fn set_tr12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Rising trigger event configuration bit of line 13"]
    #[must_use]
    #[inline(always)]
    pub const fn tr13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 13"]
    #[inline(always)]
    pub const fn set_tr13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Rising trigger event configuration bit of line 14"]
    #[must_use]
    #[inline(always)]
    pub const fn tr14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 14"]
    #[inline(always)]
    pub const fn set_tr14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Rising trigger event configuration bit of line 15"]
    #[must_use]
    #[inline(always)]
    pub const fn tr15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 15"]
    #[inline(always)]
    pub const fn set_tr15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Rising trigger event configuration bit of line 16"]
    #[must_use]
    #[inline(always)]
    pub const fn tr16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 16"]
    #[inline(always)]
    pub const fn set_tr16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Rising trigger event configuration bit of line 17"]
    #[must_use]
    #[inline(always)]
    pub const fn tr17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 17"]
    #[inline(always)]
    pub const fn set_tr17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Rising trigger event configuration bit of line 18"]
    #[must_use]
    #[inline(always)]
    pub const fn tr18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 18"]
    #[inline(always)]
    pub const fn set_tr18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Rising trigger event configuration bit of line 19"]
    #[must_use]
    #[inline(always)]
    pub const fn tr19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 19"]
    #[inline(always)]
    pub const fn set_tr19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Rising trigger event configuration bit of line 20"]
    #[must_use]
    #[inline(always)]
    pub const fn tr20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 20"]
    #[inline(always)]
    pub const fn set_tr20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Rising trigger event configuration bit of line 24"]
    #[must_use]
    #[inline(always)]
    pub const fn tr24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 24"]
    #[inline(always)]
    pub const fn set_tr24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Rtsr {
    #[inline(always)]
    fn default() -> Rtsr {
        Rtsr(0)
    }
}
impl core::fmt::Debug for Rtsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rtsr")
            .field("tr0", &self.tr0())
            .field("tr1", &self.tr1())
            .field("tr2", &self.tr2())
            .field("tr3", &self.tr3())
            .field("tr4", &self.tr4())
            .field("tr5", &self.tr5())
            .field("tr6", &self.tr6())
            .field("tr7", &self.tr7())
            .field("tr8", &self.tr8())
            .field("tr9", &self.tr9())
            .field("tr10", &self.tr10())
            .field("tr11", &self.tr11())
            .field("tr12", &self.tr12())
            .field("tr13", &self.tr13())
            .field("tr14", &self.tr14())
            .field("tr15", &self.tr15())
            .field("tr16", &self.tr16())
            .field("tr17", &self.tr17())
            .field("tr18", &self.tr18())
            .field("tr19", &self.tr19())
            .field("tr20", &self.tr20())
            .field("tr24", &self.tr24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rtsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rtsr {{ tr0: {=bool:?}, tr1: {=bool:?}, tr2: {=bool:?}, tr3: {=bool:?}, tr4: {=bool:?}, tr5: {=bool:?}, tr6: {=bool:?}, tr7: {=bool:?}, tr8: {=bool:?}, tr9: {=bool:?}, tr10: {=bool:?}, tr11: {=bool:?}, tr12: {=bool:?}, tr13: {=bool:?}, tr14: {=bool:?}, tr15: {=bool:?}, tr16: {=bool:?}, tr17: {=bool:?}, tr18: {=bool:?}, tr19: {=bool:?}, tr20: {=bool:?}, tr24: {=bool:?} }}",
            self.tr0(),
            self.tr1(),
            self.tr2(),
            self.tr3(),
            self.tr4(),
            self.tr5(),
            self.tr6(),
            self.tr7(),
            self.tr8(),
            self.tr9(),
            self.tr10(),
            self.tr11(),
            self.tr12(),
            self.tr13(),
            self.tr14(),
            self.tr15(),
            self.tr16(),
            self.tr17(),
            self.tr18(),
            self.tr19(),
            self.tr20(),
            self.tr24()
        )
    }
}
#[doc = "Software interrupt event register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swier(pub u32);
impl Swier {
    #[doc = "Software interrupt on line 0"]
    #[must_use]
    #[inline(always)]
    pub const fn swier0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 0"]
    #[inline(always)]
    pub const fn set_swier0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software interrupt on line 1"]
    #[must_use]
    #[inline(always)]
    pub const fn swier1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 1"]
    #[inline(always)]
    pub const fn set_swier1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Software interrupt on line 2"]
    #[must_use]
    #[inline(always)]
    pub const fn swier2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 2"]
    #[inline(always)]
    pub const fn set_swier2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Software interrupt on line 3"]
    #[must_use]
    #[inline(always)]
    pub const fn swier3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 3"]
    #[inline(always)]
    pub const fn set_swier3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Software interrupt on line 4"]
    #[must_use]
    #[inline(always)]
    pub const fn swier4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 4"]
    #[inline(always)]
    pub const fn set_swier4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Software interrupt on line 5"]
    #[must_use]
    #[inline(always)]
    pub const fn swier5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 5"]
    #[inline(always)]
    pub const fn set_swier5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Software interrupt on line 6"]
    #[must_use]
    #[inline(always)]
    pub const fn swier6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 6"]
    #[inline(always)]
    pub const fn set_swier6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Software interrupt on line 7"]
    #[must_use]
    #[inline(always)]
    pub const fn swier7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 7"]
    #[inline(always)]
    pub const fn set_swier7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Software interrupt on line 8"]
    #[must_use]
    #[inline(always)]
    pub const fn swier8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 8"]
    #[inline(always)]
    pub const fn set_swier8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Software interrupt on line 9"]
    #[must_use]
    #[inline(always)]
    pub const fn swier9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 9"]
    #[inline(always)]
    pub const fn set_swier9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Software interrupt on line 10"]
    #[must_use]
    #[inline(always)]
    pub const fn swier10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 10"]
    #[inline(always)]
    pub const fn set_swier10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Software interrupt on line 11"]
    #[must_use]
    #[inline(always)]
    pub const fn swier11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 11"]
    #[inline(always)]
    pub const fn set_swier11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Software interrupt on line 12"]
    #[must_use]
    #[inline(always)]
    pub const fn swier12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 12"]
    #[inline(always)]
    pub const fn set_swier12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Software interrupt on line 13"]
    #[must_use]
    #[inline(always)]
    pub const fn swier13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 13"]
    #[inline(always)]
    pub const fn set_swier13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software interrupt on line 14"]
    #[must_use]
    #[inline(always)]
    pub const fn swier14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 14"]
    #[inline(always)]
    pub const fn set_swier14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Software interrupt on line 15"]
    #[must_use]
    #[inline(always)]
    pub const fn swier15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 15"]
    #[inline(always)]
    pub const fn set_swier15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Software interrupt on line 16"]
    #[must_use]
    #[inline(always)]
    pub const fn swier16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 16"]
    #[inline(always)]
    pub const fn set_swier16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Software interrupt on line 17"]
    #[must_use]
    #[inline(always)]
    pub const fn swier17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 17"]
    #[inline(always)]
    pub const fn set_swier17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Software interrupt on line 18"]
    #[must_use]
    #[inline(always)]
    pub const fn swier18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 18"]
    #[inline(always)]
    pub const fn set_swier18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Software interrupt on line 19"]
    #[must_use]
    #[inline(always)]
    pub const fn swier19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 19"]
    #[inline(always)]
    pub const fn set_swier19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Software interrupt on line 20"]
    #[must_use]
    #[inline(always)]
    pub const fn swier20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 20"]
    #[inline(always)]
    pub const fn set_swier20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Software interrupt on line 24"]
    #[must_use]
    #[inline(always)]
    pub const fn swier24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 24"]
    #[inline(always)]
    pub const fn set_swier24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Swier {
    #[inline(always)]
    fn default() -> Swier {
        Swier(0)
    }
}
impl core::fmt::Debug for Swier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swier")
            .field("swier0", &self.swier0())
            .field("swier1", &self.swier1())
            .field("swier2", &self.swier2())
            .field("swier3", &self.swier3())
            .field("swier4", &self.swier4())
            .field("swier5", &self.swier5())
            .field("swier6", &self.swier6())
            .field("swier7", &self.swier7())
            .field("swier8", &self.swier8())
            .field("swier9", &self.swier9())
            .field("swier10", &self.swier10())
            .field("swier11", &self.swier11())
            .field("swier12", &self.swier12())
            .field("swier13", &self.swier13())
            .field("swier14", &self.swier14())
            .field("swier15", &self.swier15())
            .field("swier16", &self.swier16())
            .field("swier17", &self.swier17())
            .field("swier18", &self.swier18())
            .field("swier19", &self.swier19())
            .field("swier20", &self.swier20())
            .field("swier24", &self.swier24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swier {{ swier0: {=bool:?}, swier1: {=bool:?}, swier2: {=bool:?}, swier3: {=bool:?}, swier4: {=bool:?}, swier5: {=bool:?}, swier6: {=bool:?}, swier7: {=bool:?}, swier8: {=bool:?}, swier9: {=bool:?}, swier10: {=bool:?}, swier11: {=bool:?}, swier12: {=bool:?}, swier13: {=bool:?}, swier14: {=bool:?}, swier15: {=bool:?}, swier16: {=bool:?}, swier17: {=bool:?}, swier18: {=bool:?}, swier19: {=bool:?}, swier20: {=bool:?}, swier24: {=bool:?} }}",
            self.swier0(),
            self.swier1(),
            self.swier2(),
            self.swier3(),
            self.swier4(),
            self.swier5(),
            self.swier6(),
            self.swier7(),
            self.swier8(),
            self.swier9(),
            self.swier10(),
            self.swier11(),
            self.swier12(),
            self.swier13(),
            self.swier14(),
            self.swier15(),
            self.swier16(),
            self.swier17(),
            self.swier18(),
            self.swier19(),
            self.swier20(),
            self.swier24()
        )
    }
}
