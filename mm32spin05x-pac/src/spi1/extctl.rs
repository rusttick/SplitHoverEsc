#[doc = "Register `EXTCTL` reader"]
pub type R = crate::R<ExtctlSpec>;
#[doc = "Register `EXTCTL` writer"]
pub type W = crate::W<ExtctlSpec>;
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
    pub fn extlen(&mut self) -> ExtlenW<'_, ExtctlSpec> {
        ExtlenW::new(self, 0)
    }
}
#[doc = "EXTCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`extctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtctlSpec;
impl crate::RegisterSpec for ExtctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extctl::R`](R) reader structure"]
impl crate::Readable for ExtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`extctl::W`](W) writer structure"]
impl crate::Writable for ExtctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTCTL to value 0x08"]
impl crate::Resettable for ExtctlSpec {
    const RESET_VALUE: u32 = 0x08;
}
