#[doc = "Register `GROUP16_ACR_B` reader"]
pub type R = crate::R<Group16AcrBSpec>;
#[doc = "Register `GROUP16_ACR_B` writer"]
pub type W = crate::W<Group16AcrBSpec>;
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
    pub fn ac(&mut self) -> AcW<'_, Group16AcrBSpec> {
        AcW::new(self, 0)
    }
}
#[doc = "Basic Acceptance Code register\n\nYou can [`read`](crate::Reg::read) this register and get [`group16_acr_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group16_acr_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group16AcrBSpec;
impl crate::RegisterSpec for Group16AcrBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group16_acr_b::R`](R) reader structure"]
impl crate::Readable for Group16AcrBSpec {}
#[doc = "`write(|w| ..)` method takes [`group16_acr_b::W`](W) writer structure"]
impl crate::Writable for Group16AcrBSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GROUP16_ACR_B to value 0"]
impl crate::Resettable for Group16AcrBSpec {}
