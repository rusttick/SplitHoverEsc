#[doc = "Register `EWLR_P` reader"]
pub type R = crate::R<EwlrPSpec>;
#[doc = "Register `EWLR_P` writer"]
pub type W = crate::W<EwlrPSpec>;
#[doc = "Field `EWL` reader - Programmable error warning limit"]
pub type EwlR = crate::FieldReader;
#[doc = "Field `EWL` writer - Programmable error warning limit"]
pub type EwlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Programmable error warning limit"]
    #[inline(always)]
    pub fn ewl(&self) -> EwlR {
        EwlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Programmable error warning limit"]
    #[inline(always)]
    pub fn ewl(&mut self) -> EwlW<'_, EwlrPSpec> {
        EwlW::new(self, 0)
    }
}
#[doc = "Peli Error Warning Limit register\n\nYou can [`read`](crate::Reg::read) this register and get [`ewlr_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewlr_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwlrPSpec;
impl crate::RegisterSpec for EwlrPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ewlr_p::R`](R) reader structure"]
impl crate::Readable for EwlrPSpec {}
#[doc = "`write(|w| ..)` method takes [`ewlr_p::W`](W) writer structure"]
impl crate::Writable for EwlrPSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EWLR_P to value 0x60"]
impl crate::Resettable for EwlrPSpec {
    const RESET_VALUE: u32 = 0x60;
}
