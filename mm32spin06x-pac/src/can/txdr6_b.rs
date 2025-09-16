#[doc = "Register `TXDR6_B` reader"]
pub type R = crate::R<Txdr6BSpec>;
#[doc = "Register `TXDR6_B` writer"]
pub type W = crate::W<Txdr6BSpec>;
#[doc = "Field `TXDR6` reader - Transmit data register"]
pub type Txdr6R = crate::FieldReader<u32>;
#[doc = "Field `TXDR6` writer - Transmit data register"]
pub type Txdr6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr6(&self) -> Txdr6R {
        Txdr6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr6(&mut self) -> Txdr6W<'_, Txdr6BSpec> {
        Txdr6W::new(self, 0)
    }
}
#[doc = "Basic TX Data register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr6_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr6_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdr6BSpec;
impl crate::RegisterSpec for Txdr6BSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr6_b::R`](R) reader structure"]
impl crate::Readable for Txdr6BSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr6_b::W`](W) writer structure"]
impl crate::Writable for Txdr6BSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDR6_B to value 0"]
impl crate::Resettable for Txdr6BSpec {}
