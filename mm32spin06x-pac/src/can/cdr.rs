#[doc = "Register `CDR` reader"]
pub type R = crate::R<CdrSpec>;
#[doc = "Register `CDR` writer"]
pub type W = crate::W<CdrSpec>;
#[doc = "Field `MODE` reader - CAN mode"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - CAN mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - CAN mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - CAN mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CdrSpec> {
        ModeW::new(self, 7)
    }
}
#[doc = "Clock Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdrSpec;
impl crate::RegisterSpec for CdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr::R`](R) reader structure"]
impl crate::Readable for CdrSpec {}
#[doc = "`write(|w| ..)` method takes [`cdr::W`](W) writer structure"]
impl crate::Writable for CdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CdrSpec {}
