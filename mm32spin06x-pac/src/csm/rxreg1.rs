#[doc = "Register `RXREG1` reader"]
pub type R = crate::R<Rxreg1Spec>;
#[doc = "Field `RXREG1` reader - Receive data"]
pub type Rxreg1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive data"]
    #[inline(always)]
    pub fn rxreg1(&self) -> Rxreg1R {
        Rxreg1R::new(self.bits)
    }
}
#[doc = "Reveive register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rxreg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxreg1Spec;
impl crate::RegisterSpec for Rxreg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxreg1::R`](R) reader structure"]
impl crate::Readable for Rxreg1Spec {}
#[doc = "`reset()` method sets RXREG1 to value 0"]
impl crate::Resettable for Rxreg1Spec {}
