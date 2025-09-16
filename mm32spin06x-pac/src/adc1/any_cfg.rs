#[doc = "Register `ANY_CFG` reader"]
pub type R = crate::R<AnyCfgSpec>;
#[doc = "Register `ANY_CFG` writer"]
pub type W = crate::W<AnyCfgSpec>;
#[doc = "Field `CHANY_NUM` reader - channel number configuration"]
pub type ChanyNumR = crate::FieldReader;
#[doc = "Field `CHANY_NUM` writer - channel number configuration"]
pub type ChanyNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - channel number configuration"]
    #[inline(always)]
    pub fn chany_num(&self) -> ChanyNumR {
        ChanyNumR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - channel number configuration"]
    #[inline(always)]
    pub fn chany_num(&mut self) -> ChanyNumW<'_, AnyCfgSpec> {
        ChanyNumW::new(self, 0)
    }
}
#[doc = "Arbitrary channel configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`any_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnyCfgSpec;
impl crate::RegisterSpec for AnyCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`any_cfg::R`](R) reader structure"]
impl crate::Readable for AnyCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`any_cfg::W`](W) writer structure"]
impl crate::Writable for AnyCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANY_CFG to value 0"]
impl crate::Resettable for AnyCfgSpec {}
