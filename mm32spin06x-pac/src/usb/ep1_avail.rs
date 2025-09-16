#[doc = "Register `EP1_AVAIL` reader"]
pub type R = crate::R<Ep1AvailSpec>;
#[doc = "Field `EP1AVIL` reader - EP1 FIFO available data number"]
pub type Ep1avilR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - EP1 FIFO available data number"]
    #[inline(always)]
    pub fn ep1avil(&self) -> Ep1avilR {
        Ep1avilR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "EP1 AVAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep1_avail::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep1AvailSpec;
impl crate::RegisterSpec for Ep1AvailSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep1_avail::R`](R) reader structure"]
impl crate::Readable for Ep1AvailSpec {}
#[doc = "`reset()` method sets EP1_AVAIL to value 0"]
impl crate::Resettable for Ep1AvailSpec {}
