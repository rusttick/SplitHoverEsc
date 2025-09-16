#[doc = "Register `DMA_CNDTR1` reader"]
pub type R = crate::R<DmaCndtr1Spec>;
#[doc = "Register `DMA_CNDTR1` writer"]
pub type W = crate::W<DmaCndtr1Spec>;
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NdtR = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<'_, DmaCndtr1Spec> {
        NdtW::new(self, 0)
    }
}
#[doc = "DMA channel 1 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cndtr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCndtr1Spec;
impl crate::RegisterSpec for DmaCndtr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cndtr1::R`](R) reader structure"]
impl crate::Readable for DmaCndtr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cndtr1::W`](W) writer structure"]
impl crate::Writable for DmaCndtr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CNDTR1 to value 0"]
impl crate::Resettable for DmaCndtr1Spec {}
