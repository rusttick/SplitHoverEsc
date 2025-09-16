#[doc = "Register `CRL` reader"]
pub type R = crate::R<CrlSpec>;
#[doc = "Register `CRL` writer"]
pub type W = crate::W<CrlSpec>;
#[doc = "Field `MODE0` reader - Port 0 mode bits"]
pub type Mode0R = crate::FieldReader;
#[doc = "Field `MODE0` writer - Port 0 mode bits"]
pub type Mode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF0` reader - Port 0 configuration bits"]
pub type Cnf0R = crate::FieldReader;
#[doc = "Field `CNF0` writer - Port 0 configuration bits"]
pub type Cnf0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE1` reader - Port 1 mode bits"]
pub type Mode1R = crate::FieldReader;
#[doc = "Field `MODE1` writer - Port 1 mode bits"]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF1` reader - Port 1 configuration bits"]
pub type Cnf1R = crate::FieldReader;
#[doc = "Field `CNF1` writer - Port 1 configuration bits"]
pub type Cnf1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE2` reader - Port 2 mode bits"]
pub type Mode2R = crate::FieldReader;
#[doc = "Field `MODE2` writer - Port 2 mode bits"]
pub type Mode2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF2` reader - Port 2 configuration bits"]
pub type Cnf2R = crate::FieldReader;
#[doc = "Field `CNF2` writer - Port 2 configuration bits"]
pub type Cnf2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE3` reader - Port 3 mode bits"]
pub type Mode3R = crate::FieldReader;
#[doc = "Field `MODE3` writer - Port 3 mode bits"]
pub type Mode3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF3` reader - Port 3 configuration bits"]
pub type Cnf3R = crate::FieldReader;
#[doc = "Field `CNF3` writer - Port 3 configuration bits"]
pub type Cnf3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE4` reader - Port 4 mode bits"]
pub type Mode4R = crate::FieldReader;
#[doc = "Field `MODE4` writer - Port 4 mode bits"]
pub type Mode4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF4` reader - Port 4 configuration bits"]
pub type Cnf4R = crate::FieldReader;
#[doc = "Field `CNF4` writer - Port 4 configuration bits"]
pub type Cnf4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE5` reader - Port 5 mode bits"]
pub type Mode5R = crate::FieldReader;
#[doc = "Field `MODE5` writer - Port 5 mode bits"]
pub type Mode5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF5` reader - Port 5 configuration bits"]
pub type Cnf5R = crate::FieldReader;
#[doc = "Field `CNF5` writer - Port 5 configuration bits"]
pub type Cnf5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE6` reader - Port 6 mode bits"]
pub type Mode6R = crate::FieldReader;
#[doc = "Field `MODE6` writer - Port 6 mode bits"]
pub type Mode6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF6` reader - Port 6 configuration bits"]
pub type Cnf6R = crate::FieldReader;
#[doc = "Field `CNF6` writer - Port 6 configuration bits"]
pub type Cnf6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE7` reader - Port 7 mode bits"]
pub type Mode7R = crate::FieldReader;
#[doc = "Field `MODE7` writer - Port 7 mode bits"]
pub type Mode7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF7` reader - Port 7 configuration bits"]
pub type Cnf7R = crate::FieldReader;
#[doc = "Field `CNF7` writer - Port 7 configuration bits"]
pub type Cnf7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port 0 mode bits"]
    #[inline(always)]
    pub fn mode0(&self) -> Mode0R {
        Mode0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 0 configuration bits"]
    #[inline(always)]
    pub fn cnf0(&self) -> Cnf0R {
        Cnf0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 mode bits"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 configuration bits"]
    #[inline(always)]
    pub fn cnf1(&self) -> Cnf1R {
        Cnf1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 2 mode bits"]
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 2 configuration bits"]
    #[inline(always)]
    pub fn cnf2(&self) -> Cnf2R {
        Cnf2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 3 mode bits"]
    #[inline(always)]
    pub fn mode3(&self) -> Mode3R {
        Mode3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 3 configuration bits"]
    #[inline(always)]
    pub fn cnf3(&self) -> Cnf3R {
        Cnf3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 4 mode bits"]
    #[inline(always)]
    pub fn mode4(&self) -> Mode4R {
        Mode4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 4 configuration bits"]
    #[inline(always)]
    pub fn cnf4(&self) -> Cnf4R {
        Cnf4R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 5 mode bits"]
    #[inline(always)]
    pub fn mode5(&self) -> Mode5R {
        Mode5R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 5 configuration bits"]
    #[inline(always)]
    pub fn cnf5(&self) -> Cnf5R {
        Cnf5R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port 6 mode bits"]
    #[inline(always)]
    pub fn mode6(&self) -> Mode6R {
        Mode6R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 6 configuration bits"]
    #[inline(always)]
    pub fn cnf6(&self) -> Cnf6R {
        Cnf6R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port 7 mode bits"]
    #[inline(always)]
    pub fn mode7(&self) -> Mode7R {
        Mode7R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 7 configuration bits"]
    #[inline(always)]
    pub fn cnf7(&self) -> Cnf7R {
        Cnf7R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 0 mode bits"]
    #[inline(always)]
    pub fn mode0(&mut self) -> Mode0W<'_, CrlSpec> {
        Mode0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 0 configuration bits"]
    #[inline(always)]
    pub fn cnf0(&mut self) -> Cnf0W<'_, CrlSpec> {
        Cnf0W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 1 mode bits"]
    #[inline(always)]
    pub fn mode1(&mut self) -> Mode1W<'_, CrlSpec> {
        Mode1W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 1 configuration bits"]
    #[inline(always)]
    pub fn cnf1(&mut self) -> Cnf1W<'_, CrlSpec> {
        Cnf1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 2 mode bits"]
    #[inline(always)]
    pub fn mode2(&mut self) -> Mode2W<'_, CrlSpec> {
        Mode2W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 2 configuration bits"]
    #[inline(always)]
    pub fn cnf2(&mut self) -> Cnf2W<'_, CrlSpec> {
        Cnf2W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 3 mode bits"]
    #[inline(always)]
    pub fn mode3(&mut self) -> Mode3W<'_, CrlSpec> {
        Mode3W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 3 configuration bits"]
    #[inline(always)]
    pub fn cnf3(&mut self) -> Cnf3W<'_, CrlSpec> {
        Cnf3W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 4 mode bits"]
    #[inline(always)]
    pub fn mode4(&mut self) -> Mode4W<'_, CrlSpec> {
        Mode4W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 4 configuration bits"]
    #[inline(always)]
    pub fn cnf4(&mut self) -> Cnf4W<'_, CrlSpec> {
        Cnf4W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 5 mode bits"]
    #[inline(always)]
    pub fn mode5(&mut self) -> Mode5W<'_, CrlSpec> {
        Mode5W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port 5 configuration bits"]
    #[inline(always)]
    pub fn cnf5(&mut self) -> Cnf5W<'_, CrlSpec> {
        Cnf5W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port 6 mode bits"]
    #[inline(always)]
    pub fn mode6(&mut self) -> Mode6W<'_, CrlSpec> {
        Mode6W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port 6 configuration bits"]
    #[inline(always)]
    pub fn cnf6(&mut self) -> Cnf6W<'_, CrlSpec> {
        Cnf6W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port 7 mode bits"]
    #[inline(always)]
    pub fn mode7(&mut self) -> Mode7W<'_, CrlSpec> {
        Mode7W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port 7 configuration bits"]
    #[inline(always)]
    pub fn cnf7(&mut self) -> Cnf7W<'_, CrlSpec> {
        Cnf7W::new(self, 30)
    }
}
#[doc = "configuration low register\n\nYou can [`read`](crate::Reg::read) this register and get [`crl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrlSpec;
impl crate::RegisterSpec for CrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crl::R`](R) reader structure"]
impl crate::Readable for CrlSpec {}
#[doc = "`write(|w| ..)` method takes [`crl::W`](W) writer structure"]
impl crate::Writable for CrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRL to value 0x4444_4844"]
impl crate::Resettable for CrlSpec {
    const RESET_VALUE: u32 = 0x4444_4844;
}
