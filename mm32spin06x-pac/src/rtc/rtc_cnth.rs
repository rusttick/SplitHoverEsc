#[doc = "Register `RTC_CNTH` reader"]
pub type R = crate::R<RtcCnthSpec>;
#[doc = "Register `RTC_CNTH` writer"]
pub type W = crate::W<RtcCnthSpec>;
#[doc = "Field `CNT` reader - RTC counter high"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - RTC counter high"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC counter high"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC counter high"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, RtcCnthSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "RTC counter high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cnth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cnth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCnthSpec;
impl crate::RegisterSpec for RtcCnthSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtc_cnth::R`](R) reader structure"]
impl crate::Readable for RtcCnthSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_cnth::W`](W) writer structure"]
impl crate::Writable for RtcCnthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CNTH to value 0"]
impl crate::Resettable for RtcCnthSpec {}
