#[doc = "Register `CCR2FALL` reader"]
pub type R = crate::R<Ccr2fallSpec>;
#[doc = "Register `CCR2FALL` writer"]
pub type W = crate::W<Ccr2fallSpec>;
#[doc = "Field `CCR2FALL` reader - Capture/compare value for ch2 when counting down in PWM center-aligned mode"]
pub type Ccr2fallR = crate::FieldReader<u16>;
#[doc = "Field `CCR2FALL` writer - Capture/compare value for ch2 when counting down in PWM center-aligned mode"]
pub type Ccr2fallW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare value for ch2 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr2fall(&self) -> Ccr2fallR {
        Ccr2fallR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare value for ch2 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr2fall(&mut self) -> Ccr2fallW<'_, Ccr2fallSpec> {
        Ccr2fallW::new(self, 0)
    }
}
#[doc = "pwm shift count CCR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2fall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2fall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr2fallSpec;
impl crate::RegisterSpec for Ccr2fallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr2fall::R`](R) reader structure"]
impl crate::Readable for Ccr2fallSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr2fall::W`](W) writer structure"]
impl crate::Writable for Ccr2fallSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR2FALL to value 0"]
impl crate::Resettable for Ccr2fallSpec {}
