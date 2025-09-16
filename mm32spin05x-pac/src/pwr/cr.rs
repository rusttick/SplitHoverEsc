#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `LPDS` reader - Auto Phase Mask delay load register"]
pub type LpdsR = crate::BitReader;
#[doc = "Field `LPDS` writer - Auto Phase Mask delay load register"]
pub type LpdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDDS` reader - Power down deepsleep"]
pub type PddsR = crate::BitReader;
#[doc = "Field `PDDS` writer - Power down deepsleep"]
pub type PddsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF` reader - Clear wakeup flag"]
pub type CwufR = crate::BitReader;
#[doc = "Field `CWUF` writer - Clear wakeup flag"]
pub type CwufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSBF` reader - Clear standby flag"]
pub type CsbfR = crate::BitReader;
#[doc = "Field `CSBF` writer - Clear standby flag"]
pub type CsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PvdeR = crate::BitReader;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PvdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PVD level selection"]
pub type PlsR = crate::FieldReader;
#[doc = "Field `PLS` writer - PVD level selection"]
pub type PlsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Auto Phase Mask delay load register"]
    #[inline(always)]
    pub fn lpds(&self) -> LpdsR {
        LpdsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    pub fn pdds(&self) -> PddsR {
        PddsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    pub fn cwuf(&self) -> CwufR {
        CwufR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CsbfR {
        CsbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PvdeR {
        PvdeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 9:12 - PVD level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PlsR {
        PlsR::new(((self.bits >> 9) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Phase Mask delay load register"]
    #[inline(always)]
    pub fn lpds(&mut self) -> LpdsW<'_, CrSpec> {
        LpdsW::new(self, 0)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    pub fn pdds(&mut self) -> PddsW<'_, CrSpec> {
        PddsW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    pub fn cwuf(&mut self) -> CwufW<'_, CrSpec> {
        CwufW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    pub fn csbf(&mut self) -> CsbfW<'_, CrSpec> {
        CsbfW::new(self, 3)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PvdeW<'_, CrSpec> {
        PvdeW::new(self, 4)
    }
    #[doc = "Bits 9:12 - PVD level selection"]
    #[inline(always)]
    pub fn pls(&mut self) -> PlsW<'_, CrSpec> {
        PlsW::new(self, 9)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
