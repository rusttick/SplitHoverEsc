#[doc = "Register `RX_DONE` reader"]
pub type R = crate::R<RxDoneSpec>;
#[doc = "Field `RX_DONE` reader - Read this register to clear the RX_UNDER interrupt(bit 7)of the RAWISR register"]
pub type RxDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 7)of the RAWISR register"]
    #[inline(always)]
    pub fn rx_done(&self) -> RxDoneR {
        RxDoneR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_DONE Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_done::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxDoneSpec;
impl crate::RegisterSpec for RxDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_done::R`](R) reader structure"]
impl crate::Readable for RxDoneSpec {}
#[doc = "`reset()` method sets RX_DONE to value 0"]
impl crate::Resettable for RxDoneSpec {}
