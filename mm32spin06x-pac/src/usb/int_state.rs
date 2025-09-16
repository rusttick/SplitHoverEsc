#[doc = "Register `INT_STATE` reader"]
pub type R = crate::R<IntStateSpec>;
#[doc = "Register `INT_STATE` writer"]
pub type W = crate::W<IntStateSpec>;
#[doc = "Field `RSTF` reader - BUS reset received"]
pub type RstfR = crate::BitReader;
#[doc = "Field `RSTF` writer - BUS reset received"]
pub type RstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPENDF` reader - BUS suspend received"]
pub type SuspendfR = crate::BitReader;
#[doc = "Field `SUSPENDF` writer - BUS suspend received"]
pub type SuspendfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUMF` reader - BUS resume received"]
pub type ResumfR = crate::BitReader;
#[doc = "Field `RESUMF` writer - BUS resume received"]
pub type ResumfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFF` reader - BUS received"]
pub type SoffR = crate::BitReader;
#[doc = "Field `SOFF` writer - BUS received"]
pub type SoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPINTF` reader - EP interrupt received"]
pub type EpintfR = crate::BitReader;
#[doc = "Field `EPINTF` writer - EP interrupt received"]
pub type EpintfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BUS reset received"]
    #[inline(always)]
    pub fn rstf(&self) -> RstfR {
        RstfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BUS suspend received"]
    #[inline(always)]
    pub fn suspendf(&self) -> SuspendfR {
        SuspendfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BUS resume received"]
    #[inline(always)]
    pub fn resumf(&self) -> ResumfR {
        ResumfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUS received"]
    #[inline(always)]
    pub fn soff(&self) -> SoffR {
        SoffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP interrupt received"]
    #[inline(always)]
    pub fn epintf(&self) -> EpintfR {
        EpintfR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BUS reset received"]
    #[inline(always)]
    pub fn rstf(&mut self) -> RstfW<'_, IntStateSpec> {
        RstfW::new(self, 0)
    }
    #[doc = "Bit 1 - BUS suspend received"]
    #[inline(always)]
    pub fn suspendf(&mut self) -> SuspendfW<'_, IntStateSpec> {
        SuspendfW::new(self, 1)
    }
    #[doc = "Bit 2 - BUS resume received"]
    #[inline(always)]
    pub fn resumf(&mut self) -> ResumfW<'_, IntStateSpec> {
        ResumfW::new(self, 2)
    }
    #[doc = "Bit 3 - BUS received"]
    #[inline(always)]
    pub fn soff(&mut self) -> SoffW<'_, IntStateSpec> {
        SoffW::new(self, 3)
    }
    #[doc = "Bit 4 - EP interrupt received"]
    #[inline(always)]
    pub fn epintf(&mut self) -> EpintfW<'_, IntStateSpec> {
        EpintfW::new(self, 4)
    }
}
#[doc = "interrupt state register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStateSpec;
impl crate::RegisterSpec for IntStateSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`int_state::R`](R) reader structure"]
impl crate::Readable for IntStateSpec {}
#[doc = "`write(|w| ..)` method takes [`int_state::W`](W) writer structure"]
impl crate::Writable for IntStateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_STATE to value 0"]
impl crate::Resettable for IntStateSpec {}
