#[doc = "Register `OPAMP_CSR` reader"]
pub type R = crate::R<OpampCsrSpec>;
#[doc = "Register `OPAMP_CSR` writer"]
pub type W = crate::W<OpampCsrSpec>;
#[doc = "Field `OPAMP1_EN` reader - operational amplifier 1 enable"]
pub type Opamp1EnR = crate::BitReader;
#[doc = "Field `OPAMP1_EN` writer - operational amplifier 1 enable"]
pub type Opamp1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMP2_EN` reader - operational amplifier 2 enable"]
pub type Opamp2EnR = crate::BitReader;
#[doc = "Field `OPAMP2_EN` writer - operational amplifier 2 enable"]
pub type Opamp2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMP3_EN` reader - operational amplifier 3 enable"]
pub type Opamp3EnR = crate::BitReader;
#[doc = "Field `OPAMP3_EN` writer - operational amplifier 3 enable"]
pub type Opamp3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMP4_EN` reader - operational amplifier 4 enable"]
pub type Opamp4EnR = crate::BitReader;
#[doc = "Field `OPAMP4_EN` writer - operational amplifier 4 enable"]
pub type Opamp4EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - operational amplifier 1 enable"]
    #[inline(always)]
    pub fn opamp1_en(&self) -> Opamp1EnR {
        Opamp1EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - operational amplifier 2 enable"]
    #[inline(always)]
    pub fn opamp2_en(&self) -> Opamp2EnR {
        Opamp2EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - operational amplifier 3 enable"]
    #[inline(always)]
    pub fn opamp3_en(&self) -> Opamp3EnR {
        Opamp3EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - operational amplifier 4 enable"]
    #[inline(always)]
    pub fn opamp4_en(&self) -> Opamp4EnR {
        Opamp4EnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - operational amplifier 1 enable"]
    #[inline(always)]
    pub fn opamp1_en(&mut self) -> Opamp1EnW<'_, OpampCsrSpec> {
        Opamp1EnW::new(self, 0)
    }
    #[doc = "Bit 8 - operational amplifier 2 enable"]
    #[inline(always)]
    pub fn opamp2_en(&mut self) -> Opamp2EnW<'_, OpampCsrSpec> {
        Opamp2EnW::new(self, 8)
    }
    #[doc = "Bit 16 - operational amplifier 3 enable"]
    #[inline(always)]
    pub fn opamp3_en(&mut self) -> Opamp3EnW<'_, OpampCsrSpec> {
        Opamp3EnW::new(self, 16)
    }
    #[doc = "Bit 24 - operational amplifier 4 enable"]
    #[inline(always)]
    pub fn opamp4_en(&mut self) -> Opamp4EnW<'_, OpampCsrSpec> {
        Opamp4EnW::new(self, 24)
    }
}
#[doc = "OPAMP_CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpampCsrSpec;
impl crate::RegisterSpec for OpampCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp_csr::R`](R) reader structure"]
impl crate::Readable for OpampCsrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp_csr::W`](W) writer structure"]
impl crate::Writable for OpampCsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPAMP_CSR to value 0"]
impl crate::Resettable for OpampCsrSpec {}
