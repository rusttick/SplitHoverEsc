#[doc = "Register `ECR` reader"]
pub type R = crate::R<EcrSpec>;
#[doc = "Register `ECR` writer"]
pub type W = crate::W<EcrSpec>;
#[doc = "Field `EXTLEN` reader - Control SPI data length"]
pub type ExtlenR = crate::FieldReader;
#[doc = "Field `EXTLEN` writer - Control SPI data length"]
pub type ExtlenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Control SPI data length"]
    #[inline(always)]
    pub fn extlen(&self) -> ExtlenR {
        ExtlenR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Control SPI data length"]
    #[inline(always)]
    pub fn extlen(&mut self) -> ExtlenW<'_, EcrSpec> {
        ExtlenW::new(self, 0)
    }
}
#[doc = "ECR\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcrSpec;
impl crate::RegisterSpec for EcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for EcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for EcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECR to value 0x08"]
impl crate::Resettable for EcrSpec {
    const RESET_VALUE: u32 = 0x08;
}
