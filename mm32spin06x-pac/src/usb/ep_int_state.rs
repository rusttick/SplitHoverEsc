#[doc = "Register `EP_INT_STATE` reader"]
pub type R = crate::R<EpIntStateSpec>;
#[doc = "Field `EP0F` reader - EP0 interrupt received"]
pub type Ep0fR = crate::BitReader;
#[doc = "Field `EP1F` reader - EP1 interrupt received"]
pub type Ep1fR = crate::BitReader;
#[doc = "Field `EP2F` reader - EP2 interrupt received"]
pub type Ep2fR = crate::BitReader;
#[doc = "Field `EP3F` reader - EP3 interrupt received"]
pub type Ep3fR = crate::BitReader;
#[doc = "Field `EP4F` reader - EP4 interrupt received"]
pub type Ep4fR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EP0 interrupt received"]
    #[inline(always)]
    pub fn ep0f(&self) -> Ep0fR {
        Ep0fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EP1 interrupt received"]
    #[inline(always)]
    pub fn ep1f(&self) -> Ep1fR {
        Ep1fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EP2 interrupt received"]
    #[inline(always)]
    pub fn ep2f(&self) -> Ep2fR {
        Ep2fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EP3 interrupt received"]
    #[inline(always)]
    pub fn ep3f(&self) -> Ep3fR {
        Ep3fR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP4 interrupt received"]
    #[inline(always)]
    pub fn ep4f(&self) -> Ep4fR {
        Ep4fR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "EP interrupt state register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_int_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpIntStateSpec;
impl crate::RegisterSpec for EpIntStateSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep_int_state::R`](R) reader structure"]
impl crate::Readable for EpIntStateSpec {}
#[doc = "`reset()` method sets EP_INT_STATE to value 0"]
impl crate::Resettable for EpIntStateSpec {}
