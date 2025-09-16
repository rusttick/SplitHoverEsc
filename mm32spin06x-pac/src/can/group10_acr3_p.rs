#[doc = "Register `GROUP10_ACR3_P` reader"]
pub type R = crate::R<Group10Acr3PSpec>;
#[doc = "Register `GROUP10_ACR3_P` writer"]
pub type W = crate::W<Group10Acr3PSpec>;
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
    pub fn ac(&mut self) -> AcW<'_, Group10Acr3PSpec> {
        AcW::new(self, 0)
    }
}
#[doc = "Peli Acceptance Code register3\n\nYou can [`read`](crate::Reg::read) this register and get [`group10_acr3_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group10_acr3_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group10Acr3PSpec;
impl crate::RegisterSpec for Group10Acr3PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group10_acr3_p::R`](R) reader structure"]
impl crate::Readable for Group10Acr3PSpec {}
#[doc = "`write(|w| ..)` method takes [`group10_acr3_p::W`](W) writer structure"]
impl crate::Writable for Group10Acr3PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GROUP10_ACR3_P to value 0"]
impl crate::Resettable for Group10Acr3PSpec {}
