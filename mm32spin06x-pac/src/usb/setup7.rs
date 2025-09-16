#[doc = "Register `SETUP7` reader"]
pub type R = crate::R<Setup7Spec>;
#[doc = "Field `SETUPD7` reader - Setup Data 7"]
pub type Setupd7R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Setup Data 7"]
    #[inline(always)]
    pub fn setupd7(&self) -> Setupd7R {
        Setupd7R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Setup Date 7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setup7Spec;
impl crate::RegisterSpec for Setup7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`setup7::R`](R) reader structure"]
impl crate::Readable for Setup7Spec {}
#[doc = "`reset()` method sets SETUP7 to value 0"]
impl crate::Resettable for Setup7Spec {}
