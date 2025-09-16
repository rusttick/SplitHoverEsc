#[doc = "Register `TXDR7_B` reader"]
pub type R = crate::R<Txdr7BSpec>;
#[doc = "Register `TXDR7_B` writer"]
pub type W = crate::W<Txdr7BSpec>;
#[doc = "Field `TXDR7` reader - Transmit data register"]
pub type Txdr7R = crate::FieldReader<u32>;
#[doc = "Field `TXDR7` writer - Transmit data register"]
pub type Txdr7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr7(&self) -> Txdr7R {
        Txdr7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr7(&mut self) -> Txdr7W<'_, Txdr7BSpec> {
        Txdr7W::new(self, 0)
    }
}
#[doc = "Basic TX Data register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr7_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr7_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdr7BSpec;
impl crate::RegisterSpec for Txdr7BSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr7_b::R`](R) reader structure"]
impl crate::Readable for Txdr7BSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr7_b::W`](W) writer structure"]
impl crate::Writable for Txdr7BSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDR7_B to value 0"]
impl crate::Resettable for Txdr7BSpec {}
