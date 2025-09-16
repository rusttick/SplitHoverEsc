#[doc = "Register `RXDNR` reader"]
pub type R = crate::R<RxdnrSpec>;
#[doc = "Register `RXDNR` writer"]
pub type W = crate::W<RxdnrSpec>;
#[doc = "Field `RXDNR` reader - The register is used to hold a count of to be received bytes in next receive process"]
pub type RxdnrR = crate::FieldReader<u16>;
#[doc = "Field `RXDNR` writer - The register is used to hold a count of to be received bytes in next receive process"]
pub type RxdnrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The register is used to hold a count of to be received bytes in next receive process"]
    #[inline(always)]
    pub fn rxdnr(&self) -> RxdnrR {
        RxdnrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The register is used to hold a count of to be received bytes in next receive process"]
    #[inline(always)]
    pub fn rxdnr(&mut self) -> RxdnrW<'_, RxdnrSpec> {
        RxdnrW::new(self, 0)
    }
}
#[doc = "RXDNR\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdnrSpec;
impl crate::RegisterSpec for RxdnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdnr::R`](R) reader structure"]
impl crate::Readable for RxdnrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdnr::W`](W) writer structure"]
impl crate::Writable for RxdnrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXDNR to value 0x01"]
impl crate::Resettable for RxdnrSpec {
    const RESET_VALUE: u32 = 0x01;
}
