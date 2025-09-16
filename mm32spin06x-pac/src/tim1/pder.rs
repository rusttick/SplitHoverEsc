#[doc = "Register `PDER` reader"]
pub type R = crate::R<PderSpec>;
#[doc = "Register `PDER` writer"]
pub type W = crate::W<PderSpec>;
#[doc = "Field `CCDREPE` reader - DMA request flow enable"]
pub type CcdrepeR = crate::BitReader;
#[doc = "Field `CCDREPE` writer - DMA request flow enable"]
pub type CcdrepeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCR1SHIFTEN` reader - CCR1 pwm shift enable"]
pub type Ccr1shiftenR = crate::BitReader;
#[doc = "Field `CCR1SHIFTEN` writer - CCR1 pwm shift enable"]
pub type Ccr1shiftenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCR2SHIFTEN` reader - CCR2 pwm shift enable"]
pub type Ccr2shiftenR = crate::BitReader;
#[doc = "Field `CCR2SHIFTEN` writer - CCR2 pwm shift enable"]
pub type Ccr2shiftenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCR3SHIFTEN` reader - CCR3 pwm shift enable"]
pub type Ccr3shiftenR = crate::BitReader;
#[doc = "Field `CCR3SHIFTEN` writer - CCR3 pwm shift enable"]
pub type Ccr3shiftenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCR4SHIFTEN` reader - CCR4 pwm shift enable"]
pub type Ccr4shiftenR = crate::BitReader;
#[doc = "Field `CCR4SHIFTEN` writer - CCR4 pwm shift enable"]
pub type Ccr4shiftenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCR5SHIFTEN` reader - CCR5 pwm shift enable"]
pub type Ccr5shiftenR = crate::BitReader;
#[doc = "Field `CCR5SHIFTEN` writer - CCR5 pwm shift enable"]
pub type Ccr5shiftenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA request flow enable"]
    #[inline(always)]
    pub fn ccdrepe(&self) -> CcdrepeR {
        CcdrepeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCR1 pwm shift enable"]
    #[inline(always)]
    pub fn ccr1shiften(&self) -> Ccr1shiftenR {
        Ccr1shiftenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCR2 pwm shift enable"]
    #[inline(always)]
    pub fn ccr2shiften(&self) -> Ccr2shiftenR {
        Ccr2shiftenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCR3 pwm shift enable"]
    #[inline(always)]
    pub fn ccr3shiften(&self) -> Ccr3shiftenR {
        Ccr3shiftenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCR4 pwm shift enable"]
    #[inline(always)]
    pub fn ccr4shiften(&self) -> Ccr4shiftenR {
        Ccr4shiftenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCR5 pwm shift enable"]
    #[inline(always)]
    pub fn ccr5shiften(&self) -> Ccr5shiftenR {
        Ccr5shiftenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA request flow enable"]
    #[inline(always)]
    pub fn ccdrepe(&mut self) -> CcdrepeW<'_, PderSpec> {
        CcdrepeW::new(self, 0)
    }
    #[doc = "Bit 1 - CCR1 pwm shift enable"]
    #[inline(always)]
    pub fn ccr1shiften(&mut self) -> Ccr1shiftenW<'_, PderSpec> {
        Ccr1shiftenW::new(self, 1)
    }
    #[doc = "Bit 2 - CCR2 pwm shift enable"]
    #[inline(always)]
    pub fn ccr2shiften(&mut self) -> Ccr2shiftenW<'_, PderSpec> {
        Ccr2shiftenW::new(self, 2)
    }
    #[doc = "Bit 3 - CCR3 pwm shift enable"]
    #[inline(always)]
    pub fn ccr3shiften(&mut self) -> Ccr3shiftenW<'_, PderSpec> {
        Ccr3shiftenW::new(self, 3)
    }
    #[doc = "Bit 4 - CCR4 pwm shift enable"]
    #[inline(always)]
    pub fn ccr4shiften(&mut self) -> Ccr4shiftenW<'_, PderSpec> {
        Ccr4shiftenW::new(self, 4)
    }
    #[doc = "Bit 5 - CCR5 pwm shift enable"]
    #[inline(always)]
    pub fn ccr5shiften(&mut self) -> Ccr5shiftenW<'_, PderSpec> {
        Ccr5shiftenW::new(self, 5)
    }
}
#[doc = "PWM/DMA repeat enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`pder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PderSpec;
impl crate::RegisterSpec for PderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pder::R`](R) reader structure"]
impl crate::Readable for PderSpec {}
#[doc = "`write(|w| ..)` method takes [`pder::W`](W) writer structure"]
impl crate::Writable for PderSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDER to value 0"]
impl crate::Resettable for PderSpec {}
