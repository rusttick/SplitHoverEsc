#[doc = "Register `SSHR` reader"]
pub type R = crate::R<SshrSpec>;
#[doc = "Register `SSHR` writer"]
pub type W = crate::W<SshrSpec>;
#[doc = "Field `CNT` reader - This register sets the SCL clock high period count for standard speed"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - This register sets the SCL clock high period count for standard speed"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock high period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock high period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, SshrSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "SCL High Period Count for Std. Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sshr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sshr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SshrSpec;
impl crate::RegisterSpec for SshrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sshr::R`](R) reader structure"]
impl crate::Readable for SshrSpec {}
#[doc = "`write(|w| ..)` method takes [`sshr::W`](W) writer structure"]
impl crate::Writable for SshrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSHR to value 0x0190"]
impl crate::Resettable for SshrSpec {
    const RESET_VALUE: u32 = 0x0190;
}
