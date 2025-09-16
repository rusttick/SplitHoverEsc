#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AhbrstrSpec>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AhbrstrSpec>;
#[doc = "Field `GPIOA` reader - GPIOA reset"]
pub type GpioaR = crate::BitReader;
#[doc = "Field `GPIOA` writer - GPIOA reset"]
pub type GpioaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - GPIOB reset"]
pub type GpiobR = crate::BitReader;
#[doc = "Field `GPIOB` writer - GPIOB reset"]
pub type GpiobW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOC` reader - GPIOC reset"]
pub type GpiocR = crate::BitReader;
#[doc = "Field `GPIOC` writer - GPIOC reset"]
pub type GpiocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOD` reader - GPIOD reset"]
pub type GpiodR = crate::BitReader;
#[doc = "Field `GPIOD` writer - GPIOD reset"]
pub type GpiodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWDIV` reader - HWDIV reset"]
pub type HwdivR = crate::BitReader;
#[doc = "Field `HWDIV` writer - HWDIV reset"]
pub type HwdivW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - GPIOA reset"]
    #[inline(always)]
    pub fn gpioa(&self) -> GpioaR {
        GpioaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIOB reset"]
    #[inline(always)]
    pub fn gpiob(&self) -> GpiobR {
        GpiobR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIOC reset"]
    #[inline(always)]
    pub fn gpioc(&self) -> GpiocR {
        GpiocR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIOD reset"]
    #[inline(always)]
    pub fn gpiod(&self) -> GpiodR {
        GpiodR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 26 - HWDIV reset"]
    #[inline(always)]
    pub fn hwdiv(&self) -> HwdivR {
        HwdivR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - GPIOA reset"]
    #[inline(always)]
    pub fn gpioa(&mut self) -> GpioaW<'_, AhbrstrSpec> {
        GpioaW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIOB reset"]
    #[inline(always)]
    pub fn gpiob(&mut self) -> GpiobW<'_, AhbrstrSpec> {
        GpiobW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIOC reset"]
    #[inline(always)]
    pub fn gpioc(&mut self) -> GpiocW<'_, AhbrstrSpec> {
        GpiocW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIOD reset"]
    #[inline(always)]
    pub fn gpiod(&mut self) -> GpiodW<'_, AhbrstrSpec> {
        GpiodW::new(self, 20)
    }
    #[doc = "Bit 26 - HWDIV reset"]
    #[inline(always)]
    pub fn hwdiv(&mut self) -> HwdivW<'_, AhbrstrSpec> {
        HwdivW::new(self, 26)
    }
}
#[doc = "Advanced High Performance Bus Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstrSpec;
impl crate::RegisterSpec for AhbrstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AhbrstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AhbrstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AhbrstrSpec {}
