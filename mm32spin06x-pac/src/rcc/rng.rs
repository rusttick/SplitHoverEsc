#[doc = "Register `RNG` reader"]
pub type R = crate::R<RngSpec>;
#[doc = "Register `RNG` writer"]
pub type W = crate::W<RngSpec>;
#[doc = "Field `RNG_EN` reader - RNG enable"]
pub type RngEnR = crate::BitReader;
#[doc = "Field `RNG_EN` writer - RNG enable"]
pub type RngEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_LDSD` reader - RNG load seed"]
pub type RngLdsdR = crate::BitReader;
#[doc = "Field `RNG_LDSD` writer - RNG load seed"]
pub type RngLdsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_DONE` reader - RNG done"]
pub type RngDoneR = crate::BitReader;
#[doc = "Field `RNG_DONE` writer - RNG done"]
pub type RngDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_DATA` reader - RNG data"]
pub type RngDataR = crate::FieldReader<u16>;
#[doc = "Field `RNG_DATA` writer - RNG data"]
pub type RngDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - RNG enable"]
    #[inline(always)]
    pub fn rng_en(&self) -> RngEnR {
        RngEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RNG load seed"]
    #[inline(always)]
    pub fn rng_ldsd(&self) -> RngLdsdR {
        RngLdsdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RNG done"]
    #[inline(always)]
    pub fn rng_done(&self) -> RngDoneR {
        RngDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:31 - RNG data"]
    #[inline(always)]
    pub fn rng_data(&self) -> RngDataR {
        RngDataR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - RNG enable"]
    #[inline(always)]
    pub fn rng_en(&mut self) -> RngEnW<'_, RngSpec> {
        RngEnW::new(self, 0)
    }
    #[doc = "Bit 1 - RNG load seed"]
    #[inline(always)]
    pub fn rng_ldsd(&mut self) -> RngLdsdW<'_, RngSpec> {
        RngLdsdW::new(self, 1)
    }
    #[doc = "Bit 2 - RNG done"]
    #[inline(always)]
    pub fn rng_done(&mut self) -> RngDoneW<'_, RngSpec> {
        RngDoneW::new(self, 2)
    }
    #[doc = "Bits 16:31 - RNG data"]
    #[inline(always)]
    pub fn rng_data(&mut self) -> RngDataW<'_, RngSpec> {
        RngDataW::new(self, 16)
    }
}
#[doc = "Random number register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngSpec;
impl crate::RegisterSpec for RngSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng::R`](R) reader structure"]
impl crate::Readable for RngSpec {}
#[doc = "`write(|w| ..)` method takes [`rng::W`](W) writer structure"]
impl crate::Writable for RngSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG to value 0"]
impl crate::Resettable for RngSpec {}
