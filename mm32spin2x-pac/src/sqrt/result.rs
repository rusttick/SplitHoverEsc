#[doc = "Register `RESULT` reader"]
pub type R = crate::R<ResultSpec>;
#[doc = "Register `RESULT` writer"]
pub type W = crate::W<ResultSpec>;
#[doc = "Field `RESULT` reader - Result data"]
pub type ResultR = crate::FieldReader<u16>;
#[doc = "Field `RESULT` writer - Result data"]
pub type ResultW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Result data"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Result data"]
    #[inline(always)]
    pub fn result(&mut self) -> ResultW<'_, ResultSpec> {
        ResultW::new(self, 0)
    }
}
#[doc = "RESULT\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultSpec;
impl crate::RegisterSpec for ResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for ResultSpec {}
#[doc = "`write(|w| ..)` method takes [`result::W`](W) writer structure"]
impl crate::Writable for ResultSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for ResultSpec {}
