#[doc = "Register `KR` writer"]
pub type W = crate::W<KrSpec>;
#[doc = "Field `KEY` writer - Key value"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Key value"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, KrSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KrSpec;
impl crate::RegisterSpec for KrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`kr::W`](W) writer structure"]
impl crate::Writable for KrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KR to value 0"]
impl crate::Resettable for KrSpec {}
