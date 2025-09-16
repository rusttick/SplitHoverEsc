#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `MEM_MODE` reader - MEM_MODE"]
pub type MemModeR = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - MEM_MODE"]
pub type MemModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_DMA` reader - ADC_DMA_RMP"]
pub type AdcDmaR = crate::BitReader;
#[doc = "Field `ADC_DMA` writer - ADC_DMA_RMP"]
pub type AdcDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_TX_DMA` reader - UART1_TX_DMA_RMP"]
pub type Uart1TxDmaR = crate::BitReader;
#[doc = "Field `UART1_TX_DMA` writer - UART1_TX_DMA_RMP"]
pub type Uart1TxDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_RX_DMA` reader - UART1_RX_DMA_RMP"]
pub type Uart1RxDmaR = crate::BitReader;
#[doc = "Field `UART1_RX_DMA` writer - UART1_RX_DMA_RMP"]
pub type Uart1RxDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16_DMA` reader - TIM16_DMA_RMP"]
pub type Tim16DmaR = crate::BitReader;
#[doc = "Field `TIM16_DMA` writer - TIM16_DMA_RMP"]
pub type Tim16DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17_DMA` reader - TIM17_DMA_RMP"]
pub type Tim17DmaR = crate::BitReader;
#[doc = "Field `TIM17_DMA` writer - TIM17_DMA_RMP"]
pub type Tim17DmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - MEM_MODE"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MemModeR {
        MemModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - ADC_DMA_RMP"]
    #[inline(always)]
    pub fn adc_dma(&self) -> AdcDmaR {
        AdcDmaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART1_TX_DMA_RMP"]
    #[inline(always)]
    pub fn uart1_tx_dma(&self) -> Uart1TxDmaR {
        Uart1TxDmaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART1_RX_DMA_RMP"]
    #[inline(always)]
    pub fn uart1_rx_dma(&self) -> Uart1RxDmaR {
        Uart1RxDmaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM16_DMA_RMP"]
    #[inline(always)]
    pub fn tim16_dma(&self) -> Tim16DmaR {
        Tim16DmaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIM17_DMA_RMP"]
    #[inline(always)]
    pub fn tim17_dma(&self) -> Tim17DmaR {
        Tim17DmaR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MEM_MODE"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MemModeW<'_, CfgrSpec> {
        MemModeW::new(self, 0)
    }
    #[doc = "Bit 8 - ADC_DMA_RMP"]
    #[inline(always)]
    pub fn adc_dma(&mut self) -> AdcDmaW<'_, CfgrSpec> {
        AdcDmaW::new(self, 8)
    }
    #[doc = "Bit 9 - UART1_TX_DMA_RMP"]
    #[inline(always)]
    pub fn uart1_tx_dma(&mut self) -> Uart1TxDmaW<'_, CfgrSpec> {
        Uart1TxDmaW::new(self, 9)
    }
    #[doc = "Bit 10 - UART1_RX_DMA_RMP"]
    #[inline(always)]
    pub fn uart1_rx_dma(&mut self) -> Uart1RxDmaW<'_, CfgrSpec> {
        Uart1RxDmaW::new(self, 10)
    }
    #[doc = "Bit 11 - TIM16_DMA_RMP"]
    #[inline(always)]
    pub fn tim16_dma(&mut self) -> Tim16DmaW<'_, CfgrSpec> {
        Tim16DmaW::new(self, 11)
    }
    #[doc = "Bit 12 - TIM17_DMA_RMP"]
    #[inline(always)]
    pub fn tim17_dma(&mut self) -> Tim17DmaW<'_, CfgrSpec> {
        Tim17DmaW::new(self, 12)
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
