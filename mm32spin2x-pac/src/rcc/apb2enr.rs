#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<Apb2enrSpec>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<Apb2enrSpec>;
#[doc = "Field `EXTI` reader - External Interrupt"]
pub type ExtiR = crate::BitReader;
#[doc = "Field `EXTI` writer - External Interrupt"]
pub type ExtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - ADC1 interface enable"]
pub type Adc1R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC1 interface enable"]
pub type Adc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2` reader - ADC2 interface clock enable"]
pub type Adc2R = crate::BitReader;
#[doc = "Field `ADC2` writer - ADC2 interface clock enable"]
pub type Adc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1` reader - TIM1 enable"]
pub type Tim1R = crate::BitReader;
#[doc = "Field `TIM1` writer - TIM1 enable"]
pub type Tim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - SPI1 enable"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 enable"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8` reader - TIM8 enable"]
pub type Tim8R = crate::BitReader;
#[doc = "Field `TIM8` writer - TIM8 enable"]
pub type Tim8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - UART1 enable"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - UART1 enable"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - COMP interface enable"]
pub type CompR = crate::BitReader;
#[doc = "Field `COMP` writer - COMP interface enable"]
pub type CompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14` reader - TIM14 enable"]
pub type Tim14R = crate::BitReader;
#[doc = "Field `TIM14` writer - TIM14 enable"]
pub type Tim14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16` reader - TIM16 enable"]
pub type Tim16R = crate::BitReader;
#[doc = "Field `TIM16` writer - TIM16 enable"]
pub type Tim16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17` reader - TIM17 enable"]
pub type Tim17R = crate::BitReader;
#[doc = "Field `TIM17` writer - TIM17 enable"]
pub type Tim17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGMCU` reader - DBGMCU enable"]
pub type DbgmcuR = crate::BitReader;
#[doc = "Field `DBGMCU` writer - DBGMCU enable"]
pub type DbgmcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM` reader - *D23"]
pub type PwmR = crate::BitReader;
#[doc = "Field `PWM` writer - *D23"]
pub type PwmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Interrupt"]
    #[inline(always)]
    pub fn exti(&self) -> ExtiR {
        ExtiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 interface enable"]
    #[inline(always)]
    pub fn adc1(&self) -> Adc1R {
        Adc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC2 interface clock enable"]
    #[inline(always)]
    pub fn adc2(&self) -> Adc2R {
        Adc2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 enable"]
    #[inline(always)]
    pub fn tim1(&self) -> Tim1R {
        Tim1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 enable"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 enable"]
    #[inline(always)]
    pub fn tim8(&self) -> Tim8R {
        Tim8R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - UART1 enable"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - COMP interface enable"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM14 enable"]
    #[inline(always)]
    pub fn tim14(&self) -> Tim14R {
        Tim14R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 enable"]
    #[inline(always)]
    pub fn tim16(&self) -> Tim16R {
        Tim16R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 enable"]
    #[inline(always)]
    pub fn tim17(&self) -> Tim17R {
        Tim17R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - DBGMCU enable"]
    #[inline(always)]
    pub fn dbgmcu(&self) -> DbgmcuR {
        DbgmcuR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - *D23"]
    #[inline(always)]
    pub fn pwm(&self) -> PwmR {
        PwmR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt"]
    #[inline(always)]
    pub fn exti(&mut self) -> ExtiW<'_, Apb2enrSpec> {
        ExtiW::new(self, 0)
    }
    #[doc = "Bit 9 - ADC1 interface enable"]
    #[inline(always)]
    pub fn adc1(&mut self) -> Adc1W<'_, Apb2enrSpec> {
        Adc1W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC2 interface clock enable"]
    #[inline(always)]
    pub fn adc2(&mut self) -> Adc2W<'_, Apb2enrSpec> {
        Adc2W::new(self, 10)
    }
    #[doc = "Bit 11 - TIM1 enable"]
    #[inline(always)]
    pub fn tim1(&mut self) -> Tim1W<'_, Apb2enrSpec> {
        Tim1W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 enable"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, Apb2enrSpec> {
        Spi1W::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 enable"]
    #[inline(always)]
    pub fn tim8(&mut self) -> Tim8W<'_, Apb2enrSpec> {
        Tim8W::new(self, 13)
    }
    #[doc = "Bit 14 - UART1 enable"]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<'_, Apb2enrSpec> {
        Uart1W::new(self, 14)
    }
    #[doc = "Bit 15 - COMP interface enable"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, Apb2enrSpec> {
        CompW::new(self, 15)
    }
    #[doc = "Bit 16 - TIM14 enable"]
    #[inline(always)]
    pub fn tim14(&mut self) -> Tim14W<'_, Apb2enrSpec> {
        Tim14W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 enable"]
    #[inline(always)]
    pub fn tim16(&mut self) -> Tim16W<'_, Apb2enrSpec> {
        Tim16W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 enable"]
    #[inline(always)]
    pub fn tim17(&mut self) -> Tim17W<'_, Apb2enrSpec> {
        Tim17W::new(self, 18)
    }
    #[doc = "Bit 22 - DBGMCU enable"]
    #[inline(always)]
    pub fn dbgmcu(&mut self) -> DbgmcuW<'_, Apb2enrSpec> {
        DbgmcuW::new(self, 22)
    }
    #[doc = "Bit 23 - *D23"]
    #[inline(always)]
    pub fn pwm(&mut self) -> PwmW<'_, Apb2enrSpec> {
        PwmW::new(self, 23)
    }
}
#[doc = "Advanced Peripheral Bus 2 Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2enrSpec;
impl crate::RegisterSpec for Apb2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2enr::R`](R) reader structure"]
impl crate::Readable for Apb2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure"]
impl crate::Writable for Apb2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for Apb2enrSpec {}
