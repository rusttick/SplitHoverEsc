#[doc = "Register `STOP` reader"]
pub type R = crate::R<StopSpec>;
#[doc = "Field `STOP` reader - Read this register to clear the STOP_DET interrupt(bit 9)of the RAWISR register"]
pub type StopR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the STOP_DET interrupt(bit 9)of the RAWISR register"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear STOP_DET Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stop::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StopSpec;
impl crate::RegisterSpec for StopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stop::R`](R) reader structure"]
impl crate::Readable for StopSpec {}
#[doc = "`reset()` method sets STOP to value 0"]
impl crate::Resettable for StopSpec {}
