#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `ISR` reader - Specific bit description refer to RAWISR"]
pub type IsrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Specific bit description refer to RAWISR"]
    #[inline(always)]
    pub fn isr(&self) -> IsrR {
        IsrR::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
