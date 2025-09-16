#[doc = "Register `BTR1` reader"]
pub type R = crate::R<Btr1Spec>;
#[doc = "Register `BTR1` writer"]
pub type W = crate::W<Btr1Spec>;
#[doc = "Field `TSEG1` reader - Time segment 1"]
pub type Tseg1R = crate::FieldReader;
#[doc = "Field `TSEG1` writer - Time segment 1"]
pub type Tseg1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSEG2` reader - Time segment 2"]
pub type Tseg2R = crate::FieldReader;
#[doc = "Field `TSEG2` writer - Time segment 2"]
pub type Tseg2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SAM` reader - Sampling"]
pub type SamR = crate::BitReader;
#[doc = "Field `SAM` writer - Sampling"]
pub type SamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Time segment 1"]
    #[inline(always)]
    pub fn tseg1(&self) -> Tseg1R {
        Tseg1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Time segment 2"]
    #[inline(always)]
    pub fn tseg2(&self) -> Tseg2R {
        Tseg2R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Sampling"]
    #[inline(always)]
    pub fn sam(&self) -> SamR {
        SamR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time segment 1"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> Tseg1W<'_, Btr1Spec> {
        Tseg1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Time segment 2"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> Tseg2W<'_, Btr1Spec> {
        Tseg2W::new(self, 4)
    }
    #[doc = "Bit 7 - Sampling"]
    #[inline(always)]
    pub fn sam(&mut self) -> SamW<'_, Btr1Spec> {
        SamW::new(self, 7)
    }
}
#[doc = "Bus Timing register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`btr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Btr1Spec;
impl crate::RegisterSpec for Btr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr1::R`](R) reader structure"]
impl crate::Readable for Btr1Spec {}
#[doc = "`write(|w| ..)` method takes [`btr1::W`](W) writer structure"]
impl crate::Writable for Btr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTR1 to value 0"]
impl crate::Resettable for Btr1Spec {}
