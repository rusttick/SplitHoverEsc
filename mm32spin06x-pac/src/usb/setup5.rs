#[doc = "Register `SETUP5` reader"]
pub type R = crate::R<Setup5Spec>;
#[doc = "Field `SETUPD5` reader - Setup Data 5"]
pub type Setupd5R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Setup Data 5"]
    #[inline(always)]
    pub fn setupd5(&self) -> Setupd5R {
        Setupd5R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Setup Date 5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setup5Spec;
impl crate::RegisterSpec for Setup5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`setup5::R`](R) reader structure"]
impl crate::Readable for Setup5Spec {}
#[doc = "`reset()` method sets SETUP5 to value 0"]
impl crate::Resettable for Setup5Spec {}
