#[doc = "Register `RD_REQ` reader"]
pub type R = crate::R<RdReqSpec>;
#[doc = "Field `RD_REQ` reader - Read this register to clear the RX_UNDER interrupt(bit 5)of the RAWISR register"]
pub type RdReqR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt(bit 5)of the RAWISR register"]
    #[inline(always)]
    pub fn rd_req(&self) -> RdReqR {
        RdReqR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RD_REQ Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_req::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdReqSpec;
impl crate::RegisterSpec for RdReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_req::R`](R) reader structure"]
impl crate::Readable for RdReqSpec {}
#[doc = "`reset()` method sets RD_REQ to value 0"]
impl crate::Resettable for RdReqSpec {}
