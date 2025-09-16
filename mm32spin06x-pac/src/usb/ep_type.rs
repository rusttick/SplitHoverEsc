#[doc = "Register `EP_TYPE` reader"]
pub type R = crate::R<EpTypeSpec>;
#[doc = "Register `EP_TYPE` writer"]
pub type W = crate::W<EpTypeSpec>;
#[doc = "Field `EP1_TYPE` reader - Point 1 type"]
pub type Ep1TypeR = crate::BitReader;
#[doc = "Field `EP1_TYPE` writer - Point 1 type"]
pub type Ep1TypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2_TYPE` reader - Point 2 type"]
pub type Ep2TypeR = crate::BitReader;
#[doc = "Field `EP2_TYPE` writer - Point 2 type"]
pub type Ep2TypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_TYPE` reader - Point 3 type"]
pub type Ep3TypeR = crate::BitReader;
#[doc = "Field `EP3_TYPE` writer - Point 3 type"]
pub type Ep3TypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4_TYPE` reader - Point 4 type"]
pub type Ep4TypeR = crate::BitReader;
#[doc = "Field `EP4_TYPE` writer - Point 4 type"]
pub type Ep4TypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Point 1 type"]
    #[inline(always)]
    pub fn ep1_type(&self) -> Ep1TypeR {
        Ep1TypeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Point 2 type"]
    #[inline(always)]
    pub fn ep2_type(&self) -> Ep2TypeR {
        Ep2TypeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Point 3 type"]
    #[inline(always)]
    pub fn ep3_type(&self) -> Ep3TypeR {
        Ep3TypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Point 4 type"]
    #[inline(always)]
    pub fn ep4_type(&self) -> Ep4TypeR {
        Ep4TypeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Point 1 type"]
    #[inline(always)]
    pub fn ep1_type(&mut self) -> Ep1TypeW<'_, EpTypeSpec> {
        Ep1TypeW::new(self, 0)
    }
    #[doc = "Bit 1 - Point 2 type"]
    #[inline(always)]
    pub fn ep2_type(&mut self) -> Ep2TypeW<'_, EpTypeSpec> {
        Ep2TypeW::new(self, 1)
    }
    #[doc = "Bit 2 - Point 3 type"]
    #[inline(always)]
    pub fn ep3_type(&mut self) -> Ep3TypeW<'_, EpTypeSpec> {
        Ep3TypeW::new(self, 2)
    }
    #[doc = "Bit 3 - Point 4 type"]
    #[inline(always)]
    pub fn ep4_type(&mut self) -> Ep4TypeW<'_, EpTypeSpec> {
        Ep4TypeW::new(self, 3)
    }
}
#[doc = "Endpoint type register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpTypeSpec;
impl crate::RegisterSpec for EpTypeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep_type::R`](R) reader structure"]
impl crate::Readable for EpTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_type::W`](W) writer structure"]
impl crate::Writable for EpTypeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP_TYPE to value 0"]
impl crate::Resettable for EpTypeSpec {}
