#[doc = "Register `CCR5FALL` reader"]
pub type R = crate::R<Ccr5fallSpec>;
#[doc = "Register `CCR5FALL` writer"]
pub type W = crate::W<Ccr5fallSpec>;
#[doc = "Field `CCR5FALL` reader - Capture/compare value for ch5 when counting down in PWM center-aligned mode"]
pub type Ccr5fallR = crate::FieldReader<u16>;
#[doc = "Field `CCR5FALL` writer - Capture/compare value for ch5 when counting down in PWM center-aligned mode"]
pub type Ccr5fallW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare value for ch5 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr5fall(&self) -> Ccr5fallR {
        Ccr5fallR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare value for ch5 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr5fall(&mut self) -> Ccr5fallW<'_, Ccr5fallSpec> {
        Ccr5fallW::new(self, 0)
    }
}
#[doc = "pwm shift count CCR5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr5fall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5fall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr5fallSpec;
impl crate::RegisterSpec for Ccr5fallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr5fall::R`](R) reader structure"]
impl crate::Readable for Ccr5fallSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr5fall::W`](W) writer structure"]
impl crate::Writable for Ccr5fallSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR5FALL to value 0"]
impl crate::Resettable for Ccr5fallSpec {}
