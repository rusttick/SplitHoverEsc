#[doc = "Register `SETUP3` reader"]
pub type R = crate::R<Setup3Spec>;
#[doc = "Field `SETUPD3` reader - Setup Data 3"]
pub type Setupd3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Setup Data 3"]
    #[inline(always)]
    pub fn setupd3(&self) -> Setupd3R {
        Setupd3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Setup Date 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setup3Spec;
impl crate::RegisterSpec for Setup3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`setup3::R`](R) reader structure"]
impl crate::Readable for Setup3Spec {}
#[doc = "`reset()` method sets SETUP3 to value 0"]
impl crate::Resettable for Setup3Spec {}
