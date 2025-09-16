#[doc = "Register `SETUP6` reader"]
pub type R = crate::R<Setup6Spec>;
#[doc = "Field `SETUPD6` reader - Setup Data 6"]
pub type Setupd6R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Setup Data 6"]
    #[inline(always)]
    pub fn setupd6(&self) -> Setupd6R {
        Setupd6R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Setup Date 6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setup6Spec;
impl crate::RegisterSpec for Setup6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`setup6::R`](R) reader structure"]
impl crate::Readable for Setup6Spec {}
#[doc = "`reset()` method sets SETUP6 to value 0"]
impl crate::Resettable for Setup6Spec {}
