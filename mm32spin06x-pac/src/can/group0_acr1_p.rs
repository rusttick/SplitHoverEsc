#[doc = "Register `GROUP0_ACR1_P` reader"]
pub type R = crate::R<Group0Acr1PSpec>;
#[doc = "Register `GROUP0_ACR1_P` writer"]
pub type W = crate::W<Group0Acr1PSpec>;
#[doc = "Field `AC` reader - Acceptance code"]
pub type AcR = crate::FieldReader;
#[doc = "Field `AC` writer - Acceptance code"]
pub type AcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Acceptance code"]
    #[inline(always)]
    pub fn ac(&self) -> AcR {
        AcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance code"]
    #[inline(always)]
    pub fn ac(&mut self) -> AcW<'_, Group0Acr1PSpec> {
        AcW::new(self, 0)
    }
}
#[doc = "Peli Acceptance Code register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group0_acr1_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group0_acr1_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group0Acr1PSpec;
impl crate::RegisterSpec for Group0Acr1PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group0_acr1_p::R`](R) reader structure"]
impl crate::Readable for Group0Acr1PSpec {}
#[doc = "`write(|w| ..)` method takes [`group0_acr1_p::W`](W) writer structure"]
impl crate::Writable for Group0Acr1PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GROUP0_ACR1_P to value 0"]
impl crate::Resettable for Group0Acr1PSpec {}
