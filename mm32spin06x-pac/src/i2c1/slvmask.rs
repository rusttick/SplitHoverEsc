#[doc = "Register `SLVMASK` reader"]
pub type R = crate::R<SlvmaskSpec>;
#[doc = "Register `SLVMASK` writer"]
pub type W = crate::W<SlvmaskSpec>;
#[doc = "Field `Mask` reader - Slave Address Mask"]
pub type MaskR = crate::FieldReader<u16>;
#[doc = "Field `Mask` writer - Slave Address Mask"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, SlvmaskSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Slave Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slvmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlvmaskSpec;
impl crate::RegisterSpec for SlvmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvmask::R`](R) reader structure"]
impl crate::Readable for SlvmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`slvmask::W`](W) writer structure"]
impl crate::Writable for SlvmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLVMASK to value 0x03ff"]
impl crate::Resettable for SlvmaskSpec {
    const RESET_VALUE: u32 = 0x03ff;
}
