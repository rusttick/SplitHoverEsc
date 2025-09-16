#[doc = "Register `TXDR1_B` reader"]
pub type R = crate::R<Txdr1BSpec>;
#[doc = "Register `TXDR1_B` writer"]
pub type W = crate::W<Txdr1BSpec>;
#[doc = "Field `TXDR1` reader - Transmit data register"]
pub type Txdr1R = crate::FieldReader<u32>;
#[doc = "Field `TXDR1` writer - Transmit data register"]
pub type Txdr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr1(&self) -> Txdr1R {
        Txdr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr1(&mut self) -> Txdr1W<'_, Txdr1BSpec> {
        Txdr1W::new(self, 0)
    }
}
#[doc = "Basic TX Data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr1_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr1_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdr1BSpec;
impl crate::RegisterSpec for Txdr1BSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr1_b::R`](R) reader structure"]
impl crate::Readable for Txdr1BSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr1_b::W`](W) writer structure"]
impl crate::Writable for Txdr1BSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDR1_B to value 0"]
impl crate::Resettable for Txdr1BSpec {}
