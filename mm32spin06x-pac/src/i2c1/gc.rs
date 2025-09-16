#[doc = "Register `GC` reader"]
pub type R = crate::R<GcSpec>;
#[doc = "Field `GC` reader - Read this register to clear the GEN_CALL interrupt(bit 11)of the RAWISR register"]
pub type GcR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the GEN_CALL interrupt(bit 11)of the RAWISR register"]
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear GEN_CALL Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcSpec;
impl crate::RegisterSpec for GcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gc::R`](R) reader structure"]
impl crate::Readable for GcSpec {}
#[doc = "`reset()` method sets GC to value 0"]
impl crate::Resettable for GcSpec {}
