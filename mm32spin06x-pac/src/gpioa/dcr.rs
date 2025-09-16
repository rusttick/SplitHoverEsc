#[doc = "Register `DCR` reader"]
pub type R = crate::R<DcrSpec>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DcrSpec>;
#[doc = "Field `PX0` reader - PX0"]
pub type Px0R = crate::FieldReader;
#[doc = "Field `PX0` writer - PX0"]
pub type Px0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX1` reader - PX1"]
pub type Px1R = crate::FieldReader;
#[doc = "Field `PX1` writer - PX1"]
pub type Px1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX2` reader - PX2"]
pub type Px2R = crate::FieldReader;
#[doc = "Field `PX2` writer - PX2"]
pub type Px2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX3` reader - PX3"]
pub type Px3R = crate::FieldReader;
#[doc = "Field `PX3` writer - PX3"]
pub type Px3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX4` reader - PX4"]
pub type Px4R = crate::FieldReader;
#[doc = "Field `PX4` writer - PX4"]
pub type Px4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX5` reader - PX5"]
pub type Px5R = crate::FieldReader;
#[doc = "Field `PX5` writer - PX5"]
pub type Px5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX6` reader - PX6"]
pub type Px6R = crate::FieldReader;
#[doc = "Field `PX6` writer - PX6"]
pub type Px6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX7` reader - PX7"]
pub type Px7R = crate::FieldReader;
#[doc = "Field `PX7` writer - PX7"]
pub type Px7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX8` reader - PX8"]
pub type Px8R = crate::FieldReader;
#[doc = "Field `PX8` writer - PX8"]
pub type Px8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX9` reader - PX9"]
pub type Px9R = crate::FieldReader;
#[doc = "Field `PX9` writer - PX9"]
pub type Px9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX10` reader - PX10"]
pub type Px10R = crate::FieldReader;
#[doc = "Field `PX10` writer - PX10"]
pub type Px10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX11` reader - PX11"]
pub type Px11R = crate::FieldReader;
#[doc = "Field `PX11` writer - PX11"]
pub type Px11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX12` reader - PX12"]
pub type Px12R = crate::FieldReader;
#[doc = "Field `PX12` writer - PX12"]
pub type Px12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX13` reader - PX13"]
pub type Px13R = crate::FieldReader;
#[doc = "Field `PX13` writer - PX13"]
pub type Px13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX14` reader - PX14"]
pub type Px14R = crate::FieldReader;
#[doc = "Field `PX14` writer - PX14"]
pub type Px14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX15` reader - PX15"]
pub type Px15R = crate::FieldReader;
#[doc = "Field `PX15` writer - PX15"]
pub type Px15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PX0"]
    #[inline(always)]
    pub fn px0(&self) -> Px0R {
        Px0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PX1"]
    #[inline(always)]
    pub fn px1(&self) -> Px1R {
        Px1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PX2"]
    #[inline(always)]
    pub fn px2(&self) -> Px2R {
        Px2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PX3"]
    #[inline(always)]
    pub fn px3(&self) -> Px3R {
        Px3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PX4"]
    #[inline(always)]
    pub fn px4(&self) -> Px4R {
        Px4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PX5"]
    #[inline(always)]
    pub fn px5(&self) -> Px5R {
        Px5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PX6"]
    #[inline(always)]
    pub fn px6(&self) -> Px6R {
        Px6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PX7"]
    #[inline(always)]
    pub fn px7(&self) -> Px7R {
        Px7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PX8"]
    #[inline(always)]
    pub fn px8(&self) -> Px8R {
        Px8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PX9"]
    #[inline(always)]
    pub fn px9(&self) -> Px9R {
        Px9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PX10"]
    #[inline(always)]
    pub fn px10(&self) -> Px10R {
        Px10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PX11"]
    #[inline(always)]
    pub fn px11(&self) -> Px11R {
        Px11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PX12"]
    #[inline(always)]
    pub fn px12(&self) -> Px12R {
        Px12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PX13"]
    #[inline(always)]
    pub fn px13(&self) -> Px13R {
        Px13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PX14"]
    #[inline(always)]
    pub fn px14(&self) -> Px14R {
        Px14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PX15"]
    #[inline(always)]
    pub fn px15(&self) -> Px15R {
        Px15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PX0"]
    #[inline(always)]
    pub fn px0(&mut self) -> Px0W<'_, DcrSpec> {
        Px0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PX1"]
    #[inline(always)]
    pub fn px1(&mut self) -> Px1W<'_, DcrSpec> {
        Px1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - PX2"]
    #[inline(always)]
    pub fn px2(&mut self) -> Px2W<'_, DcrSpec> {
        Px2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - PX3"]
    #[inline(always)]
    pub fn px3(&mut self) -> Px3W<'_, DcrSpec> {
        Px3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - PX4"]
    #[inline(always)]
    pub fn px4(&mut self) -> Px4W<'_, DcrSpec> {
        Px4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - PX5"]
    #[inline(always)]
    pub fn px5(&mut self) -> Px5W<'_, DcrSpec> {
        Px5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - PX6"]
    #[inline(always)]
    pub fn px6(&mut self) -> Px6W<'_, DcrSpec> {
        Px6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - PX7"]
    #[inline(always)]
    pub fn px7(&mut self) -> Px7W<'_, DcrSpec> {
        Px7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - PX8"]
    #[inline(always)]
    pub fn px8(&mut self) -> Px8W<'_, DcrSpec> {
        Px8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - PX9"]
    #[inline(always)]
    pub fn px9(&mut self) -> Px9W<'_, DcrSpec> {
        Px9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - PX10"]
    #[inline(always)]
    pub fn px10(&mut self) -> Px10W<'_, DcrSpec> {
        Px10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - PX11"]
    #[inline(always)]
    pub fn px11(&mut self) -> Px11W<'_, DcrSpec> {
        Px11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - PX12"]
    #[inline(always)]
    pub fn px12(&mut self) -> Px12W<'_, DcrSpec> {
        Px12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - PX13"]
    #[inline(always)]
    pub fn px13(&mut self) -> Px13W<'_, DcrSpec> {
        Px13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - PX14"]
    #[inline(always)]
    pub fn px14(&mut self) -> Px14W<'_, DcrSpec> {
        Px14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - PX15"]
    #[inline(always)]
    pub fn px15(&mut self) -> Px15W<'_, DcrSpec> {
        Px15W::new(self, 30)
    }
}
#[doc = "Port output open drain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrSpec;
impl crate::RegisterSpec for DcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DcrSpec {}
