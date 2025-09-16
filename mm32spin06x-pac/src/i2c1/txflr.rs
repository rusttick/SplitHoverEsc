#[doc = "Register `TXFLR` reader"]
pub type R = crate::R<TxflrSpec>;
#[doc = "Field `CNT` reader - Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
pub type CntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 3) as u8)
    }
}
#[doc = "Transmit FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxflrSpec;
impl crate::RegisterSpec for TxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txflr::R`](R) reader structure"]
impl crate::Readable for TxflrSpec {}
#[doc = "`reset()` method sets TXFLR to value 0"]
impl crate::Resettable for TxflrSpec {}
