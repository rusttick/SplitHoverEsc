#[doc = "Register `TXDATA1_P` reader"]
pub type R = crate::R<Txdata1PSpec>;
#[doc = "Register `TXDATA1_P` writer"]
pub type W = crate::W<Txdata1PSpec>;
#[doc = "Field `DATA1` reader - DATA1"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - DATA1"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA1"]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<'_, Txdata1PSpec> {
        Data1W::new(self, 0)
    }
}
#[doc = "Peli TX Data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata1_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata1_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata1PSpec;
impl crate::RegisterSpec for Txdata1PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata1_p::R`](R) reader structure"]
impl crate::Readable for Txdata1PSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata1_p::W`](W) writer structure"]
impl crate::Writable for Txdata1PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA1_P to value 0"]
impl crate::Resettable for Txdata1PSpec {}
