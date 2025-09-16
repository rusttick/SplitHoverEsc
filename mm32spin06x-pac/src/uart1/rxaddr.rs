#[doc = "Register `RXADDR` reader"]
pub type R = crate::R<RxaddrSpec>;
#[doc = "Register `RXADDR` writer"]
pub type W = crate::W<RxaddrSpec>;
#[doc = "Field `RXADDR` reader - Synchronous frame match address"]
pub type RxaddrR = crate::FieldReader;
#[doc = "Field `RXADDR` writer - Synchronous frame match address"]
pub type RxaddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Synchronous frame match address"]
    #[inline(always)]
    pub fn rxaddr(&self) -> RxaddrR {
        RxaddrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronous frame match address"]
    #[inline(always)]
    pub fn rxaddr(&mut self) -> RxaddrW<'_, RxaddrSpec> {
        RxaddrW::new(self, 0)
    }
}
#[doc = "Receive Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxaddrSpec;
impl crate::RegisterSpec for RxaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxaddr::R`](R) reader structure"]
impl crate::Readable for RxaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxaddr::W`](W) writer structure"]
impl crate::Writable for RxaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXADDR to value 0"]
impl crate::Resettable for RxaddrSpec {}
