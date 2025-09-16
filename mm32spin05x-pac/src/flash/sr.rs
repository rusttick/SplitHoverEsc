#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `BSY` reader - Busy"]
pub type BsyR = crate::BitReader;
#[doc = "Field `BSY` writer - Busy"]
pub type BsyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGERR` reader - Programming error"]
pub type PgerrR = crate::BitReader;
#[doc = "Field `PGERR` writer - Programming error"]
pub type PgerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPRTERR` reader - Write protection error"]
pub type WrprterrR = crate::BitReader;
#[doc = "Field `WRPRTERR` writer - Write protection error"]
pub type WrprterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOP` reader - End of operation"]
pub type EopR = crate::BitReader;
#[doc = "Field `EOP` writer - End of operation"]
pub type EopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Programming error"]
    #[inline(always)]
    pub fn pgerr(&self) -> PgerrR {
        PgerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrprterr(&self) -> WrprterrR {
        WrprterrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EopR {
        EopR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn bsy(&mut self) -> BsyW<'_, SrSpec> {
        BsyW::new(self, 0)
    }
    #[doc = "Bit 2 - Programming error"]
    #[inline(always)]
    pub fn pgerr(&mut self) -> PgerrW<'_, SrSpec> {
        PgerrW::new(self, 2)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrprterr(&mut self) -> WrprterrW<'_, SrSpec> {
        WrprterrW::new(self, 4)
    }
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    pub fn eop(&mut self) -> EopW<'_, SrSpec> {
        EopW::new(self, 5)
    }
}
#[doc = "Flash status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
