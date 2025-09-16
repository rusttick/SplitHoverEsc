#[doc = "Register `SFF_P` reader"]
pub type R = crate::R<SffPSpec>;
#[doc = "Register `SFF_P` writer"]
pub type W = crate::W<SffPSpec>;
#[doc = "Field `DLC` reader - Data length code bit"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Data length code bit"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTR` reader - Remote transmission request"]
pub type RtrR = crate::BitReader;
#[doc = "Field `RTR` writer - Remote transmission request"]
pub type RtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FF` reader - Frame format"]
pub type FfR = crate::BitReader;
#[doc = "Field `FF` writer - Frame format"]
pub type FfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Data length code bit"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Remote transmission request"]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FfR {
        FfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code bit"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DlcW<'_, SffPSpec> {
        DlcW::new(self, 0)
    }
    #[doc = "Bit 6 - Remote transmission request"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RtrW<'_, SffPSpec> {
        RtrW::new(self, 6)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn ff(&mut self) -> FfW<'_, SffPSpec> {
        FfW::new(self, 7)
    }
}
#[doc = "Peli Send Frame Format register\n\nYou can [`read`](crate::Reg::read) this register and get [`sff_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sff_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SffPSpec;
impl crate::RegisterSpec for SffPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sff_p::R`](R) reader structure"]
impl crate::Readable for SffPSpec {}
#[doc = "`write(|w| ..)` method takes [`sff_p::W`](W) writer structure"]
impl crate::Writable for SffPSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFF_P to value 0"]
impl crate::Resettable for SffPSpec {}
