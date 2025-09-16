#[doc = "Register `ALC_P` reader"]
pub type R = crate::R<AlcPSpec>;
#[doc = "Register `ALC_P` writer"]
pub type W = crate::W<AlcPSpec>;
#[doc = "Field `BITNO` reader - Bit number"]
pub type BitnoR = crate::FieldReader;
#[doc = "Field `BITNO` writer - Bit number"]
pub type BitnoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Bit number"]
    #[inline(always)]
    pub fn bitno(&self) -> BitnoR {
        BitnoR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit number"]
    #[inline(always)]
    pub fn bitno(&mut self) -> BitnoW<'_, AlcPSpec> {
        BitnoW::new(self, 0)
    }
}
#[doc = "Peli Arbitration Lost Capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`alc_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alc_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlcPSpec;
impl crate::RegisterSpec for AlcPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alc_p::R`](R) reader structure"]
impl crate::Readable for AlcPSpec {}
#[doc = "`write(|w| ..)` method takes [`alc_p::W`](W) writer structure"]
impl crate::Writable for AlcPSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALC_P to value 0"]
impl crate::Resettable for AlcPSpec {}
