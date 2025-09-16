#[doc = "Register `POWER` reader"]
pub type R = crate::R<PowerSpec>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<PowerSpec>;
#[doc = "Field `SUSPEN` reader - BUS suspend enable bit"]
pub type SuspenR = crate::BitReader;
#[doc = "Field `SUSPEN` writer - BUS suspend enable bit"]
pub type SuspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - suspend"]
pub type SuspR = crate::BitReader;
#[doc = "Field `SUSP` writer - suspend"]
pub type SuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUP` reader - Enable controller wake up from suspend state"]
pub type WkupR = crate::BitReader;
#[doc = "Field `WKUP` writer - Enable controller wake up from suspend state"]
pub type WkupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BUS suspend enable bit"]
    #[inline(always)]
    pub fn suspen(&self) -> SuspenR {
        SuspenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable controller wake up from suspend state"]
    #[inline(always)]
    pub fn wkup(&self) -> WkupR {
        WkupR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BUS suspend enable bit"]
    #[inline(always)]
    pub fn suspen(&mut self) -> SuspenW<'_, PowerSpec> {
        SuspenW::new(self, 0)
    }
    #[doc = "Bit 1 - suspend"]
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<'_, PowerSpec> {
        SuspW::new(self, 1)
    }
    #[doc = "Bit 3 - Enable controller wake up from suspend state"]
    #[inline(always)]
    pub fn wkup(&mut self) -> WkupW<'_, PowerSpec> {
        WkupW::new(self, 3)
    }
}
#[doc = "Power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerSpec;
impl crate::RegisterSpec for PowerSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for PowerSpec {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for PowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for PowerSpec {}
