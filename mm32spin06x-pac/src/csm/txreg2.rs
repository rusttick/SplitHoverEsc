#[doc = "Register `TXREG2` reader"]
pub type R = crate::R<Txreg2Spec>;
#[doc = "Register `TXREG2` writer"]
pub type W = crate::W<Txreg2Spec>;
#[doc = "Field `TXREG2` reader - Transfer data"]
pub type Txreg2R = crate::FieldReader<u32>;
#[doc = "Field `TXREG2` writer - Transfer data"]
pub type Txreg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transfer data"]
    #[inline(always)]
    pub fn txreg2(&self) -> Txreg2R {
        Txreg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transfer data"]
    #[inline(always)]
    pub fn txreg2(&mut self) -> Txreg2W<'_, Txreg2Spec> {
        Txreg2W::new(self, 0)
    }
}
#[doc = "Transmit register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`txreg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txreg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txreg2Spec;
impl crate::RegisterSpec for Txreg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txreg2::R`](R) reader structure"]
impl crate::Readable for Txreg2Spec {}
#[doc = "`write(|w| ..)` method takes [`txreg2::W`](W) writer structure"]
impl crate::Writable for Txreg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXREG2 to value 0"]
impl crate::Resettable for Txreg2Spec {}
