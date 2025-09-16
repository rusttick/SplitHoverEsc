#[doc = "Register `CHSR` reader"]
pub type R = crate::R<ChsrSpec>;
#[doc = "Register `CHSR` writer"]
pub type W = crate::W<ChsrSpec>;
#[doc = "Field `CHEN0` reader - Analog input channel 0 enable"]
pub type Chen0R = crate::BitReader;
#[doc = "Field `CHEN0` writer - Analog input channel 0 enable"]
pub type Chen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN1` reader - Analog input channel 1 enable"]
pub type Chen1R = crate::BitReader;
#[doc = "Field `CHEN1` writer - Analog input channel 1 enable"]
pub type Chen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN2` reader - Analog input channel 2 enable"]
pub type Chen2R = crate::BitReader;
#[doc = "Field `CHEN2` writer - Analog input channel 2 enable"]
pub type Chen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN3` reader - Analog input channel 3 enable"]
pub type Chen3R = crate::BitReader;
#[doc = "Field `CHEN3` writer - Analog input channel 3 enable"]
pub type Chen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN4` reader - Analog input channel 4 enable"]
pub type Chen4R = crate::BitReader;
#[doc = "Field `CHEN4` writer - Analog input channel 4 enable"]
pub type Chen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN5` reader - Analog input channel 5 enable"]
pub type Chen5R = crate::BitReader;
#[doc = "Field `CHEN5` writer - Analog input channel 5 enable"]
pub type Chen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN6` reader - Analog input channel 6 enable"]
pub type Chen6R = crate::BitReader;
#[doc = "Field `CHEN6` writer - Analog input channel 6 enable"]
pub type Chen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN7` reader - Analog input channel 7 enable"]
pub type Chen7R = crate::BitReader;
#[doc = "Field `CHEN7` writer - Analog input channel 7 enable"]
pub type Chen7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN8` reader - Analog input channel 8 enable"]
pub type Chen8R = crate::BitReader;
#[doc = "Field `CHEN8` writer - Analog input channel 8 enable"]
pub type Chen8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN9` reader - Analog input channel 9 enable"]
pub type Chen9R = crate::BitReader;
#[doc = "Field `CHEN9` writer - Analog input channel 9 enable"]
pub type Chen9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENTS` reader - Temperature Sensor channel enable"]
pub type ChentsR = crate::BitReader;
#[doc = "Field `CHENTS` writer - Temperature Sensor channel enable"]
pub type ChentsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENVS` reader - Internal reference voltage channel enable"]
pub type ChenvsR = crate::BitReader;
#[doc = "Field `CHENVS` writer - Internal reference voltage channel enable"]
pub type ChenvsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog input channel 0 enable"]
    #[inline(always)]
    pub fn chen0(&self) -> Chen0R {
        Chen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog input channel 1 enable"]
    #[inline(always)]
    pub fn chen1(&self) -> Chen1R {
        Chen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog input channel 2 enable"]
    #[inline(always)]
    pub fn chen2(&self) -> Chen2R {
        Chen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog input channel 3 enable"]
    #[inline(always)]
    pub fn chen3(&self) -> Chen3R {
        Chen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog input channel 4 enable"]
    #[inline(always)]
    pub fn chen4(&self) -> Chen4R {
        Chen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog input channel 5 enable"]
    #[inline(always)]
    pub fn chen5(&self) -> Chen5R {
        Chen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog input channel 6 enable"]
    #[inline(always)]
    pub fn chen6(&self) -> Chen6R {
        Chen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog input channel 7 enable"]
    #[inline(always)]
    pub fn chen7(&self) -> Chen7R {
        Chen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog input channel 8 enable"]
    #[inline(always)]
    pub fn chen8(&self) -> Chen8R {
        Chen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog input channel 9 enable"]
    #[inline(always)]
    pub fn chen9(&self) -> Chen9R {
        Chen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Temperature Sensor channel enable"]
    #[inline(always)]
    pub fn chents(&self) -> ChentsR {
        ChentsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Internal reference voltage channel enable"]
    #[inline(always)]
    pub fn chenvs(&self) -> ChenvsR {
        ChenvsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog input channel 0 enable"]
    #[inline(always)]
    pub fn chen0(&mut self) -> Chen0W<'_, ChsrSpec> {
        Chen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Analog input channel 1 enable"]
    #[inline(always)]
    pub fn chen1(&mut self) -> Chen1W<'_, ChsrSpec> {
        Chen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Analog input channel 2 enable"]
    #[inline(always)]
    pub fn chen2(&mut self) -> Chen2W<'_, ChsrSpec> {
        Chen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Analog input channel 3 enable"]
    #[inline(always)]
    pub fn chen3(&mut self) -> Chen3W<'_, ChsrSpec> {
        Chen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog input channel 4 enable"]
    #[inline(always)]
    pub fn chen4(&mut self) -> Chen4W<'_, ChsrSpec> {
        Chen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog input channel 5 enable"]
    #[inline(always)]
    pub fn chen5(&mut self) -> Chen5W<'_, ChsrSpec> {
        Chen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Analog input channel 6 enable"]
    #[inline(always)]
    pub fn chen6(&mut self) -> Chen6W<'_, ChsrSpec> {
        Chen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog input channel 7 enable"]
    #[inline(always)]
    pub fn chen7(&mut self) -> Chen7W<'_, ChsrSpec> {
        Chen7W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog input channel 8 enable"]
    #[inline(always)]
    pub fn chen8(&mut self) -> Chen8W<'_, ChsrSpec> {
        Chen8W::new(self, 8)
    }
    #[doc = "Bit 9 - Analog input channel 9 enable"]
    #[inline(always)]
    pub fn chen9(&mut self) -> Chen9W<'_, ChsrSpec> {
        Chen9W::new(self, 9)
    }
    #[doc = "Bit 14 - Temperature Sensor channel enable"]
    #[inline(always)]
    pub fn chents(&mut self) -> ChentsW<'_, ChsrSpec> {
        ChentsW::new(self, 14)
    }
    #[doc = "Bit 15 - Internal reference voltage channel enable"]
    #[inline(always)]
    pub fn chenvs(&mut self) -> ChenvsW<'_, ChsrSpec> {
        ChenvsW::new(self, 15)
    }
}
#[doc = "Channel select register\n\nYou can [`read`](crate::Reg::read) this register and get [`chsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChsrSpec;
impl crate::RegisterSpec for ChsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsr::R`](R) reader structure"]
impl crate::Readable for ChsrSpec {}
#[doc = "`write(|w| ..)` method takes [`chsr::W`](W) writer structure"]
impl crate::Writable for ChsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSR to value 0"]
impl crate::Resettable for ChsrSpec {}
