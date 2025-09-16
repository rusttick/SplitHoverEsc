#[doc = "Register `TXDR2_B` reader"]
pub type R = crate::R<Txdr2BSpec>;
#[doc = "Register `TXDR2_B` writer"]
pub type W = crate::W<Txdr2BSpec>;
#[doc = "Field `TXDR2` reader - Transmit data register"]
pub type Txdr2R = crate::FieldReader<u32>;
#[doc = "Field `TXDR2` writer - Transmit data register"]
pub type Txdr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr2(&self) -> Txdr2R {
        Txdr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr2(&mut self) -> Txdr2W<'_, Txdr2BSpec> {
        Txdr2W::new(self, 0)
    }
}
#[doc = "Basic Send Data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr2_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr2_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdr2BSpec;
impl crate::RegisterSpec for Txdr2BSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr2_b::R`](R) reader structure"]
impl crate::Readable for Txdr2BSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr2_b::W`](W) writer structure"]
impl crate::Writable for Txdr2BSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDR2_B to value 0"]
impl crate::Resettable for Txdr2BSpec {}
