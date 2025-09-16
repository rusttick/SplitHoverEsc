#[doc = "Register `CCR5` reader"]
pub type R = crate::R<Ccr5Spec>;
#[doc = "Register `CCR5` writer"]
pub type W = crate::W<Ccr5Spec>;
#[doc = "Field `CCR5` reader - Capture/Compare 5 value"]
pub type Ccr5R = crate::FieldReader<u16>;
#[doc = "Field `CCR5` writer - Capture/Compare 5 value"]
pub type Ccr5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 5 value"]
    #[inline(always)]
    pub fn ccr5(&self) -> Ccr5R {
        Ccr5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 5 value"]
    #[inline(always)]
    pub fn ccr5(&mut self) -> Ccr5W<'_, Ccr5Spec> {
        Ccr5W::new(self, 0)
    }
}
#[doc = "capture/compare register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr5Spec;
impl crate::RegisterSpec for Ccr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr5::R`](R) reader structure"]
impl crate::Readable for Ccr5Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr5::W`](W) writer structure"]
impl crate::Writable for Ccr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR5 to value 0"]
impl crate::Resettable for Ccr5Spec {}
