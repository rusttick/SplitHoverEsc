#[doc = "Register `DCR` writer"]
pub type W = crate::W<DcrSpec>;
#[doc = "Field `DBA` writer - DMA base address"]
pub type DbaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBL` writer - DMA burst length"]
pub type DblW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl W {
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline(always)]
    pub fn dba(&mut self) -> DbaW<'_, DcrSpec> {
        DbaW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline(always)]
    pub fn dbl(&mut self) -> DblW<'_, DcrSpec> {
        DblW::new(self, 8)
    }
}
#[doc = "DMA control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrSpec;
impl crate::RegisterSpec for DcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DcrSpec {}
