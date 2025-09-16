#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Field `PEN` reader - Parity enable bit"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Parity enable bit"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSEL` reader - Parity selection bit"]
pub type PselR = crate::BitReader;
#[doc = "Field `PSEL` writer - Parity selection bit"]
pub type PselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPB0` reader - Stop bit 0 selection"]
pub type Spb0R = crate::BitReader;
#[doc = "Field `SPB0` writer - Stop bit 0 selection"]
pub type Spb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK` reader - UART transmit frame break"]
pub type BrkR = crate::BitReader;
#[doc = "Field `BRK` writer - UART transmit frame break"]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAR` reader - UART width bit"]
pub type CharR = crate::FieldReader;
#[doc = "Field `CHAR` writer - UART width bit"]
pub type CharW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPB1` reader - Stop bit 1 selection bit"]
pub type Spb1R = crate::BitReader;
#[doc = "Field `SPB1` writer - Stop bit 1 selection bit"]
pub type Spb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B8RXD` reader - Synchronous frame receive"]
pub type B8rxdR = crate::BitReader;
#[doc = "Field `B8RXD` writer - Synchronous frame receive"]
pub type B8rxdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B8TXD` reader - Synchronous frame transmit"]
pub type B8txdR = crate::BitReader;
#[doc = "Field `B8TXD` writer - Synchronous frame transmit"]
pub type B8txdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B8POL` reader - Synchronous frame polarity control bit"]
pub type B8polR = crate::BitReader;
#[doc = "Field `B8POL` writer - Synchronous frame polarity control bit"]
pub type B8polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B8TOG` reader - Synchronous frame auto toggle bit"]
pub type B8togR = crate::BitReader;
#[doc = "Field `B8TOG` writer - Synchronous frame auto toggle bit"]
pub type B8togW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B8EN` reader - Synchronous frame enable bit"]
pub type B8enR = crate::BitReader;
#[doc = "Field `B8EN` writer - Synchronous frame enable bit"]
pub type B8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWU` reader - Receive wake up method"]
pub type RwuR = crate::BitReader;
#[doc = "Field `RWU` writer - Receive wake up method"]
pub type RwuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE` reader - Wake up method"]
pub type WakeR = crate::BitReader;
#[doc = "Field `WAKE` writer - Wake up method"]
pub type WakeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity enable bit"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity selection bit"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop bit 0 selection"]
    #[inline(always)]
    pub fn spb0(&self) -> Spb0R {
        Spb0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART transmit frame break"]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - UART width bit"]
    #[inline(always)]
    pub fn char(&self) -> CharR {
        CharR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Stop bit 1 selection bit"]
    #[inline(always)]
    pub fn spb1(&self) -> Spb1R {
        Spb1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronous frame receive"]
    #[inline(always)]
    pub fn b8rxd(&self) -> B8rxdR {
        B8rxdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronous frame transmit"]
    #[inline(always)]
    pub fn b8txd(&self) -> B8txdR {
        B8txdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Synchronous frame polarity control bit"]
    #[inline(always)]
    pub fn b8pol(&self) -> B8polR {
        B8polR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronous frame auto toggle bit"]
    #[inline(always)]
    pub fn b8tog(&self) -> B8togR {
        B8togR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronous frame enable bit"]
    #[inline(always)]
    pub fn b8en(&self) -> B8enR {
        B8enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive wake up method"]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wake up method"]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity enable bit"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<'_, CcrSpec> {
        PenW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity selection bit"]
    #[inline(always)]
    pub fn psel(&mut self) -> PselW<'_, CcrSpec> {
        PselW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop bit 0 selection"]
    #[inline(always)]
    pub fn spb0(&mut self) -> Spb0W<'_, CcrSpec> {
        Spb0W::new(self, 2)
    }
    #[doc = "Bit 3 - UART transmit frame break"]
    #[inline(always)]
    pub fn brk(&mut self) -> BrkW<'_, CcrSpec> {
        BrkW::new(self, 3)
    }
    #[doc = "Bits 4:5 - UART width bit"]
    #[inline(always)]
    pub fn char(&mut self) -> CharW<'_, CcrSpec> {
        CharW::new(self, 4)
    }
    #[doc = "Bit 6 - Stop bit 1 selection bit"]
    #[inline(always)]
    pub fn spb1(&mut self) -> Spb1W<'_, CcrSpec> {
        Spb1W::new(self, 6)
    }
    #[doc = "Bit 7 - Synchronous frame receive"]
    #[inline(always)]
    pub fn b8rxd(&mut self) -> B8rxdW<'_, CcrSpec> {
        B8rxdW::new(self, 7)
    }
    #[doc = "Bit 8 - Synchronous frame transmit"]
    #[inline(always)]
    pub fn b8txd(&mut self) -> B8txdW<'_, CcrSpec> {
        B8txdW::new(self, 8)
    }
    #[doc = "Bit 9 - Synchronous frame polarity control bit"]
    #[inline(always)]
    pub fn b8pol(&mut self) -> B8polW<'_, CcrSpec> {
        B8polW::new(self, 9)
    }
    #[doc = "Bit 10 - Synchronous frame auto toggle bit"]
    #[inline(always)]
    pub fn b8tog(&mut self) -> B8togW<'_, CcrSpec> {
        B8togW::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronous frame enable bit"]
    #[inline(always)]
    pub fn b8en(&mut self) -> B8enW<'_, CcrSpec> {
        B8enW::new(self, 11)
    }
    #[doc = "Bit 12 - Receive wake up method"]
    #[inline(always)]
    pub fn rwu(&mut self) -> RwuW<'_, CcrSpec> {
        RwuW::new(self, 12)
    }
    #[doc = "Bit 13 - Wake up method"]
    #[inline(always)]
    pub fn wake(&mut self) -> WakeW<'_, CcrSpec> {
        WakeW::new(self, 13)
    }
}
#[doc = "common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR to value 0x30"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0x30;
}
