#[doc = "Register `EP3_AVAIL` reader"]
pub type R = crate::R<Ep3AvailSpec>;
#[doc = "Field `EP3AVIL` reader - EP3 FIFO available data number"]
pub type Ep3avilR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - EP3 FIFO available data number"]
    #[inline(always)]
    pub fn ep3avil(&self) -> Ep3avilR {
        Ep3avilR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "EP3 AVAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep3_avail::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep3AvailSpec;
impl crate::RegisterSpec for Ep3AvailSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep3_avail::R`](R) reader structure"]
impl crate::Readable for Ep3AvailSpec {}
#[doc = "`reset()` method sets EP3_AVAIL to value 0"]
impl crate::Resettable for Ep3AvailSpec {}
