#[doc = "Register `DMA_NUMH` reader"]
pub type R = crate::R<DmaNumhSpec>;
#[doc = "Register `DMA_NUMH` writer"]
pub type W = crate::W<DmaNumhSpec>;
#[doc = "Field `DMA1_NUMH` reader - EP2 DMA number 8-15"]
pub type Dma1NumhR = crate::FieldReader;
#[doc = "Field `DMA1_NUMH` writer - EP2 DMA number 8-15"]
pub type Dma1NumhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EP2 DMA number 8-15"]
    #[inline(always)]
    pub fn dma1_numh(&self) -> Dma1NumhR {
        Dma1NumhR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EP2 DMA number 8-15"]
    #[inline(always)]
    pub fn dma1_numh(&mut self) -> Dma1NumhW<'_, DmaNumhSpec> {
        Dma1NumhW::new(self, 0)
    }
}
#[doc = "DMA NUMH register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_numh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_numh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaNumhSpec;
impl crate::RegisterSpec for DmaNumhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma_numh::R`](R) reader structure"]
impl crate::Readable for DmaNumhSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_numh::W`](W) writer structure"]
impl crate::Writable for DmaNumhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_NUMH to value 0"]
impl crate::Resettable for DmaNumhSpec {}
