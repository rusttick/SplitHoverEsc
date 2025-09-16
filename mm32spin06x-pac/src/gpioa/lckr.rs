#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LckrSpec>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LckrSpec>;
#[doc = "Field `LCK` reader - Port x Lock bit y"]
pub type LckR = crate::FieldReader<u16>;
#[doc = "Field `LCK` writer - Port x Lock bit y"]
pub type LckW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LCKK` reader - Lock key"]
pub type LckkR = crate::BitReader;
#[doc = "Field `LCKK` writer - Lock key"]
pub type LckkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Port x Lock bit y"]
    #[inline(always)]
    pub fn lck(&self) -> LckR {
        LckR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Lock key"]
    #[inline(always)]
    pub fn lckk(&self) -> LckkR {
        LckkR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port x Lock bit y"]
    #[inline(always)]
    pub fn lck(&mut self) -> LckW<'_, LckrSpec> {
        LckW::new(self, 0)
    }
    #[doc = "Bit 16 - Lock key"]
    #[inline(always)]
    pub fn lckk(&mut self) -> LckkW<'_, LckrSpec> {
        LckkW::new(self, 16)
    }
}
#[doc = "Port configuration lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LckrSpec;
impl crate::RegisterSpec for LckrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lckr::R`](R) reader structure"]
impl crate::Readable for LckrSpec {}
#[doc = "`write(|w| ..)` method takes [`lckr::W`](W) writer structure"]
impl crate::Writable for LckrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCKR to value 0"]
impl crate::Resettable for LckrSpec {}
