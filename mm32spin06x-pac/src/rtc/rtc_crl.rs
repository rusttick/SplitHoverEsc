#[doc = "Register `RTC_CRL` reader"]
pub type R = crate::R<RtcCrlSpec>;
#[doc = "Register `RTC_CRL` writer"]
pub type W = crate::W<RtcCrlSpec>;
#[doc = "Field `SECF` reader - Second flag"]
pub type SecfR = crate::BitReader;
#[doc = "Field `SECF` writer - Second flag"]
pub type SecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRF` reader - Alarm flag"]
pub type AlrfR = crate::BitReader;
#[doc = "Field `ALRF` writer - Alarm flag"]
pub type AlrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OWF` reader - Overflow flag"]
pub type OwfR = crate::BitReader;
#[doc = "Field `OWF` writer - Overflow flag"]
pub type OwfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - Registers synchronized flag"]
pub type RsfR = crate::BitReader;
#[doc = "Field `RSF` writer - Registers synchronized flag"]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNF` reader - Configuration flag"]
pub type CnfR = crate::BitReader;
#[doc = "Field `CNF` writer - Configuration flag"]
pub type CnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOFF` reader - RTC operation OFF"]
pub type RtoffR = crate::BitReader;
#[doc = "Field `RTOFF` writer - RTC operation OFF"]
pub type RtoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALPEN` reader - RTC alarm loop enable"]
pub type AlpenR = crate::BitReader;
#[doc = "Field `ALPEN` writer - RTC alarm loop enable"]
pub type AlpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Second flag"]
    #[inline(always)]
    pub fn secf(&self) -> SecfR {
        SecfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm flag"]
    #[inline(always)]
    pub fn alrf(&self) -> AlrfR {
        AlrfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow flag"]
    #[inline(always)]
    pub fn owf(&self) -> OwfR {
        OwfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration flag"]
    #[inline(always)]
    pub fn cnf(&self) -> CnfR {
        CnfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC operation OFF"]
    #[inline(always)]
    pub fn rtoff(&self) -> RtoffR {
        RtoffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC alarm loop enable"]
    #[inline(always)]
    pub fn alpen(&self) -> AlpenR {
        AlpenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second flag"]
    #[inline(always)]
    pub fn secf(&mut self) -> SecfW<'_, RtcCrlSpec> {
        SecfW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm flag"]
    #[inline(always)]
    pub fn alrf(&mut self) -> AlrfW<'_, RtcCrlSpec> {
        AlrfW::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow flag"]
    #[inline(always)]
    pub fn owf(&mut self) -> OwfW<'_, RtcCrlSpec> {
        OwfW::new(self, 2)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RsfW<'_, RtcCrlSpec> {
        RsfW::new(self, 3)
    }
    #[doc = "Bit 4 - Configuration flag"]
    #[inline(always)]
    pub fn cnf(&mut self) -> CnfW<'_, RtcCrlSpec> {
        CnfW::new(self, 4)
    }
    #[doc = "Bit 5 - RTC operation OFF"]
    #[inline(always)]
    pub fn rtoff(&mut self) -> RtoffW<'_, RtcCrlSpec> {
        RtoffW::new(self, 5)
    }
    #[doc = "Bit 6 - RTC alarm loop enable"]
    #[inline(always)]
    pub fn alpen(&mut self) -> AlpenW<'_, RtcCrlSpec> {
        AlpenW::new(self, 6)
    }
}
#[doc = "RTC configuration low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_crl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_crl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCrlSpec;
impl crate::RegisterSpec for RtcCrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtc_crl::R`](R) reader structure"]
impl crate::Readable for RtcCrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_crl::W`](W) writer structure"]
impl crate::Writable for RtcCrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CRL to value 0x20"]
impl crate::Resettable for RtcCrlSpec {
    const RESET_VALUE: u16 = 0x20;
}
