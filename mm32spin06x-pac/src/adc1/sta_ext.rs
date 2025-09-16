#[doc = "Register `STA_EXT` reader"]
pub type R = crate::R<StaExtSpec>;
#[doc = "Field `VALID` reader - Valid flag"]
pub type ValidR = crate::FieldReader;
#[doc = "Field `OVERRUN` reader - Overrun flag"]
pub type OverrunR = crate::FieldReader;
impl R {
    #[doc = "Bits 2:3 - Valid flag"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Overrun flag"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Extended status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sta_ext::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StaExtSpec;
impl crate::RegisterSpec for StaExtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta_ext::R`](R) reader structure"]
impl crate::Readable for StaExtSpec {}
#[doc = "`reset()` method sets STA_EXT to value 0"]
impl crate::Resettable for StaExtSpec {}
