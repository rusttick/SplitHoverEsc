#[doc = "Register `RXERR_P` reader"]
pub type R = crate::R<RxerrPSpec>;
#[doc = "Register `RXERR_P` writer"]
pub type W = crate::W<RxerrPSpec>;
#[doc = "Field `RXERR` reader - RX error counter register"]
pub type RxerrR = crate::FieldReader;
#[doc = "Field `RXERR` writer - RX error counter register"]
pub type RxerrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RX error counter register"]
    #[inline(always)]
    pub fn rxerr(&self) -> RxerrR {
        RxerrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RX error counter register"]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RxerrW<'_, RxerrPSpec> {
        RxerrW::new(self, 0)
    }
}
#[doc = "Peli RX Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxerr_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxerr_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxerrPSpec;
impl crate::RegisterSpec for RxerrPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxerr_p::R`](R) reader structure"]
impl crate::Readable for RxerrPSpec {}
#[doc = "`write(|w| ..)` method takes [`rxerr_p::W`](W) writer structure"]
impl crate::Writable for RxerrPSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXERR_P to value 0"]
impl crate::Resettable for RxerrPSpec {}
