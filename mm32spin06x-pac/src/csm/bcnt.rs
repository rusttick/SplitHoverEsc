#[doc = "Register `BCNT` reader"]
pub type R = crate::R<BcntSpec>;
#[doc = "Register `BCNT` writer"]
pub type W = crate::W<BcntSpec>;
#[doc = "Field `CNT` reader - send or receive bit count"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - send or receive bit count"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - send or receive bit count"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - send or receive bit count"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, BcntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "bit count\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcntSpec;
impl crate::RegisterSpec for BcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcnt::R`](R) reader structure"]
impl crate::Readable for BcntSpec {}
#[doc = "`write(|w| ..)` method takes [`bcnt::W`](W) writer structure"]
impl crate::Writable for BcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCNT to value 0"]
impl crate::Resettable for BcntSpec {}
