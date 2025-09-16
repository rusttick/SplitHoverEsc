#[doc = "Register `RTC_ALRL` reader"]
pub type R = crate::R<RtcAlrlSpec>;
#[doc = "Register `RTC_ALRL` writer"]
pub type W = crate::W<RtcAlrlSpec>;
#[doc = "Field `ALR` reader - RTC alarm low"]
pub type AlrR = crate::FieldReader<u16>;
#[doc = "Field `ALR` writer - RTC alarm low"]
pub type AlrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC alarm low"]
    #[inline(always)]
    pub fn alr(&self) -> AlrR {
        AlrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm low"]
    #[inline(always)]
    pub fn alr(&mut self) -> AlrW<'_, RtcAlrlSpec> {
        AlrW::new(self, 0)
    }
}
#[doc = "RTC alarm low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAlrlSpec;
impl crate::RegisterSpec for RtcAlrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtc_alrl::R`](R) reader structure"]
impl crate::Readable for RtcAlrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_alrl::W`](W) writer structure"]
impl crate::Writable for RtcAlrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_ALRL to value 0xffff"]
impl crate::Resettable for RtcAlrlSpec {
    const RESET_VALUE: u16 = 0xffff;
}
