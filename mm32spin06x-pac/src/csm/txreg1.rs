#[doc = "Register `TXREG1` reader"]
pub type R = crate::R<Txreg1Spec>;
#[doc = "Register `TXREG1` writer"]
pub type W = crate::W<Txreg1Spec>;
#[doc = "Field `TXREG1` reader - Transfer data"]
pub type Txreg1R = crate::FieldReader<u32>;
#[doc = "Field `TXREG1` writer - Transfer data"]
pub type Txreg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transfer data"]
    #[inline(always)]
    pub fn txreg1(&self) -> Txreg1R {
        Txreg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transfer data"]
    #[inline(always)]
    pub fn txreg1(&mut self) -> Txreg1W<'_, Txreg1Spec> {
        Txreg1W::new(self, 0)
    }
}
#[doc = "Transmit register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txreg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txreg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txreg1Spec;
impl crate::RegisterSpec for Txreg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txreg1::R`](R) reader structure"]
impl crate::Readable for Txreg1Spec {}
#[doc = "`write(|w| ..)` method takes [`txreg1::W`](W) writer structure"]
impl crate::Writable for Txreg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXREG1 to value 0"]
impl crate::Resettable for Txreg1Spec {}
