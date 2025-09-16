#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `CTE` reader - Clear tamper event"]
pub type CteR = crate::BitReader;
#[doc = "Field `CTE` writer - Clear tamper event"]
pub type CteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTI` reader - Clear tamper interrupt"]
pub type CtiR = crate::BitReader;
#[doc = "Field `CTI` writer - Clear tamper interrupt"]
pub type CtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIE` reader - TAMPER pin interrupt enable"]
pub type TpieR = crate::BitReader;
#[doc = "Field `TPIE` writer - TAMPER pin interrupt enable"]
pub type TpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEF` reader - Tamper event flag"]
pub type TefR = crate::BitReader;
#[doc = "Field `TEF` writer - Tamper event flag"]
pub type TefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - Tamper interrupt flag"]
pub type TifR = crate::BitReader;
#[doc = "Field `TIF` writer - Tamper interrupt flag"]
pub type TifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear tamper event"]
    #[inline(always)]
    pub fn cte(&self) -> CteR {
        CteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear tamper interrupt"]
    #[inline(always)]
    pub fn cti(&self) -> CtiR {
        CtiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMPER pin interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TpieR {
        TpieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    pub fn tef(&self) -> TefR {
        TefR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear tamper event"]
    #[inline(always)]
    pub fn cte(&mut self) -> CteW<'_, CsrSpec> {
        CteW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear tamper interrupt"]
    #[inline(always)]
    pub fn cti(&mut self) -> CtiW<'_, CsrSpec> {
        CtiW::new(self, 1)
    }
    #[doc = "Bit 2 - TAMPER pin interrupt enable"]
    #[inline(always)]
    pub fn tpie(&mut self) -> TpieW<'_, CsrSpec> {
        TpieW::new(self, 2)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    pub fn tef(&mut self) -> TefW<'_, CsrSpec> {
        TefW::new(self, 8)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    pub fn tif(&mut self) -> TifW<'_, CsrSpec> {
        TifW::new(self, 9)
    }
}
#[doc = "BKP control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
