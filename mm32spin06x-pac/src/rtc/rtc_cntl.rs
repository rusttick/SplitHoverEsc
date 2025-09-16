#[doc = "Register `RTC_CNTL` reader"]
pub type R = crate::R<RtcCntlSpec>;
#[doc = "Register `RTC_CNTL` writer"]
pub type W = crate::W<RtcCntlSpec>;
#[doc = "Field `CNT` reader - RTC counter low"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - RTC counter low"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC counter low"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC counter low"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, RtcCntlSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "RTC counter low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCntlSpec;
impl crate::RegisterSpec for RtcCntlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtc_cntl::R`](R) reader structure"]
impl crate::Readable for RtcCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_cntl::W`](W) writer structure"]
impl crate::Writable for RtcCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CNTL to value 0"]
impl crate::Resettable for RtcCntlSpec {}
