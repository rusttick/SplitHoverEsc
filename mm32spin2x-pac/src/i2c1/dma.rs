#[doc = "Register `DMA` reader"]
pub type R = crate::R<DmaSpec>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DmaSpec>;
#[doc = "Field `RXEN` reader - Receive DMA enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receive DMA enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` reader - Transmit DMA enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmit DMA enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, DmaSpec> {
        RxenW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, DmaSpec> {
        TxenW::new(self, 1)
    }
}
#[doc = "DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSpec;
impl crate::RegisterSpec for DmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DmaSpec {}
