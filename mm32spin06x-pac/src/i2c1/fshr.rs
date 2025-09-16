#[doc = "Register `FSHR` reader"]
pub type R = crate::R<FshrSpec>;
#[doc = "Register `FSHR` writer"]
pub type W = crate::W<FshrSpec>;
#[doc = "Field `CNT` reader - This register sets the SCL clock high_period count for standard speed"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - This register sets the SCL clock high_period count for standard speed"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock high_period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock high_period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, FshrSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "SCL High Period Count for Fast Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fshr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fshr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FshrSpec;
impl crate::RegisterSpec for FshrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fshr::R`](R) reader structure"]
impl crate::Readable for FshrSpec {}
#[doc = "`write(|w| ..)` method takes [`fshr::W`](W) writer structure"]
impl crate::Writable for FshrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSHR to value 0x3c"]
impl crate::Resettable for FshrSpec {
    const RESET_VALUE: u32 = 0x3c;
}
