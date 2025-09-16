#[doc = "Register `EP_INT_EN` reader"]
pub type R = crate::R<EpIntEnSpec>;
#[doc = "Register `EP_INT_EN` writer"]
pub type W = crate::W<EpIntEnSpec>;
#[doc = "Field `EP0IE` reader - EP0 interrupt enable"]
pub type Ep0ieR = crate::BitReader;
#[doc = "Field `EP0IE` writer - EP0 interrupt enable"]
pub type Ep0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1IE` reader - EP1 interrupt enable"]
pub type Ep1ieR = crate::BitReader;
#[doc = "Field `EP1IE` writer - EP1 interrupt enable"]
pub type Ep1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2IE` reader - EP2 interrupt enable"]
pub type Ep2ieR = crate::BitReader;
#[doc = "Field `EP2IE` writer - EP2 interrupt enable"]
pub type Ep2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3IE` reader - EP3 interrupt enable"]
pub type Ep3ieR = crate::BitReader;
#[doc = "Field `EP3IE` writer - EP3 interrupt enable"]
pub type Ep3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4IE` reader - EP4 interrupt enable"]
pub type Ep4ieR = crate::BitReader;
#[doc = "Field `EP4IE` writer - EP4 interrupt enable"]
pub type Ep4ieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EP0 interrupt enable"]
    #[inline(always)]
    pub fn ep0ie(&self) -> Ep0ieR {
        Ep0ieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EP1 interrupt enable"]
    #[inline(always)]
    pub fn ep1ie(&self) -> Ep1ieR {
        Ep1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EP2 interrupt enable"]
    #[inline(always)]
    pub fn ep2ie(&self) -> Ep2ieR {
        Ep2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EP3 interrupt enable"]
    #[inline(always)]
    pub fn ep3ie(&self) -> Ep3ieR {
        Ep3ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP4 interrupt enable"]
    #[inline(always)]
    pub fn ep4ie(&self) -> Ep4ieR {
        Ep4ieR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EP0 interrupt enable"]
    #[inline(always)]
    pub fn ep0ie(&mut self) -> Ep0ieW<'_, EpIntEnSpec> {
        Ep0ieW::new(self, 0)
    }
    #[doc = "Bit 1 - EP1 interrupt enable"]
    #[inline(always)]
    pub fn ep1ie(&mut self) -> Ep1ieW<'_, EpIntEnSpec> {
        Ep1ieW::new(self, 1)
    }
    #[doc = "Bit 2 - EP2 interrupt enable"]
    #[inline(always)]
    pub fn ep2ie(&mut self) -> Ep2ieW<'_, EpIntEnSpec> {
        Ep2ieW::new(self, 2)
    }
    #[doc = "Bit 3 - EP3 interrupt enable"]
    #[inline(always)]
    pub fn ep3ie(&mut self) -> Ep3ieW<'_, EpIntEnSpec> {
        Ep3ieW::new(self, 3)
    }
    #[doc = "Bit 4 - EP4 interrupt enable"]
    #[inline(always)]
    pub fn ep4ie(&mut self) -> Ep4ieW<'_, EpIntEnSpec> {
        Ep4ieW::new(self, 4)
    }
}
#[doc = "EP interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpIntEnSpec;
impl crate::RegisterSpec for EpIntEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep_int_en::R`](R) reader structure"]
impl crate::Readable for EpIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_int_en::W`](W) writer structure"]
impl crate::Writable for EpIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP_INT_EN to value 0"]
impl crate::Resettable for EpIntEnSpec {}
