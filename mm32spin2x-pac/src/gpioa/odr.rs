#[doc = "Register `ODR` reader"]
pub type R = crate::R<OdrSpec>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<OdrSpec>;
#[doc = "Field `ODR` reader - Port output data"]
pub type OdrR = crate::FieldReader<u16>;
#[doc = "Field `ODR` writer - Port output data"]
pub type OdrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port output data"]
    #[inline(always)]
    pub fn odr(&self) -> OdrR {
        OdrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port output data"]
    #[inline(always)]
    pub fn odr(&mut self) -> OdrW<'_, OdrSpec> {
        OdrW::new(self, 0)
    }
}
#[doc = "output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdrSpec;
impl crate::RegisterSpec for OdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odr::R`](R) reader structure"]
impl crate::Readable for OdrSpec {}
#[doc = "`write(|w| ..)` method takes [`odr::W`](W) writer structure"]
impl crate::Writable for OdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ODR to value 0x0204"]
impl crate::Resettable for OdrSpec {
    const RESET_VALUE: u32 = 0x0204;
}
