#[doc = "Register `TX_ABRT` reader"]
pub type R = crate::R<TxAbrtSpec>;
#[doc = "Field `TX_ABRT` reader - Read this register to clear the RX_UNDER interrupt(bit 6)of the RAWISR register"]
pub type TxAbrtR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 6)of the RAWISR register"]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TxAbrtR {
        TxAbrtR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear TX_ABRT Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_abrt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxAbrtSpec;
impl crate::RegisterSpec for TxAbrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_abrt::R`](R) reader structure"]
impl crate::Readable for TxAbrtSpec {}
#[doc = "`reset()` method sets TX_ABRT to value 0"]
impl crate::Resettable for TxAbrtSpec {}
