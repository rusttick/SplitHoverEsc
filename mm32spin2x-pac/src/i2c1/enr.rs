#[doc = "Register `ENR` reader"]
pub type R = crate::R<EnrSpec>;
#[doc = "Register `ENR` writer"]
pub type W = crate::W<EnrSpec>;
#[doc = "Field `ENABLE` reader - I2C mode enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - I2C mode enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - I2C transfer abort"]
pub type AbortR = crate::BitReader;
#[doc = "Field `ABORT` writer - I2C transfer abort"]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C mode enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C transfer abort"]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C mode enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, EnrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C transfer abort"]
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<'_, EnrSpec> {
        AbortW::new(self, 1)
    }
}
#[doc = "Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnrSpec;
impl crate::RegisterSpec for EnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enr::R`](R) reader structure"]
impl crate::Readable for EnrSpec {}
#[doc = "`write(|w| ..)` method takes [`enr::W`](W) writer structure"]
impl crate::Writable for EnrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENR to value 0"]
impl crate::Resettable for EnrSpec {}
