#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `HSION` reader - Internal high-speed clock enable"]
pub type HsionR = crate::BitReader;
#[doc = "Field `HSION` writer - Internal high-speed clock enable"]
pub type HsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - Internal high-speed clock ready flag"]
pub type HsirdyR = crate::BitReader;
#[doc = "Field `HSIRDY` writer - Internal high-speed clock ready flag"]
pub type HsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSITEN` reader - Internal high-speed clock temperature enable"]
pub type HsitenR = crate::BitReader;
#[doc = "Field `HSITEN` writer - Internal high-speed clock temperature enable"]
pub type HsitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSICAL` reader - Internal high-speed clock calibration"]
pub type HsicalR = crate::FieldReader;
#[doc = "Field `HSICAL` writer - Internal high-speed clock calibration"]
pub type HsicalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HSEON` reader - External high-speed clock enable"]
pub type HseonR = crate::BitReader;
#[doc = "Field `HSEON` writer - External high-speed clock enable"]
pub type HseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - External high-speed clock ready flag"]
pub type HserdyR = crate::BitReader;
#[doc = "Field `HSERDY` writer - External high-speed clock ready flag"]
pub type HserdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEBYP` reader - External high-speed clock bypass"]
pub type HsebypR = crate::BitReader;
#[doc = "Field `HSEBYP` writer - External high-speed clock bypass"]
pub type HsebypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCSON` reader - Clock security system enable"]
pub type CcsonR = crate::BitReader;
#[doc = "Field `CCSON` writer - Clock security system enable"]
pub type CcsonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI_72M` reader - HSI output 72M"]
pub type Hsi72mR = crate::BitReader;
#[doc = "Field `HSI_72M` writer - HSI output 72M"]
pub type Hsi72mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STDBY_FS_WK` reader - Quickly wake-up standby mode selection"]
pub type StdbyFsWkR = crate::BitReader;
#[doc = "Field `STDBY_FS_WK` writer - Quickly wake-up standby mode selection"]
pub type StdbyFsWkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HsionR {
        HsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal high-speed clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal high-speed clock temperature enable"]
    #[inline(always)]
    pub fn hsiten(&self) -> HsitenR {
        HsitenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Internal high-speed clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HsicalR {
        HsicalR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - External high-speed clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HseonR {
        HseonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External high-speed clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External high-speed clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HsebypR {
        HsebypR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    pub fn ccson(&self) -> CcsonR {
        CcsonR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - HSI output 72M"]
    #[inline(always)]
    pub fn hsi_72m(&self) -> Hsi72mR {
        Hsi72mR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Quickly wake-up standby mode selection"]
    #[inline(always)]
    pub fn stdby_fs_wk(&self) -> StdbyFsWkR {
        StdbyFsWkR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HsionW<'_, CrSpec> {
        HsionW::new(self, 0)
    }
    #[doc = "Bit 1 - Internal high-speed clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&mut self) -> HsirdyW<'_, CrSpec> {
        HsirdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Internal high-speed clock temperature enable"]
    #[inline(always)]
    pub fn hsiten(&mut self) -> HsitenW<'_, CrSpec> {
        HsitenW::new(self, 2)
    }
    #[doc = "Bits 8:13 - Internal high-speed clock calibration"]
    #[inline(always)]
    pub fn hsical(&mut self) -> HsicalW<'_, CrSpec> {
        HsicalW::new(self, 8)
    }
    #[doc = "Bit 16 - External high-speed clock enable"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HseonW<'_, CrSpec> {
        HseonW::new(self, 16)
    }
    #[doc = "Bit 17 - External high-speed clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&mut self) -> HserdyW<'_, CrSpec> {
        HserdyW::new(self, 17)
    }
    #[doc = "Bit 18 - External high-speed clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HsebypW<'_, CrSpec> {
        HsebypW::new(self, 18)
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    pub fn ccson(&mut self) -> CcsonW<'_, CrSpec> {
        CcsonW::new(self, 19)
    }
    #[doc = "Bit 20 - HSI output 72M"]
    #[inline(always)]
    pub fn hsi_72m(&mut self) -> Hsi72mW<'_, CrSpec> {
        Hsi72mW::new(self, 20)
    }
    #[doc = "Bit 21 - Quickly wake-up standby mode selection"]
    #[inline(always)]
    pub fn stdby_fs_wk(&mut self) -> StdbyFsWkW<'_, CrSpec> {
        StdbyFsWkW::new(self, 21)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x03"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x03;
}
