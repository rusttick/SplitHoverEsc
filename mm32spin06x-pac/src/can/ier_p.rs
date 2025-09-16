#[doc = "Register `IER_P` reader"]
pub type R = crate::R<IerPSpec>;
#[doc = "Register `IER_P` writer"]
pub type W = crate::W<IerPSpec>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOIE` reader - Data overrun interrupt enable"]
pub type DoieR = crate::BitReader;
#[doc = "Field `DOIE` writer - Data overrun interrupt enable"]
pub type DoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPIE` reader - Error passive interrupt enable"]
pub type EpieR = crate::BitReader;
#[doc = "Field `EPIE` writer - Error passive interrupt enable"]
pub type EpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIE` reader - Arbitration lost interrupt enable"]
pub type AlieR = crate::BitReader;
#[doc = "Field `ALIE` writer - Arbitration lost interrupt enable"]
pub type AlieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIE` reader - Bus error interrupt enable"]
pub type BeieR = crate::BitReader;
#[doc = "Field `BEIE` writer - Bus error interrupt enable"]
pub type BeieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data overrun interrupt enable"]
    #[inline(always)]
    pub fn doie(&self) -> DoieR {
        DoieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epie(&self) -> EpieR {
        EpieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn alie(&self) -> AlieR {
        AlieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus error interrupt enable"]
    #[inline(always)]
    pub fn beie(&self) -> BeieR {
        BeieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<'_, IerPSpec> {
        RieW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, IerPSpec> {
        TieW::new(self, 1)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<'_, IerPSpec> {
        EieW::new(self, 2)
    }
    #[doc = "Bit 3 - Data overrun interrupt enable"]
    #[inline(always)]
    pub fn doie(&mut self) -> DoieW<'_, IerPSpec> {
        DoieW::new(self, 3)
    }
    #[doc = "Bit 5 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epie(&mut self) -> EpieW<'_, IerPSpec> {
        EpieW::new(self, 5)
    }
    #[doc = "Bit 6 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn alie(&mut self) -> AlieW<'_, IerPSpec> {
        AlieW::new(self, 6)
    }
    #[doc = "Bit 7 - Bus error interrupt enable"]
    #[inline(always)]
    pub fn beie(&mut self) -> BeieW<'_, IerPSpec> {
        BeieW::new(self, 7)
    }
}
#[doc = "Peli Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerPSpec;
impl crate::RegisterSpec for IerPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier_p::R`](R) reader structure"]
impl crate::Readable for IerPSpec {}
#[doc = "`write(|w| ..)` method takes [`ier_p::W`](W) writer structure"]
impl crate::Writable for IerPSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER_P to value 0"]
impl crate::Resettable for IerPSpec {}
