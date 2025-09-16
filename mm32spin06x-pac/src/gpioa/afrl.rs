#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AfrlSpec>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AfrlSpec>;
#[doc = "Field `AFR0` reader - Multiplexing function selection for bit 0 of portx"]
pub type Afr0R = crate::FieldReader;
#[doc = "Field `AFR0` writer - Multiplexing function selection for bit 0 of portx"]
pub type Afr0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR1` reader - Multiplexing function selection for bit 1 of portx"]
pub type Afr1R = crate::FieldReader;
#[doc = "Field `AFR1` writer - Multiplexing function selection for bit 1 of portx"]
pub type Afr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR2` reader - Multiplexing function selection for bit 2 of portx"]
pub type Afr2R = crate::FieldReader;
#[doc = "Field `AFR2` writer - Multiplexing function selection for bit 2 of portx"]
pub type Afr2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR3` reader - Multiplexing function selection for bit 3 of portx"]
pub type Afr3R = crate::FieldReader;
#[doc = "Field `AFR3` writer - Multiplexing function selection for bit 3 of portx"]
pub type Afr3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR4` reader - Multiplexing function selection for bit 4 of portx"]
pub type Afr4R = crate::FieldReader;
#[doc = "Field `AFR4` writer - Multiplexing function selection for bit 4 of portx"]
pub type Afr4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR5` reader - Multiplexing function selection for bit 5 of portx"]
pub type Afr5R = crate::FieldReader;
#[doc = "Field `AFR5` writer - Multiplexing function selection for bit 5 of portx"]
pub type Afr5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR6` reader - Multiplexing function selection for bit 6 of portx"]
pub type Afr6R = crate::FieldReader;
#[doc = "Field `AFR6` writer - Multiplexing function selection for bit 6 of portx"]
pub type Afr6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR7` reader - Multiplexing function selection for bit 7 of portx"]
pub type Afr7R = crate::FieldReader;
#[doc = "Field `AFR7` writer - Multiplexing function selection for bit 7 of portx"]
pub type Afr7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Multiplexing function selection for bit 0 of portx"]
    #[inline(always)]
    pub fn afr0(&self) -> Afr0R {
        Afr0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Multiplexing function selection for bit 1 of portx"]
    #[inline(always)]
    pub fn afr1(&self) -> Afr1R {
        Afr1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Multiplexing function selection for bit 2 of portx"]
    #[inline(always)]
    pub fn afr2(&self) -> Afr2R {
        Afr2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Multiplexing function selection for bit 3 of portx"]
    #[inline(always)]
    pub fn afr3(&self) -> Afr3R {
        Afr3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Multiplexing function selection for bit 4 of portx"]
    #[inline(always)]
    pub fn afr4(&self) -> Afr4R {
        Afr4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Multiplexing function selection for bit 5 of portx"]
    #[inline(always)]
    pub fn afr5(&self) -> Afr5R {
        Afr5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Multiplexing function selection for bit 6 of portx"]
    #[inline(always)]
    pub fn afr6(&self) -> Afr6R {
        Afr6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Multiplexing function selection for bit 7 of portx"]
    #[inline(always)]
    pub fn afr7(&self) -> Afr7R {
        Afr7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Multiplexing function selection for bit 0 of portx"]
    #[inline(always)]
    pub fn afr0(&mut self) -> Afr0W<'_, AfrlSpec> {
        Afr0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Multiplexing function selection for bit 1 of portx"]
    #[inline(always)]
    pub fn afr1(&mut self) -> Afr1W<'_, AfrlSpec> {
        Afr1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Multiplexing function selection for bit 2 of portx"]
    #[inline(always)]
    pub fn afr2(&mut self) -> Afr2W<'_, AfrlSpec> {
        Afr2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Multiplexing function selection for bit 3 of portx"]
    #[inline(always)]
    pub fn afr3(&mut self) -> Afr3W<'_, AfrlSpec> {
        Afr3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Multiplexing function selection for bit 4 of portx"]
    #[inline(always)]
    pub fn afr4(&mut self) -> Afr4W<'_, AfrlSpec> {
        Afr4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Multiplexing function selection for bit 5 of portx"]
    #[inline(always)]
    pub fn afr5(&mut self) -> Afr5W<'_, AfrlSpec> {
        Afr5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Multiplexing function selection for bit 6 of portx"]
    #[inline(always)]
    pub fn afr6(&mut self) -> Afr6W<'_, AfrlSpec> {
        Afr6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Multiplexing function selection for bit 7 of portx"]
    #[inline(always)]
    pub fn afr7(&mut self) -> Afr7W<'_, AfrlSpec> {
        Afr7W::new(self, 28)
    }
}
#[doc = "Port Multiplexing Function Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrlSpec;
impl crate::RegisterSpec for AfrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AfrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRL to value 0xffff_ffff"]
impl crate::Resettable for AfrlSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
