#[doc = "Register `RXTLR` reader"]
pub type R = crate::R<RxtlrSpec>;
#[doc = "Register `RXTLR` writer"]
pub type W = crate::W<RxtlrSpec>;
#[doc = "Field `TL` reader - Receive FIFO threshold level"]
pub type TlR = crate::FieldReader;
#[doc = "Field `TL` writer - Receive FIFO threshold level"]
pub type TlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO threshold level"]
    #[inline(always)]
    pub fn tl(&self) -> TlR {
        TlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive FIFO threshold level"]
    #[inline(always)]
    pub fn tl(&mut self) -> TlW<'_, RxtlrSpec> {
        TlW::new(self, 0)
    }
}
#[doc = "Receive FIFO Threshold Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtlrSpec;
impl crate::RegisterSpec for RxtlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtlr::R`](R) reader structure"]
impl crate::Readable for RxtlrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxtlr::W`](W) writer structure"]
impl crate::Writable for RxtlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXTLR to value 0"]
impl crate::Resettable for RxtlrSpec {}
