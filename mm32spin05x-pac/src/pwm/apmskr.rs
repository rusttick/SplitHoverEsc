#[doc = "Register `APMSKR` reader"]
pub type R = crate::R<ApmskrSpec>;
#[doc = "Register `APMSKR` writer"]
pub type W = crate::W<ApmskrSpec>;
#[doc = "Field `MSKDAT` reader - PWM Mask Data"]
pub type MskdatR = crate::FieldReader;
#[doc = "Field `MSKDAT` writer - PWM Mask Data"]
pub type MskdatW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MSKEN` reader - PWM Mask Function Enable"]
pub type MskenR = crate::FieldReader;
#[doc = "Field `MSKEN` writer - PWM Mask Function Enable"]
pub type MskenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `APM_TRIGSEL` reader - Auto Phase Mask Tigger Selection"]
pub type ApmTrigselR = crate::FieldReader;
#[doc = "Field `APM_TRIGSEL` writer - Auto Phase Mask Tigger Selection"]
pub type ApmTrigselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APM_STRG` reader - Auto Phase Mask Software Trigger"]
pub type ApmStrgR = crate::BitReader;
#[doc = "Field `APM_STRG` writer - Auto Phase Mask Software Trigger"]
pub type ApmStrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTRGI` reader - Expect Next Trigger Input"]
pub type EntrgiR = crate::FieldReader;
#[doc = "Field `ENTRGI` writer - Expect Next Trigger Input"]
pub type EntrgiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTRGI` reader - Current Trigger Input"]
pub type CtrgiR = crate::FieldReader;
#[doc = "Field `CTRGI` writer - Current Trigger Input"]
pub type CtrgiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - PWM Mask Data"]
    #[inline(always)]
    pub fn mskdat(&self) -> MskdatR {
        MskdatR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - PWM Mask Function Enable"]
    #[inline(always)]
    pub fn msken(&self) -> MskenR {
        MskenR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - Auto Phase Mask Tigger Selection"]
    #[inline(always)]
    pub fn apm_trigsel(&self) -> ApmTrigselR {
        ApmTrigselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Auto Phase Mask Software Trigger"]
    #[inline(always)]
    pub fn apm_strg(&self) -> ApmStrgR {
        ApmStrgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Expect Next Trigger Input"]
    #[inline(always)]
    pub fn entrgi(&self) -> EntrgiR {
        EntrgiR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Current Trigger Input"]
    #[inline(always)]
    pub fn ctrgi(&self) -> CtrgiR {
        CtrgiR::new(((self.bits >> 23) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PWM Mask Data"]
    #[inline(always)]
    pub fn mskdat(&mut self) -> MskdatW<'_, ApmskrSpec> {
        MskdatW::new(self, 0)
    }
    #[doc = "Bits 8:13 - PWM Mask Function Enable"]
    #[inline(always)]
    pub fn msken(&mut self) -> MskenW<'_, ApmskrSpec> {
        MskenW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Auto Phase Mask Tigger Selection"]
    #[inline(always)]
    pub fn apm_trigsel(&mut self) -> ApmTrigselW<'_, ApmskrSpec> {
        ApmTrigselW::new(self, 16)
    }
    #[doc = "Bit 18 - Auto Phase Mask Software Trigger"]
    #[inline(always)]
    pub fn apm_strg(&mut self) -> ApmStrgW<'_, ApmskrSpec> {
        ApmStrgW::new(self, 18)
    }
    #[doc = "Bits 20:22 - Expect Next Trigger Input"]
    #[inline(always)]
    pub fn entrgi(&mut self) -> EntrgiW<'_, ApmskrSpec> {
        EntrgiW::new(self, 20)
    }
    #[doc = "Bits 23:25 - Current Trigger Input"]
    #[inline(always)]
    pub fn ctrgi(&mut self) -> CtrgiW<'_, ApmskrSpec> {
        CtrgiW::new(self, 23)
    }
}
#[doc = "Auto phase mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`apmskr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apmskr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApmskrSpec;
impl crate::RegisterSpec for ApmskrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apmskr::R`](R) reader structure"]
impl crate::Readable for ApmskrSpec {}
#[doc = "`write(|w| ..)` method takes [`apmskr::W`](W) writer structure"]
impl crate::Writable for ApmskrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APMSKR to value 0"]
impl crate::Resettable for ApmskrSpec {}
