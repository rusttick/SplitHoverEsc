#[doc = "Register `AFM2` reader"]
pub type R = crate::R<Afm2Spec>;
#[doc = "Register `AFM2` writer"]
pub type W = crate::W<Afm2Spec>;
#[doc = "Field `AFM_19_16` reader - Acceptance filter mode"]
pub type Afm19_16R = crate::FieldReader;
#[doc = "Field `AFM_19_16` writer - Acceptance filter mode"]
pub type Afm19_16W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm_19_16(&self) -> Afm19_16R {
        Afm19_16R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm_19_16(&mut self) -> Afm19_16W<'_, Afm2Spec> {
        Afm19_16W::new(self, 0)
    }
}
#[doc = "Filter Mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`afm2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afm2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afm2Spec;
impl crate::RegisterSpec for Afm2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm2::R`](R) reader structure"]
impl crate::Readable for Afm2Spec {}
#[doc = "`write(|w| ..)` method takes [`afm2::W`](W) writer structure"]
impl crate::Writable for Afm2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFM2 to value 0"]
impl crate::Resettable for Afm2Spec {}
