#[doc = "Register `TXDR4_B` reader"]
pub type R = crate::R<Txdr4BSpec>;
#[doc = "Register `TXDR4_B` writer"]
pub type W = crate::W<Txdr4BSpec>;
#[doc = "Field `TXDR4` reader - Transmit data register"]
pub type Txdr4R = crate::FieldReader<u32>;
#[doc = "Field `TXDR4` writer - Transmit data register"]
pub type Txdr4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr4(&self) -> Txdr4R {
        Txdr4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr4(&mut self) -> Txdr4W<'_, Txdr4BSpec> {
        Txdr4W::new(self, 0)
    }
}
#[doc = "Basic TX Data register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr4_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr4_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdr4BSpec;
impl crate::RegisterSpec for Txdr4BSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr4_b::R`](R) reader structure"]
impl crate::Readable for Txdr4BSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr4_b::W`](W) writer structure"]
impl crate::Writable for Txdr4BSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDR4_B to value 0"]
impl crate::Resettable for Txdr4BSpec {}
