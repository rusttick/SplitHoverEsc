#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `DBG_SLEEP` writer - Debug Sleep mode"]
pub type DbgSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` writer - Debug Stop mode"]
pub type DbgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STANDBY` writer - Debug Standby mode"]
pub type DbgStandbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug independent watchdog stopped when core is stopped"]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` writer - Debug window watchdog when core is halted"]
pub type DbgWwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIMx_STOP` writer - TIMx counter stopped when core is halted"]
pub type DbgTimxStopW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bit 0 - Debug Sleep mode"]
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DbgSleepW<'_, CrSpec> {
        DbgSleepW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Stop mode"]
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DbgStopW<'_, CrSpec> {
        DbgStopW::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Standby mode"]
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DbgStandbyW<'_, CrSpec> {
        DbgStandbyW::new(self, 2)
    }
    #[doc = "Bit 8 - Debug independent watchdog stopped when core is stopped"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<'_, CrSpec> {
        DbgIwdgStopW::new(self, 8)
    }
    #[doc = "Bit 9 - Debug window watchdog when core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DbgWwdgStopW<'_, CrSpec> {
        DbgWwdgStopW::new(self, 9)
    }
    #[doc = "Bits 10:13 - TIMx counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_timx_stop(&mut self) -> DbgTimxStopW<'_, CrSpec> {
        DbgTimxStopW::new(self, 10)
    }
}
#[doc = "CR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
