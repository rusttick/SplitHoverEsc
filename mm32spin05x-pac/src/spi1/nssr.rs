#[doc = "Register `NSSR` reader"]
pub type R = crate::R<NssrSpec>;
#[doc = "Register `NSSR` writer"]
pub type W = crate::W<NssrSpec>;
#[doc = "Field `NSS` reader - Chip select output signal in Master mode"]
pub type NssR = crate::BitReader;
#[doc = "Field `NSS` writer - Chip select output signal in Master mode"]
pub type NssW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Chip select output signal in Master mode"]
    #[inline(always)]
    pub fn nss(&self) -> NssR {
        NssR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Chip select output signal in Master mode"]
    #[inline(always)]
    pub fn nss(&mut self) -> NssW<'_, NssrSpec> {
        NssW::new(self, 0)
    }
}
#[doc = "NSSR\n\nYou can [`read`](crate::Reg::read) this register and get [`nssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NssrSpec;
impl crate::RegisterSpec for NssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nssr::R`](R) reader structure"]
impl crate::Readable for NssrSpec {}
#[doc = "`write(|w| ..)` method takes [`nssr::W`](W) writer structure"]
impl crate::Writable for NssrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NSSR to value 0xff"]
impl crate::Resettable for NssrSpec {
    const RESET_VALUE: u32 = 0xff;
}
