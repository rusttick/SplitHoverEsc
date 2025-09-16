#[doc = "Register `RCR` reader"]
pub type R = crate::R<RcrSpec>;
#[doc = "Register `RCR` writer"]
pub type W = crate::W<RcrSpec>;
#[doc = "Field `REP` reader - Repetition counter value"]
pub type RepR = crate::FieldReader;
#[doc = "Field `REP` writer - Repetition counter value"]
pub type RepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REP_CNT` reader - Repetition counter value of real-time writing"]
pub type RepCntR = crate::FieldReader;
#[doc = "Field `REP_CNT` writer - Repetition counter value of real-time writing"]
pub type RepCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repetition counter value"]
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Repetition counter value of real-time writing"]
    #[inline(always)]
    pub fn rep_cnt(&self) -> RepCntR {
        RepCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition counter value"]
    #[inline(always)]
    pub fn rep(&mut self) -> RepW<'_, RcrSpec> {
        RepW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Repetition counter value of real-time writing"]
    #[inline(always)]
    pub fn rep_cnt(&mut self) -> RepCntW<'_, RcrSpec> {
        RepCntW::new(self, 8)
    }
}
#[doc = "repetition counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcrSpec;
impl crate::RegisterSpec for RcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::Readable for RcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::Writable for RcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RcrSpec {}
