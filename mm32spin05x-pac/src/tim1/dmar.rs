#[doc = "Register `DMAR` writer"]
pub type W = crate::W<DmarSpec>;
#[doc = "Field `DMAB` writer - DMA register for burst accesses"]
pub type DmabW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - DMA register for burst accesses"]
    #[inline(always)]
    pub fn dmab(&mut self) -> DmabW<'_, DmarSpec> {
        DmabW::new(self, 0)
    }
}
#[doc = "DMA address for full transfer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmarSpec;
impl crate::RegisterSpec for DmarSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmar::W`](W) writer structure"]
impl crate::Writable for DmarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAR to value 0"]
impl crate::Resettable for DmarSpec {}
