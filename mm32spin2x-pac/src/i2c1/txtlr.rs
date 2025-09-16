#[doc = "Register `TXTLR` reader"]
pub type R = crate::R<TxtlrSpec>;
#[doc = "Field `TL` reader - Transmit FIFO threshold level"]
pub type TlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO threshold level"]
    #[inline(always)]
    pub fn tl(&self) -> TlR {
        TlR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Transmit FIFO Threshold Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txtlr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxtlrSpec;
impl crate::RegisterSpec for TxtlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txtlr::R`](R) reader structure"]
impl crate::Readable for TxtlrSpec {}
#[doc = "`reset()` method sets TXTLR to value 0"]
impl crate::Resettable for TxtlrSpec {}
