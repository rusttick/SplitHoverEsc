#[doc = "Register `EP0_AVAIL` reader"]
pub type R = crate::R<Ep0AvailSpec>;
#[doc = "Field `EP0AVIL` reader - EP0 FIFO available data number"]
pub type Ep0avilR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - EP0 FIFO available data number"]
    #[inline(always)]
    pub fn ep0avil(&self) -> Ep0avilR {
        Ep0avilR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "EP0 AVAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_avail::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep0AvailSpec;
impl crate::RegisterSpec for Ep0AvailSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep0_avail::R`](R) reader structure"]
impl crate::Readable for Ep0AvailSpec {}
#[doc = "`reset()` method sets EP0_AVAIL to value 0"]
impl crate::Resettable for Ep0AvailSpec {}
