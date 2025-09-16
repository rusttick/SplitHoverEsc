#[doc = "Register `UID2` reader"]
pub type R = crate::R<Uid2Spec>;
#[doc = "Field `U_ID` reader - 63:32 unique ID bits"]
pub type UIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 63:32 unique ID bits"]
    #[inline(always)]
    pub fn u_id(&self) -> UIdR {
        UIdR::new(self.bits)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uid2Spec;
impl crate::RegisterSpec for Uid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uid2::R`](R) reader structure"]
impl crate::Readable for Uid2Spec {}
#[doc = "`reset()` method sets UID2 to value 0"]
impl crate::Resettable for Uid2Spec {}
