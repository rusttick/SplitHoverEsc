#[doc = "Register `EP2_AVAIL` reader"]
pub type R = crate::R<Ep2AvailSpec>;
#[doc = "Field `EP2AVIL` reader - EP2 FIFO available data number"]
pub type Ep2avilR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - EP2 FIFO available data number"]
    #[inline(always)]
    pub fn ep2avil(&self) -> Ep2avilR {
        Ep2avilR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "EP2 AVAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep2_avail::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep2AvailSpec;
impl crate::RegisterSpec for Ep2AvailSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep2_avail::R`](R) reader structure"]
impl crate::Readable for Ep2AvailSpec {}
#[doc = "`reset()` method sets EP2_AVAIL to value 0"]
impl crate::Resettable for Ep2AvailSpec {}
