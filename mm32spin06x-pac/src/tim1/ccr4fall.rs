#[doc = "Register `CCR4FALL` reader"]
pub type R = crate::R<Ccr4fallSpec>;
#[doc = "Register `CCR4FALL` writer"]
pub type W = crate::W<Ccr4fallSpec>;
#[doc = "Field `CCR4FALL` reader - Capture/compare value for ch4 when counting down in PWM center-aligned mode"]
pub type Ccr4fallR = crate::FieldReader<u16>;
#[doc = "Field `CCR4FALL` writer - Capture/compare value for ch4 when counting down in PWM center-aligned mode"]
pub type Ccr4fallW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare value for ch4 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr4fall(&self) -> Ccr4fallR {
        Ccr4fallR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare value for ch4 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr4fall(&mut self) -> Ccr4fallW<'_, Ccr4fallSpec> {
        Ccr4fallW::new(self, 0)
    }
}
#[doc = "pwm shift count CCR4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4fall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4fall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr4fallSpec;
impl crate::RegisterSpec for Ccr4fallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr4fall::R`](R) reader structure"]
impl crate::Readable for Ccr4fallSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr4fall::W`](W) writer structure"]
impl crate::Writable for Ccr4fallSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR4FALL to value 0"]
impl crate::Resettable for Ccr4fallSpec {}
