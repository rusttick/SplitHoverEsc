#[doc = "Register `CRH` reader"]
pub type R = crate::R<CrhSpec>;
#[doc = "Register `CRH` writer"]
pub type W = crate::W<CrhSpec>;
#[doc = "Field `MODE8` reader - Port 8 mode bits"]
pub type Mode8R = crate::FieldReader;
#[doc = "Field `MODE8` writer - Port 8 mode bits"]
pub type Mode8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF8` reader - Port 8 configuration bits"]
pub type Cnf8R = crate::FieldReader;
#[doc = "Field `CNF8` writer - Port 8 configuration bits"]
pub type Cnf8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE9` reader - Port 9 mode bits"]
pub type Mode9R = crate::FieldReader;
#[doc = "Field `MODE9` writer - Port 9 mode bits"]
pub type Mode9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF9` reader - Port 9 configuration bits"]
pub type Cnf9R = crate::FieldReader;
#[doc = "Field `CNF9` writer - Port 9 configuration bits"]
pub type Cnf9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE10` reader - Port 10 mode bits"]
pub type Mode10R = crate::FieldReader;
#[doc = "Field `MODE10` writer - Port 10 mode bits"]
pub type Mode10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF10` reader - Port 10 configuration bits"]
pub type Cnf10R = crate::FieldReader;
#[doc = "Field `CNF10` writer - Port 10 configuration bits"]
pub type Cnf10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE11` reader - Port 11 mode bits"]
pub type Mode11R = crate::FieldReader;
#[doc = "Field `MODE11` writer - Port 11 mode bits"]
pub type Mode11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF11` reader - Port 11 configuration bits"]
pub type Cnf11R = crate::FieldReader;
#[doc = "Field `CNF11` writer - Port 11 configuration bits"]
pub type Cnf11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE12` reader - Port 12 mode bits"]
pub type Mode12R = crate::FieldReader;
#[doc = "Field `MODE12` writer - Port 12 mode bits"]
pub type Mode12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF12` reader - Port 12 configuration bits"]
pub type Cnf12R = crate::FieldReader;
#[doc = "Field `CNF12` writer - Port 12 configuration bits"]
pub type Cnf12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE13` reader - Port 13 mode bits"]
pub type Mode13R = crate::FieldReader;
#[doc = "Field `MODE13` writer - Port 13 mode bits"]
pub type Mode13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF13` reader - Port 13 configuration bits"]
pub type Cnf13R = crate::FieldReader;
#[doc = "Field `CNF13` writer - Port 13 configuration bits"]
pub type Cnf13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE14` reader - Port 14 mode bits"]
pub type Mode14R = crate::FieldReader;
#[doc = "Field `MODE14` writer - Port 14 mode bits"]
pub type Mode14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF14` reader - Port 14 configuration bits"]
pub type Cnf14R = crate::FieldReader;
#[doc = "Field `CNF14` writer - Port 14 configuration bits"]
pub type Cnf14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE15` reader - Port 15 mode bits"]
pub type Mode15R = crate::FieldReader;
#[doc = "Field `MODE15` writer - Port 15 mode bits"]
pub type Mode15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF15` reader - Port 15 configuration bits"]
pub type Cnf15R = crate::FieldReader;
#[doc = "Field `CNF15` writer - Port 15 configuration bits"]
pub type Cnf15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port 8 mode bits"]
    #[inline(always)]
    pub fn mode8(&self) -> Mode8R {
        Mode8R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 8 configuration bits"]
    #[inline(always)]
    pub fn cnf8(&self) -> Cnf8R {
        Cnf8R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 9 mode bits"]
    #[inline(always)]
    pub fn mode9(&self) -> Mode9R {
        Mode9R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 9 configuration bits"]
    #[inline(always)]
    pub fn cnf9(&self) -> Cnf9R {
        Cnf9R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 10 mode bits"]
    #[inline(always)]
    pub fn mode10(&self) -> Mode10R {
        Mode10R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 10 configuration bits"]
    #[inline(always)]
    pub fn cnf10(&self) -> Cnf10R {
        Cnf10R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 11 mode bits"]
    #[inline(always)]
    pub fn mode11(&self) -> Mode11R {
        Mode11R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 11 configuration bits"]
    #[inline(always)]
    pub fn cnf11(&self) -> Cnf11R {
        Cnf11R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 12 mode bits"]
    #[inline(always)]
    pub fn mode12(&self) -> Mode12R {
        Mode12R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 12 configuration bits"]
    #[inline(always)]
    pub fn cnf12(&self) -> Cnf12R {
        Cnf12R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 13 mode bits"]
    #[inline(always)]
    pub fn mode13(&self) -> Mode13R {
        Mode13R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 13 configuration bits"]
    #[inline(always)]
    pub fn cnf13(&self) -> Cnf13R {
        Cnf13R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port 14 mode bits"]
    #[inline(always)]
    pub fn mode14(&self) -> Mode14R {
        Mode14R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 14 configuration bits"]
    #[inline(always)]
    pub fn cnf14(&self) -> Cnf14R {
        Cnf14R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port 15 mode bits"]
    #[inline(always)]
    pub fn mode15(&self) -> Mode15R {
        Mode15R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 15 configuration bits"]
    #[inline(always)]
    pub fn cnf15(&self) -> Cnf15R {
        Cnf15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 8 mode bits"]
    #[inline(always)]
    pub fn mode8(&mut self) -> Mode8W<'_, CrhSpec> {
        Mode8W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 8 configuration bits"]
    #[inline(always)]
    pub fn cnf8(&mut self) -> Cnf8W<'_, CrhSpec> {
        Cnf8W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 9 mode bits"]
    #[inline(always)]
    pub fn mode9(&mut self) -> Mode9W<'_, CrhSpec> {
        Mode9W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 9 configuration bits"]
    #[inline(always)]
    pub fn cnf9(&mut self) -> Cnf9W<'_, CrhSpec> {
        Cnf9W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 10 mode bits"]
    #[inline(always)]
    pub fn mode10(&mut self) -> Mode10W<'_, CrhSpec> {
        Mode10W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 10 configuration bits"]
    #[inline(always)]
    pub fn cnf10(&mut self) -> Cnf10W<'_, CrhSpec> {
        Cnf10W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 11 mode bits"]
    #[inline(always)]
    pub fn mode11(&mut self) -> Mode11W<'_, CrhSpec> {
        Mode11W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 11 configuration bits"]
    #[inline(always)]
    pub fn cnf11(&mut self) -> Cnf11W<'_, CrhSpec> {
        Cnf11W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 12 mode bits"]
    #[inline(always)]
    pub fn mode12(&mut self) -> Mode12W<'_, CrhSpec> {
        Mode12W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 12 configuration bits"]
    #[inline(always)]
    pub fn cnf12(&mut self) -> Cnf12W<'_, CrhSpec> {
        Cnf12W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 13 mode bits"]
    #[inline(always)]
    pub fn mode13(&mut self) -> Mode13W<'_, CrhSpec> {
        Mode13W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port 13 configuration bits"]
    #[inline(always)]
    pub fn cnf13(&mut self) -> Cnf13W<'_, CrhSpec> {
        Cnf13W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port 14 mode bits"]
    #[inline(always)]
    pub fn mode14(&mut self) -> Mode14W<'_, CrhSpec> {
        Mode14W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port 14 configuration bits"]
    #[inline(always)]
    pub fn cnf14(&mut self) -> Cnf14W<'_, CrhSpec> {
        Cnf14W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port 15 mode bits"]
    #[inline(always)]
    pub fn mode15(&mut self) -> Mode15W<'_, CrhSpec> {
        Mode15W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port 15 configuration bits"]
    #[inline(always)]
    pub fn cnf15(&mut self) -> Cnf15W<'_, CrhSpec> {
        Cnf15W::new(self, 30)
    }
}
#[doc = "configuration high register\n\nYou can [`read`](crate::Reg::read) this register and get [`crh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrhSpec;
impl crate::RegisterSpec for CrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crh::R`](R) reader structure"]
impl crate::Readable for CrhSpec {}
#[doc = "`write(|w| ..)` method takes [`crh::W`](W) writer structure"]
impl crate::Writable for CrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRH to value 0x4444_4484"]
impl crate::Resettable for CrhSpec {
    const RESET_VALUE: u32 = 0x4444_4484;
}
