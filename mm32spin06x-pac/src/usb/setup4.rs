#[doc = "Register `SETUP4` reader"]
pub type R = crate::R<Setup4Spec>;
#[doc = "Field `SETUPD4` reader - Setup Data 4"]
pub type Setupd4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Setup Data 4"]
    #[inline(always)]
    pub fn setupd4(&self) -> Setupd4R {
        Setupd4R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Setup Date 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Setup4Spec;
impl crate::RegisterSpec for Setup4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`setup4::R`](R) reader structure"]
impl crate::Readable for Setup4Spec {}
#[doc = "`reset()` method sets SETUP4 to value 0"]
impl crate::Resettable for Setup4Spec {}
