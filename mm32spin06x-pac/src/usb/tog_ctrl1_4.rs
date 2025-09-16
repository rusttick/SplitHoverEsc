#[doc = "Register `TOG_CTRL1_4` reader"]
pub type R = crate::R<TogCtrl1_4Spec>;
#[doc = "Register `TOG_CTRL1_4` writer"]
pub type W = crate::W<TogCtrl1_4Spec>;
#[doc = "Field `DTOG1` reader - Set End Point 1 Toggle"]
pub type Dtog1R = crate::BitReader;
#[doc = "Field `DTOG1` writer - Set End Point 1 Toggle"]
pub type Dtog1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOG1EN` reader - Set End Point 1 enable"]
pub type Dtog1enR = crate::BitReader;
#[doc = "Field `DTOG1EN` writer - Set End Point 1 enable"]
pub type Dtog1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOG2` reader - Set End Point 2 Toggle"]
pub type Dtog2R = crate::BitReader;
#[doc = "Field `DTOG2` writer - Set End Point 2 Toggle"]
pub type Dtog2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOG2EN` reader - Set End Point 2 enable"]
pub type Dtog2enR = crate::BitReader;
#[doc = "Field `DTOG2EN` writer - Set End Point 2 enable"]
pub type Dtog2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOG3` reader - Set End Point 3 Toggle"]
pub type Dtog3R = crate::BitReader;
#[doc = "Field `DTOG3` writer - Set End Point 3 Toggle"]
pub type Dtog3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOG3EN` reader - Set End Point 3 enable"]
pub type Dtog3enR = crate::BitReader;
#[doc = "Field `DTOG3EN` writer - Set End Point 3 enable"]
pub type Dtog3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOG4` reader - Set End Point 4 Toggle"]
pub type Dtog4R = crate::BitReader;
#[doc = "Field `DTOG4` writer - Set End Point 4 Toggle"]
pub type Dtog4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOG4EN` reader - Set End Point 4 enable"]
pub type Dtog4enR = crate::BitReader;
#[doc = "Field `DTOG4EN` writer - Set End Point 4 enable"]
pub type Dtog4enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set End Point 1 Toggle"]
    #[inline(always)]
    pub fn dtog1(&self) -> Dtog1R {
        Dtog1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set End Point 1 enable"]
    #[inline(always)]
    pub fn dtog1en(&self) -> Dtog1enR {
        Dtog1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set End Point 2 Toggle"]
    #[inline(always)]
    pub fn dtog2(&self) -> Dtog2R {
        Dtog2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set End Point 2 enable"]
    #[inline(always)]
    pub fn dtog2en(&self) -> Dtog2enR {
        Dtog2enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set End Point 3 Toggle"]
    #[inline(always)]
    pub fn dtog3(&self) -> Dtog3R {
        Dtog3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set End Point 3 enable"]
    #[inline(always)]
    pub fn dtog3en(&self) -> Dtog3enR {
        Dtog3enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set End Point 4 Toggle"]
    #[inline(always)]
    pub fn dtog4(&self) -> Dtog4R {
        Dtog4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set End Point 4 enable"]
    #[inline(always)]
    pub fn dtog4en(&self) -> Dtog4enR {
        Dtog4enR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set End Point 1 Toggle"]
    #[inline(always)]
    pub fn dtog1(&mut self) -> Dtog1W<'_, TogCtrl1_4Spec> {
        Dtog1W::new(self, 0)
    }
    #[doc = "Bit 1 - Set End Point 1 enable"]
    #[inline(always)]
    pub fn dtog1en(&mut self) -> Dtog1enW<'_, TogCtrl1_4Spec> {
        Dtog1enW::new(self, 1)
    }
    #[doc = "Bit 2 - Set End Point 2 Toggle"]
    #[inline(always)]
    pub fn dtog2(&mut self) -> Dtog2W<'_, TogCtrl1_4Spec> {
        Dtog2W::new(self, 2)
    }
    #[doc = "Bit 3 - Set End Point 2 enable"]
    #[inline(always)]
    pub fn dtog2en(&mut self) -> Dtog2enW<'_, TogCtrl1_4Spec> {
        Dtog2enW::new(self, 3)
    }
    #[doc = "Bit 4 - Set End Point 3 Toggle"]
    #[inline(always)]
    pub fn dtog3(&mut self) -> Dtog3W<'_, TogCtrl1_4Spec> {
        Dtog3W::new(self, 4)
    }
    #[doc = "Bit 5 - Set End Point 3 enable"]
    #[inline(always)]
    pub fn dtog3en(&mut self) -> Dtog3enW<'_, TogCtrl1_4Spec> {
        Dtog3enW::new(self, 5)
    }
    #[doc = "Bit 6 - Set End Point 4 Toggle"]
    #[inline(always)]
    pub fn dtog4(&mut self) -> Dtog4W<'_, TogCtrl1_4Spec> {
        Dtog4W::new(self, 6)
    }
    #[doc = "Bit 7 - Set End Point 4 enable"]
    #[inline(always)]
    pub fn dtog4en(&mut self) -> Dtog4enW<'_, TogCtrl1_4Spec> {
        Dtog4enW::new(self, 7)
    }
}
#[doc = "TOG CTRL1_4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`tog_ctrl1_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tog_ctrl1_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TogCtrl1_4Spec;
impl crate::RegisterSpec for TogCtrl1_4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tog_ctrl1_4::R`](R) reader structure"]
impl crate::Readable for TogCtrl1_4Spec {}
#[doc = "`write(|w| ..)` method takes [`tog_ctrl1_4::W`](W) writer structure"]
impl crate::Writable for TogCtrl1_4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOG_CTRL1_4 to value 0"]
impl crate::Resettable for TogCtrl1_4Spec {}
