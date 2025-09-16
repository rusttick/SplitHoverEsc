#[doc = "Register `BTR0` reader"]
pub type R = crate::R<Btr0Spec>;
#[doc = "Register `BTR0` writer"]
pub type W = crate::W<Btr0Spec>;
#[doc = "Field `BRP` reader - Baud rate prescaler"]
pub type BrpR = crate::FieldReader;
#[doc = "Field `BRP` writer - Baud rate prescaler"]
pub type BrpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SJW` reader - Synchronization jump width"]
pub type SjwR = crate::FieldReader;
#[doc = "Field `SJW` writer - Synchronization jump width"]
pub type SjwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - Baud rate prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BrpR {
        BrpR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Synchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SjwR {
        SjwR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud rate prescaler"]
    #[inline(always)]
    pub fn brp(&mut self) -> BrpW<'_, Btr0Spec> {
        BrpW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Synchronization jump width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SjwW<'_, Btr0Spec> {
        SjwW::new(self, 6)
    }
}
#[doc = "Bus Timing register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`btr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Btr0Spec;
impl crate::RegisterSpec for Btr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr0::R`](R) reader structure"]
impl crate::Readable for Btr0Spec {}
#[doc = "`write(|w| ..)` method takes [`btr0::W`](W) writer structure"]
impl crate::Writable for Btr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTR0 to value 0"]
impl crate::Resettable for Btr0Spec {}
