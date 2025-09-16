#[doc = "Register `UID1` reader"]
pub type R = crate::R<Uid1Spec>;
#[doc = "Field `U_ID` reader - 31:0 unique ID bits"]
pub type UIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0 unique ID bits"]
    #[inline(always)]
    pub fn u_id(&self) -> UIdR {
        UIdR::new(self.bits)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uid1Spec;
impl crate::RegisterSpec for Uid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uid1::R`](R) reader structure"]
impl crate::Readable for Uid1Spec {}
#[doc = "`reset()` method sets UID1 to value 0"]
impl crate::Resettable for Uid1Spec {}
