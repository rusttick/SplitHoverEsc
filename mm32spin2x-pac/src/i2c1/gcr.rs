#[doc = "Register `GCR` reader"]
pub type R = crate::R<GcrSpec>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GcrSpec>;
#[doc = "Field `GC` reader - ACK general call"]
pub type GcR = crate::BitReader;
#[doc = "Field `GC` writer - ACK general call"]
pub type GcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ACK general call"]
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACK general call"]
    #[inline(always)]
    pub fn gc(&mut self) -> GcW<'_, GcrSpec> {
        GcW::new(self, 0)
    }
}
#[doc = "ACK General Call Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcrSpec;
impl crate::RegisterSpec for GcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GcrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCR to value 0x01"]
impl crate::Resettable for GcrSpec {
    const RESET_VALUE: u32 = 0x01;
}
