#[doc = "Register `COMP1_POLL` reader"]
pub type R = crate::R<Comp1PollSpec>;
#[doc = "Register `COMP1_POLL` writer"]
pub type W = crate::W<Comp1PollSpec>;
#[doc = "Field `EN` reader - Comparator 1 polling enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Comparator 1 polling enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH` reader - Comparator 1 polling channel"]
pub type ChR = crate::BitReader;
#[doc = "Field `CH` writer - Comparator 1 polling channel"]
pub type ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIXN` reader - Comparator 1 Polling inverting input fix"]
pub type FixnR = crate::BitReader;
#[doc = "Field `FIXN` writer - Comparator 1 Polling inverting input fix"]
pub type FixnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIOD` reader - Comparator 1 polling wait cycle"]
pub type PeriodR = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Comparator 1 polling wait cycle"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POUT` reader - Comparator 1 Polling output"]
pub type PoutR = crate::FieldReader;
#[doc = "Field `POUT` writer - Comparator 1 Polling output"]
pub type PoutW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Comparator 1 polling enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 polling channel"]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 1 Polling inverting input fix"]
    #[inline(always)]
    pub fn fixn(&self) -> FixnR {
        FixnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 1 polling wait cycle"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 1 Polling output"]
    #[inline(always)]
    pub fn pout(&self) -> PoutR {
        PoutR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 polling enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Comp1PollSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 polling channel"]
    #[inline(always)]
    pub fn ch(&mut self) -> ChW<'_, Comp1PollSpec> {
        ChW::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator 1 Polling inverting input fix"]
    #[inline(always)]
    pub fn fixn(&mut self) -> FixnW<'_, Comp1PollSpec> {
        FixnW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 1 polling wait cycle"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, Comp1PollSpec> {
        PeriodW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Comparator 1 Polling output"]
    #[inline(always)]
    pub fn pout(&mut self) -> PoutW<'_, Comp1PollSpec> {
        PoutW::new(self, 8)
    }
}
#[doc = "COMP1_POLL\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_poll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_poll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp1PollSpec;
impl crate::RegisterSpec for Comp1PollSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1_poll::R`](R) reader structure"]
impl crate::Readable for Comp1PollSpec {}
#[doc = "`write(|w| ..)` method takes [`comp1_poll::W`](W) writer structure"]
impl crate::Writable for Comp1PollSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP1_POLL to value 0"]
impl crate::Resettable for Comp1PollSpec {}
