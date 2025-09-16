#[doc = "Register `RDNR` reader"]
pub type R = crate::R<RdnrSpec>;
#[doc = "Register `RDNR` writer"]
pub type W = crate::W<RdnrSpec>;
#[doc = "Field `RDN` reader - The register is used to hold a count of to be received bytes in next receive process"]
pub type RdnR = crate::FieldReader<u16>;
#[doc = "Field `RDN` writer - The register is used to hold a count of to be received bytes in next receive process"]
pub type RdnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The register is used to hold a count of to be received bytes in next receive process"]
    #[inline(always)]
    pub fn rdn(&self) -> RdnR {
        RdnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The register is used to hold a count of to be received bytes in next receive process"]
    #[inline(always)]
    pub fn rdn(&mut self) -> RdnW<'_, RdnrSpec> {
        RdnW::new(self, 0)
    }
}
#[doc = "RDNR\n\nYou can [`read`](crate::Reg::read) this register and get [`rdnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdnrSpec;
impl crate::RegisterSpec for RdnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdnr::R`](R) reader structure"]
impl crate::Readable for RdnrSpec {}
#[doc = "`write(|w| ..)` method takes [`rdnr::W`](W) writer structure"]
impl crate::Writable for RdnrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RDNR to value 0x01"]
impl crate::Resettable for RdnrSpec {
    const RESET_VALUE: u32 = 0x01;
}
