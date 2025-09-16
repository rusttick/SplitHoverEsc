#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `STARTSEL` reader - start select"]
pub type StartselR = crate::BitReader;
#[doc = "Field `STARTSEL` writer - start select"]
pub type StartselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPSEL` reader - stop select"]
pub type StopselR = crate::BitReader;
#[doc = "Field `STOPSEL` writer - stop select"]
pub type StopselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSEL` reader - master or slave select"]
pub type MselR = crate::BitReader;
#[doc = "Field `MSEL` writer - master or slave select"]
pub type MselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTXEN` reader - Vtx enable"]
pub type VtxenR = crate::BitReader;
#[doc = "Field `VTXEN` writer - Vtx enable"]
pub type VtxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTXSEL` reader - Vtx select"]
pub type VtxselR = crate::BitReader;
#[doc = "Field `VTXSEL` writer - Vtx select"]
pub type VtxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAXBIT` reader - max tx and rx bit"]
pub type MaxbitR = crate::FieldReader<u16>;
#[doc = "Field `MAXBIT` writer - max tx and rx bit"]
pub type MaxbitW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - start select"]
    #[inline(always)]
    pub fn startsel(&self) -> StartselR {
        StartselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - stop select"]
    #[inline(always)]
    pub fn stopsel(&self) -> StopselR {
        StopselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - master or slave select"]
    #[inline(always)]
    pub fn msel(&self) -> MselR {
        MselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vtx enable"]
    #[inline(always)]
    pub fn vtxen(&self) -> VtxenR {
        VtxenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Vtx select"]
    #[inline(always)]
    pub fn vtxsel(&self) -> VtxselR {
        VtxselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:31 - max tx and rx bit"]
    #[inline(always)]
    pub fn maxbit(&self) -> MaxbitR {
        MaxbitR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - start select"]
    #[inline(always)]
    pub fn startsel(&mut self) -> StartselW<'_, CfgSpec> {
        StartselW::new(self, 0)
    }
    #[doc = "Bit 1 - stop select"]
    #[inline(always)]
    pub fn stopsel(&mut self) -> StopselW<'_, CfgSpec> {
        StopselW::new(self, 1)
    }
    #[doc = "Bit 2 - master or slave select"]
    #[inline(always)]
    pub fn msel(&mut self) -> MselW<'_, CfgSpec> {
        MselW::new(self, 2)
    }
    #[doc = "Bit 3 - Vtx enable"]
    #[inline(always)]
    pub fn vtxen(&mut self) -> VtxenW<'_, CfgSpec> {
        VtxenW::new(self, 3)
    }
    #[doc = "Bit 4 - Vtx select"]
    #[inline(always)]
    pub fn vtxsel(&mut self) -> VtxselW<'_, CfgSpec> {
        VtxselW::new(self, 4)
    }
    #[doc = "Bits 16:31 - max tx and rx bit"]
    #[inline(always)]
    pub fn maxbit(&mut self) -> MaxbitW<'_, CfgSpec> {
        MaxbitW::new(self, 16)
    }
}
#[doc = "Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0x08"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x08;
}
