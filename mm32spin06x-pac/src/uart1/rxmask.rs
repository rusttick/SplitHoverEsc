#[doc = "Register `RXMASK` reader"]
pub type R = crate::R<RxmaskSpec>;
#[doc = "Register `RXMASK` writer"]
pub type W = crate::W<RxmaskSpec>;
#[doc = "Field `RXMASK` reader - Synchronous frame match address mask"]
pub type RxmaskR = crate::FieldReader;
#[doc = "Field `RXMASK` writer - Synchronous frame match address mask"]
pub type RxmaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Synchronous frame match address mask"]
    #[inline(always)]
    pub fn rxmask(&self) -> RxmaskR {
        RxmaskR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronous frame match address mask"]
    #[inline(always)]
    pub fn rxmask(&mut self) -> RxmaskW<'_, RxmaskSpec> {
        RxmaskW::new(self, 0)
    }
}
#[doc = "Receive Mask Registe\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxmaskSpec;
impl crate::RegisterSpec for RxmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmask::R`](R) reader structure"]
impl crate::Readable for RxmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`rxmask::W`](W) writer structure"]
impl crate::Writable for RxmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXMASK to value 0xff"]
impl crate::Resettable for RxmaskSpec {
    const RESET_VALUE: u32 = 0xff;
}
