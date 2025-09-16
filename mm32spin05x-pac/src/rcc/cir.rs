#[doc = "Register `CIR` reader"]
pub type R = crate::R<CirSpec>;
#[doc = "Register `CIR` writer"]
pub type W = crate::W<CirSpec>;
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LsirdyfR = crate::BitReader;
#[doc = "Field `LSIRDYF` writer - LSI ready interrupt flag"]
pub type LsirdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub type HsirdyfR = crate::BitReader;
#[doc = "Field `HSIRDYF` writer - HSI ready interrupt flag"]
pub type HsirdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub type HserdyfR = crate::BitReader;
#[doc = "Field `HSERDYF` writer - HSE ready interrupt flag"]
pub type HserdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub type CssfR = crate::BitReader;
#[doc = "Field `CSSF` writer - Clock security system interrupt flag"]
pub type CssfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub type LsirdyieR = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub type LsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub type HsirdyieR = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub type HsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub type HserdyieR = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub type HserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDYC` reader - LSI ready interrupt clear"]
pub type LsirdycR = crate::BitReader;
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYC` reader - HSI ready interrupt clear"]
pub type HsirdycR = crate::BitReader;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub type HsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYC` reader - HSE ready interrupt clear"]
pub type HserdycR = crate::BitReader;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub type HserdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSC` reader - Clock security system interrupt clear"]
pub type CsscR = crate::BitReader;
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CsscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LsirdyfR {
        LsirdyfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HsirdyfR {
        HsirdyfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HserdyfR {
        HserdyfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CssfR {
        CssfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LsirdyieR {
        LsirdyieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HsirdyieR {
        HsirdyieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HserdyieR {
        HserdyieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    pub fn lsirdyc(&self) -> LsirdycR {
        LsirdycR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    pub fn hsirdyc(&self) -> HsirdycR {
        HsirdycR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    pub fn hserdyc(&self) -> HserdycR {
        HserdycR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    pub fn cssc(&self) -> CsscR {
        CsscR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&mut self) -> LsirdyfW<'_, CirSpec> {
        LsirdyfW::new(self, 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&mut self) -> HsirdyfW<'_, CirSpec> {
        HsirdyfW::new(self, 2)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&mut self) -> HserdyfW<'_, CirSpec> {
        HserdyfW::new(self, 3)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&mut self) -> CssfW<'_, CirSpec> {
        CssfW::new(self, 7)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LsirdyieW<'_, CirSpec> {
        LsirdyieW::new(self, 8)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HsirdyieW<'_, CirSpec> {
        HsirdyieW::new(self, 10)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HserdyieW<'_, CirSpec> {
        HserdyieW::new(self, 11)
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LsirdycW<'_, CirSpec> {
        LsirdycW::new(self, 16)
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HsirdycW<'_, CirSpec> {
        HsirdycW::new(self, 18)
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HserdycW<'_, CirSpec> {
        HserdycW::new(self, 19)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    pub fn cssc(&mut self) -> CsscW<'_, CirSpec> {
        CsscW::new(self, 23)
    }
}
#[doc = "Clock Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CirSpec;
impl crate::RegisterSpec for CirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir::R`](R) reader structure"]
impl crate::Readable for CirSpec {}
#[doc = "`write(|w| ..)` method takes [`cir::W`](W) writer structure"]
impl crate::Writable for CirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CirSpec {}
