#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "Field `EN2` reader - enable Control bit"]
pub type En2R = crate::BitReader;
#[doc = "Field `EN2` writer - enable Control bit"]
pub type En2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEP2` reader - idel state polority"]
pub type Idlep2R = crate::BitReader;
#[doc = "Field `IDLEP2` writer - idel state polority"]
pub type Idlep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSEL2` reader - input signal select"]
pub type Insel2R = crate::BitReader;
#[doc = "Field `INSEL2` writer - input signal select"]
pub type Insel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN2` reader - DMA enable bit"]
pub type Dmaen2R = crate::BitReader;
#[doc = "Field `DMAEN2` writer - DMA enable bit"]
pub type Dmaen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSEL2` reader - TXandRX data select control"]
pub type Txsel2R = crate::BitReader;
#[doc = "Field `TXSEL2` writer - TXandRX data select control"]
pub type Txsel2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable Control bit"]
    #[inline(always)]
    pub fn en2(&self) -> En2R {
        En2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - idel state polority"]
    #[inline(always)]
    pub fn idlep2(&self) -> Idlep2R {
        Idlep2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - input signal select"]
    #[inline(always)]
    pub fn insel2(&self) -> Insel2R {
        Insel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen2(&self) -> Dmaen2R {
        Dmaen2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXandRX data select control"]
    #[inline(always)]
    pub fn txsel2(&self) -> Txsel2R {
        Txsel2R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable Control bit"]
    #[inline(always)]
    pub fn en2(&mut self) -> En2W<'_, Ctl2Spec> {
        En2W::new(self, 0)
    }
    #[doc = "Bit 1 - idel state polority"]
    #[inline(always)]
    pub fn idlep2(&mut self) -> Idlep2W<'_, Ctl2Spec> {
        Idlep2W::new(self, 1)
    }
    #[doc = "Bit 2 - input signal select"]
    #[inline(always)]
    pub fn insel2(&mut self) -> Insel2W<'_, Ctl2Spec> {
        Insel2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen2(&mut self) -> Dmaen2W<'_, Ctl2Spec> {
        Dmaen2W::new(self, 3)
    }
    #[doc = "Bit 4 - TXandRX data select control"]
    #[inline(always)]
    pub fn txsel2(&mut self) -> Txsel2W<'_, Ctl2Spec> {
        Txsel2W::new(self, 4)
    }
}
#[doc = "Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL2 to value 0x08"]
impl crate::Resettable for Ctl2Spec {
    const RESET_VALUE: u32 = 0x08;
}
