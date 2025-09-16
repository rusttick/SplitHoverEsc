#[doc = "Register `CCR1FALL` reader"]
pub type R = crate::R<Ccr1fallSpec>;
#[doc = "Register `CCR1FALL` writer"]
pub type W = crate::W<Ccr1fallSpec>;
#[doc = "Field `CCR1FALL` reader - Capture/compare value for ch1 when counting down in PWM center-aligned mode"]
pub type Ccr1fallR = crate::FieldReader<u16>;
#[doc = "Field `CCR1FALL` writer - Capture/compare value for ch1 when counting down in PWM center-aligned mode"]
pub type Ccr1fallW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare value for ch1 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr1fall(&self) -> Ccr1fallR {
        Ccr1fallR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare value for ch1 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr1fall(&mut self) -> Ccr1fallW<'_, Ccr1fallSpec> {
        Ccr1fallW::new(self, 0)
    }
}
#[doc = "pwm shift count CCR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1fall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1fall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr1fallSpec;
impl crate::RegisterSpec for Ccr1fallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr1fall::R`](R) reader structure"]
impl crate::Readable for Ccr1fallSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr1fall::W`](W) writer structure"]
impl crate::Writable for Ccr1fallSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR1FALL to value 0"]
impl crate::Resettable for Ccr1fallSpec {}
