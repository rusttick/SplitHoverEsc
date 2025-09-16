#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `TX_INTF` reader - Transmit buffer empty interrupt flag bit"]
pub type TxIntfR = crate::BitReader;
#[doc = "Field `RX_INTF` reader - Receive valid data interrupt flag bit"]
pub type RxIntfR = crate::BitReader;
#[doc = "Field `TXC_INTF` reader - UART Transmit Complete Interrupt Flag bit"]
pub type TxcIntfR = crate::BitReader;
#[doc = "Field `RXOERR_INTF` reader - Receive overflow error interrupt flag bit"]
pub type RxoerrIntfR = crate::BitReader;
#[doc = "Field `RXPERR_INTF` reader - Parity error interrupt flag bit"]
pub type RxperrIntfR = crate::BitReader;
#[doc = "Field `RXFERR_INTF` reader - Frame error interrupt flag bit"]
pub type RxferrIntfR = crate::BitReader;
#[doc = "Field `RXBRK_INTF` reader - Receive frame break interrupt flag bit"]
pub type RxbrkIntfR = crate::BitReader;
#[doc = "Field `TXBRK_INTF` reader - Transmit Break Frame Interrupt Flag Bit"]
pub type TxbrkIntfR = crate::BitReader;
#[doc = "Field `RXB8_INTF` reader - Receive Bit 8 Interrupt Flag Bit"]
pub type Rxb8IntfR = crate::BitReader;
#[doc = "Field `RXIDLE_INTF` reader - Receive frame idle interrupt flag bit"]
pub type RxidleIntfR = crate::BitReader;
#[doc = "Field `ABREND_INTF` reader - Automatic baud rate end interrupt flag bit"]
pub type AbrendIntfR = crate::BitReader;
#[doc = "Field `ABRERR_INTF` reader - Automatic baud rate error interrupt flag bit"]
pub type AbrerrIntfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit buffer empty interrupt flag bit"]
    #[inline(always)]
    pub fn tx_intf(&self) -> TxIntfR {
        TxIntfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive valid data interrupt flag bit"]
    #[inline(always)]
    pub fn rx_intf(&self) -> RxIntfR {
        RxIntfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Transmit Complete Interrupt Flag bit"]
    #[inline(always)]
    pub fn txc_intf(&self) -> TxcIntfR {
        TxcIntfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt flag bit"]
    #[inline(always)]
    pub fn rxoerr_intf(&self) -> RxoerrIntfR {
        RxoerrIntfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error interrupt flag bit"]
    #[inline(always)]
    pub fn rxperr_intf(&self) -> RxperrIntfR {
        RxperrIntfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame error interrupt flag bit"]
    #[inline(always)]
    pub fn rxferr_intf(&self) -> RxferrIntfR {
        RxferrIntfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive frame break interrupt flag bit"]
    #[inline(always)]
    pub fn rxbrk_intf(&self) -> RxbrkIntfR {
        RxbrkIntfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt Flag Bit"]
    #[inline(always)]
    pub fn txbrk_intf(&self) -> TxbrkIntfR {
        TxbrkIntfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt Flag Bit"]
    #[inline(always)]
    pub fn rxb8_intf(&self) -> Rxb8IntfR {
        Rxb8IntfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive frame idle interrupt flag bit"]
    #[inline(always)]
    pub fn rxidle_intf(&self) -> RxidleIntfR {
        RxidleIntfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic baud rate end interrupt flag bit"]
    #[inline(always)]
    pub fn abrend_intf(&self) -> AbrendIntfR {
        AbrendIntfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Automatic baud rate error interrupt flag bit"]
    #[inline(always)]
    pub fn abrerr_intf(&self) -> AbrerrIntfR {
        AbrerrIntfR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
