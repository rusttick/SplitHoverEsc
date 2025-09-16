#[doc = "Register `DMA_ISR` reader"]
pub type R = crate::R<DmaIsrSpec>;
#[doc = "Field `GIF1` reader - channel 1 global interrupt flag"]
pub type Gif1R = crate::BitReader;
#[doc = "Field `TCIF1` reader - channel 1 transfer complete flag"]
pub type Tcif1R = crate::BitReader;
#[doc = "Field `HTIF1` reader - channel 1 half transfer flag"]
pub type Htif1R = crate::BitReader;
#[doc = "Field `TEIF1` reader - channel 1 transfer error flag"]
pub type Teif1R = crate::BitReader;
#[doc = "Field `GIF2` reader - channel 2 global interrupt flag"]
pub type Gif2R = crate::BitReader;
#[doc = "Field `TCIF2` reader - channel 2 transfer complete flag"]
pub type Tcif2R = crate::BitReader;
#[doc = "Field `HTIF2` reader - channel 2 half transfer flag"]
pub type Htif2R = crate::BitReader;
#[doc = "Field `TEIF2` reader - channel 2 transfer error flag"]
pub type Teif2R = crate::BitReader;
#[doc = "Field `GIF3` reader - channel 3 global interrupt flag"]
pub type Gif3R = crate::BitReader;
#[doc = "Field `TCIF3` reader - channel 3 transfer complete flag"]
pub type Tcif3R = crate::BitReader;
#[doc = "Field `HTIF3` reader - channel 3 half transfer flag"]
pub type Htif3R = crate::BitReader;
#[doc = "Field `TEIF3` reader - channel 3 transfer error flag"]
pub type Teif3R = crate::BitReader;
#[doc = "Field `GIF4` reader - channel 4 global interrupt flag"]
pub type Gif4R = crate::BitReader;
#[doc = "Field `TCIF4` reader - channel 4 transfer complete flag"]
pub type Tcif4R = crate::BitReader;
#[doc = "Field `HTIF4` reader - channel 4 half transfer flag"]
pub type Htif4R = crate::BitReader;
#[doc = "Field `TEIF4` reader - channel 4 transfer error flag"]
pub type Teif4R = crate::BitReader;
#[doc = "Field `GIF5` reader - channel 5 global interrupt flag"]
pub type Gif5R = crate::BitReader;
#[doc = "Field `TCIF5` reader - channel 5 transfer complete flag"]
pub type Tcif5R = crate::BitReader;
#[doc = "Field `HTIF5` reader - channel 5 half transfer flag"]
pub type Htif5R = crate::BitReader;
#[doc = "Field `TEIF5` reader - channel 5 transfer error flag"]
pub type Teif5R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - channel 1 global interrupt flag"]
    #[inline(always)]
    pub fn gif1(&self) -> Gif1R {
        Gif1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - channel 1 transfer complete flag"]
    #[inline(always)]
    pub fn tcif1(&self) -> Tcif1R {
        Tcif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - channel 1 half transfer flag"]
    #[inline(always)]
    pub fn htif1(&self) -> Htif1R {
        Htif1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - channel 1 transfer error flag"]
    #[inline(always)]
    pub fn teif1(&self) -> Teif1R {
        Teif1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel 2 global interrupt flag"]
    #[inline(always)]
    pub fn gif2(&self) -> Gif2R {
        Gif2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - channel 2 transfer complete flag"]
    #[inline(always)]
    pub fn tcif2(&self) -> Tcif2R {
        Tcif2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - channel 2 half transfer flag"]
    #[inline(always)]
    pub fn htif2(&self) -> Htif2R {
        Htif2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - channel 2 transfer error flag"]
    #[inline(always)]
    pub fn teif2(&self) -> Teif2R {
        Teif2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - channel 3 global interrupt flag"]
    #[inline(always)]
    pub fn gif3(&self) -> Gif3R {
        Gif3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - channel 3 transfer complete flag"]
    #[inline(always)]
    pub fn tcif3(&self) -> Tcif3R {
        Tcif3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - channel 3 half transfer flag"]
    #[inline(always)]
    pub fn htif3(&self) -> Htif3R {
        Htif3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - channel 3 transfer error flag"]
    #[inline(always)]
    pub fn teif3(&self) -> Teif3R {
        Teif3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - channel 4 global interrupt flag"]
    #[inline(always)]
    pub fn gif4(&self) -> Gif4R {
        Gif4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - channel 4 transfer complete flag"]
    #[inline(always)]
    pub fn tcif4(&self) -> Tcif4R {
        Tcif4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - channel 4 half transfer flag"]
    #[inline(always)]
    pub fn htif4(&self) -> Htif4R {
        Htif4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - channel 4 transfer error flag"]
    #[inline(always)]
    pub fn teif4(&self) -> Teif4R {
        Teif4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - channel 5 global interrupt flag"]
    #[inline(always)]
    pub fn gif5(&self) -> Gif5R {
        Gif5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - channel 5 transfer complete flag"]
    #[inline(always)]
    pub fn tcif5(&self) -> Tcif5R {
        Tcif5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - channel 5 half transfer flag"]
    #[inline(always)]
    pub fn htif5(&self) -> Htif5R {
        Htif5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - channel 5 transfer error flag"]
    #[inline(always)]
    pub fn teif5(&self) -> Teif5R {
        Teif5R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "DMA interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIsrSpec;
impl crate::RegisterSpec for DmaIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_isr::R`](R) reader structure"]
impl crate::Readable for DmaIsrSpec {}
#[doc = "`reset()` method sets DMA_ISR to value 0"]
impl crate::Resettable for DmaIsrSpec {}
