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
#[doc = "Field `PLLDIV` reader - PLL divider factor"]
pub type PlldivR = crate::FieldReader;
#[doc = "Field `PLLDIV` writer - PLL divider factor"]
pub type PlldivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLON` reader - PLL enable"]
pub type PllonR = crate::BitReader;
#[doc = "Field `PLLON` writer - PLL enable"]
pub type PllonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDY` reader - PLL clock ready flag"]
pub type PllrdyR = crate::BitReader;
#[doc = "Field `PLLRDY` writer - PLL clock ready flag"]
pub type PllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLMUL` reader - PLL multiplication factor"]
pub type PllmulR = crate::FieldReader;
#[doc = "Field `PLLMUL` writer - PLL multiplication factor"]
pub type PllmulW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
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
    #[doc = "Bits 20:22 - PLL divider factor"]
    #[inline(always)]
    pub fn plldiv(&self) -> PlldivR {
        PlldivR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PllonR {
        PllonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - PLL multiplication factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PllmulR {
        PllmulR::new(((self.bits >> 26) & 0x3f) as u8)
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
    #[doc = "Bits 20:22 - PLL divider factor"]
    #[inline(always)]
    pub fn plldiv(&mut self) -> PlldivW<'_, CrSpec> {
        PlldivW::new(self, 20)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PllonW<'_, CrSpec> {
        PllonW::new(self, 24)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&mut self) -> PllrdyW<'_, CrSpec> {
        PllrdyW::new(self, 25)
    }
    #[doc = "Bits 26:31 - PLL multiplication factor"]
    #[inline(always)]
    pub fn pllmul(&mut self) -> PllmulW<'_, CrSpec> {
        PllmulW::new(self, 26)
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
#[doc = "`reset()` method sets CR to value 0x01"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x01;
}
