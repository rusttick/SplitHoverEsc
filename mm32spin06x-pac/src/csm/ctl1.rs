#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `EN1` reader - enable Control bit"]
pub type En1R = crate::BitReader;
#[doc = "Field `EN1` writer - enable Control bit"]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEP1` reader - idel state polority"]
pub type Idlep1R = crate::BitReader;
#[doc = "Field `IDLEP1` writer - idel state polority"]
pub type Idlep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSEL1` reader - input signal select"]
pub type Insel1R = crate::BitReader;
#[doc = "Field `INSEL1` writer - input signal select"]
pub type Insel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN1` reader - DMA enable bit"]
pub type Dmaen1R = crate::BitReader;
#[doc = "Field `DMAEN1` writer - DMA enable bit"]
pub type Dmaen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSEL1` reader - TXandRX data select control"]
pub type Txsel1R = crate::BitReader;
#[doc = "Field `TXSEL1` writer - TXandRX data select control"]
pub type Txsel1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable Control bit"]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - idel state polority"]
    #[inline(always)]
    pub fn idlep1(&self) -> Idlep1R {
        Idlep1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - input signal select"]
    #[inline(always)]
    pub fn insel1(&self) -> Insel1R {
        Insel1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen1(&self) -> Dmaen1R {
        Dmaen1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXandRX data select control"]
    #[inline(always)]
    pub fn txsel1(&self) -> Txsel1R {
        Txsel1R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable Control bit"]
    #[inline(always)]
    pub fn en1(&mut self) -> En1W<'_, Ctl1Spec> {
        En1W::new(self, 0)
    }
    #[doc = "Bit 1 - idel state polority"]
    #[inline(always)]
    pub fn idlep1(&mut self) -> Idlep1W<'_, Ctl1Spec> {
        Idlep1W::new(self, 1)
    }
    #[doc = "Bit 2 - input signal select"]
    #[inline(always)]
    pub fn insel1(&mut self) -> Insel1W<'_, Ctl1Spec> {
        Insel1W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen1(&mut self) -> Dmaen1W<'_, Ctl1Spec> {
        Dmaen1W::new(self, 3)
    }
    #[doc = "Bit 4 - TXandRX data select control"]
    #[inline(always)]
    pub fn txsel1(&mut self) -> Txsel1W<'_, Ctl1Spec> {
        Txsel1W::new(self, 4)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0x08"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0x08;
}
