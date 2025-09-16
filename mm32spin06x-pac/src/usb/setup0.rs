#[doc = "Register `SETUP0` reader"]
pub type R = crate::R<Setup0Spec>;
#[doc = "Field `SETUPD0` reader - Setup Data 0"]
pub type Setupd0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Setup Data 0"]
    #[inline(always)]
    pub fn setupd0(&self) -> Setupd0R {
        Setupd0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Setup Date 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setup0Spec;
impl crate::RegisterSpec for Setup0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`setup0::R`](R) reader structure"]
impl crate::Readable for Setup0Spec {}
#[doc = "`reset()` method sets SETUP0 to value 0"]
impl crate::Resettable for Setup0Spec {}
