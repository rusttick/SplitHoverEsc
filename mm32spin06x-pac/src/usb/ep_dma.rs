#[doc = "Register `EP_DMA` reader"]
pub type R = crate::R<EpDmaSpec>;
#[doc = "Register `EP_DMA` writer"]
pub type W = crate::W<EpDmaSpec>;
#[doc = "Field `DMA0EN` reader - EP0 DMA enable"]
pub type Dma0enR = crate::BitReader;
#[doc = "Field `DMA0EN` writer - EP0 DMA enable"]
pub type Dma0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1EN` reader - EP1 DMA enable"]
pub type Dma1enR = crate::BitReader;
#[doc = "Field `DMA1EN` writer - EP1 DMA enable"]
pub type Dma1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - EP2 DMA enable"]
pub type Dma2enR = crate::BitReader;
#[doc = "Field `DMA2EN` writer - EP2 DMA enable"]
pub type Dma2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA3EN` reader - EP3 DMA enable"]
pub type Dma3enR = crate::BitReader;
#[doc = "Field `DMA3EN` writer - EP3 DMA enable"]
pub type Dma3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA4EN` reader - EP4 DMA enable"]
pub type Dma4enR = crate::BitReader;
#[doc = "Field `DMA4EN` writer - EP4 DMA enable"]
pub type Dma4enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EP0 DMA enable"]
    #[inline(always)]
    pub fn dma0en(&self) -> Dma0enR {
        Dma0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EP1 DMA enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EP2 DMA enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> Dma2enR {
        Dma2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EP3 DMA enable"]
    #[inline(always)]
    pub fn dma3en(&self) -> Dma3enR {
        Dma3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP4 DMA enable"]
    #[inline(always)]
    pub fn dma4en(&self) -> Dma4enR {
        Dma4enR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EP0 DMA enable"]
    #[inline(always)]
    pub fn dma0en(&mut self) -> Dma0enW<'_, EpDmaSpec> {
        Dma0enW::new(self, 0)
    }
    #[doc = "Bit 1 - EP1 DMA enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> Dma1enW<'_, EpDmaSpec> {
        Dma1enW::new(self, 1)
    }
    #[doc = "Bit 2 - EP2 DMA enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> Dma2enW<'_, EpDmaSpec> {
        Dma2enW::new(self, 2)
    }
    #[doc = "Bit 3 - EP3 DMA enable"]
    #[inline(always)]
    pub fn dma3en(&mut self) -> Dma3enW<'_, EpDmaSpec> {
        Dma3enW::new(self, 3)
    }
    #[doc = "Bit 4 - EP4 DMA enable"]
    #[inline(always)]
    pub fn dma4en(&mut self) -> Dma4enW<'_, EpDmaSpec> {
        Dma4enW::new(self, 4)
    }
}
#[doc = "EP DMA register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpDmaSpec;
impl crate::RegisterSpec for EpDmaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep_dma::R`](R) reader structure"]
impl crate::Readable for EpDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_dma::W`](W) writer structure"]
impl crate::Writable for EpDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP_DMA to value 0"]
impl crate::Resettable for EpDmaSpec {}
