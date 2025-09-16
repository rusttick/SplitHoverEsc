#[doc = "Register `DMA_CPAR1` reader"]
pub type R = crate::R<DmaCpar1Spec>;
#[doc = "Register `DMA_CPAR1` writer"]
pub type W = crate::W<DmaCpar1Spec>;
#[doc = "Field `PA` reader - Peripheral address"]
pub type PaR = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<'_, DmaCpar1Spec> {
        PaW::new(self, 0)
    }
}
#[doc = "DMA channel 1 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cpar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCpar1Spec;
impl crate::RegisterSpec for DmaCpar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cpar1::R`](R) reader structure"]
impl crate::Readable for DmaCpar1Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cpar1::W`](W) writer structure"]
impl crate::Writable for DmaCpar1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CPAR1 to value 0"]
impl crate::Resettable for DmaCpar1Spec {}
