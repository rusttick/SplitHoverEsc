#[doc = "Register `APMDLR` reader"]
pub type R = crate::R<ApmdlrSpec>;
#[doc = "Register `APMDLR` writer"]
pub type W = crate::W<ApmdlrSpec>;
#[doc = "Field `APMDLR` reader - *D0"]
pub type ApmdlrR = crate::FieldReader<u32>;
#[doc = "Field `APMDLR` writer - *D0"]
pub type ApmdlrW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - *D0"]
    #[inline(always)]
    pub fn apmdlr(&self) -> ApmdlrR {
        ApmdlrR::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - *D0"]
    #[inline(always)]
    pub fn apmdlr(&mut self) -> ApmdlrW<'_, ApmdlrSpec> {
        ApmdlrW::new(self, 0)
    }
}
#[doc = "Auto phase mask dalay register\n\nYou can [`read`](crate::Reg::read) this register and get [`apmdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apmdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApmdlrSpec;
impl crate::RegisterSpec for ApmdlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apmdlr::R`](R) reader structure"]
impl crate::Readable for ApmdlrSpec {}
#[doc = "`write(|w| ..)` method takes [`apmdlr::W`](W) writer structure"]
impl crate::Writable for ApmdlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APMDLR to value 0x01"]
impl crate::Resettable for ApmdlrSpec {
    const RESET_VALUE: u32 = 0x01;
}
