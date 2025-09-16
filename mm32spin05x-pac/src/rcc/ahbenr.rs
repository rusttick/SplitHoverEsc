#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AhbenrSpec>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AhbenrSpec>;
#[doc = "Field `DMA1` reader - DMA1 clock enable"]
pub type Dma1R = crate::BitReader;
#[doc = "Field `DMA1` writer - DMA1 clock enable"]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM` reader - SRAM interface clock enable"]
pub type SramR = crate::BitReader;
#[doc = "Field `SRAM` writer - SRAM interface clock enable"]
pub type SramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLITF` reader - FLITF clock enable"]
pub type FlitfR = crate::BitReader;
#[doc = "Field `FLITF` writer - FLITF clock enable"]
pub type FlitfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - CRC clock enable"]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - CRC clock enable"]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOA` reader - I/O port A clock enable"]
pub type GpioaR = crate::BitReader;
#[doc = "Field `GPIOA` writer - I/O port A clock enable"]
pub type GpioaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - I/O port B clock enable"]
pub type GpiobR = crate::BitReader;
#[doc = "Field `GPIOB` writer - I/O port B clock enable"]
pub type GpiobW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOC` reader - I/O port C clock enable"]
pub type GpiocR = crate::BitReader;
#[doc = "Field `GPIOC` writer - I/O port C clock enable"]
pub type GpiocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOD` reader - I/O port D clock enable"]
pub type GpiodR = crate::BitReader;
#[doc = "Field `GPIOD` writer - I/O port D clock enable"]
pub type GpiodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWDIV` reader - HWDIV clock enable"]
pub type HwdivR = crate::BitReader;
#[doc = "Field `HWDIV` writer - HWDIV clock enable"]
pub type HwdivW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1(&self) -> Dma1R {
        Dma1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sram(&self) -> SramR {
        SramR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitf(&self) -> FlitfR {
        FlitfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn gpioa(&self) -> GpioaR {
        GpioaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn gpiob(&self) -> GpiobR {
        GpiobR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    pub fn gpioc(&self) -> GpiocR {
        GpiocR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
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
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1(&mut self) -> Dma1W<'_, AhbenrSpec> {
        Dma1W::new(self, 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sram(&mut self) -> SramW<'_, AhbenrSpec> {
        SramW::new(self, 2)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitf(&mut self) -> FlitfW<'_, AhbenrSpec> {
        FlitfW::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, AhbenrSpec> {
        CrcW::new(self, 6)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn gpioa(&mut self) -> GpioaW<'_, AhbenrSpec> {
        GpioaW::new(self, 17)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn gpiob(&mut self) -> GpiobW<'_, AhbenrSpec> {
        GpiobW::new(self, 18)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    pub fn gpioc(&mut self) -> GpiocW<'_, AhbenrSpec> {
        GpiocW::new(self, 19)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
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
