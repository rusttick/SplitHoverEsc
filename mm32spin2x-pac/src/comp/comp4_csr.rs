#[doc = "Register `COMP4_CSR` reader"]
pub type R = crate::R<Comp4CsrSpec>;
#[doc = "Register `COMP4_CSR` writer"]
pub type W = crate::W<Comp4CsrSpec>;
#[doc = "Field `EN` reader - Comparator 4 enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Comparator 4 enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Comparator 4 mode"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Comparator 4 mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INM` reader - Comparator 4 inverting input selection"]
pub type InmR = crate::FieldReader;
#[doc = "Field `INM` writer - Comparator 4 inverting input selection"]
pub type InmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INP` reader - Comparator 4 normal phase input selection"]
pub type InpR = crate::FieldReader;
#[doc = "Field `INP` writer - Comparator 4 normal phase input selection"]
pub type InpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUT` reader - Comparator 4 output selection"]
pub type OutR = crate::FieldReader;
#[doc = "Field `OUT` writer - Comparator 4 output selection"]
pub type OutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `POL` reader - Comparator 4 output polarity"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - Comparator 4 output polarity"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Comparator 4 hysteresis"]
pub type HystR = crate::FieldReader;
#[doc = "Field `HYST` writer - Comparator 4 hysteresis"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFLT` reader - Comparator 4 output filter"]
pub type OfltR = crate::FieldReader;
#[doc = "Field `OFLT` writer - Comparator 4 output filter"]
pub type OfltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STA` reader - Comparator 4 output status"]
pub type StaR = crate::BitReader;
#[doc = "Field `STA` writer - Comparator 4 output status"]
pub type StaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Comparator 4 lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Comparator 4 lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn inm(&self) -> InmR {
        InmR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 7:8 - Comparator 4 normal phase input selection"]
    #[inline(always)]
    pub fn inp(&self) -> InpR {
        InpR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 4 output filter"]
    #[inline(always)]
    pub fn oflt(&self) -> OfltR {
        OfltR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 4 output status"]
    #[inline(always)]
    pub fn sta(&self) -> StaR {
        StaR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Comp4CsrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Comp4CsrSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn inm(&mut self) -> InmW<'_, Comp4CsrSpec> {
        InmW::new(self, 4)
    }
    #[doc = "Bits 7:8 - Comparator 4 normal phase input selection"]
    #[inline(always)]
    pub fn inp(&mut self) -> InpW<'_, Comp4CsrSpec> {
        InpW::new(self, 7)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<'_, Comp4CsrSpec> {
        OutW::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, Comp4CsrSpec> {
        PolW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, Comp4CsrSpec> {
        HystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 4 output filter"]
    #[inline(always)]
    pub fn oflt(&mut self) -> OfltW<'_, Comp4CsrSpec> {
        OfltW::new(self, 18)
    }
    #[doc = "Bit 30 - Comparator 4 output status"]
    #[inline(always)]
    pub fn sta(&mut self) -> StaW<'_, Comp4CsrSpec> {
        StaW::new(self, 30)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, Comp4CsrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "COMP4_CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`comp4_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp4_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp4CsrSpec;
impl crate::RegisterSpec for Comp4CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp4_csr::R`](R) reader structure"]
impl crate::Readable for Comp4CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp4_csr::W`](W) writer structure"]
impl crate::Writable for Comp4CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP4_CSR to value 0"]
impl crate::Resettable for Comp4CsrSpec {}
