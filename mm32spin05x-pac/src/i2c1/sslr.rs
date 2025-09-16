#[doc = "Register `SSLR` reader"]
pub type R = crate::R<SslrSpec>;
#[doc = "Register `SSLR` writer"]
pub type W = crate::W<SslrSpec>;
#[doc = "Field `CNT` reader - This register sets the SCL clock low period count for standard speed"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - This register sets the SCL clock low period count for standard speed"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, SslrSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "SCL Low Period Count for Std. Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sslr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SslrSpec;
impl crate::RegisterSpec for SslrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sslr::R`](R) reader structure"]
impl crate::Readable for SslrSpec {}
#[doc = "`write(|w| ..)` method takes [`sslr::W`](W) writer structure"]
impl crate::Writable for SslrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSLR to value 0x01d6"]
impl crate::Resettable for SslrSpec {
    const RESET_VALUE: u32 = 0x01d6;
}
