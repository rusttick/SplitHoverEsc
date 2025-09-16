#[doc = "Register `RTC_CRH` reader"]
pub type R = crate::R<RtcCrhSpec>;
#[doc = "Register `RTC_CRH` writer"]
pub type W = crate::W<RtcCrhSpec>;
#[doc = "Field `SECIE` reader - Second interrupt enable"]
pub type SecieR = crate::BitReader;
#[doc = "Field `SECIE` writer - Second interrupt enable"]
pub type SecieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRIE` reader - Alarm interrupt enable"]
pub type AlrieR = crate::BitReader;
#[doc = "Field `ALRIE` writer - Alarm interrupt enable"]
pub type AlrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OWIE` reader - Overflow interrupt enable"]
pub type OwieR = crate::BitReader;
#[doc = "Field `OWIE` writer - Overflow interrupt enable"]
pub type OwieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Second interrupt enable"]
    #[inline(always)]
    pub fn secie(&self) -> SecieR {
        SecieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alrie(&self) -> AlrieR {
        AlrieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn owie(&self) -> OwieR {
        OwieR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second interrupt enable"]
    #[inline(always)]
    pub fn secie(&mut self) -> SecieW<'_, RtcCrhSpec> {
        SecieW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alrie(&mut self) -> AlrieW<'_, RtcCrhSpec> {
        AlrieW::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn owie(&mut self) -> OwieW<'_, RtcCrhSpec> {
        OwieW::new(self, 2)
    }
}
#[doc = "RTC configuration high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_crh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_crh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCrhSpec;
impl crate::RegisterSpec for RtcCrhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtc_crh::R`](R) reader structure"]
impl crate::Readable for RtcCrhSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_crh::W`](W) writer structure"]
impl crate::Writable for RtcCrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CRH to value 0"]
impl crate::Resettable for RtcCrhSpec {}
