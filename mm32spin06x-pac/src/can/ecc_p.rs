#[doc = "Register `ECC_P` reader"]
pub type R = crate::R<EccPSpec>;
#[doc = "Field `SEG` reader - SEG"]
pub type SegR = crate::FieldReader;
#[doc = "Field `DIR` reader - Direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `ERRC` reader - Error code"]
pub type ErrcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - SEG"]
    #[inline(always)]
    pub fn seg(&self) -> SegR {
        SegR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Error code"]
    #[inline(always)]
    pub fn errc(&self) -> ErrcR {
        ErrcR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Peli Error Code Capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_p::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccPSpec;
impl crate::RegisterSpec for EccPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_p::R`](R) reader structure"]
impl crate::Readable for EccPSpec {}
#[doc = "`reset()` method sets ECC_P to value 0"]
impl crate::Resettable for EccPSpec {}
