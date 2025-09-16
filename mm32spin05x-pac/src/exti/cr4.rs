#[doc = "Register `CR4` reader"]
pub type R = crate::R<Cr4Spec>;
#[doc = "Register `CR4` writer"]
pub type W = crate::W<Cr4Spec>;
#[doc = "Field `EXTI12` reader - EXTI12 configuration"]
pub type Exti12R = crate::FieldReader;
#[doc = "Field `EXTI12` writer - EXTI12 configuration"]
pub type Exti12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI13` reader - EXTI13 configuration"]
pub type Exti13R = crate::FieldReader;
#[doc = "Field `EXTI13` writer - EXTI13 configuration"]
pub type Exti13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI14` reader - EXTI14 configuration"]
pub type Exti14R = crate::FieldReader;
#[doc = "Field `EXTI14` writer - EXTI14 configuration"]
pub type Exti14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI15` reader - EXTI15 configuration"]
pub type Exti15R = crate::FieldReader;
#[doc = "Field `EXTI15` writer - EXTI15 configuration"]
pub type Exti15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI12 configuration"]
    #[inline(always)]
    pub fn exti12(&self) -> Exti12R {
        Exti12R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI13 configuration"]
    #[inline(always)]
    pub fn exti13(&self) -> Exti13R {
        Exti13R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI14 configuration"]
    #[inline(always)]
    pub fn exti14(&self) -> Exti14R {
        Exti14R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI15 configuration"]
    #[inline(always)]
    pub fn exti15(&self) -> Exti15R {
        Exti15R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI12 configuration"]
    #[inline(always)]
    pub fn exti12(&mut self) -> Exti12W<'_, Cr4Spec> {
        Exti12W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI13 configuration"]
    #[inline(always)]
    pub fn exti13(&mut self) -> Exti13W<'_, Cr4Spec> {
        Exti13W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI14 configuration"]
    #[inline(always)]
    pub fn exti14(&mut self) -> Exti14W<'_, Cr4Spec> {
        Exti14W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI15 configuration"]
    #[inline(always)]
    pub fn exti15(&mut self) -> Exti15W<'_, Cr4Spec> {
        Exti15W::new(self, 12)
    }
}
#[doc = "External interrupt configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr4Spec;
impl crate::RegisterSpec for Cr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr4::R`](R) reader structure"]
impl crate::Readable for Cr4Spec {}
#[doc = "`write(|w| ..)` method takes [`cr4::W`](W) writer structure"]
impl crate::Writable for Cr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for Cr4Spec {}
