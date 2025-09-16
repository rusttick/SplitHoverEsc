#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `MEM_MODE` reader - EXTI_Memory Remap Config"]
pub type MemModeR = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - EXTI_Memory Remap Config"]
pub type MemModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA11_RERMP` reader - PA11 DMA remap"]
pub type Pa11RermpR = crate::BitReader;
#[doc = "Field `PA11_RERMP` writer - PA11 DMA remap"]
pub type Pa11RermpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA12_RERMP` reader - PA12 DMA remap"]
pub type Pa12RermpR = crate::BitReader;
#[doc = "Field `PA12_RERMP` writer - PA12 DMA remap"]
pub type Pa12RermpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_DMA_RMP` reader - ADC DMA remap"]
pub type AdcDmaRmpR = crate::BitReader;
#[doc = "Field `ADC_DMA_RMP` writer - ADC DMA remap"]
pub type AdcDmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_TX_DMA_RMP` reader - UART1 TX DMA remap"]
pub type Uart1TxDmaRmpR = crate::BitReader;
#[doc = "Field `UART1_TX_DMA_RMP` writer - UART1 TX DMA remap"]
pub type Uart1TxDmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_RX_DMA_RMP` reader - UART1 RX DMA remap"]
pub type Uart1RxDmaRmpR = crate::BitReader;
#[doc = "Field `UART1_RX_DMA_RMP` writer - UART1 RX DMA remap"]
pub type Uart1RxDmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16_DMA_RMP` reader - Timer 16 DMA remap"]
pub type Tim16DmaRmpR = crate::BitReader;
#[doc = "Field `TIM16_DMA_RMP` writer - Timer 16 DMA remap"]
pub type Tim16DmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17_DMA_RMP` reader - Timer 17 DMA remap"]
pub type Tim17DmaRmpR = crate::BitReader;
#[doc = "Field `TIM17_DMA_RMP` writer - Timer 17 DMA remap"]
pub type Tim17DmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMCH2_DMA_RMP` reader - CSMCH2 DMA remap"]
pub type Csmch2DmaRmpR = crate::BitReader;
#[doc = "Field `CSMCH2_DMA_RMP` writer - CSMCH2 DMA remap"]
pub type Csmch2DmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMCH1_DMA_RMP` reader - CSMCH1 DMA remap"]
pub type Csmch1DmaRmpR = crate::BitReader;
#[doc = "Field `CSMCH1_DMA_RMP` writer - CSMCH1 DMA remap"]
pub type Csmch1DmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA11_PA12_RMP` reader - PA11 PA12 remap"]
pub type Pa11Pa12RmpR = crate::BitReader;
#[doc = "Field `PA11_PA12_RMP` writer - PA11 PA12 remap"]
pub type Pa11Pa12RmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - EXTI_Memory Remap Config"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MemModeR {
        MemModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - PA11 DMA remap"]
    #[inline(always)]
    pub fn pa11_rermp(&self) -> Pa11RermpR {
        Pa11RermpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PA12 DMA remap"]
    #[inline(always)]
    pub fn pa12_rermp(&self) -> Pa12RermpR {
        Pa12RermpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC DMA remap"]
    #[inline(always)]
    pub fn adc_dma_rmp(&self) -> AdcDmaRmpR {
        AdcDmaRmpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART1 TX DMA remap"]
    #[inline(always)]
    pub fn uart1_tx_dma_rmp(&self) -> Uart1TxDmaRmpR {
        Uart1TxDmaRmpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART1 RX DMA remap"]
    #[inline(always)]
    pub fn uart1_rx_dma_rmp(&self) -> Uart1RxDmaRmpR {
        Uart1RxDmaRmpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 16 DMA remap"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&self) -> Tim16DmaRmpR {
        Tim16DmaRmpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer 17 DMA remap"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&self) -> Tim17DmaRmpR {
        Tim17DmaRmpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CSMCH2 DMA remap"]
    #[inline(always)]
    pub fn csmch2_dma_rmp(&self) -> Csmch2DmaRmpR {
        Csmch2DmaRmpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CSMCH1 DMA remap"]
    #[inline(always)]
    pub fn csmch1_dma_rmp(&self) -> Csmch1DmaRmpR {
        Csmch1DmaRmpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PA11 PA12 remap"]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&self) -> Pa11Pa12RmpR {
        Pa11Pa12RmpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - EXTI_Memory Remap Config"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MemModeW<'_, CfgrSpec> {
        MemModeW::new(self, 0)
    }
    #[doc = "Bit 3 - PA11 DMA remap"]
    #[inline(always)]
    pub fn pa11_rermp(&mut self) -> Pa11RermpW<'_, CfgrSpec> {
        Pa11RermpW::new(self, 3)
    }
    #[doc = "Bit 4 - PA12 DMA remap"]
    #[inline(always)]
    pub fn pa12_rermp(&mut self) -> Pa12RermpW<'_, CfgrSpec> {
        Pa12RermpW::new(self, 4)
    }
    #[doc = "Bit 8 - ADC DMA remap"]
    #[inline(always)]
    pub fn adc_dma_rmp(&mut self) -> AdcDmaRmpW<'_, CfgrSpec> {
        AdcDmaRmpW::new(self, 8)
    }
    #[doc = "Bit 9 - UART1 TX DMA remap"]
    #[inline(always)]
    pub fn uart1_tx_dma_rmp(&mut self) -> Uart1TxDmaRmpW<'_, CfgrSpec> {
        Uart1TxDmaRmpW::new(self, 9)
    }
    #[doc = "Bit 10 - UART1 RX DMA remap"]
    #[inline(always)]
    pub fn uart1_rx_dma_rmp(&mut self) -> Uart1RxDmaRmpW<'_, CfgrSpec> {
        Uart1RxDmaRmpW::new(self, 10)
    }
    #[doc = "Bit 11 - Timer 16 DMA remap"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&mut self) -> Tim16DmaRmpW<'_, CfgrSpec> {
        Tim16DmaRmpW::new(self, 11)
    }
    #[doc = "Bit 12 - Timer 17 DMA remap"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&mut self) -> Tim17DmaRmpW<'_, CfgrSpec> {
        Tim17DmaRmpW::new(self, 12)
    }
    #[doc = "Bit 13 - CSMCH2 DMA remap"]
    #[inline(always)]
    pub fn csmch2_dma_rmp(&mut self) -> Csmch2DmaRmpW<'_, CfgrSpec> {
        Csmch2DmaRmpW::new(self, 13)
    }
    #[doc = "Bit 14 - CSMCH1 DMA remap"]
    #[inline(always)]
    pub fn csmch1_dma_rmp(&mut self) -> Csmch1DmaRmpW<'_, CfgrSpec> {
        Csmch1DmaRmpW::new(self, 14)
    }
    #[doc = "Bit 15 - PA11 PA12 remap"]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&mut self) -> Pa11Pa12RmpW<'_, CfgrSpec> {
        Pa11Pa12RmpW::new(self, 15)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
