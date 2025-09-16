#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TXEPT` reader - Transmitter empty bit"]
pub type TxeptR = crate::BitReader;
#[doc = "Field `RXAVL` reader - Receive available byte data message"]
pub type RxavlR = crate::BitReader;
#[doc = "Field `TXFULL` reader - Transmitter FIFO full status bit"]
pub type TxfullR = crate::BitReader;
#[doc = "Field `RXAVL_4BYTE` reader - Receive available 4 byte data message"]
pub type Rxavl4byteR = crate::BitReader;
#[doc = "Field `TXFADDR` reader - Receive FIFO address"]
pub type TxfaddrR = crate::FieldReader;
#[doc = "Field `RXFADDR` reader - Transmit FIFO address"]
pub type RxfaddrR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmitter empty bit"]
    #[inline(always)]
    pub fn txept(&self) -> TxeptR {
        TxeptR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive available byte data message"]
    #[inline(always)]
    pub fn rxavl(&self) -> RxavlR {
        RxavlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter FIFO full status bit"]
    #[inline(always)]
    pub fn txfull(&self) -> TxfullR {
        TxfullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive available 4 byte data message"]
    #[inline(always)]
    pub fn rxavl_4byte(&self) -> Rxavl4byteR {
        Rxavl4byteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Receive FIFO address"]
    #[inline(always)]
    pub fn txfaddr(&self) -> TxfaddrR {
        TxfaddrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transmit FIFO address"]
    #[inline(always)]
    pub fn rxfaddr(&self) -> RxfaddrR {
        RxfaddrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x01;
}
