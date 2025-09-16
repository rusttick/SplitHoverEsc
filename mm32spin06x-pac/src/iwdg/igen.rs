#[doc = "Register `IGEN` reader"]
pub type R = crate::R<IgenSpec>;
#[doc = "Register `IGEN` writer"]
pub type W = crate::W<IgenSpec>;
#[doc = "Field `IGEN` reader - Watchdog Interrupt Generate value"]
pub type IgenR = crate::FieldReader<u16>;
#[doc = "Field `IGEN` writer - Watchdog Interrupt Generate value"]
pub type IgenW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog Interrupt Generate value"]
    #[inline(always)]
    pub fn igen(&self) -> IgenR {
        IgenR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog Interrupt Generate value"]
    #[inline(always)]
    pub fn igen(&mut self) -> IgenW<'_, IgenSpec> {
        IgenW::new(self, 0)
    }
}
#[doc = "Interruput generate value register\n\nYou can [`read`](crate::Reg::read) this register and get [`igen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IgenSpec;
impl crate::RegisterSpec for IgenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`igen::R`](R) reader structure"]
impl crate::Readable for IgenSpec {}
#[doc = "`write(|w| ..)` method takes [`igen::W`](W) writer structure"]
impl crate::Writable for IgenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IGEN to value 0"]
impl crate::Resettable for IgenSpec {}
