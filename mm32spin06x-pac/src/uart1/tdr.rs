#[doc = "Register `TDR` reader"]
pub type R = crate::R<TdrSpec>;
#[doc = "Register `TDR` writer"]
pub type W = crate::W<TdrSpec>;
#[doc = "Field `TXREG` reader - Transmit data register"]
pub type TxregR = crate::FieldReader;
#[doc = "Field `TXREG` writer - Transmit data register"]
pub type TxregW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit data register"]
    #[inline(always)]
    pub fn txreg(&self) -> TxregR {
        TxregR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data register"]
    #[inline(always)]
    pub fn txreg(&mut self) -> TxregW<'_, TdrSpec> {
        TxregW::new(self, 0)
    }
}
#[doc = "Transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdrSpec;
impl crate::RegisterSpec for TdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdr::R`](R) reader structure"]
impl crate::Readable for TdrSpec {}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TdrSpec {}
