#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `WINDOW` reader - 7-bit window value"]
pub type WindowR = crate::FieldReader;
#[doc = "Field `WINDOW` writer - 7-bit window value"]
pub type WindowW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WDGTB` reader - Timer base"]
pub type WdgtbR = crate::FieldReader;
#[doc = "Field `WDGTB` writer - Timer base"]
pub type WdgtbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EWI` reader - Early wakeup interrupt"]
pub type EwiR = crate::BitReader;
#[doc = "Field `EWI` writer - Early wakeup interrupt"]
pub type EwiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn window(&self) -> WindowR {
        WindowR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Timer base"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WdgtbR {
        WdgtbR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewi(&self) -> EwiR {
        EwiR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn window(&mut self) -> WindowW<'_, CfgrSpec> {
        WindowW::new(self, 0)
    }
    #[doc = "Bits 7:8 - Timer base"]
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WdgtbW<'_, CfgrSpec> {
        WdgtbW::new(self, 7)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewi(&mut self) -> EwiW<'_, CfgrSpec> {
        EwiW::new(self, 9)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0x7f"]
impl crate::Resettable for CfgrSpec {
    const RESET_VALUE: u32 = 0x7f;
}
