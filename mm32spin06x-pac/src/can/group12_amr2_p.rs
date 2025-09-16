#[doc = "Register `GROUP12_AMR2_P` reader"]
pub type R = crate::R<Group12Amr2PSpec>;
#[doc = "Register `GROUP12_AMR2_P` writer"]
pub type W = crate::W<Group12Amr2PSpec>;
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
    pub fn am(&mut self) -> AmW<'_, Group12Amr2PSpec> {
        AmW::new(self, 0)
    }
}
#[doc = "Peli Acceptance Mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group12_amr2_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group12_amr2_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group12Amr2PSpec;
impl crate::RegisterSpec for Group12Amr2PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group12_amr2_p::R`](R) reader structure"]
impl crate::Readable for Group12Amr2PSpec {}
#[doc = "`write(|w| ..)` method takes [`group12_amr2_p::W`](W) writer structure"]
impl crate::Writable for Group12Amr2PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GROUP12_AMR2_P to value 0"]
impl crate::Resettable for Group12Amr2PSpec {}
