#[doc = "Register `SETUP1` reader"]
pub type R = crate::R<Setup1Spec>;
#[doc = "Field `SETUPD1` reader - Setup Data 1"]
pub type Setupd1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Setup Data 1"]
    #[inline(always)]
    pub fn setupd1(&self) -> Setupd1R {
        Setupd1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Setup Date 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setup1Spec;
impl crate::RegisterSpec for Setup1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`setup1::R`](R) reader structure"]
impl crate::Readable for Setup1Spec {}
#[doc = "`reset()` method sets SETUP1 to value 0"]
impl crate::Resettable for Setup1Spec {}
