#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Field `ICR` reader - Read this register to clear the combined interrupt"]
pub type IcrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the combined interrupt"]
    #[inline(always)]
    pub fn icr(&self) -> IcrR {
        IcrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear All Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
