#[doc = "Register `CNDTR1` reader"]
pub type R = crate::R<Cndtr1Spec>;
#[doc = "Register `CNDTR1` writer"]
pub type W = crate::W<Cndtr1Spec>;
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NdtR = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<'_, Cndtr1Spec> {
        NdtW::new(self, 0)
    }
}
#[doc = "CNDTR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cndtr1Spec;
impl crate::RegisterSpec for Cndtr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndtr1::R`](R) reader structure"]
impl crate::Readable for Cndtr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cndtr1::W`](W) writer structure"]
impl crate::Writable for Cndtr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNDTR1 to value 0"]
impl crate::Resettable for Cndtr1Spec {}
