#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `TX_IEN` reader - Transmit FIFO empty interrupt enable bit"]
pub type TxIenR = crate::BitReader;
#[doc = "Field `TX_IEN` writer - Transmit FIFO empty interrupt enable bit"]
pub type TxIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IEN` reader - Receive FIFO interrupt enable bit"]
pub type RxIenR = crate::BitReader;
#[doc = "Field `RX_IEN` writer - Receive FIFO interrupt enable bit"]
pub type RxIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN_IEN` reader - Transmitter underrun interrupt enable bit(SPI slave mode only)"]
pub type UnderrunIenR = crate::BitReader;
#[doc = "Field `UNDERRUN_IEN` writer - Transmitter underrun interrupt enable bit(SPI slave mode only)"]
pub type UnderrunIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOERR_IEN` reader - Overrun error interrupt enable bit"]
pub type RxoerrIenR = crate::BitReader;
#[doc = "Field `RXOERR_IEN` writer - Overrun error interrupt enable bit"]
pub type RxoerrIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMATCH_IEN` reader - Receive data complete interrupt enable bit"]
pub type RxmatchIenR = crate::BitReader;
#[doc = "Field `RXMATCH_IEN` writer - Receive data complete interrupt enable bit"]
pub type RxmatchIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL_IEN` reader - Receive FIFO full interrupt enable bit"]
pub type RxfullIenR = crate::BitReader;
#[doc = "Field `RXFULL_IEN` writer - Receive FIFO full interrupt enable bit"]
pub type RxfullIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEPT_IEN` reader - Transmit empty interrupt enable bit"]
pub type TxeptIenR = crate::BitReader;
#[doc = "Field `TXEPT_IEN` writer - Transmit empty interrupt enable bit"]
pub type TxeptIenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty interrupt enable bit"]
    #[inline(always)]
    pub fn tx_ien(&self) -> TxIenR {
        TxIenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO interrupt enable bit"]
    #[inline(always)]
    pub fn rx_ien(&self) -> RxIenR {
        RxIenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter underrun interrupt enable bit(SPI slave mode only)"]
    #[inline(always)]
    pub fn underrun_ien(&self) -> UnderrunIenR {
        UnderrunIenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error interrupt enable bit"]
    #[inline(always)]
    pub fn rxoerr_ien(&self) -> RxoerrIenR {
        RxoerrIenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive data complete interrupt enable bit"]
    #[inline(always)]
    pub fn rxmatch_ien(&self) -> RxmatchIenR {
        RxmatchIenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO full interrupt enable bit"]
    #[inline(always)]
    pub fn rxfull_ien(&self) -> RxfullIenR {
        RxfullIenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit empty interrupt enable bit"]
    #[inline(always)]
    pub fn txept_ien(&self) -> TxeptIenR {
        TxeptIenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO empty interrupt enable bit"]
    #[inline(always)]
    pub fn tx_ien(&mut self) -> TxIenW<'_, IntenSpec> {
        TxIenW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO interrupt enable bit"]
    #[inline(always)]
    pub fn rx_ien(&mut self) -> RxIenW<'_, IntenSpec> {
        RxIenW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter underrun interrupt enable bit(SPI slave mode only)"]
    #[inline(always)]
    pub fn underrun_ien(&mut self) -> UnderrunIenW<'_, IntenSpec> {
        UnderrunIenW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error interrupt enable bit"]
    #[inline(always)]
    pub fn rxoerr_ien(&mut self) -> RxoerrIenW<'_, IntenSpec> {
        RxoerrIenW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive data complete interrupt enable bit"]
    #[inline(always)]
    pub fn rxmatch_ien(&mut self) -> RxmatchIenW<'_, IntenSpec> {
        RxmatchIenW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO full interrupt enable bit"]
    #[inline(always)]
    pub fn rxfull_ien(&mut self) -> RxfullIenW<'_, IntenSpec> {
        RxfullIenW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit empty interrupt enable bit"]
    #[inline(always)]
    pub fn txept_ien(&mut self) -> TxeptIenW<'_, IntenSpec> {
        TxeptIenW::new(self, 6)
    }
}
#[doc = "INTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
