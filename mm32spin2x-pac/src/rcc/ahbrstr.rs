#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AhbrstrSpec>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AhbrstrSpec>;
#[doc = "Field `DMA1` reader - DMA1 clock reset"]
pub type Dma1R = crate::BitReader;
#[doc = "Field `DMA1` writer - DMA1 clock reset"]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM` reader - SRAM interface clock reset"]
pub type SramR = crate::BitReader;
#[doc = "Field `SRAM` writer - SRAM interface clock reset"]
pub type SramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLITF` reader - FLITF clock reset"]
pub type FlitfR = crate::BitReader;
#[doc = "Field `FLITF` writer - FLITF clock reset"]
pub type FlitfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - CRC clock reset"]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - CRC clock reset"]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOA` reader - I/O port A clock reset"]
pub type GpioaR = crate::BitReader;
#[doc = "Field `GPIOA` writer - I/O port A clock reset"]
pub type GpioaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - I/O port B clock reset"]
pub type GpiobR = crate::BitReader;
#[doc = "Field `GPIOB` writer - I/O port B clock reset"]
pub type GpiobW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOC` reader - I/O port C clock reset"]
pub type GpiocR = crate::BitReader;
#[doc = "Field `GPIOC` writer - I/O port C clock reset"]
pub type GpiocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOD` reader - I/O port D clock reset"]
pub type GpiodR = crate::BitReader;
#[doc = "Field `GPIOD` writer - I/O port D clock reset"]
pub type GpiodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWSQRT` reader - HWSQRT clock reset"]
pub type HwsqrtR = crate::BitReader;
#[doc = "Field `HWSQRT` writer - HWSQRT clock reset"]
pub type HwsqrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWDIV` reader - HWDIV clock reset"]
pub type HwdivR = crate::BitReader;
#[doc = "Field `HWDIV` writer - HWDIV clock reset"]
pub type HwdivW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock reset"]
    #[inline(always)]
    pub fn dma1(&self) -> Dma1R {
        Dma1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock reset"]
    #[inline(always)]
    pub fn sram(&self) -> SramR {
        SramR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLITF clock reset"]
    #[inline(always)]
    pub fn flitf(&self) -> FlitfR {
        FlitfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock reset"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - I/O port A clock reset"]
    #[inline(always)]
    pub fn gpioa(&self) -> GpioaR {
        GpioaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B clock reset"]
    #[inline(always)]
    pub fn gpiob(&self) -> GpiobR {
        GpiobR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I/O port C clock reset"]
    #[inline(always)]
    pub fn gpioc(&self) -> GpiocR {
        GpiocR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I/O port D clock reset"]
    #[inline(always)]
    pub fn gpiod(&self) -> GpiodR {
        GpiodR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - HWSQRT clock reset"]
    #[inline(always)]
    pub fn hwsqrt(&self) -> HwsqrtR {
        HwsqrtR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - HWDIV clock reset"]
    #[inline(always)]
    pub fn hwdiv(&self) -> HwdivR {
        HwdivR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock reset"]
    #[inline(always)]
    pub fn dma1(&mut self) -> Dma1W<'_, AhbrstrSpec> {
        Dma1W::new(self, 0)
    }
    #[doc = "Bit 2 - SRAM interface clock reset"]
    #[inline(always)]
    pub fn sram(&mut self) -> SramW<'_, AhbrstrSpec> {
        SramW::new(self, 2)
    }
    #[doc = "Bit 4 - FLITF clock reset"]
    #[inline(always)]
    pub fn flitf(&mut self) -> FlitfW<'_, AhbrstrSpec> {
        FlitfW::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock reset"]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, AhbrstrSpec> {
        CrcW::new(self, 6)
    }
    #[doc = "Bit 17 - I/O port A clock reset"]
    #[inline(always)]
    pub fn gpioa(&mut self) -> GpioaW<'_, AhbrstrSpec> {
        GpioaW::new(self, 17)
    }
    #[doc = "Bit 18 - I/O port B clock reset"]
    #[inline(always)]
    pub fn gpiob(&mut self) -> GpiobW<'_, AhbrstrSpec> {
        GpiobW::new(self, 18)
    }
    #[doc = "Bit 19 - I/O port C clock reset"]
    #[inline(always)]
    pub fn gpioc(&mut self) -> GpiocW<'_, AhbrstrSpec> {
        GpiocW::new(self, 19)
    }
    #[doc = "Bit 20 - I/O port D clock reset"]
    #[inline(always)]
    pub fn gpiod(&mut self) -> GpiodW<'_, AhbrstrSpec> {
        GpiodW::new(self, 20)
    }
    #[doc = "Bit 25 - HWSQRT clock reset"]
    #[inline(always)]
    pub fn hwsqrt(&mut self) -> HwsqrtW<'_, AhbrstrSpec> {
        HwsqrtW::new(self, 25)
    }
    #[doc = "Bit 26 - HWDIV clock reset"]
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
