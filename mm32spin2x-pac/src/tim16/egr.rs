#[doc = "Register `EGR` reader"]
pub type R = crate::R<EgrSpec>;
#[doc = "Register `EGR` writer"]
pub type W = crate::W<EgrSpec>;
#[doc = "Field `UG` reader - Update generation"]
pub type UgR = crate::BitReader;
#[doc = "Field `UG` writer - Update generation"]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` reader - Capture/compare 1 generation"]
pub type Cc1gR = crate::BitReader;
#[doc = "Field `CC1G` writer - Capture/compare 1 generation"]
pub type Cc1gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMG` reader - Capture/Compare control update generation"]
pub type ComgR = crate::BitReader;
#[doc = "Field `COMG` writer - Capture/Compare control update generation"]
pub type ComgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BG` reader - Break generation"]
pub type BgR = crate::BitReader;
#[doc = "Field `BG` writer - Break generation"]
pub type BgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&self) -> UgR {
        UgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&self) -> Cc1gR {
        Cc1gR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline(always)]
    pub fn comg(&self) -> ComgR {
        ComgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    pub fn bg(&self) -> BgR {
        BgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&mut self) -> UgW<'_, EgrSpec> {
        UgW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> Cc1gW<'_, EgrSpec> {
        Cc1gW::new(self, 1)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline(always)]
    pub fn comg(&mut self) -> ComgW<'_, EgrSpec> {
        ComgW::new(self, 5)
    }
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    pub fn bg(&mut self) -> BgW<'_, EgrSpec> {
        BgW::new(self, 7)
    }
}
#[doc = "event generation register\n\nYou can [`read`](crate::Reg::read) this register and get [`egr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EgrSpec;
impl crate::RegisterSpec for EgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`egr::R`](R) reader structure"]
impl crate::Readable for EgrSpec {}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EgrSpec {}
