#[doc = "Register `TX_OVER` reader"]
pub type R = crate::R<TxOverSpec>;
#[doc = "Field `TX_OVER` reader - Read this register to clear the RX_UNDER interrupt(bit 3)of the RAWISR register"]
pub type TxOverR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 3)of the RAWISR register"]
    #[inline(always)]
    pub fn tx_over(&self) -> TxOverR {
        TxOverR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear TX_OVER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_over::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxOverSpec;
impl crate::RegisterSpec for TxOverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_over::R`](R) reader structure"]
impl crate::Readable for TxOverSpec {}
#[doc = "`reset()` method sets TX_OVER to value 0"]
impl crate::Resettable for TxOverSpec {}
