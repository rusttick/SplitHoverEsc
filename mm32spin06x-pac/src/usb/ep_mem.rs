#[doc = "Register `EP_MEM` reader"]
pub type R = crate::R<EpMemSpec>;
#[doc = "Register `EP_MEM` writer"]
pub type W = crate::W<EpMemSpec>;
#[doc = "Field `EP_MEM` reader - EP memory"]
pub type EpMemR = crate::FieldReader;
#[doc = "Field `EP_MEM` writer - EP memory"]
pub type EpMemW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - EP memory"]
    #[inline(always)]
    pub fn ep_mem(&self) -> EpMemR {
        EpMemR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - EP memory"]
    #[inline(always)]
    pub fn ep_mem(&mut self) -> EpMemW<'_, EpMemSpec> {
        EpMemW::new(self, 0)
    }
}
#[doc = "EP MEM register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpMemSpec;
impl crate::RegisterSpec for EpMemSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep_mem::R`](R) reader structure"]
impl crate::Readable for EpMemSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_mem::W`](W) writer structure"]
impl crate::Writable for EpMemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP_MEM to value 0"]
impl crate::Resettable for EpMemSpec {}
