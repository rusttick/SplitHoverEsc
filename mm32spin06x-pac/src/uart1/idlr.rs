#[doc = "Register `IDLR` reader"]
pub type R = crate::R<IdlrSpec>;
#[doc = "Register `IDLR` writer"]
pub type W = crate::W<IdlrSpec>;
#[doc = "Field `IDLR` reader - Idle data length register"]
pub type IdlrR = crate::FieldReader<u16>;
#[doc = "Field `IDLR` writer - Idle data length register"]
pub type IdlrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Idle data length register"]
    #[inline(always)]
    pub fn idlr(&self) -> IdlrR {
        IdlrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Idle data length register"]
    #[inline(always)]
    pub fn idlr(&mut self) -> IdlrW<'_, IdlrSpec> {
        IdlrW::new(self, 0)
    }
}
#[doc = "Data length register\n\nYou can [`read`](crate::Reg::read) this register and get [`idlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdlrSpec;
impl crate::RegisterSpec for IdlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idlr::R`](R) reader structure"]
impl crate::Readable for IdlrSpec {}
#[doc = "`write(|w| ..)` method takes [`idlr::W`](W) writer structure"]
impl crate::Writable for IdlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDLR to value 0x0c"]
impl crate::Resettable for IdlrSpec {
    const RESET_VALUE: u32 = 0x0c;
}
