#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `SCEN` reader - ISO7816 enable control bit"]
pub type ScenR = crate::BitReader;
#[doc = "Field `SCEN` writer - ISO7816 enable control bit"]
pub type ScenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAEN` reader - ISO7816 check auto-response bit"]
pub type ScaenR = crate::BitReader;
#[doc = "Field `SCAEN` writer - ISO7816 check auto-response bit"]
pub type ScaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - Master receive frame answer bit"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - Master receive frame answer bit"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCFCNT` reader - ISO7816 protection counter bit"]
pub type ScfcntR = crate::FieldReader;
#[doc = "Field `SCFCNT` writer - ISO7816 protection counter bit"]
pub type ScfcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HDSEL` reader - Single-wire half-duplex mode selection bit"]
pub type HdselR = crate::BitReader;
#[doc = "Field `HDSEL` writer - Single-wire half-duplex mode selection bit"]
pub type HdselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ISO7816 enable control bit"]
    #[inline(always)]
    pub fn scen(&self) -> ScenR {
        ScenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISO7816 check auto-response bit"]
    #[inline(always)]
    pub fn scaen(&self) -> ScaenR {
        ScaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master receive frame answer bit"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:11 - ISO7816 protection counter bit"]
    #[inline(always)]
    pub fn scfcnt(&self) -> ScfcntR {
        ScfcntR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - Single-wire half-duplex mode selection bit"]
    #[inline(always)]
    pub fn hdsel(&self) -> HdselR {
        HdselR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ISO7816 enable control bit"]
    #[inline(always)]
    pub fn scen(&mut self) -> ScenW<'_, ScrSpec> {
        ScenW::new(self, 0)
    }
    #[doc = "Bit 1 - ISO7816 check auto-response bit"]
    #[inline(always)]
    pub fn scaen(&mut self) -> ScaenW<'_, ScrSpec> {
        ScaenW::new(self, 1)
    }
    #[doc = "Bit 2 - Master receive frame answer bit"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, ScrSpec> {
        NackW::new(self, 2)
    }
    #[doc = "Bits 4:11 - ISO7816 protection counter bit"]
    #[inline(always)]
    pub fn scfcnt(&mut self) -> ScfcntW<'_, ScrSpec> {
        ScfcntW::new(self, 4)
    }
    #[doc = "Bit 12 - Single-wire half-duplex mode selection bit"]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HdselW<'_, ScrSpec> {
        HdselW::new(self, 12)
    }
}
#[doc = "SCR\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {}
