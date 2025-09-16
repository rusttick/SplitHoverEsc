#[doc = "Register `EP_INDEX1_2` reader"]
pub type R = crate::R<EpIndex1_2Spec>;
#[doc = "Register `EP_INDEX1_2` writer"]
pub type W = crate::W<EpIndex1_2Spec>;
#[doc = "Field `INDEX1` reader - Point 1 index"]
pub type Index1R = crate::FieldReader;
#[doc = "Field `INDEX1` writer - Point 1 index"]
pub type Index1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INDEX2` reader - Point 2 index"]
pub type Index2R = crate::FieldReader;
#[doc = "Field `INDEX2` writer - Point 2 index"]
pub type Index2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Point 1 index"]
    #[inline(always)]
    pub fn index1(&self) -> Index1R {
        Index1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Point 2 index"]
    #[inline(always)]
    pub fn index2(&self) -> Index2R {
        Index2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Point 1 index"]
    #[inline(always)]
    pub fn index1(&mut self) -> Index1W<'_, EpIndex1_2Spec> {
        Index1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Point 2 index"]
    #[inline(always)]
    pub fn index2(&mut self) -> Index2W<'_, EpIndex1_2Spec> {
        Index2W::new(self, 4)
    }
}
#[doc = "End-point index register 1_2\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_index1_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_index1_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpIndex1_2Spec;
impl crate::RegisterSpec for EpIndex1_2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep_index1_2::R`](R) reader structure"]
impl crate::Readable for EpIndex1_2Spec {}
#[doc = "`write(|w| ..)` method takes [`ep_index1_2::W`](W) writer structure"]
impl crate::Writable for EpIndex1_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP_INDEX1_2 to value 0x21"]
impl crate::Resettable for EpIndex1_2Spec {
    const RESET_VALUE: u16 = 0x21;
}
