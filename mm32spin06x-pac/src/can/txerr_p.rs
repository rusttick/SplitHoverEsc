#[doc = "Register `TXERR_P` reader"]
pub type R = crate::R<TxerrPSpec>;
#[doc = "Register `TXERR_P` writer"]
pub type W = crate::W<TxerrPSpec>;
#[doc = "Field `TXERR` reader - TX error counter register"]
pub type TxerrR = crate::FieldReader;
#[doc = "Field `TXERR` writer - TX error counter register"]
pub type TxerrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TX error counter register"]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX error counter register"]
    #[inline(always)]
    pub fn txerr(&mut self) -> TxerrW<'_, TxerrPSpec> {
        TxerrW::new(self, 0)
    }
}
#[doc = "Peli TX Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`txerr_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txerr_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxerrPSpec;
impl crate::RegisterSpec for TxerrPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txerr_p::R`](R) reader structure"]
impl crate::Readable for TxerrPSpec {}
#[doc = "`write(|w| ..)` method takes [`txerr_p::W`](W) writer structure"]
impl crate::Writable for TxerrPSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXERR_P to value 0"]
impl crate::Resettable for TxerrPSpec {}
