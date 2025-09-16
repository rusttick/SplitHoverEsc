#[doc = "Register `WRPR` reader"]
pub type R = crate::R<WrprSpec>;
#[doc = "Field `WRP` reader - Write protect"]
pub type WrpR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write protect"]
    #[inline(always)]
    pub fn wrp(&self) -> WrpR {
        WrpR::new(self.bits)
    }
}
#[doc = "Write protect register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrprSpec;
impl crate::RegisterSpec for WrprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpr::R`](R) reader structure"]
impl crate::Readable for WrprSpec {}
#[doc = "`reset()` method sets WRPR to value 0xffff_ffff"]
impl crate::Resettable for WrprSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
