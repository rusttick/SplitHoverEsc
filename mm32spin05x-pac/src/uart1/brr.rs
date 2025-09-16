#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `DIV_MANTISSA` reader - Mantissa part of UARTDIV"]
pub type DivMantissaR = crate::FieldReader<u16>;
#[doc = "Field `DIV_MANTISSA` writer - Mantissa part of UARTDIV"]
pub type DivMantissaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Mantissa part of UARTDIV"]
    #[inline(always)]
    pub fn div_mantissa(&self) -> DivMantissaR {
        DivMantissaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mantissa part of UARTDIV"]
    #[inline(always)]
    pub fn div_mantissa(&mut self) -> DivMantissaW<'_, BrrSpec> {
        DivMantissaW::new(self, 0)
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BrrSpec {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0x01"]
impl crate::Resettable for BrrSpec {
    const RESET_VALUE: u32 = 0x01;
}
