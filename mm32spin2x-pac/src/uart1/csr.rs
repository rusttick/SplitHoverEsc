#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Field `TXC` reader - Transmit complete flag bit"]
pub type TxcR = crate::BitReader;
#[doc = "Field `RXAVL` reader - Receive valid data flag bit"]
pub type RxavlR = crate::BitReader;
#[doc = "Field `TXFULL` reader - Transmit buffer full flag bit"]
pub type TxfullR = crate::BitReader;
#[doc = "Field `TXBUF_EMPTY` reader - Transmit buffer empty flag bit"]
pub type TxbufEmptyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit complete flag bit"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive valid data flag bit"]
    #[inline(always)]
    pub fn rxavl(&self) -> RxavlR {
        RxavlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer full flag bit"]
    #[inline(always)]
    pub fn txfull(&self) -> TxfullR {
        TxfullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit buffer empty flag bit"]
    #[inline(always)]
    pub fn txbuf_empty(&self) -> TxbufEmptyR {
        TxbufEmptyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Current status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`reset()` method sets CSR to value 0x09"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0x09;
}
