#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<IntclrSpec>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
#[doc = "Field `TX_ICLR` reader - Transmitter FIFO empty interrupt clear bit"]
pub type TxIclrR = crate::BitReader;
#[doc = "Field `TX_ICLR` writer - Transmitter FIFO empty interrupt clear bit"]
pub type TxIclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ICLR` reader - Receive interrupt clear bit"]
pub type RxIclrR = crate::BitReader;
#[doc = "Field `RX_ICLR` writer - Receive interrupt clear bit"]
pub type RxIclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN_ICLR` reader - Transmitter underrun interrupt clear bit(SPI slave mode only)"]
pub type UnderrunIclrR = crate::BitReader;
#[doc = "Field `UNDERRUN_ICLR` writer - Transmitter underrun interrupt clear bit(SPI slave mode only)"]
pub type UnderrunIclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOERR_ICLR` reader - Overrun error interrupt clear bit"]
pub type RxoerrIclrR = crate::BitReader;
#[doc = "Field `RXOERR_ICLR` writer - Overrun error interrupt clear bit"]
pub type RxoerrIclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMATCH_ICLR` reader - Receive completed interrupt clear bit"]
pub type RxmatchIclrR = crate::BitReader;
#[doc = "Field `RXMATCH_ICLR` writer - Receive completed interrupt clear bit"]
pub type RxmatchIclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL_ICLR` reader - Receiver buffer full interrupt clear bit"]
pub type RxfullIclrR = crate::BitReader;
#[doc = "Field `RXFULL_ICLR` writer - Receiver buffer full interrupt clear bit"]
pub type RxfullIclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEPT_ICLR` reader - Transmitter empty interrupt clear bit"]
pub type TxeptIclrR = crate::BitReader;
#[doc = "Field `TXEPT_ICLR` writer - Transmitter empty interrupt clear bit"]
pub type TxeptIclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmitter FIFO empty interrupt clear bit"]
    #[inline(always)]
    pub fn tx_iclr(&self) -> TxIclrR {
        TxIclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive interrupt clear bit"]
    #[inline(always)]
    pub fn rx_iclr(&self) -> RxIclrR {
        RxIclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter underrun interrupt clear bit(SPI slave mode only)"]
    #[inline(always)]
    pub fn underrun_iclr(&self) -> UnderrunIclrR {
        UnderrunIclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error interrupt clear bit"]
    #[inline(always)]
    pub fn rxoerr_iclr(&self) -> RxoerrIclrR {
        RxoerrIclrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive completed interrupt clear bit"]
    #[inline(always)]
    pub fn rxmatch_iclr(&self) -> RxmatchIclrR {
        RxmatchIclrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver buffer full interrupt clear bit"]
    #[inline(always)]
    pub fn rxfull_iclr(&self) -> RxfullIclrR {
        RxfullIclrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter empty interrupt clear bit"]
    #[inline(always)]
    pub fn txept_iclr(&self) -> TxeptIclrR {
        TxeptIclrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter FIFO empty interrupt clear bit"]
    #[inline(always)]
    pub fn tx_iclr(&mut self) -> TxIclrW<'_, IntclrSpec> {
        TxIclrW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive interrupt clear bit"]
    #[inline(always)]
    pub fn rx_iclr(&mut self) -> RxIclrW<'_, IntclrSpec> {
        RxIclrW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter underrun interrupt clear bit(SPI slave mode only)"]
    #[inline(always)]
    pub fn underrun_iclr(&mut self) -> UnderrunIclrW<'_, IntclrSpec> {
        UnderrunIclrW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error interrupt clear bit"]
    #[inline(always)]
    pub fn rxoerr_iclr(&mut self) -> RxoerrIclrW<'_, IntclrSpec> {
        RxoerrIclrW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive completed interrupt clear bit"]
    #[inline(always)]
    pub fn rxmatch_iclr(&mut self) -> RxmatchIclrW<'_, IntclrSpec> {
        RxmatchIclrW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver buffer full interrupt clear bit"]
    #[inline(always)]
    pub fn rxfull_iclr(&mut self) -> RxfullIclrW<'_, IntclrSpec> {
        RxfullIclrW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter empty interrupt clear bit"]
    #[inline(always)]
    pub fn txept_iclr(&mut self) -> TxeptIclrW<'_, IntclrSpec> {
        TxeptIclrW::new(self, 6)
    }
}
#[doc = "INTCLR\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for IntclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {}
