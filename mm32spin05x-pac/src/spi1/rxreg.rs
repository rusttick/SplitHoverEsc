#[doc = "Register `RXREG` reader"]
pub type R = crate::R<RxregSpec>;
#[doc = "Field `RXREG` reader - Receive data register"]
pub type RxregR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive data register"]
    #[inline(always)]
    pub fn rxreg(&self) -> RxregR {
        RxregR::new(self.bits)
    }
}
#[doc = "RXREG\n\nYou can [`read`](crate::Reg::read) this register and get [`rxreg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxregSpec;
impl crate::RegisterSpec for RxregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxreg::R`](R) reader structure"]
impl crate::Readable for RxregSpec {}
#[doc = "`reset()` method sets RXREG to value 0"]
impl crate::Resettable for RxregSpec {}
