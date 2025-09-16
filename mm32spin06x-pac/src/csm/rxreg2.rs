#[doc = "Register `RXREG2` reader"]
pub type R = crate::R<Rxreg2Spec>;
#[doc = "Field `RXREG2` reader - Receive data"]
pub type Rxreg2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive data"]
    #[inline(always)]
    pub fn rxreg2(&self) -> Rxreg2R {
        Rxreg2R::new(self.bits)
    }
}
#[doc = "Reveive register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rxreg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxreg2Spec;
impl crate::RegisterSpec for Rxreg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxreg2::R`](R) reader structure"]
impl crate::Readable for Rxreg2Spec {}
#[doc = "`reset()` method sets RXREG2 to value 0"]
impl crate::Resettable for Rxreg2Spec {}
