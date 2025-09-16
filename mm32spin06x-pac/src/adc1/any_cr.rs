#[doc = "Register `ANY_CR` reader"]
pub type R = crate::R<AnyCrSpec>;
#[doc = "Register `ANY_CR` writer"]
pub type W = crate::W<AnyCrSpec>;
#[doc = "Field `CHANY_MDEN` reader - Any channel configuration mode enable bit"]
pub type ChanyMdenR = crate::BitReader;
#[doc = "Field `CHANY_MDEN` writer - Any channel configuration mode enable bit"]
pub type ChanyMdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Any channel configuration mode enable bit"]
    #[inline(always)]
    pub fn chany_mden(&self) -> ChanyMdenR {
        ChanyMdenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Any channel configuration mode enable bit"]
    #[inline(always)]
    pub fn chany_mden(&mut self) -> ChanyMdenW<'_, AnyCrSpec> {
        ChanyMdenW::new(self, 0)
    }
}
#[doc = "Arbitrary channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`any_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnyCrSpec;
impl crate::RegisterSpec for AnyCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`any_cr::R`](R) reader structure"]
impl crate::Readable for AnyCrSpec {}
#[doc = "`write(|w| ..)` method takes [`any_cr::W`](W) writer structure"]
impl crate::Writable for AnyCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANY_CR to value 0"]
impl crate::Resettable for AnyCrSpec {}
