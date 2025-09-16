#[doc = "Register `RTC_DIVL` reader"]
pub type R = crate::R<RtcDivlSpec>;
#[doc = "Field `DIV` reader - RTC clock divider low"]
pub type DivR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RTC clock divider low"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(self.bits)
    }
}
#[doc = "RTC prescaler divider factor low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_divl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcDivlSpec;
impl crate::RegisterSpec for RtcDivlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtc_divl::R`](R) reader structure"]
impl crate::Readable for RtcDivlSpec {}
#[doc = "`reset()` method sets RTC_DIVL to value 0"]
impl crate::Resettable for RtcDivlSpec {}
