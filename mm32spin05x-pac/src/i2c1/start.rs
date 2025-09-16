#[doc = "Register `START` reader"]
pub type R = crate::R<StartSpec>;
#[doc = "Field `START` reader - Read this register to clear the START_DET interrupt(bit 10)of the RAWISR register"]
pub type StartR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the START_DET interrupt(bit 10)of the RAWISR register"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear START_DET Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartSpec;
impl crate::RegisterSpec for StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start::R`](R) reader structure"]
impl crate::Readable for StartSpec {}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for StartSpec {}
