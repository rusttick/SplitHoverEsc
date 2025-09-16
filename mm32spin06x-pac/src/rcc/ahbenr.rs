#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AhbenrSpec>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AhbenrSpec>;
#[doc = "Field `DMA` reader - DMA clock enable"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - DMA clock enable"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM` reader - SRAM interface clock enable"]
pub type SramR = crate::BitReader;
#[doc = "Field `SRAM` writer - SRAM interface clock enable"]
pub type SramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH` reader - FLASH clock enable"]
pub type FlashR = crate::BitReader;
#[doc = "Field `FLASH` writer - FLASH clock enable"]
pub type FlashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - CRC clock enable"]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - CRC clock enable"]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOA` reader - GPIOA clock enable"]
pub type GpioaR = crate::BitReader;
#[doc = "Field `GPIOA` writer - GPIOA clock enable"]
pub type GpioaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - GPIOB clock enable"]
pub type GpiobR = crate::BitReader;
#[doc = "Field `GPIOB` writer - GPIOB clock enable"]
pub type GpiobW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOC` reader - GPIOC clock enable"]
pub type GpiocR = crate::BitReader;
#[doc = "Field `GPIOC` writer - GPIOC clock enable"]
pub type GpiocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOD` reader - GPIOD clock enable"]
pub type GpiodR = crate::BitReader;
#[doc = "Field `GPIOD` writer - GPIOD clock enable"]
pub type GpiodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWDIV` reader - HWDIV clock enable"]
pub type HwdivR = crate::BitReader;
#[doc = "Field `HWDIV` writer - HWDIV clock enable"]
pub type HwdivW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sram(&self) -> SramR {
        SramR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    pub fn flash(&self) -> FlashR {
        FlashR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIOA clock enable"]
    #[inline(always)]
    pub fn gpioa(&self) -> GpioaR {
        GpioaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIOB clock enable"]
    #[inline(always)]
    pub fn gpiob(&self) -> GpiobR {
        GpiobR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIOC clock enable"]
    #[inline(always)]
    pub fn gpioc(&self) -> GpiocR {
        GpiocR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIOD clock enable"]
    #[inline(always)]
    pub fn gpiod(&self) -> GpiodR {
        GpiodR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 26 - HWDIV clock enable"]
    #[inline(always)]
    pub fn hwdiv(&self) -> HwdivR {
        HwdivR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, AhbenrSpec> {
        DmaW::new(self, 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sram(&mut self) -> SramW<'_, AhbenrSpec> {
        SramW::new(self, 2)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    pub fn flash(&mut self) -> FlashW<'_, AhbenrSpec> {
        FlashW::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, AhbenrSpec> {
        CrcW::new(self, 6)
    }
    #[doc = "Bit 17 - GPIOA clock enable"]
    #[inline(always)]
    pub fn gpioa(&mut self) -> GpioaW<'_, AhbenrSpec> {
        GpioaW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIOB clock enable"]
    #[inline(always)]
    pub fn gpiob(&mut self) -> GpiobW<'_, AhbenrSpec> {
        GpiobW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIOC clock enable"]
    #[inline(always)]
    pub fn gpioc(&mut self) -> GpiocW<'_, AhbenrSpec> {
        GpiocW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIOD clock enable"]
    #[inline(always)]
    pub fn gpiod(&mut self) -> GpiodW<'_, AhbenrSpec> {
        GpiodW::new(self, 20)
    }
    #[doc = "Bit 26 - HWDIV clock enable"]
    #[inline(always)]
    pub fn hwdiv(&mut self) -> HwdivW<'_, AhbenrSpec> {
        HwdivW::new(self, 26)
    }
}
#[doc = "Advanced High Performance Bus Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbenrSpec;
impl crate::RegisterSpec for AhbenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbenr::R`](R) reader structure"]
impl crate::Readable for AhbenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure"]
impl crate::Writable for AhbenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBENR to value 0x14"]
impl crate::Resettable for AhbenrSpec {
    const RESET_VALUE: u32 = 0x14;
}
