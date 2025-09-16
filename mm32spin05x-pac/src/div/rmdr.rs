#[doc = "Register `RMDR` reader"]
pub type R = crate::R<RmdrSpec>;
#[doc = "Field `REMAINDER` reader - Remainder data"]
pub type RemainderR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Remainder data"]
    #[inline(always)]
    pub fn remainder(&self) -> RemainderR {
        RemainderR::new(self.bits)
    }
}
#[doc = "RMDR\n\nYou can [`read`](crate::Reg::read) this register and get [`rmdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmdrSpec;
impl crate::RegisterSpec for RmdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmdr::R`](R) reader structure"]
impl crate::Readable for RmdrSpec {}
#[doc = "`reset()` method sets RMDR to value 0"]
impl crate::Resettable for RmdrSpec {}
