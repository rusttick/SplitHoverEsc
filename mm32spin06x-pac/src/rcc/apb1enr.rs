#[doc = "Register `APB1ENR` reader"]
pub type R = crate::R<Apb1enrSpec>;
#[doc = "Register `APB1ENR` writer"]
pub type W = crate::W<Apb1enrSpec>;
#[doc = "Field `TIM2` reader - TIM2 clock enable"]
pub type Tim2R = crate::BitReader;
#[doc = "Field `TIM2` writer - TIM2 clock enable"]
pub type Tim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3` reader - TIM3 clock enable"]
pub type Tim3R = crate::BitReader;
#[doc = "Field `TIM3` writer - TIM3 clock enable"]
pub type Tim3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG` reader - Window watchdog clock enable"]
pub type WwdgR = crate::BitReader;
#[doc = "Field `WWDG` writer - Window watchdog clock enable"]
pub type WwdgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - SPI2 clock enable"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI2` writer - SPI2 clock enable"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2` reader - UART2 clock enable"]
pub type Uart2R = crate::BitReader;
#[doc = "Field `UART2` writer - UART2 clock enable"]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 clock enable"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 clock enable"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB` reader - USB clock enable"]
pub type UsbR = crate::BitReader;
#[doc = "Field `USB` writer - USB clock enable"]
pub type UsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN` reader - CAN 2C1 clock enable"]
pub type CanR = crate::BitReader;
#[doc = "Field `CAN` writer - CAN 2C1 clock enable"]
pub type CanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSM` reader - CSM clock enable"]
pub type CsmR = crate::BitReader;
#[doc = "Field `CSM` writer - CSM clock enable"]
pub type CsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRS` reader - CRS clock enable"]
pub type CrsR = crate::BitReader;
#[doc = "Field `CRS` writer - CRS clock enable"]
pub type CrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWR` reader - Power interface clock enable"]
pub type PwrR = crate::BitReader;
#[doc = "Field `PWR` writer - Power interface clock enable"]
pub type PwrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable"]
    #[inline(always)]
    pub fn tim2(&self) -> Tim2R {
        Tim2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable"]
    #[inline(always)]
    pub fn tim3(&self) -> Tim3R {
        Tim3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdg(&self) -> WwdgR {
        WwdgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - UART2 clock enable"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN 2C1 clock enable"]
    #[inline(always)]
    pub fn can(&self) -> CanR {
        CanR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CSM clock enable"]
    #[inline(always)]
    pub fn csm(&self) -> CsmR {
        CsmR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CRS clock enable"]
    #[inline(always)]
    pub fn crs(&self) -> CrsR {
        CrsR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwr(&self) -> PwrR {
        PwrR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable"]
    #[inline(always)]
    pub fn tim2(&mut self) -> Tim2W<'_, Apb1enrSpec> {
        Tim2W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable"]
    #[inline(always)]
    pub fn tim3(&mut self) -> Tim3W<'_, Apb1enrSpec> {
        Tim3W::new(self, 1)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdg(&mut self) -> WwdgW<'_, Apb1enrSpec> {
        WwdgW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2(&mut self) -> Spi2W<'_, Apb1enrSpec> {
        Spi2W::new(self, 14)
    }
    #[doc = "Bit 17 - UART2 clock enable"]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<'_, Apb1enrSpec> {
        Uart2W::new(self, 17)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, Apb1enrSpec> {
        I2c1W::new(self, 21)
    }
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    pub fn usb(&mut self) -> UsbW<'_, Apb1enrSpec> {
        UsbW::new(self, 23)
    }
    #[doc = "Bit 25 - CAN 2C1 clock enable"]
    #[inline(always)]
    pub fn can(&mut self) -> CanW<'_, Apb1enrSpec> {
        CanW::new(self, 25)
    }
    #[doc = "Bit 26 - CSM clock enable"]
    #[inline(always)]
    pub fn csm(&mut self) -> CsmW<'_, Apb1enrSpec> {
        CsmW::new(self, 26)
    }
    #[doc = "Bit 27 - CRS clock enable"]
    #[inline(always)]
    pub fn crs(&mut self) -> CrsW<'_, Apb1enrSpec> {
        CrsW::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwr(&mut self) -> PwrW<'_, Apb1enrSpec> {
        PwrW::new(self, 28)
    }
}
#[doc = "Advanced Peripheral Bus 1 Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1enrSpec;
impl crate::RegisterSpec for Apb1enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr::R`](R) reader structure"]
impl crate::Readable for Apb1enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1enr::W`](W) writer structure"]
impl crate::Writable for Apb1enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1ENR to value 0"]
impl crate::Resettable for Apb1enrSpec {}
