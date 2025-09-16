#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `USIGN` reader - unsigned enable"]
pub type UsignR = crate::BitReader;
#[doc = "Field `USIGN` writer - unsigned enable"]
pub type UsignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFE` reader - Overflow interrupt enable"]
pub type OvfeR = crate::BitReader;
#[doc = "Field `OVFE` writer - Overflow interrupt enable"]
pub type OvfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - unsigned enable"]
    #[inline(always)]
    pub fn usign(&self) -> UsignR {
        UsignR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfe(&self) -> OvfeR {
        OvfeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - unsigned enable"]
    #[inline(always)]
    pub fn usign(&mut self) -> UsignW<'_, CrSpec> {
        UsignW::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfe(&mut self) -> OvfeW<'_, CrSpec> {
        OvfeW::new(self, 1)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x01"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x01;
}
