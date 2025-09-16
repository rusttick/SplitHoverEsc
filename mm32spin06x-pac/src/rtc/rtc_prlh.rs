#[doc = "Register `RTC_PRLH` writer"]
pub type W = crate::W<RtcPrlhSpec>;
#[doc = "Field `PRL` writer - RTC prescaler reload value high"]
pub type PrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - RTC prescaler reload value high"]
    #[inline(always)]
    pub fn prl(&mut self) -> PrlW<'_, RtcPrlhSpec> {
        PrlW::new(self, 0)
    }
}
#[doc = "RTC Prescaler load high register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_prlh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcPrlhSpec;
impl crate::RegisterSpec for RtcPrlhSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`rtc_prlh::W`](W) writer structure"]
impl crate::Writable for RtcPrlhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_PRLH to value 0"]
impl crate::Resettable for RtcPrlhSpec {}
