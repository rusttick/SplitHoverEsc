#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `IMR0` reader - Interrupt Mask on line 0"]
pub type Imr0R = crate::BitReader;
#[doc = "Field `IMR0` writer - Interrupt Mask on line 0"]
pub type Imr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR1` reader - Interrupt Mask on line 1"]
pub type Imr1R = crate::BitReader;
#[doc = "Field `IMR1` writer - Interrupt Mask on line 1"]
pub type Imr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR2` reader - Interrupt Mask on line 2"]
pub type Imr2R = crate::BitReader;
#[doc = "Field `IMR2` writer - Interrupt Mask on line 2"]
pub type Imr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR3` reader - Interrupt Mask on line 3"]
pub type Imr3R = crate::BitReader;
#[doc = "Field `IMR3` writer - Interrupt Mask on line 3"]
pub type Imr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR4` reader - Interrupt Mask on line 4"]
pub type Imr4R = crate::BitReader;
#[doc = "Field `IMR4` writer - Interrupt Mask on line 4"]
pub type Imr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR5` reader - Interrupt Mask on line 5"]
pub type Imr5R = crate::BitReader;
#[doc = "Field `IMR5` writer - Interrupt Mask on line 5"]
pub type Imr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR6` reader - Interrupt Mask on line 6"]
pub type Imr6R = crate::BitReader;
#[doc = "Field `IMR6` writer - Interrupt Mask on line 6"]
pub type Imr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR7` reader - Interrupt Mask on line 7"]
pub type Imr7R = crate::BitReader;
#[doc = "Field `IMR7` writer - Interrupt Mask on line 7"]
pub type Imr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR8` reader - Interrupt Mask on line 8"]
pub type Imr8R = crate::BitReader;
#[doc = "Field `IMR8` writer - Interrupt Mask on line 8"]
pub type Imr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR9` reader - Interrupt Mask on line 9"]
pub type Imr9R = crate::BitReader;
#[doc = "Field `IMR9` writer - Interrupt Mask on line 9"]
pub type Imr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR10` reader - Interrupt Mask on line 10"]
pub type Imr10R = crate::BitReader;
#[doc = "Field `IMR10` writer - Interrupt Mask on line 10"]
pub type Imr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR11` reader - Interrupt Mask on line 11"]
pub type Imr11R = crate::BitReader;
#[doc = "Field `IMR11` writer - Interrupt Mask on line 11"]
pub type Imr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR12` reader - Interrupt Mask on line 12"]
pub type Imr12R = crate::BitReader;
#[doc = "Field `IMR12` writer - Interrupt Mask on line 12"]
pub type Imr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR13` reader - Interrupt Mask on line 13"]
pub type Imr13R = crate::BitReader;
#[doc = "Field `IMR13` writer - Interrupt Mask on line 13"]
pub type Imr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR14` reader - Interrupt Mask on line 14"]
pub type Imr14R = crate::BitReader;
#[doc = "Field `IMR14` writer - Interrupt Mask on line 14"]
pub type Imr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR15` reader - Interrupt Mask on line 15"]
pub type Imr15R = crate::BitReader;
#[doc = "Field `IMR15` writer - Interrupt Mask on line 15"]
pub type Imr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR16` reader - Interrupt Mask on line 16"]
pub type Imr16R = crate::BitReader;
#[doc = "Field `IMR16` writer - Interrupt Mask on line 16"]
pub type Imr16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR19` reader - Interrupt Mask on line 19"]
pub type Imr19R = crate::BitReader;
#[doc = "Field `IMR19` writer - Interrupt Mask on line 19"]
pub type Imr19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR24` reader - Interrupt Mask on line 24"]
pub type Imr24R = crate::BitReader;
#[doc = "Field `IMR24` writer - Interrupt Mask on line 24"]
pub type Imr24W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    pub fn imr0(&self) -> Imr0R {
        Imr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    pub fn imr1(&self) -> Imr1R {
        Imr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    pub fn imr2(&self) -> Imr2R {
        Imr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    pub fn imr3(&self) -> Imr3R {
        Imr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    pub fn imr4(&self) -> Imr4R {
        Imr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    pub fn imr5(&self) -> Imr5R {
        Imr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    pub fn imr6(&self) -> Imr6R {
        Imr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    pub fn imr7(&self) -> Imr7R {
        Imr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    pub fn imr8(&self) -> Imr8R {
        Imr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    pub fn imr9(&self) -> Imr9R {
        Imr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    pub fn imr10(&self) -> Imr10R {
        Imr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    pub fn imr11(&self) -> Imr11R {
        Imr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    pub fn imr12(&self) -> Imr12R {
        Imr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    pub fn imr13(&self) -> Imr13R {
        Imr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    pub fn imr14(&self) -> Imr14R {
        Imr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    pub fn imr15(&self) -> Imr15R {
        Imr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    pub fn imr16(&self) -> Imr16R {
        Imr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Mask on line 19"]
    #[inline(always)]
    pub fn imr19(&self) -> Imr19R {
        Imr19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Mask on line 24"]
    #[inline(always)]
    pub fn imr24(&self) -> Imr24R {
        Imr24R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    pub fn imr0(&mut self) -> Imr0W<'_, ImrSpec> {
        Imr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    pub fn imr1(&mut self) -> Imr1W<'_, ImrSpec> {
        Imr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    pub fn imr2(&mut self) -> Imr2W<'_, ImrSpec> {
        Imr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    pub fn imr3(&mut self) -> Imr3W<'_, ImrSpec> {
        Imr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    pub fn imr4(&mut self) -> Imr4W<'_, ImrSpec> {
        Imr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    pub fn imr5(&mut self) -> Imr5W<'_, ImrSpec> {
        Imr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    pub fn imr6(&mut self) -> Imr6W<'_, ImrSpec> {
        Imr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    pub fn imr7(&mut self) -> Imr7W<'_, ImrSpec> {
        Imr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    pub fn imr8(&mut self) -> Imr8W<'_, ImrSpec> {
        Imr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    pub fn imr9(&mut self) -> Imr9W<'_, ImrSpec> {
        Imr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    pub fn imr10(&mut self) -> Imr10W<'_, ImrSpec> {
        Imr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    pub fn imr11(&mut self) -> Imr11W<'_, ImrSpec> {
        Imr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    pub fn imr12(&mut self) -> Imr12W<'_, ImrSpec> {
        Imr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    pub fn imr13(&mut self) -> Imr13W<'_, ImrSpec> {
        Imr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    pub fn imr14(&mut self) -> Imr14W<'_, ImrSpec> {
        Imr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    pub fn imr15(&mut self) -> Imr15W<'_, ImrSpec> {
        Imr15W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    pub fn imr16(&mut self) -> Imr16W<'_, ImrSpec> {
        Imr16W::new(self, 16)
    }
    #[doc = "Bit 19 - Interrupt Mask on line 19"]
    #[inline(always)]
    pub fn imr19(&mut self) -> Imr19W<'_, ImrSpec> {
        Imr19W::new(self, 19)
    }
    #[doc = "Bit 24 - Interrupt Mask on line 24"]
    #[inline(always)]
    pub fn imr24(&mut self) -> Imr24W<'_, ImrSpec> {
        Imr24W::new(self, 24)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
