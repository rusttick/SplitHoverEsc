#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<Apb2rstrSpec>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<Apb2rstrSpec>;
#[doc = "Field `EXTI` reader - External Interrupt"]
pub type ExtiR = crate::BitReader;
#[doc = "Field `EXTI` writer - External Interrupt"]
pub type ExtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - ADC1 interface reset"]
pub type Adc1R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC1 interface reset"]
pub type Adc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2` reader - ADC2 interface reset"]
pub type Adc2R = crate::BitReader;
#[doc = "Field `ADC2` writer - ADC2 interface reset"]
pub type Adc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1` reader - TIM1 reset"]
pub type Tim1R = crate::BitReader;
#[doc = "Field `TIM1` writer - TIM1 reset"]
pub type Tim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - SPI1 reset"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 reset"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8` reader - TIM8 reset"]
pub type Tim8R = crate::BitReader;
#[doc = "Field `TIM8` writer - TIM8 reset"]
pub type Tim8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - UART1 reset"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - UART1 reset"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - COMP interface reset"]
pub type CompR = crate::BitReader;
#[doc = "Field `COMP` writer - COMP interface reset"]
pub type CompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14` reader - TIM14 reset"]
pub type Tim14R = crate::BitReader;
#[doc = "Field `TIM14` writer - TIM14 reset"]
pub type Tim14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16` reader - TIM16 reset"]
pub type Tim16R = crate::BitReader;
#[doc = "Field `TIM16` writer - TIM16 reset"]
pub type Tim16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17` reader - TIM17 reset"]
pub type Tim17R = crate::BitReader;
#[doc = "Field `TIM17` writer - TIM17 reset"]
pub type Tim17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGMCU` reader - DBGMCU reset"]
pub type DbgmcuR = crate::BitReader;
#[doc = "Field `DBGMCU` writer - DBGMCU reset"]
pub type DbgmcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM` reader - PWM reset"]
pub type PwmR = crate::BitReader;
#[doc = "Field `PWM` writer - PWM reset"]
pub type PwmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Interrupt"]
    #[inline(always)]
    pub fn exti(&self) -> ExtiR {
        ExtiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 interface reset"]
    #[inline(always)]
    pub fn adc1(&self) -> Adc1R {
        Adc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC2 interface reset"]
    #[inline(always)]
    pub fn adc2(&self) -> Adc2R {
        Adc2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 reset"]
    #[inline(always)]
    pub fn tim1(&self) -> Tim1R {
        Tim1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 reset"]
    #[inline(always)]
    pub fn tim8(&self) -> Tim8R {
        Tim8R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - UART1 reset"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - COMP interface reset"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM14 reset"]
    #[inline(always)]
    pub fn tim14(&self) -> Tim14R {
        Tim14R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 reset"]
    #[inline(always)]
    pub fn tim16(&self) -> Tim16R {
        Tim16R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 reset"]
    #[inline(always)]
    pub fn tim17(&self) -> Tim17R {
        Tim17R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - DBGMCU reset"]
    #[inline(always)]
    pub fn dbgmcu(&self) -> DbgmcuR {
        DbgmcuR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PWM reset"]
    #[inline(always)]
    pub fn pwm(&self) -> PwmR {
        PwmR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt"]
    #[inline(always)]
    pub fn exti(&mut self) -> ExtiW<'_, Apb2rstrSpec> {
        ExtiW::new(self, 0)
    }
    #[doc = "Bit 9 - ADC1 interface reset"]
    #[inline(always)]
    pub fn adc1(&mut self) -> Adc1W<'_, Apb2rstrSpec> {
        Adc1W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC2 interface reset"]
    #[inline(always)]
    pub fn adc2(&mut self) -> Adc2W<'_, Apb2rstrSpec> {
        Adc2W::new(self, 10)
    }
    #[doc = "Bit 11 - TIM1 reset"]
    #[inline(always)]
    pub fn tim1(&mut self) -> Tim1W<'_, Apb2rstrSpec> {
        Tim1W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, Apb2rstrSpec> {
        Spi1W::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 reset"]
    #[inline(always)]
    pub fn tim8(&mut self) -> Tim8W<'_, Apb2rstrSpec> {
        Tim8W::new(self, 13)
    }
    #[doc = "Bit 14 - UART1 reset"]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<'_, Apb2rstrSpec> {
        Uart1W::new(self, 14)
    }
    #[doc = "Bit 15 - COMP interface reset"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, Apb2rstrSpec> {
        CompW::new(self, 15)
    }
    #[doc = "Bit 16 - TIM14 reset"]
    #[inline(always)]
    pub fn tim14(&mut self) -> Tim14W<'_, Apb2rstrSpec> {
        Tim14W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 reset"]
    #[inline(always)]
    pub fn tim16(&mut self) -> Tim16W<'_, Apb2rstrSpec> {
        Tim16W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 reset"]
    #[inline(always)]
    pub fn tim17(&mut self) -> Tim17W<'_, Apb2rstrSpec> {
        Tim17W::new(self, 18)
    }
    #[doc = "Bit 22 - DBGMCU reset"]
    #[inline(always)]
    pub fn dbgmcu(&mut self) -> DbgmcuW<'_, Apb2rstrSpec> {
        DbgmcuW::new(self, 22)
    }
    #[doc = "Bit 23 - PWM reset"]
    #[inline(always)]
    pub fn pwm(&mut self) -> PwmW<'_, Apb2rstrSpec> {
        PwmW::new(self, 23)
    }
}
#[doc = "Advanced Peripheral Bus 2 Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstrSpec;
impl crate::RegisterSpec for Apb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for Apb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for Apb2rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for Apb2rstrSpec {}
