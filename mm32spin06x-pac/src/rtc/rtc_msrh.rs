#[doc = "Register `RTC_MSRH` reader"]
pub type R = crate::R<RtcMsrhSpec>;
#[doc = "Register `RTC_MSRH` writer"]
pub type W = crate::W<RtcMsrhSpec>;
#[doc = "Field `MSR` reader - RTC msec high"]
pub type MsrR = crate::FieldReader;
#[doc = "Field `MSR` writer - RTC msec high"]
pub type MsrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RTC msec high"]
    #[inline(always)]
    pub fn msr(&self) -> MsrR {
        MsrR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC msec high"]
    #[inline(always)]
    pub fn msr(&mut self) -> MsrW<'_, RtcMsrhSpec> {
        MsrW::new(self, 0)
    }
}
#[doc = "RTC millisecond alarm high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_msrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_msrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcMsrhSpec;
impl crate::RegisterSpec for RtcMsrhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtc_msrh::R`](R) reader structure"]
impl crate::Readable for RtcMsrhSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_msrh::W`](W) writer structure"]
impl crate::Writable for RtcMsrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_MSRH to value 0"]
impl crate::Resettable for RtcMsrhSpec {}
