#[doc = "Register `IDCODE` reader"]
pub type R = crate::R<IdcodeSpec>;
#[doc = "Field `DEV_ID` reader - Device identifier"]
pub type DevIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device identifier"]
    #[inline(always)]
    pub fn dev_id(&self) -> DevIdR {
        DevIdR::new(self.bits)
    }
}
#[doc = "ID CODE\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdcodeSpec;
impl crate::RegisterSpec for IdcodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idcode::R`](R) reader structure"]
impl crate::Readable for IdcodeSpec {}
#[doc = "`reset()` method sets IDCODE to value 0"]
impl crate::Resettable for IdcodeSpec {}
