#[doc = "Register `EP_HALT` reader"]
pub type R = crate::R<EpHaltSpec>;
#[doc = "Register `EP_HALT` writer"]
pub type W = crate::W<EpHaltSpec>;
#[doc = "Field `HALT0` reader - EP0 halt"]
pub type Halt0R = crate::BitReader;
#[doc = "Field `HALT0` writer - EP0 halt"]
pub type Halt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALT1` reader - EP1 halt"]
pub type Halt1R = crate::BitReader;
#[doc = "Field `HALT1` writer - EP1 halt"]
pub type Halt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALT2` reader - EP2 halt"]
pub type Halt2R = crate::BitReader;
#[doc = "Field `HALT2` writer - EP2 halt"]
pub type Halt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALT3` reader - EP3 halt"]
pub type Halt3R = crate::BitReader;
#[doc = "Field `HALT3` writer - EP3 halt"]
pub type Halt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALT4` reader - EP4 halt"]
pub type Halt4R = crate::BitReader;
#[doc = "Field `HALT4` writer - EP4 halt"]
pub type Halt4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EP0 halt"]
    #[inline(always)]
    pub fn halt0(&self) -> Halt0R {
        Halt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EP1 halt"]
    #[inline(always)]
    pub fn halt1(&self) -> Halt1R {
        Halt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EP2 halt"]
    #[inline(always)]
    pub fn halt2(&self) -> Halt2R {
        Halt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EP3 halt"]
    #[inline(always)]
    pub fn halt3(&self) -> Halt3R {
        Halt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP4 halt"]
    #[inline(always)]
    pub fn halt4(&self) -> Halt4R {
        Halt4R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EP0 halt"]
    #[inline(always)]
    pub fn halt0(&mut self) -> Halt0W<'_, EpHaltSpec> {
        Halt0W::new(self, 0)
    }
    #[doc = "Bit 1 - EP1 halt"]
    #[inline(always)]
    pub fn halt1(&mut self) -> Halt1W<'_, EpHaltSpec> {
        Halt1W::new(self, 1)
    }
    #[doc = "Bit 2 - EP2 halt"]
    #[inline(always)]
    pub fn halt2(&mut self) -> Halt2W<'_, EpHaltSpec> {
        Halt2W::new(self, 2)
    }
    #[doc = "Bit 3 - EP3 halt"]
    #[inline(always)]
    pub fn halt3(&mut self) -> Halt3W<'_, EpHaltSpec> {
        Halt3W::new(self, 3)
    }
    #[doc = "Bit 4 - EP4 halt"]
    #[inline(always)]
    pub fn halt4(&mut self) -> Halt4W<'_, EpHaltSpec> {
        Halt4W::new(self, 4)
    }
}
#[doc = "EP HALT register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_halt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_halt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpHaltSpec;
impl crate::RegisterSpec for EpHaltSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep_halt::R`](R) reader structure"]
impl crate::Readable for EpHaltSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_halt::W`](W) writer structure"]
impl crate::Writable for EpHaltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP_HALT to value 0"]
impl crate::Resettable for EpHaltSpec {}
