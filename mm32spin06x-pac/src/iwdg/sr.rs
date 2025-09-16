#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `PVU` reader - Watchdog prescaler value update"]
pub type PvuR = crate::BitReader;
#[doc = "Field `RVU` reader - Watchdog counter reload value update"]
pub type RvuR = crate::BitReader;
#[doc = "Field `IVU` reader - Watchdog interrupt generate value update"]
pub type IvuR = crate::BitReader;
#[doc = "Field `UPDATE` reader - Watchdog reload value update flag"]
pub type UpdateR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Watchdog prescaler value update"]
    #[inline(always)]
    pub fn pvu(&self) -> PvuR {
        PvuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog counter reload value update"]
    #[inline(always)]
    pub fn rvu(&self) -> RvuR {
        RvuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog interrupt generate value update"]
    #[inline(always)]
    pub fn ivu(&self) -> IvuR {
        IvuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog reload value update flag"]
    #[inline(always)]
    pub fn update(&self) -> UpdateR {
        UpdateR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
