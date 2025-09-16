#[doc = "Register `RDR` reader"]
pub type R = crate::R<RdrSpec>;
#[doc = "Field `RXREG` reader - Receive data register"]
pub type RxregR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive data register"]
    #[inline(always)]
    pub fn rxreg(&self) -> RxregR {
        RxregR::new(self.bits)
    }
}
#[doc = "RDR\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdrSpec;
impl crate::RegisterSpec for RdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RdrSpec {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RdrSpec {}
