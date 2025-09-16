#[doc = "Register `UID3` reader"]
pub type R = crate::R<Uid3Spec>;
#[doc = "Field `U_ID` reader - 95:64 unique ID bits"]
pub type UIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 95:64 unique ID bits"]
    #[inline(always)]
    pub fn u_id(&self) -> UIdR {
        UIdR::new(self.bits)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uid3Spec;
impl crate::RegisterSpec for Uid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uid3::R`](R) reader structure"]
impl crate::Readable for Uid3Spec {}
#[doc = "`reset()` method sets UID3 to value 0"]
impl crate::Resettable for Uid3Spec {}
