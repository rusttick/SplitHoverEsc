#[doc = "Register `TXDR5_B` reader"]
pub type R = crate::R<Txdr5BSpec>;
#[doc = "Register `TXDR5_B` writer"]
pub type W = crate::W<Txdr5BSpec>;
#[doc = "Field `TXDR5` reader - Transmit data register"]
pub type Txdr5R = crate::FieldReader<u32>;
#[doc = "Field `TXDR5` writer - Transmit data register"]
pub type Txdr5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr5(&self) -> Txdr5R {
        Txdr5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr5(&mut self) -> Txdr5W<'_, Txdr5BSpec> {
        Txdr5W::new(self, 0)
    }
}
#[doc = "Basic TX Data register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr5_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr5_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdr5BSpec;
impl crate::RegisterSpec for Txdr5BSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr5_b::R`](R) reader structure"]
impl crate::Readable for Txdr5BSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr5_b::W`](W) writer structure"]
impl crate::Writable for Txdr5BSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDR5_B to value 0"]
impl crate::Resettable for Txdr5BSpec {}
