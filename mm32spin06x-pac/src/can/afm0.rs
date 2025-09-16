#[doc = "Register `AFM0` reader"]
pub type R = crate::R<Afm0Spec>;
#[doc = "Register `AFM0` writer"]
pub type W = crate::W<Afm0Spec>;
#[doc = "Field `AFM_7_1` reader - Acceptance filter mode"]
pub type Afm7_1R = crate::FieldReader;
#[doc = "Field `AFM_7_1` writer - Acceptance filter mode"]
pub type Afm7_1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm_7_1(&self) -> Afm7_1R {
        Afm7_1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm_7_1(&mut self) -> Afm7_1W<'_, Afm0Spec> {
        Afm7_1W::new(self, 1)
    }
}
#[doc = "Filter Mode register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`afm0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afm0Spec;
impl crate::RegisterSpec for Afm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm0::R`](R) reader structure"]
impl crate::Readable for Afm0Spec {}
#[doc = "`write(|w| ..)` method takes [`afm0::W`](W) writer structure"]
impl crate::Writable for Afm0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFM0 to value 0"]
impl crate::Resettable for Afm0Spec {}
