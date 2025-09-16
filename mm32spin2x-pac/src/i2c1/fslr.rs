#[doc = "Register `FSLR` reader"]
pub type R = crate::R<FslrSpec>;
#[doc = "Register `FSLR` writer"]
pub type W = crate::W<FslrSpec>;
#[doc = "Field `CNT` reader - This register sets the SCL clock low period count for standard speed"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - This register sets the SCL clock low period count for standard speed"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, FslrSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "SCL Low Period Count for Fast Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fslr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fslr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FslrSpec;
impl crate::RegisterSpec for FslrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fslr::R`](R) reader structure"]
impl crate::Readable for FslrSpec {}
#[doc = "`write(|w| ..)` method takes [`fslr::W`](W) writer structure"]
impl crate::Writable for FslrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSLR to value 0x82"]
impl crate::Resettable for FslrSpec {
    const RESET_VALUE: u32 = 0x82;
}
