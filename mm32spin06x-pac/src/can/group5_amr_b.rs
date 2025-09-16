#[doc = "Register `GROUP5_AMR_B` reader"]
pub type R = crate::R<Group5AmrBSpec>;
#[doc = "Register `GROUP5_AMR_B` writer"]
pub type W = crate::W<Group5AmrBSpec>;
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
    pub fn am(&mut self) -> AmW<'_, Group5AmrBSpec> {
        AmW::new(self, 0)
    }
}
#[doc = "Basic Acceptance Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`group5_amr_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group5_amr_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group5AmrBSpec;
impl crate::RegisterSpec for Group5AmrBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group5_amr_b::R`](R) reader structure"]
impl crate::Readable for Group5AmrBSpec {}
#[doc = "`write(|w| ..)` method takes [`group5_amr_b::W`](W) writer structure"]
impl crate::Writable for Group5AmrBSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GROUP5_AMR_B to value 0"]
impl crate::Resettable for Group5AmrBSpec {}
