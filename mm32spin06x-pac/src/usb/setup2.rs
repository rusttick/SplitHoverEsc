#[doc = "Register `SETUP2` reader"]
pub type R = crate::R<Setup2Spec>;
#[doc = "Field `SETUPD2` reader - Setup Data 2"]
pub type Setupd2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Setup Data 2"]
    #[inline(always)]
    pub fn setupd2(&self) -> Setupd2R {
        Setupd2R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Setup Date 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setup2Spec;
impl crate::RegisterSpec for Setup2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`setup2::R`](R) reader structure"]
impl crate::Readable for Setup2Spec {}
#[doc = "`reset()` method sets SETUP2 to value 0"]
impl crate::Resettable for Setup2Spec {}
