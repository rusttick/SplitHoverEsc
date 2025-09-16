#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `CCE` reader - Current Compensation Enable"]
pub type CceR = crate::BitReader;
#[doc = "Field `CCE` writer - Current Compensation Enable"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPE` reader - Current Protection Enable"]
pub type CpeR = crate::BitReader;
#[doc = "Field `CPE` writer - Current Protection Enable"]
pub type CpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APMTIE` reader - Auto Phase Mask Trigger Interrupt Enable"]
pub type ApmtieR = crate::BitReader;
#[doc = "Field `APMTIE` writer - Auto Phase Mask Trigger Interrupt Enable"]
pub type ApmtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRIE` reader - Trigger Error Interrupt Enable"]
pub type TerrieR = crate::BitReader;
#[doc = "Field `TERRIE` writer - Trigger Error Interrupt Enable"]
pub type TerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC_TRGSEL` reader - Current Protection Trigger Selection"]
pub type CcTrgselR = crate::FieldReader;
#[doc = "Field `CC_TRGSEL` writer - Current Protection Trigger Selection"]
pub type CcTrgselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CC_STRG` reader - Current Compensation Software Trigger"]
pub type CcStrgR = crate::BitReader;
#[doc = "Field `CC_STRG` writer - Current Compensation Software Trigger"]
pub type CcStrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP_MDSEL` reader - Current Protection Mode Selection"]
pub type CpMdselR = crate::BitReader;
#[doc = "Field `CP_MDSEL` writer - Current Protection Mode Selection"]
pub type CpMdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APMTIF` reader - Auto Phase Mask Trigger Flag"]
pub type ApmtifR = crate::BitReader;
#[doc = "Field `APMTIF` writer - Auto Phase Mask Trigger Flag"]
pub type ApmtifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRIF` reader - Trigger Error Flag"]
pub type TerrifR = crate::BitReader;
#[doc = "Field `TERRIF` writer - Trigger Error Flag"]
pub type TerrifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOFLT` reader - GPIO Input Filter"]
pub type IofltR = crate::FieldReader;
#[doc = "Field `IOFLT` writer - GPIO Input Filter"]
pub type IofltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HALL_TRGSEL` reader - Hall Sensor Trigger 3-channel select"]
pub type HallTrgselR = crate::FieldReader;
#[doc = "Field `HALL_TRGSEL` writer - Hall Sensor Trigger 3-channel select"]
pub type HallTrgselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CUREN` reader - Enable Current Input Status Value"]
pub type CurenR = crate::BitReader;
#[doc = "Field `CUREN` writer - Enable Current Input Status Value"]
pub type CurenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSKDAT` reader - Immediate Output of The Port when PWM is Masked"]
pub type MskdatR = crate::FieldReader;
#[doc = "Field `MSKDAT` writer - Immediate Output of The Port when PWM is Masked"]
pub type MskdatW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MSKEN_CURR` reader - PWM Output Mask Immediately Enable"]
pub type MskenCurrR = crate::FieldReader;
#[doc = "Field `MSKEN_CURR` writer - PWM Output Mask Immediately Enable"]
pub type MskenCurrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Current Compensation Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Current Protection Enable"]
    #[inline(always)]
    pub fn cpe(&self) -> CpeR {
        CpeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto Phase Mask Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn apmtie(&self) -> ApmtieR {
        ApmtieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger Error Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&self) -> TerrieR {
        TerrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Current Protection Trigger Selection"]
    #[inline(always)]
    pub fn cc_trgsel(&self) -> CcTrgselR {
        CcTrgselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Current Compensation Software Trigger"]
    #[inline(always)]
    pub fn cc_strg(&self) -> CcStrgR {
        CcStrgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Current Protection Mode Selection"]
    #[inline(always)]
    pub fn cp_mdsel(&self) -> CpMdselR {
        CpMdselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Auto Phase Mask Trigger Flag"]
    #[inline(always)]
    pub fn apmtif(&self) -> ApmtifR {
        ApmtifR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Trigger Error Flag"]
    #[inline(always)]
    pub fn terrif(&self) -> TerrifR {
        TerrifR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - GPIO Input Filter"]
    #[inline(always)]
    pub fn ioflt(&self) -> IofltR {
        IofltR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Hall Sensor Trigger 3-channel select"]
    #[inline(always)]
    pub fn hall_trgsel(&self) -> HallTrgselR {
        HallTrgselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Enable Current Input Status Value"]
    #[inline(always)]
    pub fn curen(&self) -> CurenR {
        CurenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:24 - Immediate Output of The Port when PWM is Masked"]
    #[inline(always)]
    pub fn mskdat(&self) -> MskdatR {
        MskdatR::new(((self.bits >> 19) & 0x3f) as u8)
    }
    #[doc = "Bits 25:30 - PWM Output Mask Immediately Enable"]
    #[inline(always)]
    pub fn msken_curr(&self) -> MskenCurrR {
        MskenCurrR::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current Compensation Enable"]
    #[inline(always)]
    pub fn cce(&mut self) -> CceW<'_, CsrSpec> {
        CceW::new(self, 0)
    }
    #[doc = "Bit 1 - Current Protection Enable"]
    #[inline(always)]
    pub fn cpe(&mut self) -> CpeW<'_, CsrSpec> {
        CpeW::new(self, 1)
    }
    #[doc = "Bit 2 - Auto Phase Mask Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn apmtie(&mut self) -> ApmtieW<'_, CsrSpec> {
        ApmtieW::new(self, 2)
    }
    #[doc = "Bit 3 - Trigger Error Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&mut self) -> TerrieW<'_, CsrSpec> {
        TerrieW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Current Protection Trigger Selection"]
    #[inline(always)]
    pub fn cc_trgsel(&mut self) -> CcTrgselW<'_, CsrSpec> {
        CcTrgselW::new(self, 4)
    }
    #[doc = "Bit 6 - Current Compensation Software Trigger"]
    #[inline(always)]
    pub fn cc_strg(&mut self) -> CcStrgW<'_, CsrSpec> {
        CcStrgW::new(self, 6)
    }
    #[doc = "Bit 10 - Current Protection Mode Selection"]
    #[inline(always)]
    pub fn cp_mdsel(&mut self) -> CpMdselW<'_, CsrSpec> {
        CpMdselW::new(self, 10)
    }
    #[doc = "Bit 11 - Auto Phase Mask Trigger Flag"]
    #[inline(always)]
    pub fn apmtif(&mut self) -> ApmtifW<'_, CsrSpec> {
        ApmtifW::new(self, 11)
    }
    #[doc = "Bit 12 - Trigger Error Flag"]
    #[inline(always)]
    pub fn terrif(&mut self) -> TerrifW<'_, CsrSpec> {
        TerrifW::new(self, 12)
    }
    #[doc = "Bits 13:15 - GPIO Input Filter"]
    #[inline(always)]
    pub fn ioflt(&mut self) -> IofltW<'_, CsrSpec> {
        IofltW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Hall Sensor Trigger 3-channel select"]
    #[inline(always)]
    pub fn hall_trgsel(&mut self) -> HallTrgselW<'_, CsrSpec> {
        HallTrgselW::new(self, 16)
    }
    #[doc = "Bit 18 - Enable Current Input Status Value"]
    #[inline(always)]
    pub fn curen(&mut self) -> CurenW<'_, CsrSpec> {
        CurenW::new(self, 18)
    }
    #[doc = "Bits 19:24 - Immediate Output of The Port when PWM is Masked"]
    #[inline(always)]
    pub fn mskdat(&mut self) -> MskdatW<'_, CsrSpec> {
        MskdatW::new(self, 19)
    }
    #[doc = "Bits 25:30 - PWM Output Mask Immediately Enable"]
    #[inline(always)]
    pub fn msken_curr(&mut self) -> MskenCurrW<'_, CsrSpec> {
        MskenCurrW::new(self, 25)
    }
}
#[doc = "Control PWM output status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
