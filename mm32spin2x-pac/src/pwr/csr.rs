#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WufR = crate::BitReader;
#[doc = "Field `WUF` writer - Wakeup flag"]
pub type WufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBF` reader - Standby flag"]
pub type SbfR = crate::BitReader;
#[doc = "Field `SBF` writer - Standby flag"]
pub type SbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDO` reader - PVD output"]
pub type PvdoR = crate::BitReader;
#[doc = "Field `PVDO` writer - PVD output"]
pub type PvdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP` reader - Enable WKUP pin"]
pub type EwupR = crate::BitReader;
#[doc = "Field `EWUP` writer - Enable WKUP pin"]
pub type EwupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SbfR {
        SbfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PvdoR {
        PvdoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn ewup(&self) -> EwupR {
        EwupR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&mut self) -> WufW<'_, CsrSpec> {
        WufW::new(self, 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&mut self) -> SbfW<'_, CsrSpec> {
        SbfW::new(self, 1)
    }
    #[doc = "Bit 2 - PVD output"]
    #[inline(always)]
    pub fn pvdo(&mut self) -> PvdoW<'_, CsrSpec> {
        PvdoW::new(self, 2)
    }
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn ewup(&mut self) -> EwupW<'_, CsrSpec> {
        EwupW::new(self, 8)
    }
}
#[doc = "CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
