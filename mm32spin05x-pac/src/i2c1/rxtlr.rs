#[doc = "Register `RXTLR` reader"]
pub type R = crate::R<RxtlrSpec>;
#[doc = "Field `TL` reader - Receive FIFO threshold level"]
pub type TlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO threshold level"]
    #[inline(always)]
    pub fn tl(&self) -> TlR {
        TlR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive FIFO Threshold Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtlr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtlrSpec;
impl crate::RegisterSpec for RxtlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtlr::R`](R) reader structure"]
impl crate::Readable for RxtlrSpec {}
#[doc = "`reset()` method sets RXTLR to value 0"]
impl crate::Resettable for RxtlrSpec {}
