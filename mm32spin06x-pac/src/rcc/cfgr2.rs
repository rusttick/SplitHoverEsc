#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<Cfgr2Spec>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<Cfgr2Spec>;
#[doc = "Field `TIMADV_CLKSEL` reader - TIMADV clock selection"]
pub type TimadvClkselR = crate::BitReader;
#[doc = "Field `TIMADV_CLKSEL` writer - TIMADV clock selection"]
pub type TimadvClkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMADV_PRE` reader - TIMADV lock prescaler"]
pub type TimadvPreR = crate::FieldReader;
#[doc = "Field `TIMADV_PRE` writer - TIMADV lock prescaler"]
pub type TimadvPreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - TIMADV clock selection"]
    #[inline(always)]
    pub fn timadv_clksel(&self) -> TimadvClkselR {
        TimadvClkselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - TIMADV lock prescaler"]
    #[inline(always)]
    pub fn timadv_pre(&self) -> TimadvPreR {
        TimadvPreR::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TIMADV clock selection"]
    #[inline(always)]
    pub fn timadv_clksel(&mut self) -> TimadvClkselW<'_, Cfgr2Spec> {
        TimadvClkselW::new(self, 0)
    }
    #[doc = "Bits 1:3 - TIMADV lock prescaler"]
    #[inline(always)]
    pub fn timadv_pre(&mut self) -> TimadvPreW<'_, Cfgr2Spec> {
        TimadvPreW::new(self, 1)
    }
}
#[doc = "Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr2Spec;
impl crate::RegisterSpec for Cfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for Cfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for Cfgr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR2 to value 0x20"]
impl crate::Resettable for Cfgr2Spec {
    const RESET_VALUE: u32 = 0x20;
}
