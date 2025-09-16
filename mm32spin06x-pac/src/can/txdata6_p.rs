#[doc = "Register `TXDATA6_P` reader"]
pub type R = crate::R<Txdata6PSpec>;
#[doc = "Register `TXDATA6_P` writer"]
pub type W = crate::W<Txdata6PSpec>;
#[doc = "Field `DATA6` reader - DATA6"]
pub type Data6R = crate::FieldReader;
#[doc = "Field `DATA6` writer - DATA6"]
pub type Data6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> Data6R {
        Data6R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA6"]
    #[inline(always)]
    pub fn data6(&mut self) -> Data6W<'_, Txdata6PSpec> {
        Data6W::new(self, 0)
    }
}
#[doc = "Peli TX Data register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata6_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata6_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata6PSpec;
impl crate::RegisterSpec for Txdata6PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata6_p::R`](R) reader structure"]
impl crate::Readable for Txdata6PSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata6_p::W`](W) writer structure"]
impl crate::Writable for Txdata6PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA6_P to value 0"]
impl crate::Resettable for Txdata6PSpec {}
