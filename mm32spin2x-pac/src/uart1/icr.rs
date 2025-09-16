#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `TXICLR` writer - Transmit buffer empty interrupt clear bit"]
pub type TxiclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXICLR` writer - Receive interrupt clear bit"]
pub type RxiclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC_CLR` writer - Transmit complete interrupt clear bit"]
pub type TxcClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOERRCLR` writer - Receive overflow error interrupt clear bit"]
pub type RxoerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPERRCLR` writer - Parity error interrupt clear bit"]
pub type RxperrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFERRCLR` writer - Frame error interrupt clear bit"]
pub type RxferrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRKCLR` writer - Receive frame break interrupt clear bit"]
pub type RxbrkclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBRK_CLR` writer - Transmit Break Frame Interrupt clear Bit"]
pub type TxbrkClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXB8_CLR` writer - Receive Bit 8 Interrupt clear Bit"]
pub type Rxb8ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmit buffer empty interrupt clear bit"]
    #[inline(always)]
    pub fn txiclr(&mut self) -> TxiclrW<'_, IcrSpec> {
        TxiclrW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive interrupt clear bit"]
    #[inline(always)]
    pub fn rxiclr(&mut self) -> RxiclrW<'_, IcrSpec> {
        RxiclrW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit complete interrupt clear bit"]
    #[inline(always)]
    pub fn txc_clr(&mut self) -> TxcClrW<'_, IcrSpec> {
        TxcClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt clear bit"]
    #[inline(always)]
    pub fn rxoerrclr(&mut self) -> RxoerrclrW<'_, IcrSpec> {
        RxoerrclrW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity error interrupt clear bit"]
    #[inline(always)]
    pub fn rxperrclr(&mut self) -> RxperrclrW<'_, IcrSpec> {
        RxperrclrW::new(self, 4)
    }
    #[doc = "Bit 5 - Frame error interrupt clear bit"]
    #[inline(always)]
    pub fn rxferrclr(&mut self) -> RxferrclrW<'_, IcrSpec> {
        RxferrclrW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive frame break interrupt clear bit"]
    #[inline(always)]
    pub fn rxbrkclr(&mut self) -> RxbrkclrW<'_, IcrSpec> {
        RxbrkclrW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt clear Bit"]
    #[inline(always)]
    pub fn txbrk_clr(&mut self) -> TxbrkClrW<'_, IcrSpec> {
        TxbrkClrW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    pub fn rxb8_clr(&mut self) -> Rxb8ClrW<'_, IcrSpec> {
        Rxb8ClrW::new(self, 8)
    }
}
#[doc = "ICR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
