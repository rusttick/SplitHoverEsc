#[doc = "Register `COMP2_CSR` reader"]
pub type R = crate::R<Comp2CsrSpec>;
#[doc = "Register `COMP2_CSR` writer"]
pub type W = crate::W<Comp2CsrSpec>;
#[doc = "Field `EN` reader - Comparator 2 enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Comparator 2 enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Comparator 2 mode"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Comparator 2 mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INM_SEL` reader - Comparator 2 inverting input selection"]
pub type InmSelR = crate::FieldReader;
#[doc = "Field `INM_SEL` writer - Comparator 2 inverting input selection"]
pub type InmSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INP_SEL` reader - Comparator 2 normal phase input selection"]
pub type InpSelR = crate::FieldReader;
#[doc = "Field `INP_SEL` writer - Comparator 2 normal phase input selection"]
pub type InpSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUT_SEL` reader - Comparator 2 output selection"]
pub type OutSelR = crate::FieldReader;
#[doc = "Field `OUT_SEL` writer - Comparator 2 output selection"]
pub type OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `POL` reader - Comparator 2 output polarity"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - Comparator 2 output polarity"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Comparator 2 hysteresis"]
pub type HystR = crate::FieldReader;
#[doc = "Field `HYST` writer - Comparator 2 hysteresis"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFLT` reader - Comparator output filter"]
pub type OfltR = crate::FieldReader;
#[doc = "Field `OFLT` writer - Comparator output filter"]
pub type OfltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUT` reader - Comparator 2 lock"]
pub type OutR = crate::BitReader;
#[doc = "Field `OUT` writer - Comparator 2 lock"]
pub type OutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Comparator lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Comparator lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn inm_sel(&self) -> InmSelR {
        InmSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 7:8 - Comparator 2 normal phase input selection"]
    #[inline(always)]
    pub fn inp_sel(&self) -> InpSelR {
        InpSelR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn out_sel(&self) -> OutSelR {
        OutSelR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator output filter"]
    #[inline(always)]
    pub fn oflt(&self) -> OfltR {
        OfltR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 lock"]
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
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Comp2CsrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Comp2CsrSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn inm_sel(&mut self) -> InmSelW<'_, Comp2CsrSpec> {
        InmSelW::new(self, 4)
    }
    #[doc = "Bits 7:8 - Comparator 2 normal phase input selection"]
    #[inline(always)]
    pub fn inp_sel(&mut self) -> InpSelW<'_, Comp2CsrSpec> {
        InpSelW::new(self, 7)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn out_sel(&mut self) -> OutSelW<'_, Comp2CsrSpec> {
        OutSelW::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, Comp2CsrSpec> {
        PolW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, Comp2CsrSpec> {
        HystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator output filter"]
    #[inline(always)]
    pub fn oflt(&mut self) -> OfltW<'_, Comp2CsrSpec> {
        OfltW::new(self, 18)
    }
    #[doc = "Bit 30 - Comparator 2 lock"]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<'_, Comp2CsrSpec> {
        OutW::new(self, 30)
    }
    #[doc = "Bit 31 - Comparator lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, Comp2CsrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "COMP2 Control State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp2CsrSpec;
impl crate::RegisterSpec for Comp2CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp2_csr::R`](R) reader structure"]
impl crate::Readable for Comp2CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp2_csr::W`](W) writer structure"]
impl crate::Writable for Comp2CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for Comp2CsrSpec {}
