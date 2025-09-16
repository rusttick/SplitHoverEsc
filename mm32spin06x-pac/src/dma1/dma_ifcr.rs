#[doc = "Register `DMA_IFCR` writer"]
pub type W = crate::W<DmaIfcrSpec>;
#[doc = "Field `CGIF1` writer - channel 1 global interrupt clear"]
pub type Cgif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF1` writer - channel 1 transfer complete clear"]
pub type Ctcif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF1` writer - channel 1 half transfer clear"]
pub type Chtif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF1` writer - channel 1 transfer error clear"]
pub type Cteif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF2` writer - channel 2 global interrupt clear"]
pub type Cgif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF2` writer - channel 2 transfer complete clear"]
pub type Ctcif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF2` writer - channel 2 half transfer clear"]
pub type Chtif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF2` writer - channel 2 transfer error clear"]
pub type Cteif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF3` writer - channel 3 global interrupt clear"]
pub type Cgif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF3` writer - channel 3 transfer complete clear"]
pub type Ctcif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF3` writer - channel 3 half transfer clear"]
pub type Chtif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF3` writer - channel 3 transfer error clear"]
pub type Cteif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF4` writer - channel 4 global interrupt clear"]
pub type Cgif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF4` writer - channel 4 transfer complete clear"]
pub type Ctcif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF4` writer - channel 4 half transfer clear"]
pub type Chtif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF4` writer - channel 4 transfer error clear"]
pub type Cteif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF5` writer - channel 5 global interrupt clear"]
pub type Cgif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF5` writer - channel 5 transfer complete clear"]
pub type Ctcif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF5` writer - channel 5 half transfer clear"]
pub type Chtif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF5` writer - channel 5 transfer error clear"]
pub type Cteif5W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - channel 1 global interrupt clear"]
    #[inline(always)]
    pub fn cgif1(&mut self) -> Cgif1W<'_, DmaIfcrSpec> {
        Cgif1W::new(self, 0)
    }
    #[doc = "Bit 1 - channel 1 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> Ctcif1W<'_, DmaIfcrSpec> {
        Ctcif1W::new(self, 1)
    }
    #[doc = "Bit 2 - channel 1 half transfer clear"]
    #[inline(always)]
    pub fn chtif1(&mut self) -> Chtif1W<'_, DmaIfcrSpec> {
        Chtif1W::new(self, 2)
    }
    #[doc = "Bit 3 - channel 1 transfer error clear"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> Cteif1W<'_, DmaIfcrSpec> {
        Cteif1W::new(self, 3)
    }
    #[doc = "Bit 4 - channel 2 global interrupt clear"]
    #[inline(always)]
    pub fn cgif2(&mut self) -> Cgif2W<'_, DmaIfcrSpec> {
        Cgif2W::new(self, 4)
    }
    #[doc = "Bit 5 - channel 2 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> Ctcif2W<'_, DmaIfcrSpec> {
        Ctcif2W::new(self, 5)
    }
    #[doc = "Bit 6 - channel 2 half transfer clear"]
    #[inline(always)]
    pub fn chtif2(&mut self) -> Chtif2W<'_, DmaIfcrSpec> {
        Chtif2W::new(self, 6)
    }
    #[doc = "Bit 7 - channel 2 transfer error clear"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> Cteif2W<'_, DmaIfcrSpec> {
        Cteif2W::new(self, 7)
    }
    #[doc = "Bit 8 - channel 3 global interrupt clear"]
    #[inline(always)]
    pub fn cgif3(&mut self) -> Cgif3W<'_, DmaIfcrSpec> {
        Cgif3W::new(self, 8)
    }
    #[doc = "Bit 9 - channel 3 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> Ctcif3W<'_, DmaIfcrSpec> {
        Ctcif3W::new(self, 9)
    }
    #[doc = "Bit 10 - channel 3 half transfer clear"]
    #[inline(always)]
    pub fn chtif3(&mut self) -> Chtif3W<'_, DmaIfcrSpec> {
        Chtif3W::new(self, 10)
    }
    #[doc = "Bit 11 - channel 3 transfer error clear"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> Cteif3W<'_, DmaIfcrSpec> {
        Cteif3W::new(self, 11)
    }
    #[doc = "Bit 12 - channel 4 global interrupt clear"]
    #[inline(always)]
    pub fn cgif4(&mut self) -> Cgif4W<'_, DmaIfcrSpec> {
        Cgif4W::new(self, 12)
    }
    #[doc = "Bit 13 - channel 4 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif4(&mut self) -> Ctcif4W<'_, DmaIfcrSpec> {
        Ctcif4W::new(self, 13)
    }
    #[doc = "Bit 14 - channel 4 half transfer clear"]
    #[inline(always)]
    pub fn chtif4(&mut self) -> Chtif4W<'_, DmaIfcrSpec> {
        Chtif4W::new(self, 14)
    }
    #[doc = "Bit 15 - channel 4 transfer error clear"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> Cteif4W<'_, DmaIfcrSpec> {
        Cteif4W::new(self, 15)
    }
    #[doc = "Bit 16 - channel 5 global interrupt clear"]
    #[inline(always)]
    pub fn cgif5(&mut self) -> Cgif5W<'_, DmaIfcrSpec> {
        Cgif5W::new(self, 16)
    }
    #[doc = "Bit 17 - channel 5 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif5(&mut self) -> Ctcif5W<'_, DmaIfcrSpec> {
        Ctcif5W::new(self, 17)
    }
    #[doc = "Bit 18 - channel 5 half transfer clear"]
    #[inline(always)]
    pub fn chtif5(&mut self) -> Chtif5W<'_, DmaIfcrSpec> {
        Chtif5W::new(self, 18)
    }
    #[doc = "Bit 19 - channel 5 transfer error clear"]
    #[inline(always)]
    pub fn cteif5(&mut self) -> Cteif5W<'_, DmaIfcrSpec> {
        Cteif5W::new(self, 19)
    }
}
#[doc = "DMA interrupt flag clear reigster\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIfcrSpec;
impl crate::RegisterSpec for DmaIfcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_ifcr::W`](W) writer structure"]
impl crate::Writable for DmaIfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_IFCR to value 0"]
impl crate::Resettable for DmaIfcrSpec {}
