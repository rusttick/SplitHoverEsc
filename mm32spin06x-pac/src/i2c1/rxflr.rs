#[doc = "Register `RXFLR` reader"]
pub type R = crate::R<RxflrSpec>;
#[doc = "Field `CNT` reader - Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
pub type CntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 3) as u8)
    }
}
#[doc = "Receive FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxflrSpec;
impl crate::RegisterSpec for RxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxflr::R`](R) reader structure"]
impl crate::Readable for RxflrSpec {}
#[doc = "`reset()` method sets RXFLR to value 0"]
impl crate::Resettable for RxflrSpec {}
