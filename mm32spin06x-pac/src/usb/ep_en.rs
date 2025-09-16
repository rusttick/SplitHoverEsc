#[doc = "Register `EP_EN` reader"]
pub type R = crate::R<EpEnSpec>;
#[doc = "Register `EP_EN` writer"]
pub type W = crate::W<EpEnSpec>;
#[doc = "Field `EP0EN` reader - Enable End Point 0"]
pub type Ep0enR = crate::BitReader;
#[doc = "Field `EP0EN` writer - Enable End Point 0"]
pub type Ep0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1EN` reader - Enable End Point 1"]
pub type Ep1enR = crate::BitReader;
#[doc = "Field `EP1EN` writer - Enable End Point 1"]
pub type Ep1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2EN` reader - Enable End Point 2"]
pub type Ep2enR = crate::BitReader;
#[doc = "Field `EP2EN` writer - Enable End Point 2"]
pub type Ep2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3EN` reader - Enable End Point 3"]
pub type Ep3enR = crate::BitReader;
#[doc = "Field `EP3EN` writer - Enable End Point 3"]
pub type Ep3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4EN` reader - Enable End Point 4"]
pub type Ep4enR = crate::BitReader;
#[doc = "Field `EP4EN` writer - Enable End Point 4"]
pub type Ep4enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable End Point 0"]
    #[inline(always)]
    pub fn ep0en(&self) -> Ep0enR {
        Ep0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable End Point 1"]
    #[inline(always)]
    pub fn ep1en(&self) -> Ep1enR {
        Ep1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable End Point 2"]
    #[inline(always)]
    pub fn ep2en(&self) -> Ep2enR {
        Ep2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable End Point 3"]
    #[inline(always)]
    pub fn ep3en(&self) -> Ep3enR {
        Ep3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable End Point 4"]
    #[inline(always)]
    pub fn ep4en(&self) -> Ep4enR {
        Ep4enR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable End Point 0"]
    #[inline(always)]
    pub fn ep0en(&mut self) -> Ep0enW<'_, EpEnSpec> {
        Ep0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable End Point 1"]
    #[inline(always)]
    pub fn ep1en(&mut self) -> Ep1enW<'_, EpEnSpec> {
        Ep1enW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable End Point 2"]
    #[inline(always)]
    pub fn ep2en(&mut self) -> Ep2enW<'_, EpEnSpec> {
        Ep2enW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable End Point 3"]
    #[inline(always)]
    pub fn ep3en(&mut self) -> Ep3enW<'_, EpEnSpec> {
        Ep3enW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable End Point 4"]
    #[inline(always)]
    pub fn ep4en(&mut self) -> Ep4enW<'_, EpEnSpec> {
        Ep4enW::new(self, 4)
    }
}
#[doc = "EP Enable End\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpEnSpec;
impl crate::RegisterSpec for EpEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep_en::R`](R) reader structure"]
impl crate::Readable for EpEnSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_en::W`](W) writer structure"]
impl crate::Writable for EpEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP_EN to value 0"]
impl crate::Resettable for EpEnSpec {}
