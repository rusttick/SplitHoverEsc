#[doc = "Register `TXDATA3_P` reader"]
pub type R = crate::R<Txdata3PSpec>;
#[doc = "Register `TXDATA3_P` writer"]
pub type W = crate::W<Txdata3PSpec>;
#[doc = "Field `DATA3` reader - DATA3"]
pub type Data3R = crate::FieldReader;
#[doc = "Field `DATA3` writer - DATA3"]
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA3"]
    #[inline(always)]
    pub fn data3(&mut self) -> Data3W<'_, Txdata3PSpec> {
        Data3W::new(self, 0)
    }
}
#[doc = "Peli TX Data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata3_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata3_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata3PSpec;
impl crate::RegisterSpec for Txdata3PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata3_p::R`](R) reader structure"]
impl crate::Readable for Txdata3PSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata3_p::W`](W) writer structure"]
impl crate::Writable for Txdata3PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA3_P to value 0"]
impl crate::Resettable for Txdata3PSpec {}
