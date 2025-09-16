#[doc = "Register `EP4_AVAIL` reader"]
pub type R = crate::R<Ep4AvailSpec>;
#[doc = "Field `EP4AVIL` reader - EP4 FIFO available data number"]
pub type Ep4avilR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - EP4 FIFO available data number"]
    #[inline(always)]
    pub fn ep4avil(&self) -> Ep4avilR {
        Ep4avilR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "EP4 AVAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep4_avail::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep4AvailSpec;
impl crate::RegisterSpec for Ep4AvailSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep4_avail::R`](R) reader structure"]
impl crate::Readable for Ep4AvailSpec {}
#[doc = "`reset()` method sets EP4_AVAIL to value 0"]
impl crate::Resettable for Ep4AvailSpec {}
