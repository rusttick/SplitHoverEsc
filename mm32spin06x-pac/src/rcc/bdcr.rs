#[doc = "Register `BDCR` reader"]
pub type R = crate::R<BdcrSpec>;
#[doc = "Register `BDCR` writer"]
pub type W = crate::W<BdcrSpec>;
#[doc = "Field `LSEON` reader - External low-speed oscillator enable"]
pub type LseonR = crate::BitReader;
#[doc = "Field `LSEON` writer - External low-speed oscillator enable"]
pub type LseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDY` reader - External low-speed oscillator ready"]
pub type LserdyR = crate::BitReader;
#[doc = "Field `LSERDY` writer - External low-speed oscillator ready"]
pub type LserdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEBYP` reader - External low-speed oscillator bypass"]
pub type LsebypR = crate::BitReader;
#[doc = "Field `LSEBYP` writer - External low-speed oscillator bypass"]
pub type LsebypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSEL` reader - RTC clock source selection"]
pub type RtcselR = crate::FieldReader;
#[doc = "Field `RTCSEL` writer - RTC clock source selection"]
pub type RtcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RtcenR = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDRST` reader - Backup domain software reset"]
pub type BdrstR = crate::BitReader;
#[doc = "Field `BDRST` writer - Backup domain software reset"]
pub type BdrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LseonR {
        LseonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LserdyR {
        LserdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LsebypR {
        LsebypR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 9:10 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RtcselR {
        RtcselR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BdrstR {
        BdrstR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LseonW<'_, BdcrSpec> {
        LseonW::new(self, 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&mut self) -> LserdyW<'_, BdcrSpec> {
        LserdyW::new(self, 1)
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LsebypW<'_, BdcrSpec> {
        LsebypW::new(self, 2)
    }
    #[doc = "Bits 9:10 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RtcselW<'_, BdcrSpec> {
        RtcselW::new(self, 9)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RtcenW<'_, BdcrSpec> {
        RtcenW::new(self, 15)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&mut self) -> BdrstW<'_, BdcrSpec> {
        BdrstW::new(self, 16)
    }
}
#[doc = "Backup Domain Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdcrSpec;
impl crate::RegisterSpec for BdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdcr::R`](R) reader structure"]
impl crate::Readable for BdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bdcr::W`](W) writer structure"]
impl crate::Writable for BdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BdcrSpec {}
