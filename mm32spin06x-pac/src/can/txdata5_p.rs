#[doc = "Register `TXDATA5_P` reader"]
pub type R = crate::R<Txdata5PSpec>;
#[doc = "Register `TXDATA5_P` writer"]
pub type W = crate::W<Txdata5PSpec>;
#[doc = "Field `DATA5` reader - DATA5"]
pub type Data5R = crate::FieldReader;
#[doc = "Field `DATA5` writer - DATA5"]
pub type Data5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> Data5R {
        Data5R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA5"]
    #[inline(always)]
    pub fn data5(&mut self) -> Data5W<'_, Txdata5PSpec> {
        Data5W::new(self, 0)
    }
}
#[doc = "Peli TX Data register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata5_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata5_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata5PSpec;
impl crate::RegisterSpec for Txdata5PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata5_p::R`](R) reader structure"]
impl crate::Readable for Txdata5PSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata5_p::W`](W) writer structure"]
impl crate::Writable for Txdata5PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA5_P to value 0"]
impl crate::Resettable for Txdata5PSpec {}
