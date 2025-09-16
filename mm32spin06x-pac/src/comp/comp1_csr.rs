#[doc = "Register `COMP1_CSR` reader"]
pub type R = crate::R<Comp1CsrSpec>;
#[doc = "Register `COMP1_CSR` writer"]
pub type W = crate::W<Comp1CsrSpec>;
#[doc = "Field `EN` reader - Comparator 1 enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Comparator 1 enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Comparator 1 mode"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Comparator 1 mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INM_SEL` reader - Comparator 1 inverting input selection"]
pub type InmSelR = crate::FieldReader;
#[doc = "Field `INM_SEL` writer - Comparator 1 inverting input selection"]
pub type InmSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INP_SEL` reader - Comparator 1 normal phase input selection"]
pub type InpSelR = crate::FieldReader;
#[doc = "Field `INP_SEL` writer - Comparator 1 normal phase input selection"]
pub type InpSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUT_SEL` reader - Comparator 1 output selection"]
pub type OutSelR = crate::FieldReader;
#[doc = "Field `OUT_SEL` writer - Comparator 1 output selection"]
pub type OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `POL` reader - Comparator 1 output polarity"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - Comparator 1 output polarity"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Comparator 1 hysteresis"]
pub type HystR = crate::FieldReader;
#[doc = "Field `HYST` writer - Comparator 1 hysteresis"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFLT` reader - Comparator output filter"]
pub type OfltR = crate::FieldReader;
#[doc = "Field `OFLT` writer - Comparator output filter"]
pub type OfltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUT` reader - Comparator 1 lock"]
pub type OutR = crate::BitReader;
#[doc = "Field `OUT` writer - Comparator 1 lock"]
pub type OutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Comparator lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Comparator lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn inm_sel(&self) -> InmSelR {
        InmSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 7:8 - Comparator 1 normal phase input selection"]
    #[inline(always)]
    pub fn inp_sel(&self) -> InpSelR {
        InpSelR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn out_sel(&self) -> OutSelR {
        OutSelR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator output filter"]
    #[inline(always)]
    pub fn oflt(&self) -> OfltR {
        OfltR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 1 lock"]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Comp1CsrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Comp1CsrSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn inm_sel(&mut self) -> InmSelW<'_, Comp1CsrSpec> {
        InmSelW::new(self, 4)
    }
    #[doc = "Bits 7:8 - Comparator 1 normal phase input selection"]
    #[inline(always)]
    pub fn inp_sel(&mut self) -> InpSelW<'_, Comp1CsrSpec> {
        InpSelW::new(self, 7)
    }
    #[doc = "Bits 10:13 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn out_sel(&mut self) -> OutSelW<'_, Comp1CsrSpec> {
        OutSelW::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, Comp1CsrSpec> {
        PolW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, Comp1CsrSpec> {
        HystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator output filter"]
    #[inline(always)]
    pub fn oflt(&mut self) -> OfltW<'_, Comp1CsrSpec> {
        OfltW::new(self, 18)
    }
    #[doc = "Bit 30 - Comparator 1 lock"]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<'_, Comp1CsrSpec> {
        OutW::new(self, 30)
    }
    #[doc = "Bit 31 - Comparator lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, Comp1CsrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "COMP1 Control State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp1CsrSpec;
impl crate::RegisterSpec for Comp1CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1_csr::R`](R) reader structure"]
impl crate::Readable for Comp1CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp1_csr::W`](W) writer structure"]
impl crate::Writable for Comp1CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for Comp1CsrSpec {}
