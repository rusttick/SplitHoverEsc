#[doc = "Register `CCR3FALL` reader"]
pub type R = crate::R<Ccr3fallSpec>;
#[doc = "Register `CCR3FALL` writer"]
pub type W = crate::W<Ccr3fallSpec>;
#[doc = "Field `CCR3FALL` reader - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
pub type Ccr3fallR = crate::FieldReader<u16>;
#[doc = "Field `CCR3FALL` writer - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
pub type Ccr3fallW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr3fall(&self) -> Ccr3fallR {
        Ccr3fallR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare value for ch3 when counting down in PWM center-aligned mode"]
    #[inline(always)]
    pub fn ccr3fall(&mut self) -> Ccr3fallW<'_, Ccr3fallSpec> {
        Ccr3fallW::new(self, 0)
    }
}
#[doc = "pwm shift count CCR3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3fall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3fall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr3fallSpec;
impl crate::RegisterSpec for Ccr3fallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr3fall::R`](R) reader structure"]
impl crate::Readable for Ccr3fallSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr3fall::W`](W) writer structure"]
impl crate::Writable for Ccr3fallSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR3FALL to value 0"]
impl crate::Resettable for Ccr3fallSpec {}
