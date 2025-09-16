#[doc = "Register `TXDR0_B` reader"]
pub type R = crate::R<Txdr0BSpec>;
#[doc = "Register `TXDR0_B` writer"]
pub type W = crate::W<Txdr0BSpec>;
#[doc = "Field `TXDR0` reader - Transmit data register"]
pub type Txdr0R = crate::FieldReader<u32>;
#[doc = "Field `TXDR0` writer - Transmit data register"]
pub type Txdr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr0(&self) -> Txdr0R {
        Txdr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr0(&mut self) -> Txdr0W<'_, Txdr0BSpec> {
        Txdr0W::new(self, 0)
    }
}
#[doc = "Basic TX Data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr0_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr0_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdr0BSpec;
impl crate::RegisterSpec for Txdr0BSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr0_b::R`](R) reader structure"]
impl crate::Readable for Txdr0BSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr0_b::W`](W) writer structure"]
impl crate::Writable for Txdr0BSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDR0_B to value 0"]
impl crate::Resettable for Txdr0BSpec {}
