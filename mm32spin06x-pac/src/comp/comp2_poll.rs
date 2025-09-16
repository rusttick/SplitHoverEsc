#[doc = "Register `COMP2_POLL` reader"]
pub type R = crate::R<Comp2PollSpec>;
#[doc = "Register `COMP2_POLL` writer"]
pub type W = crate::W<Comp2PollSpec>;
#[doc = "Field `POLL_EN` reader - Comparator 2 polling enable"]
pub type PollEnR = crate::BitReader;
#[doc = "Field `POLL_EN` writer - Comparator 2 polling enable"]
pub type PollEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLL_CH` reader - Comparator 2 polling channel"]
pub type PollChR = crate::BitReader;
#[doc = "Field `POLL_CH` writer - Comparator 2 polling channel"]
pub type PollChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIXN` reader - Comparator 2 Polling inverting input fix"]
pub type FixnR = crate::BitReader;
#[doc = "Field `FIXN` writer - Comparator 2 Polling inverting input fix"]
pub type FixnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIOD` reader - Comparator 2 polling wait cycle"]
pub type PeriodR = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Comparator 2 polling wait cycle"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POUT` reader - Comparator 2 Polling output"]
pub type PoutR = crate::FieldReader;
#[doc = "Field `POUT` writer - Comparator 2 Polling output"]
pub type PoutW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Comparator 2 polling enable"]
    #[inline(always)]
    pub fn poll_en(&self) -> PollEnR {
        PollEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 2 polling channel"]
    #[inline(always)]
    pub fn poll_ch(&self) -> PollChR {
        PollChR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 Polling inverting input fix"]
    #[inline(always)]
    pub fn fixn(&self) -> FixnR {
        FixnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 2 polling wait cycle"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 2 Polling output"]
    #[inline(always)]
    pub fn pout(&self) -> PoutR {
        PoutR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 polling enable"]
    #[inline(always)]
    pub fn poll_en(&mut self) -> PollEnW<'_, Comp2PollSpec> {
        PollEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 2 polling channel"]
    #[inline(always)]
    pub fn poll_ch(&mut self) -> PollChW<'_, Comp2PollSpec> {
        PollChW::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator 2 Polling inverting input fix"]
    #[inline(always)]
    pub fn fixn(&mut self) -> FixnW<'_, Comp2PollSpec> {
        FixnW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 2 polling wait cycle"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, Comp2PollSpec> {
        PeriodW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Comparator 2 Polling output"]
    #[inline(always)]
    pub fn pout(&mut self) -> PoutW<'_, Comp2PollSpec> {
        PoutW::new(self, 8)
    }
}
#[doc = "COMP2 Polling Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp2_poll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_poll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp2PollSpec;
impl crate::RegisterSpec for Comp2PollSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp2_poll::R`](R) reader structure"]
impl crate::Readable for Comp2PollSpec {}
#[doc = "`write(|w| ..)` method takes [`comp2_poll::W`](W) writer structure"]
impl crate::Writable for Comp2PollSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP2_POLL to value 0"]
impl crate::Resettable for Comp2PollSpec {}
