#[doc = "Register `COMP4_POLL` reader"]
pub type R = crate::R<Comp4PollSpec>;
#[doc = "Register `COMP4_POLL` writer"]
pub type W = crate::W<Comp4PollSpec>;
#[doc = "Field `EN` reader - Comparator 4 polling enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Comparator 4 polling enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH` reader - Comparator 4 polling channel"]
pub type ChR = crate::BitReader;
#[doc = "Field `CH` writer - Comparator 4 polling channel"]
pub type ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIXN` reader - Comparator 4 Polling inverting input fix"]
pub type FixnR = crate::BitReader;
#[doc = "Field `FIXN` writer - Comparator 4 Polling inverting input fix"]
pub type FixnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIOD` reader - Comparator 4 polling wait cycle"]
pub type PeriodR = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Comparator 4 polling wait cycle"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POUT` reader - Comparator 4 Polling output"]
pub type PoutR = crate::FieldReader;
#[doc = "Field `POUT` writer - Comparator 4 Polling output"]
pub type PoutW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Comparator 4 polling enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 4 polling channel"]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 4 Polling inverting input fix"]
    #[inline(always)]
    pub fn fixn(&self) -> FixnR {
        FixnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 4 polling wait cycle"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 4 Polling output"]
    #[inline(always)]
    pub fn pout(&self) -> PoutR {
        PoutR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 4 polling enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Comp4PollSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 4 polling channel"]
    #[inline(always)]
    pub fn ch(&mut self) -> ChW<'_, Comp4PollSpec> {
        ChW::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator 4 Polling inverting input fix"]
    #[inline(always)]
    pub fn fixn(&mut self) -> FixnW<'_, Comp4PollSpec> {
        FixnW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 4 polling wait cycle"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, Comp4PollSpec> {
        PeriodW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Comparator 4 Polling output"]
    #[inline(always)]
    pub fn pout(&mut self) -> PoutW<'_, Comp4PollSpec> {
        PoutW::new(self, 8)
    }
}
#[doc = "COMP4_POLL\n\nYou can [`read`](crate::Reg::read) this register and get [`comp4_poll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp4_poll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp4PollSpec;
impl crate::RegisterSpec for Comp4PollSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp4_poll::R`](R) reader structure"]
impl crate::Readable for Comp4PollSpec {}
#[doc = "`write(|w| ..)` method takes [`comp4_poll::W`](W) writer structure"]
impl crate::Writable for Comp4PollSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP4_POLL to value 0"]
impl crate::Resettable for Comp4PollSpec {}
