#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "Field `TX_INTF` reader - Transmit FIFO avialable interrupt flag bit"]
pub type TxIntfR = crate::BitReader;
#[doc = "Field `TX_INTF` writer - Transmit FIFO avialable interrupt flag bit"]
pub type TxIntfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_INTF` reader - Receive data available interrupt flag bit"]
pub type RxIntfR = crate::BitReader;
#[doc = "Field `RX_INTF` writer - Receive data available interrupt flag bit"]
pub type RxIntfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN_INTF` reader - SPI underrun interrupt flag bit"]
pub type UnderrunIntfR = crate::BitReader;
#[doc = "Field `UNDERRUN_INTF` writer - SPI underrun interrupt flag bit"]
pub type UnderrunIntfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOERR_INTF` reader - Receive overrun error interrupt flag bit"]
pub type RxoerrIntfR = crate::BitReader;
#[doc = "Field `RXOERR_INTF` writer - Receive overrun error interrupt flag bit"]
pub type RxoerrIntfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMATCH_INTF` reader - Receive data match the RXDNR number"]
pub type RxmatchIntfR = crate::BitReader;
#[doc = "Field `RXMATCH_INTF` writer - Receive data match the RXDNR number"]
pub type RxmatchIntfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL_INTF` reader - RX FIFO full interrupt flag bit"]
pub type RxfullIntfR = crate::BitReader;
#[doc = "Field `RXFULL_INTF` writer - RX FIFO full interrupt flag bit"]
pub type RxfullIntfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEPT_INTF` reader - Transmitter empty interrupt flag bit"]
pub type TxeptIntfR = crate::BitReader;
#[doc = "Field `TXEPT_INTF` writer - Transmitter empty interrupt flag bit"]
pub type TxeptIntfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO avialable interrupt flag bit"]
    #[inline(always)]
    pub fn tx_intf(&self) -> TxIntfR {
        TxIntfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive data available interrupt flag bit"]
    #[inline(always)]
    pub fn rx_intf(&self) -> RxIntfR {
        RxIntfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI underrun interrupt flag bit"]
    #[inline(always)]
    pub fn underrun_intf(&self) -> UnderrunIntfR {
        UnderrunIntfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overrun error interrupt flag bit"]
    #[inline(always)]
    pub fn rxoerr_intf(&self) -> RxoerrIntfR {
        RxoerrIntfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive data match the RXDNR number"]
    #[inline(always)]
    pub fn rxmatch_intf(&self) -> RxmatchIntfR {
        RxmatchIntfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO full interrupt flag bit"]
    #[inline(always)]
    pub fn rxfull_intf(&self) -> RxfullIntfR {
        RxfullIntfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter empty interrupt flag bit"]
    #[inline(always)]
    pub fn txept_intf(&self) -> TxeptIntfR {
        TxeptIntfR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO avialable interrupt flag bit"]
    #[inline(always)]
    pub fn tx_intf(&mut self) -> TxIntfW<'_, IntstatSpec> {
        TxIntfW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive data available interrupt flag bit"]
    #[inline(always)]
    pub fn rx_intf(&mut self) -> RxIntfW<'_, IntstatSpec> {
        RxIntfW::new(self, 1)
    }
    #[doc = "Bit 2 - SPI underrun interrupt flag bit"]
    #[inline(always)]
    pub fn underrun_intf(&mut self) -> UnderrunIntfW<'_, IntstatSpec> {
        UnderrunIntfW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive overrun error interrupt flag bit"]
    #[inline(always)]
    pub fn rxoerr_intf(&mut self) -> RxoerrIntfW<'_, IntstatSpec> {
        RxoerrIntfW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive data match the RXDNR number"]
    #[inline(always)]
    pub fn rxmatch_intf(&mut self) -> RxmatchIntfW<'_, IntstatSpec> {
        RxmatchIntfW::new(self, 4)
    }
    #[doc = "Bit 5 - RX FIFO full interrupt flag bit"]
    #[inline(always)]
    pub fn rxfull_intf(&mut self) -> RxfullIntfW<'_, IntstatSpec> {
        RxfullIntfW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter empty interrupt flag bit"]
    #[inline(always)]
    pub fn txept_intf(&mut self) -> TxeptIntfW<'_, IntstatSpec> {
        TxeptIntfW::new(self, 6)
    }
}
#[doc = "INTSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {}
