#[doc = "Register `DVSR` reader"]
pub type R = crate::R<DvsrSpec>;
#[doc = "Register `DVSR` writer"]
pub type W = crate::W<DvsrSpec>;
#[doc = "Field `DIVISOR` reader - Divisor data"]
pub type DivisorR = crate::FieldReader<u32>;
#[doc = "Field `DIVISOR` writer - Divisor data"]
pub type DivisorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Divisor data"]
    #[inline(always)]
    pub fn divisor(&self) -> DivisorR {
        DivisorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Divisor data"]
    #[inline(always)]
    pub fn divisor(&mut self) -> DivisorW<'_, DvsrSpec> {
        DivisorW::new(self, 0)
    }
}
#[doc = "DVSR\n\nYou can [`read`](crate::Reg::read) this register and get [`dvsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvsrSpec;
impl crate::RegisterSpec for DvsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvsr::R`](R) reader structure"]
impl crate::Readable for DvsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dvsr::W`](W) writer structure"]
impl crate::Writable for DvsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVSR to value 0x01"]
impl crate::Resettable for DvsrSpec {
    const RESET_VALUE: u32 = 0x01;
}
