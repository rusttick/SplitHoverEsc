#[doc = "Register `TXREG` reader"]
pub type R = crate::R<TxregSpec>;
#[doc = "Register `TXREG` writer"]
pub type W = crate::W<TxregSpec>;
#[doc = "Field `TXREG` reader - Transmit data register"]
pub type TxregR = crate::FieldReader<u32>;
#[doc = "Field `TXREG` writer - Transmit data register"]
pub type TxregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txreg(&self) -> TxregR {
        TxregR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txreg(&mut self) -> TxregW<'_, TxregSpec> {
        TxregW::new(self, 0)
    }
}
#[doc = "TXREG\n\nYou can [`read`](crate::Reg::read) this register and get [`txreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxregSpec;
impl crate::RegisterSpec for TxregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txreg::R`](R) reader structure"]
impl crate::Readable for TxregSpec {}
#[doc = "`write(|w| ..)` method takes [`txreg::W`](W) writer structure"]
impl crate::Writable for TxregSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXREG to value 0"]
impl crate::Resettable for TxregSpec {}
