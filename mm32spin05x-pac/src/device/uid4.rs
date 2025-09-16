#[doc = "Register `UID4` reader"]
pub type R = crate::R<Uid4Spec>;
#[doc = "Field `U_ID` reader - 95:64 unique ID bits"]
pub type UIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 95:64 unique ID bits"]
    #[inline(always)]
    pub fn u_id(&self) -> UIdR {
        UIdR::new(self.bits)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uid4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uid4Spec;
impl crate::RegisterSpec for Uid4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uid4::R`](R) reader structure"]
impl crate::Readable for Uid4Spec {}
#[doc = "`reset()` method sets UID4 to value 0"]
impl crate::Resettable for Uid4Spec {}
