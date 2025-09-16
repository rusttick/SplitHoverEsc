#[doc = "Register `AFM1` reader"]
pub type R = crate::R<Afm1Spec>;
#[doc = "Register `AFM1` writer"]
pub type W = crate::W<Afm1Spec>;
#[doc = "Field `AFM_15_8` reader - Acceptance filter mode"]
pub type Afm15_8R = crate::FieldReader;
#[doc = "Field `AFM_15_8` writer - Acceptance filter mode"]
pub type Afm15_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm_15_8(&self) -> Afm15_8R {
        Afm15_8R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm_15_8(&mut self) -> Afm15_8W<'_, Afm1Spec> {
        Afm15_8W::new(self, 0)
    }
}
#[doc = "Filter Mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`afm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afm1Spec;
impl crate::RegisterSpec for Afm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm1::R`](R) reader structure"]
impl crate::Readable for Afm1Spec {}
#[doc = "`write(|w| ..)` method takes [`afm1::W`](W) writer structure"]
impl crate::Writable for Afm1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFM1 to value 0"]
impl crate::Resettable for Afm1Spec {}
