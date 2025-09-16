#[doc = "Register `TXDR3_B` reader"]
pub type R = crate::R<Txdr3BSpec>;
#[doc = "Register `TXDR3_B` writer"]
pub type W = crate::W<Txdr3BSpec>;
#[doc = "Field `TXDR3` reader - Transmit data register"]
pub type Txdr3R = crate::FieldReader<u32>;
#[doc = "Field `TXDR3` writer - Transmit data register"]
pub type Txdr3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr3(&self) -> Txdr3R {
        Txdr3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr3(&mut self) -> Txdr3W<'_, Txdr3BSpec> {
        Txdr3W::new(self, 0)
    }
}
#[doc = "Basic TX Data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr3_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr3_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdr3BSpec;
impl crate::RegisterSpec for Txdr3BSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr3_b::R`](R) reader structure"]
impl crate::Readable for Txdr3BSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr3_b::W`](W) writer structure"]
impl crate::Writable for Txdr3BSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDR3_B to value 0"]
impl crate::Resettable for Txdr3BSpec {}
