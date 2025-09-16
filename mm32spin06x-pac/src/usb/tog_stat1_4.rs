#[doc = "Register `TOG_STAT1_4` reader"]
pub type R = crate::R<TogStat1_4Spec>;
#[doc = "Register `TOG_STAT1_4` writer"]
pub type W = crate::W<TogStat1_4Spec>;
#[doc = "Field `IN_TOG1` reader - End Point 1 Toggle IN State"]
pub type InTog1R = crate::BitReader;
#[doc = "Field `IN_TOG1` writer - End Point 1 Toggle IN State"]
pub type InTog1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOG1` reader - End Point 1 Toggle ON State"]
pub type OutTog1R = crate::BitReader;
#[doc = "Field `OUT_TOG1` writer - End Point 1 Toggle ON State"]
pub type OutTog1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_TOG2` reader - End Point 2 Toggle IN State"]
pub type InTog2R = crate::BitReader;
#[doc = "Field `IN_TOG2` writer - End Point 2 Toggle IN State"]
pub type InTog2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOG2` reader - End Point 2 Toggle ON State"]
pub type OutTog2R = crate::BitReader;
#[doc = "Field `OUT_TOG2` writer - End Point 2 Toggle ON State"]
pub type OutTog2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_TOG3` reader - End Point 3 Toggle IN State"]
pub type InTog3R = crate::BitReader;
#[doc = "Field `IN_TOG3` writer - End Point 3 Toggle IN State"]
pub type InTog3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOG3` reader - End Point 3 Toggle ON State"]
pub type OutTog3R = crate::BitReader;
#[doc = "Field `OUT_TOG3` writer - End Point 3 Toggle ON State"]
pub type OutTog3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_TOG4` reader - End Point 4 Toggle IN State"]
pub type InTog4R = crate::BitReader;
#[doc = "Field `IN_TOG4` writer - End Point 4 Toggle IN State"]
pub type InTog4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOG4` reader - End Point 4 Toggle ON State"]
pub type OutTog4R = crate::BitReader;
#[doc = "Field `OUT_TOG4` writer - End Point 4 Toggle ON State"]
pub type OutTog4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End Point 1 Toggle IN State"]
    #[inline(always)]
    pub fn in_tog1(&self) -> InTog1R {
        InTog1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End Point 1 Toggle ON State"]
    #[inline(always)]
    pub fn out_tog1(&self) -> OutTog1R {
        OutTog1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End Point 2 Toggle IN State"]
    #[inline(always)]
    pub fn in_tog2(&self) -> InTog2R {
        InTog2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End Point 2 Toggle ON State"]
    #[inline(always)]
    pub fn out_tog2(&self) -> OutTog2R {
        OutTog2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End Point 3 Toggle IN State"]
    #[inline(always)]
    pub fn in_tog3(&self) -> InTog3R {
        InTog3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Point 3 Toggle ON State"]
    #[inline(always)]
    pub fn out_tog3(&self) -> OutTog3R {
        OutTog3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End Point 4 Toggle IN State"]
    #[inline(always)]
    pub fn in_tog4(&self) -> InTog4R {
        InTog4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End Point 4 Toggle ON State"]
    #[inline(always)]
    pub fn out_tog4(&self) -> OutTog4R {
        OutTog4R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End Point 1 Toggle IN State"]
    #[inline(always)]
    pub fn in_tog1(&mut self) -> InTog1W<'_, TogStat1_4Spec> {
        InTog1W::new(self, 0)
    }
    #[doc = "Bit 1 - End Point 1 Toggle ON State"]
    #[inline(always)]
    pub fn out_tog1(&mut self) -> OutTog1W<'_, TogStat1_4Spec> {
        OutTog1W::new(self, 1)
    }
    #[doc = "Bit 2 - End Point 2 Toggle IN State"]
    #[inline(always)]
    pub fn in_tog2(&mut self) -> InTog2W<'_, TogStat1_4Spec> {
        InTog2W::new(self, 2)
    }
    #[doc = "Bit 3 - End Point 2 Toggle ON State"]
    #[inline(always)]
    pub fn out_tog2(&mut self) -> OutTog2W<'_, TogStat1_4Spec> {
        OutTog2W::new(self, 3)
    }
    #[doc = "Bit 4 - End Point 3 Toggle IN State"]
    #[inline(always)]
    pub fn in_tog3(&mut self) -> InTog3W<'_, TogStat1_4Spec> {
        InTog3W::new(self, 4)
    }
    #[doc = "Bit 5 - End Point 3 Toggle ON State"]
    #[inline(always)]
    pub fn out_tog3(&mut self) -> OutTog3W<'_, TogStat1_4Spec> {
        OutTog3W::new(self, 5)
    }
    #[doc = "Bit 6 - End Point 4 Toggle IN State"]
    #[inline(always)]
    pub fn in_tog4(&mut self) -> InTog4W<'_, TogStat1_4Spec> {
        InTog4W::new(self, 6)
    }
    #[doc = "Bit 7 - End Point 4 Toggle ON State"]
    #[inline(always)]
    pub fn out_tog4(&mut self) -> OutTog4W<'_, TogStat1_4Spec> {
        OutTog4W::new(self, 7)
    }
}
#[doc = "TOG STAT1_4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`tog_stat1_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tog_stat1_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TogStat1_4Spec;
impl crate::RegisterSpec for TogStat1_4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tog_stat1_4::R`](R) reader structure"]
impl crate::Readable for TogStat1_4Spec {}
#[doc = "`write(|w| ..)` method takes [`tog_stat1_4::W`](W) writer structure"]
impl crate::Writable for TogStat1_4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOG_STAT1_4 to value 0"]
impl crate::Resettable for TogStat1_4Spec {}
