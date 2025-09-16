#[doc = "Register `SQR` reader"]
pub type R = crate::R<SqrSpec>;
#[doc = "Register `SQR` writer"]
pub type W = crate::W<SqrSpec>;
#[doc = "Field `SQUARE` reader - Square data"]
pub type SquareR = crate::FieldReader<u32>;
#[doc = "Field `SQUARE` writer - Square data"]
pub type SquareW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Square data"]
    #[inline(always)]
    pub fn square(&self) -> SquareR {
        SquareR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Square data"]
    #[inline(always)]
    pub fn square(&mut self) -> SquareW<'_, SqrSpec> {
        SquareW::new(self, 0)
    }
}
#[doc = "SQR\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SqrSpec;
impl crate::RegisterSpec for SqrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr::R`](R) reader structure"]
impl crate::Readable for SqrSpec {}
#[doc = "`write(|w| ..)` method takes [`sqr::W`](W) writer structure"]
impl crate::Writable for SqrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SQR to value 0"]
impl crate::Resettable for SqrSpec {}
