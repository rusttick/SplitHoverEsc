#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `TXIEN` reader - Transmit buffer empty interrupt enable bit"]
pub type TxienR = crate::BitReader;
#[doc = "Field `TXIEN` writer - Transmit buffer empty interrupt enable bit"]
pub type TxienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIEN` reader - Receive buffer interrupt enable bit"]
pub type RxienR = crate::BitReader;
#[doc = "Field `RXIEN` writer - Receive buffer interrupt enable bit"]
pub type RxienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC_IEN` reader - Transmit complete interrupt enable bit"]
pub type TxcIenR = crate::BitReader;
#[doc = "Field `TXC_IEN` writer - Transmit complete interrupt enable bit"]
pub type TxcIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOERREN` reader - Receive overflow error interrupt enable bit"]
pub type RxoerrenR = crate::BitReader;
#[doc = "Field `RXOERREN` writer - Receive overflow error interrupt enable bit"]
pub type RxoerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPERREN` reader - Parity error interrupt enable bit"]
pub type RxperrenR = crate::BitReader;
#[doc = "Field `RXPERREN` writer - Parity error interrupt enable bit"]
pub type RxperrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFERREN` reader - Frame error interrupt enable bit"]
pub type RxferrenR = crate::BitReader;
#[doc = "Field `RXFERREN` writer - Frame error interrupt enable bit"]
pub type RxferrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRKEN` reader - Receive frame break interrupt enable bit"]
pub type RxbrkenR = crate::BitReader;
#[doc = "Field `RXBRKEN` writer - Receive frame break interrupt enable bit"]
pub type RxbrkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBRK_IEN` reader - Transmit Break Frame Interrupt Enable Bit"]
pub type TxbrkIenR = crate::BitReader;
#[doc = "Field `TXBRK_IEN` writer - Transmit Break Frame Interrupt Enable Bit"]
pub type TxbrkIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXB8_IEN` reader - Receive Bit 8 Interrupt Enable Bit"]
pub type Rxb8IenR = crate::BitReader;
#[doc = "Field `RXB8_IEN` writer - Receive Bit 8 Interrupt Enable Bit"]
pub type Rxb8IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIDLEN` reader - Receive frame idle interrupt enable bit"]
pub type RxidlenR = crate::BitReader;
#[doc = "Field `RXIDLEN` writer - Receive frame idle interrupt enable bit"]
pub type RxidlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABREND_IEN` reader - Automatic baud rate end interrupt enable"]
pub type AbrendIenR = crate::BitReader;
#[doc = "Field `ABREND_IEN` writer - Automatic baud rate end interrupt enable"]
pub type AbrendIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRERR_IEN` reader - Automatic baud rate error interrupt enable"]
pub type AbrerrIenR = crate::BitReader;
#[doc = "Field `ABRERR_IEN` writer - Automatic baud rate error interrupt enable"]
pub type AbrerrIenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit buffer empty interrupt enable bit"]
    #[inline(always)]
    pub fn txien(&self) -> TxienR {
        TxienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive buffer interrupt enable bit"]
    #[inline(always)]
    pub fn rxien(&self) -> RxienR {
        RxienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit complete interrupt enable bit"]
    #[inline(always)]
    pub fn txc_ien(&self) -> TxcIenR {
        TxcIenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt enable bit"]
    #[inline(always)]
    pub fn rxoerren(&self) -> RxoerrenR {
        RxoerrenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error interrupt enable bit"]
    #[inline(always)]
    pub fn rxperren(&self) -> RxperrenR {
        RxperrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame error interrupt enable bit"]
    #[inline(always)]
    pub fn rxferren(&self) -> RxferrenR {
        RxferrenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive frame break interrupt enable bit"]
    #[inline(always)]
    pub fn rxbrken(&self) -> RxbrkenR {
        RxbrkenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt Enable Bit"]
    #[inline(always)]
    pub fn txbrk_ien(&self) -> TxbrkIenR {
        TxbrkIenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxb8_ien(&self) -> Rxb8IenR {
        Rxb8IenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive frame idle interrupt enable bit"]
    #[inline(always)]
    pub fn rxidlen(&self) -> RxidlenR {
        RxidlenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic baud rate end interrupt enable"]
    #[inline(always)]
    pub fn abrend_ien(&self) -> AbrendIenR {
        AbrendIenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Automatic baud rate error interrupt enable"]
    #[inline(always)]
    pub fn abrerr_ien(&self) -> AbrerrIenR {
        AbrerrIenR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit buffer empty interrupt enable bit"]
    #[inline(always)]
    pub fn txien(&mut self) -> TxienW<'_, IerSpec> {
        TxienW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive buffer interrupt enable bit"]
    #[inline(always)]
    pub fn rxien(&mut self) -> RxienW<'_, IerSpec> {
        RxienW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit complete interrupt enable bit"]
    #[inline(always)]
    pub fn txc_ien(&mut self) -> TxcIenW<'_, IerSpec> {
        TxcIenW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt enable bit"]
    #[inline(always)]
    pub fn rxoerren(&mut self) -> RxoerrenW<'_, IerSpec> {
        RxoerrenW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity error interrupt enable bit"]
    #[inline(always)]
    pub fn rxperren(&mut self) -> RxperrenW<'_, IerSpec> {
        RxperrenW::new(self, 4)
    }
    #[doc = "Bit 5 - Frame error interrupt enable bit"]
    #[inline(always)]
    pub fn rxferren(&mut self) -> RxferrenW<'_, IerSpec> {
        RxferrenW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive frame break interrupt enable bit"]
    #[inline(always)]
    pub fn rxbrken(&mut self) -> RxbrkenW<'_, IerSpec> {
        RxbrkenW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt Enable Bit"]
    #[inline(always)]
    pub fn txbrk_ien(&mut self) -> TxbrkIenW<'_, IerSpec> {
        TxbrkIenW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rxb8_ien(&mut self) -> Rxb8IenW<'_, IerSpec> {
        Rxb8IenW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive frame idle interrupt enable bit"]
    #[inline(always)]
    pub fn rxidlen(&mut self) -> RxidlenW<'_, IerSpec> {
        RxidlenW::new(self, 9)
    }
    #[doc = "Bit 10 - Automatic baud rate end interrupt enable"]
    #[inline(always)]
    pub fn abrend_ien(&mut self) -> AbrendIenW<'_, IerSpec> {
        AbrendIenW::new(self, 10)
    }
    #[doc = "Bit 11 - Automatic baud rate error interrupt enable"]
    #[inline(always)]
    pub fn abrerr_ien(&mut self) -> AbrerrIenW<'_, IerSpec> {
        AbrerrIenW::new(self, 11)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
