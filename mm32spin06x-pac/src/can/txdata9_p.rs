#[doc = "Register `TXDATA9_P` reader"]
pub type R = crate::R<Txdata9PSpec>;
#[doc = "Register `TXDATA9_P` writer"]
pub type W = crate::W<Txdata9PSpec>;
#[doc = "Field `DATA9` reader - DATA9"]
pub type Data9R = crate::FieldReader;
#[doc = "Field `DATA9` writer - DATA9"]
pub type Data9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA9"]
    #[inline(always)]
    pub fn data9(&self) -> Data9R {
        Data9R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA9"]
    #[inline(always)]
    pub fn data9(&mut self) -> Data9W<'_, Txdata9PSpec> {
        Data9W::new(self, 0)
    }
}
#[doc = "Peli TX Data register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata9_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata9_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata9PSpec;
impl crate::RegisterSpec for Txdata9PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata9_p::R`](R) reader structure"]
impl crate::Readable for Txdata9PSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata9_p::W`](W) writer structure"]
impl crate::Writable for Txdata9PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA9_P to value 0"]
impl crate::Resettable for Txdata9PSpec {}
