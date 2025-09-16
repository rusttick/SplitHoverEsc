#[doc = "Register `EP_INDEX3_4` reader"]
pub type R = crate::R<EpIndex3_4Spec>;
#[doc = "Register `EP_INDEX3_4` writer"]
pub type W = crate::W<EpIndex3_4Spec>;
#[doc = "Field `INDEX3` reader - Point 3 index"]
pub type Index3R = crate::FieldReader;
#[doc = "Field `INDEX3` writer - Point 3 index"]
pub type Index3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INDEX4` reader - Point 4 index"]
pub type Index4R = crate::FieldReader;
#[doc = "Field `INDEX4` writer - Point 4 index"]
pub type Index4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Point 3 index"]
    #[inline(always)]
    pub fn index3(&self) -> Index3R {
        Index3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Point 4 index"]
    #[inline(always)]
    pub fn index4(&self) -> Index4R {
        Index4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Point 3 index"]
    #[inline(always)]
    pub fn index3(&mut self) -> Index3W<'_, EpIndex3_4Spec> {
        Index3W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Point 4 index"]
    #[inline(always)]
    pub fn index4(&mut self) -> Index4W<'_, EpIndex3_4Spec> {
        Index4W::new(self, 4)
    }
}
#[doc = "End-point index register 3_4\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_index3_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_index3_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpIndex3_4Spec;
impl crate::RegisterSpec for EpIndex3_4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep_index3_4::R`](R) reader structure"]
impl crate::Readable for EpIndex3_4Spec {}
#[doc = "`write(|w| ..)` method takes [`ep_index3_4::W`](W) writer structure"]
impl crate::Writable for EpIndex3_4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP_INDEX3_4 to value 0x43"]
impl crate::Resettable for EpIndex3_4Spec {
    const RESET_VALUE: u16 = 0x43;
}
