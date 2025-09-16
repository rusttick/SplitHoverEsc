#[doc = "Register `RTC_PRLL` writer"]
pub type W = crate::W<RtcPrllSpec>;
#[doc = "Field `PRL` writer - RTC prescaler reload value low"]
pub type PrlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC prescaler reload value low"]
    #[inline(always)]
    pub fn prl(&mut self) -> PrlW<'_, RtcPrllSpec> {
        PrlW::new(self, 0)
    }
}
#[doc = "RTC Prescaler load low register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_prll::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcPrllSpec;
impl crate::RegisterSpec for RtcPrllSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`rtc_prll::W`](W) writer structure"]
impl crate::Writable for RtcPrllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_PRLL to value 0"]
impl crate::Resettable for RtcPrllSpec {}
