#[doc = "Register `APB1RSTR` reader"]
pub type R = crate::R<Apb1rstrSpec>;
#[doc = "Register `APB1RSTR` writer"]
pub type W = crate::W<Apb1rstrSpec>;
#[doc = "Field `TIM2` reader - TIM2 reset"]
pub type Tim2R = crate::BitReader;
#[doc = "Field `TIM2` writer - TIM2 reset"]
pub type Tim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3` reader - TIM3 reset"]
pub type Tim3R = crate::BitReader;
#[doc = "Field `TIM3` writer - TIM3 reset"]
pub type Tim3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG` reader - Window watchdog reset"]
pub type WwdgR = crate::BitReader;
#[doc = "Field `WWDG` writer - Window watchdog reset"]
pub type WwdgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - SPI2 reset"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI2` writer - SPI2 reset"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2` reader - UART2 reset"]
pub type Uart2R = crate::BitReader;
#[doc = "Field `UART2` writer - UART2 reset"]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 reset"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 reset"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB` reader - USB reset"]
pub type UsbR = crate::BitReader;
#[doc = "Field `USB` writer - USB reset"]
pub type UsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN` reader - CAN reset"]
pub type CanR = crate::BitReader;
#[doc = "Field `CAN` writer - CAN reset"]
pub type CanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSM` reader - CSM reset"]
pub type CsmR = crate::BitReader;
#[doc = "Field `CSM` writer - CSM reset"]
pub type CsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRS` reader - CRS interface reset"]
pub type CrsR = crate::BitReader;
#[doc = "Field `CRS` writer - CRS interface reset"]
pub type CrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWR` reader - Power interface reset"]
pub type PwrR = crate::BitReader;
#[doc = "Field `PWR` writer - Power interface reset"]
pub type PwrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 reset"]
    #[inline(always)]
    pub fn tim2(&self) -> Tim2R {
        Tim2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 reset"]
    #[inline(always)]
    pub fn tim3(&self) -> Tim3R {
        Tim3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdg(&self) -> WwdgR {
        WwdgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - UART2 reset"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN reset"]
    #[inline(always)]
    pub fn can(&self) -> CanR {
        CanR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CSM reset"]
    #[inline(always)]
    pub fn csm(&self) -> CsmR {
        CsmR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CRS interface reset"]
    #[inline(always)]
    pub fn crs(&self) -> CrsR {
        CrsR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwr(&self) -> PwrR {
        PwrR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 reset"]
    #[inline(always)]
    pub fn tim2(&mut self) -> Tim2W<'_, Apb1rstrSpec> {
        Tim2W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 reset"]
    #[inline(always)]
    pub fn tim3(&mut self) -> Tim3W<'_, Apb1rstrSpec> {
        Tim3W::new(self, 1)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdg(&mut self) -> WwdgW<'_, Apb1rstrSpec> {
        WwdgW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2(&mut self) -> Spi2W<'_, Apb1rstrSpec> {
        Spi2W::new(self, 14)
    }
    #[doc = "Bit 17 - UART2 reset"]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<'_, Apb1rstrSpec> {
        Uart2W::new(self, 17)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, Apb1rstrSpec> {
        I2c1W::new(self, 21)
    }
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    pub fn usb(&mut self) -> UsbW<'_, Apb1rstrSpec> {
        UsbW::new(self, 23)
    }
    #[doc = "Bit 25 - CAN reset"]
    #[inline(always)]
    pub fn can(&mut self) -> CanW<'_, Apb1rstrSpec> {
        CanW::new(self, 25)
    }
    #[doc = "Bit 26 - CSM reset"]
    #[inline(always)]
    pub fn csm(&mut self) -> CsmW<'_, Apb1rstrSpec> {
        CsmW::new(self, 26)
    }
    #[doc = "Bit 27 - CRS interface reset"]
    #[inline(always)]
    pub fn crs(&mut self) -> CrsW<'_, Apb1rstrSpec> {
        CrsW::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwr(&mut self) -> PwrW<'_, Apb1rstrSpec> {
        PwrW::new(self, 28)
    }
}
#[doc = "Advanced Peripheral Bus 1 Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1rstrSpec;
impl crate::RegisterSpec for Apb1rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr::R`](R) reader structure"]
impl crate::Readable for Apb1rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr::W`](W) writer structure"]
impl crate::Writable for Apb1rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1RSTR to value 0"]
impl crate::Resettable for Apb1rstrSpec {}
