#[doc = "Register `CMR_P` writer"]
pub type W = crate::W<CmrPSpec>;
#[doc = "Field `TR` writer - Transmission request"]
pub type TrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AT` writer - Abort transmission"]
pub type AtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRB` writer - Release receive buffer"]
pub type RrbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDO` writer - Clear data overrun"]
pub type CdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRR` writer - Self reset request"]
pub type SrrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERB` writer - Empty Rxbuffer"]
pub type ErbW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmission request"]
    #[inline(always)]
    pub fn tr(&mut self) -> TrW<'_, CmrPSpec> {
        TrW::new(self, 0)
    }
    #[doc = "Bit 1 - Abort transmission"]
    #[inline(always)]
    pub fn at(&mut self) -> AtW<'_, CmrPSpec> {
        AtW::new(self, 1)
    }
    #[doc = "Bit 2 - Release receive buffer"]
    #[inline(always)]
    pub fn rrb(&mut self) -> RrbW<'_, CmrPSpec> {
        RrbW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear data overrun"]
    #[inline(always)]
    pub fn cdo(&mut self) -> CdoW<'_, CmrPSpec> {
        CdoW::new(self, 3)
    }
    #[doc = "Bit 4 - Self reset request"]
    #[inline(always)]
    pub fn srr(&mut self) -> SrrW<'_, CmrPSpec> {
        SrrW::new(self, 4)
    }
    #[doc = "Bit 5 - Empty Rxbuffer"]
    #[inline(always)]
    pub fn erb(&mut self) -> ErbW<'_, CmrPSpec> {
        ErbW::new(self, 5)
    }
}
#[doc = "Peli Command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr_p::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmrPSpec;
impl crate::RegisterSpec for CmrPSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmr_p::W`](W) writer structure"]
impl crate::Writable for CmrPSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMR_P to value 0"]
impl crate::Resettable for CmrPSpec {}
