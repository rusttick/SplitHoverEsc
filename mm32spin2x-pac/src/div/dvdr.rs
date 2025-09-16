#[doc = "Register `DVDR` reader"]
pub type R = crate::R<DvdrSpec>;
#[doc = "Register `DVDR` writer"]
pub type W = crate::W<DvdrSpec>;
#[doc = "Field `DIVIDEND` reader - Dividend data"]
pub type DividendR = crate::FieldReader<u32>;
#[doc = "Field `DIVIDEND` writer - Dividend data"]
pub type DividendW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Dividend data"]
    #[inline(always)]
    pub fn dividend(&self) -> DividendR {
        DividendR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dividend data"]
    #[inline(always)]
    pub fn dividend(&mut self) -> DividendW<'_, DvdrSpec> {
        DividendW::new(self, 0)
    }
}
#[doc = "DVDR\n\nYou can [`read`](crate::Reg::read) this register and get [`dvdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvdrSpec;
impl crate::RegisterSpec for DvdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvdr::R`](R) reader structure"]
impl crate::Readable for DvdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dvdr::W`](W) writer structure"]
impl crate::Writable for DvdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVDR to value 0"]
impl crate::Resettable for DvdrSpec {}
