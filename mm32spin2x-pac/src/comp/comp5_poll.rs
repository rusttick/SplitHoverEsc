#[doc = "Register `COMP5_POLL` reader"]
pub type R = crate::R<Comp5PollSpec>;
#[doc = "Register `COMP5_POLL` writer"]
pub type W = crate::W<Comp5PollSpec>;
#[doc = "Field `EN` reader - Comparator 5 polling enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Comparator 5 polling enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH` reader - Comparator 5 polling channel"]
pub type ChR = crate::BitReader;
#[doc = "Field `CH` writer - Comparator 5 polling channel"]
pub type ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIXN` reader - Comparator 5 Polling inverting input fix"]
pub type FixnR = crate::BitReader;
#[doc = "Field `FIXN` writer - Comparator 5 Polling inverting input fix"]
pub type FixnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIOD` reader - Comparator 5 polling wait cycle"]
pub type PeriodR = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Comparator 5 polling wait cycle"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POUT` reader - Comparator 5 Polling output"]
pub type PoutR = crate::FieldReader;
#[doc = "Field `POUT` writer - Comparator 5 Polling output"]
pub type PoutW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Comparator 5 polling enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 5 polling channel"]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 5 Polling inverting input fix"]
    #[inline(always)]
    pub fn fixn(&self) -> FixnR {
        FixnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 5 polling wait cycle"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 5 Polling output"]
    #[inline(always)]
    pub fn pout(&self) -> PoutR {
        PoutR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 5 polling enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Comp5PollSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 5 polling channel"]
    #[inline(always)]
    pub fn ch(&mut self) -> ChW<'_, Comp5PollSpec> {
        ChW::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator 5 Polling inverting input fix"]
    #[inline(always)]
    pub fn fixn(&mut self) -> FixnW<'_, Comp5PollSpec> {
        FixnW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 5 polling wait cycle"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, Comp5PollSpec> {
        PeriodW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Comparator 5 Polling output"]
    #[inline(always)]
    pub fn pout(&mut self) -> PoutW<'_, Comp5PollSpec> {
        PoutW::new(self, 8)
    }
}
#[doc = "COMP5_POLL\n\nYou can [`read`](crate::Reg::read) this register and get [`comp5_poll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp5_poll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp5PollSpec;
impl crate::RegisterSpec for Comp5PollSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp5_poll::R`](R) reader structure"]
impl crate::Readable for Comp5PollSpec {}
#[doc = "`write(|w| ..)` method takes [`comp5_poll::W`](W) writer structure"]
impl crate::Writable for Comp5PollSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP5_POLL to value 0"]
impl crate::Resettable for Comp5PollSpec {}
