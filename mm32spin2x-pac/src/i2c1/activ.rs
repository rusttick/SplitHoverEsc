#[doc = "Register `ACTIV` reader"]
pub type R = crate::R<ActivSpec>;
#[doc = "Field `ACTIV` reader - Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore"]
pub type ActivR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore"]
    #[inline(always)]
    pub fn activ(&self) -> ActivR {
        ActivR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear ACTIVITY Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`activ::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActivSpec;
impl crate::RegisterSpec for ActivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`activ::R`](R) reader structure"]
impl crate::Readable for ActivSpec {}
#[doc = "`reset()` method sets ACTIV to value 0"]
impl crate::Resettable for ActivSpec {}
