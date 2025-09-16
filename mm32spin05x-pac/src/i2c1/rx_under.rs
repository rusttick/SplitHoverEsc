#[doc = "Register `RX_UNDER` reader"]
pub type R = crate::R<RxUnderSpec>;
#[doc = "Field `RX_UNDER` reader - Read this register to clear the RX_UNDER interrupt(bit 0)of the RAWISR register"]
pub type RxUnderR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 0)of the RAWISR register"]
    #[inline(always)]
    pub fn rx_under(&self) -> RxUnderR {
        RxUnderR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_UNDER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_under::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxUnderSpec;
impl crate::RegisterSpec for RxUnderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_under::R`](R) reader structure"]
impl crate::Readable for RxUnderSpec {}
#[doc = "`reset()` method sets RX_UNDER to value 0"]
impl crate::Resettable for RxUnderSpec {}
