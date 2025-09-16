#[doc = "Register `RTC_DIVH` reader"]
pub type R = crate::R<RtcDivhSpec>;
#[doc = "Field `DIV` reader - RTC clock divider high"]
pub type DivR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - RTC clock divider high"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "RTC prescaler divider factor high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_divh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcDivhSpec;
impl crate::RegisterSpec for RtcDivhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtc_divh::R`](R) reader structure"]
impl crate::Readable for RtcDivhSpec {}
#[doc = "`reset()` method sets RTC_DIVH to value 0"]
impl crate::Resettable for RtcDivhSpec {}
