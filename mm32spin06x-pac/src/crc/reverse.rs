#[doc = "Register `REVERSE` reader"]
pub type R = crate::R<ReverseSpec>;
#[doc = "Register `REVERSE` writer"]
pub type W = crate::W<ReverseSpec>;
#[doc = "Field `REVERSE` reader - Data reverse register bits"]
pub type ReverseR = crate::BitReader;
#[doc = "Field `REVERSE` writer - Data reverse register bits"]
pub type ReverseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data reverse register bits"]
    #[inline(always)]
    pub fn reverse(&self) -> ReverseR {
        ReverseR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data reverse register bits"]
    #[inline(always)]
    pub fn reverse(&mut self) -> ReverseW<'_, ReverseSpec> {
        ReverseW::new(self, 0)
    }
}
#[doc = "Reversed data register\n\nYou can [`read`](crate::Reg::read) this register and get [`reverse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reverse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReverseSpec;
impl crate::RegisterSpec for ReverseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reverse::R`](R) reader structure"]
impl crate::Readable for ReverseSpec {}
#[doc = "`write(|w| ..)` method takes [`reverse::W`](W) writer structure"]
impl crate::Writable for ReverseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REVERSE to value 0"]
impl crate::Resettable for ReverseSpec {}
