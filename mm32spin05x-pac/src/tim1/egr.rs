#[doc = "Register `EGR` writer"]
pub type W = crate::W<EgrSpec>;
#[doc = "Field `UG` writer - Update generation"]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - Capture/Compare 1 generation"]
pub type Cc1gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2G` writer - Capture/Compare 2 generation"]
pub type Cc2gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3G` writer - Capture/Compare 3 generation"]
pub type Cc3gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4G` writer - Capture/Compare 4 generation"]
pub type Cc4gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMG` writer - Capture/Compare control update generation"]
pub type ComgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` writer - Trigger generation"]
pub type TgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BG` writer - Break generation"]
pub type BgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5G` writer - Capture/Compare 5 generation"]
pub type Cc5gW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&mut self) -> UgW<'_, EgrSpec> {
        UgW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> Cc1gW<'_, EgrSpec> {
        Cc1gW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 generation"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> Cc2gW<'_, EgrSpec> {
        Cc2gW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 3 generation"]
    #[inline(always)]
    pub fn cc3g(&mut self) -> Cc3gW<'_, EgrSpec> {
        Cc3gW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 4 generation"]
    #[inline(always)]
    pub fn cc4g(&mut self) -> Cc4gW<'_, EgrSpec> {
        Cc4gW::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline(always)]
    pub fn comg(&mut self) -> ComgW<'_, EgrSpec> {
        ComgW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<'_, EgrSpec> {
        TgW::new(self, 6)
    }
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    pub fn bg(&mut self) -> BgW<'_, EgrSpec> {
        BgW::new(self, 7)
    }
    #[doc = "Bit 16 - Capture/Compare 5 generation"]
    #[inline(always)]
    pub fn cc5g(&mut self) -> Cc5gW<'_, EgrSpec> {
        Cc5gW::new(self, 16)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EgrSpec;
impl crate::RegisterSpec for EgrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EgrSpec {}
