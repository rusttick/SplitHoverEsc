#[doc = "Register `CHANY0` reader"]
pub type R = crate::R<Chany0Spec>;
#[doc = "Register `CHANY0` writer"]
pub type W = crate::W<Chany0Spec>;
#[doc = "Field `CHANY_SEL0` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel0R = crate::FieldReader;
#[doc = "Field `CHANY_SEL0` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL1` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel1R = crate::FieldReader;
#[doc = "Field `CHANY_SEL1` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL2` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel2R = crate::FieldReader;
#[doc = "Field `CHANY_SEL2` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL3` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel3R = crate::FieldReader;
#[doc = "Field `CHANY_SEL3` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL4` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel4R = crate::FieldReader;
#[doc = "Field `CHANY_SEL4` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL5` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel5R = crate::FieldReader;
#[doc = "Field `CHANY_SEL5` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL6` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel6R = crate::FieldReader;
#[doc = "Field `CHANY_SEL6` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANY_SEL7` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel7R = crate::FieldReader;
#[doc = "Field `CHANY_SEL7` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type ChanySel7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel0(&self) -> ChanySel0R {
        ChanySel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel1(&self) -> ChanySel1R {
        ChanySel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel2(&self) -> ChanySel2R {
        ChanySel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel3(&self) -> ChanySel3R {
        ChanySel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel4(&self) -> ChanySel4R {
        ChanySel4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel5(&self) -> ChanySel5R {
        ChanySel5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel6(&self) -> ChanySel6R {
        ChanySel6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel7(&self) -> ChanySel7R {
        ChanySel7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel0(&mut self) -> ChanySel0W<'_, Chany0Spec> {
        ChanySel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel1(&mut self) -> ChanySel1W<'_, Chany0Spec> {
        ChanySel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel2(&mut self) -> ChanySel2W<'_, Chany0Spec> {
        ChanySel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel3(&mut self) -> ChanySel3W<'_, Chany0Spec> {
        ChanySel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel4(&mut self) -> ChanySel4W<'_, Chany0Spec> {
        ChanySel4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel5(&mut self) -> ChanySel5W<'_, Chany0Spec> {
        ChanySel5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel6(&mut self) -> ChanySel6W<'_, Chany0Spec> {
        ChanySel6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn chany_sel7(&mut self) -> ChanySel7W<'_, Chany0Spec> {
        ChanySel7W::new(self, 28)
    }
}
#[doc = "Arbitrary channel channel selection register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`chany0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chany0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chany0Spec;
impl crate::RegisterSpec for Chany0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chany0::R`](R) reader structure"]
impl crate::Readable for Chany0Spec {}
#[doc = "`write(|w| ..)` method takes [`chany0::W`](W) writer structure"]
impl crate::Writable for Chany0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHANY0 to value 0"]
impl crate::Resettable for Chany0Spec {}
