#[doc = "Register `RX_OVER` reader"]
pub type R = crate::R<RxOverSpec>;
#[doc = "Field `RX_OVER` reader - Read this register to clear the RX_UNDER interrupt(bit 1)of the RAWISR register"]
pub type RxOverR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 1)of the RAWISR register"]
    #[inline(always)]
    pub fn rx_over(&self) -> RxOverR {
        RxOverR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_OVER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_over::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOverSpec;
impl crate::RegisterSpec for RxOverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_over::R`](R) reader structure"]
impl crate::Readable for RxOverSpec {}
#[doc = "`reset()` method sets RX_OVER to value 0"]
impl crate::Resettable for RxOverSpec {}
