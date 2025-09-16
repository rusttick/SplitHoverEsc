#[doc = "Register `GROUP1_AMR3_P` reader"]
pub type R = crate::R<Group1Amr3PSpec>;
#[doc = "Register `GROUP1_AMR3_P` writer"]
pub type W = crate::W<Group1Amr3PSpec>;
#[doc = "Field `AM` reader - Acceptance mask"]
pub type AmR = crate::FieldReader;
#[doc = "Field `AM` writer - Acceptance mask"]
pub type AmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Acceptance mask"]
    #[inline(always)]
    pub fn am(&self) -> AmR {
        AmR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance mask"]
    #[inline(always)]
    pub fn am(&mut self) -> AmW<'_, Group1Amr3PSpec> {
        AmW::new(self, 0)
    }
}
#[doc = "Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group1_amr3_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group1_amr3_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group1Amr3PSpec;
impl crate::RegisterSpec for Group1Amr3PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group1_amr3_p::R`](R) reader structure"]
impl crate::Readable for Group1Amr3PSpec {}
#[doc = "`write(|w| ..)` method takes [`group1_amr3_p::W`](W) writer structure"]
impl crate::Writable for Group1Amr3PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GROUP1_AMR3_P to value 0"]
impl crate::Resettable for Group1Amr3PSpec {}
