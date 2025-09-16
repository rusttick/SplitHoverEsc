#[doc = "Register `TXTLR` reader"]
pub type R = crate::R<TxtlrSpec>;
#[doc = "Register `TXTLR` writer"]
pub type W = crate::W<TxtlrSpec>;
#[doc = "Field `TL` reader - Transmit FIFO threshold level"]
pub type TlR = crate::FieldReader;
#[doc = "Field `TL` writer - Transmit FIFO threshold level"]
pub type TlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO threshold level"]
    #[inline(always)]
    pub fn tl(&self) -> TlR {
        TlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit FIFO threshold level"]
    #[inline(always)]
    pub fn tl(&mut self) -> TlW<'_, TxtlrSpec> {
        TlW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Threshold Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txtlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txtlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxtlrSpec;
impl crate::RegisterSpec for TxtlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txtlr::R`](R) reader structure"]
impl crate::Readable for TxtlrSpec {}
#[doc = "`write(|w| ..)` method takes [`txtlr::W`](W) writer structure"]
impl crate::Writable for TxtlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXTLR to value 0"]
impl crate::Resettable for TxtlrSpec {}
