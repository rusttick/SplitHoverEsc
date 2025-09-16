#[doc = "Register `AR` writer"]
pub type W = crate::W<ArSpec>;
#[doc = "Field `FAR` writer - Flash Address"]
pub type FarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash Address"]
    #[inline(always)]
    pub fn far(&mut self) -> FarW<'_, ArSpec> {
        FarW::new(self, 0)
    }
}
#[doc = "Flash address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArSpec;
impl crate::RegisterSpec for ArSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ar::W`](W) writer structure"]
impl crate::Writable for ArSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AR to value 0"]
impl crate::Resettable for ArSpec {}
