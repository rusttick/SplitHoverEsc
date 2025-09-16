#[doc = "Register `DMA_NUML` reader"]
pub type R = crate::R<DmaNumlSpec>;
#[doc = "Register `DMA_NUML` writer"]
pub type W = crate::W<DmaNumlSpec>;
#[doc = "Field `DMA_NUML` reader - EP2 DMA number 0-7"]
pub type DmaNumlR = crate::FieldReader;
#[doc = "Field `DMA_NUML` writer - EP2 DMA number 0-7"]
pub type DmaNumlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EP2 DMA number 0-7"]
    #[inline(always)]
    pub fn dma_numl(&self) -> DmaNumlR {
        DmaNumlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EP2 DMA number 0-7"]
    #[inline(always)]
    pub fn dma_numl(&mut self) -> DmaNumlW<'_, DmaNumlSpec> {
        DmaNumlW::new(self, 0)
    }
}
#[doc = "DMA NUML register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_numl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_numl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaNumlSpec;
impl crate::RegisterSpec for DmaNumlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma_numl::R`](R) reader structure"]
impl crate::Readable for DmaNumlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_numl::W`](W) writer structure"]
impl crate::Writable for DmaNumlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_NUML to value 0"]
impl crate::Resettable for DmaNumlSpec {}
