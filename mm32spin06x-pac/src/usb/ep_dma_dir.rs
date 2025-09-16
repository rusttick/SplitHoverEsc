#[doc = "Register `EP_DMA_DIR` reader"]
pub type R = crate::R<EpDmaDirSpec>;
#[doc = "Register `EP_DMA_DIR` writer"]
pub type W = crate::W<EpDmaDirSpec>;
#[doc = "Field `DAM_DIR1` reader - Point 1 Dma Dir"]
pub type DamDir1R = crate::BitReader;
#[doc = "Field `DAM_DIR1` writer - Point 1 Dma Dir"]
pub type DamDir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAM_DIR2` reader - Point 2 Dma Dir"]
pub type DamDir2R = crate::BitReader;
#[doc = "Field `DAM_DIR2` writer - Point 2 Dma Dir"]
pub type DamDir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAM_DIR3` reader - Point 3 Dma Dir"]
pub type DamDir3R = crate::BitReader;
#[doc = "Field `DAM_DIR3` writer - Point 3 Dma Dir"]
pub type DamDir3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAM_DIR4` reader - Point 4 Dma Dir"]
pub type DamDir4R = crate::BitReader;
#[doc = "Field `DAM_DIR4` writer - Point 4 Dma Dir"]
pub type DamDir4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Point 1 Dma Dir"]
    #[inline(always)]
    pub fn dam_dir1(&self) -> DamDir1R {
        DamDir1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Point 2 Dma Dir"]
    #[inline(always)]
    pub fn dam_dir2(&self) -> DamDir2R {
        DamDir2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Point 3 Dma Dir"]
    #[inline(always)]
    pub fn dam_dir3(&self) -> DamDir3R {
        DamDir3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Point 4 Dma Dir"]
    #[inline(always)]
    pub fn dam_dir4(&self) -> DamDir4R {
        DamDir4R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Point 1 Dma Dir"]
    #[inline(always)]
    pub fn dam_dir1(&mut self) -> DamDir1W<'_, EpDmaDirSpec> {
        DamDir1W::new(self, 0)
    }
    #[doc = "Bit 1 - Point 2 Dma Dir"]
    #[inline(always)]
    pub fn dam_dir2(&mut self) -> DamDir2W<'_, EpDmaDirSpec> {
        DamDir2W::new(self, 1)
    }
    #[doc = "Bit 2 - Point 3 Dma Dir"]
    #[inline(always)]
    pub fn dam_dir3(&mut self) -> DamDir3W<'_, EpDmaDirSpec> {
        DamDir3W::new(self, 2)
    }
    #[doc = "Bit 3 - Point 4 Dma Dir"]
    #[inline(always)]
    pub fn dam_dir4(&mut self) -> DamDir4W<'_, EpDmaDirSpec> {
        DamDir4W::new(self, 3)
    }
}
#[doc = "DMA End-point direction register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_dma_dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_dma_dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpDmaDirSpec;
impl crate::RegisterSpec for EpDmaDirSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep_dma_dir::R`](R) reader structure"]
impl crate::Readable for EpDmaDirSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_dma_dir::W`](W) writer structure"]
impl crate::Writable for EpDmaDirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP_DMA_DIR to value 0x01"]
impl crate::Resettable for EpDmaDirSpec {
    const RESET_VALUE: u16 = 0x01;
}
