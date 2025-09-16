#[doc = "Register `QUOTR` reader"]
pub type R = crate::R<QuotrSpec>;
#[doc = "Field `QUOTIENT` reader - Quotient data"]
pub type QuotientR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Quotient data"]
    #[inline(always)]
    pub fn quotient(&self) -> QuotientR {
        QuotientR::new(self.bits)
    }
}
#[doc = "QUOTR\n\nYou can [`read`](crate::Reg::read) this register and get [`quotr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QuotrSpec;
impl crate::RegisterSpec for QuotrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quotr::R`](R) reader structure"]
impl crate::Readable for QuotrSpec {}
#[doc = "`reset()` method sets QUOTR to value 0"]
impl crate::Resettable for QuotrSpec {}
