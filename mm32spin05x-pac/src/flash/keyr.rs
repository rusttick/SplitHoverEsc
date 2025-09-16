#[doc = "Register `KEYR` writer"]
pub type W = crate::W<KeyrSpec>;
#[doc = "Field `FKEYR` writer - Flash key"]
pub type FkeyrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash key"]
    #[inline(always)]
    pub fn fkeyr(&mut self) -> FkeyrW<'_, KeyrSpec> {
        FkeyrW::new(self, 0)
    }
}
#[doc = "Flash key\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyrSpec;
impl crate::RegisterSpec for KeyrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyr::W`](W) writer structure"]
impl crate::Writable for KeyrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYR to value 0"]
impl crate::Resettable for KeyrSpec {}
