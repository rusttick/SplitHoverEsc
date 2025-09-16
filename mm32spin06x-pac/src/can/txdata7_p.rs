#[doc = "Register `TXDATA7_P` reader"]
pub type R = crate::R<Txdata7PSpec>;
#[doc = "Register `TXDATA7_P` writer"]
pub type W = crate::W<Txdata7PSpec>;
#[doc = "Field `DATA7` reader - DATA7"]
pub type Data7R = crate::FieldReader;
#[doc = "Field `DATA7` writer - DATA7"]
pub type Data7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> Data7R {
        Data7R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA7"]
    #[inline(always)]
    pub fn data7(&mut self) -> Data7W<'_, Txdata7PSpec> {
        Data7W::new(self, 0)
    }
}
#[doc = "Peli TX Data register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata7_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata7_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata7PSpec;
impl crate::RegisterSpec for Txdata7PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata7_p::R`](R) reader structure"]
impl crate::Readable for Txdata7PSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata7_p::W`](W) writer structure"]
impl crate::Writable for Txdata7PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA7_P to value 0"]
impl crate::Resettable for Txdata7PSpec {}
