#[doc = "Register `FRA` reader"]
pub type R = crate::R<FraSpec>;
#[doc = "Register `FRA` writer"]
pub type W = crate::W<FraSpec>;
#[doc = "Field `DIV_Fraction` reader - Fractional part of UARTDIV"]
pub type DivFractionR = crate::FieldReader;
#[doc = "Field `DIV_Fraction` writer - Fractional part of UARTDIV"]
pub type DivFractionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Fractional part of UARTDIV"]
    #[inline(always)]
    pub fn div_fraction(&self) -> DivFractionR {
        DivFractionR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Fractional part of UARTDIV"]
    #[inline(always)]
    pub fn div_fraction(&mut self) -> DivFractionW<'_, FraSpec> {
        DivFractionW::new(self, 0)
    }
}
#[doc = "Fractional baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`fra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FraSpec;
impl crate::RegisterSpec for FraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fra::R`](R) reader structure"]
impl crate::Readable for FraSpec {}
#[doc = "`write(|w| ..)` method takes [`fra::W`](W) writer structure"]
impl crate::Writable for FraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRA to value 0"]
impl crate::Resettable for FraSpec {}
