#[doc = "Register `TXDATA8_P` reader"]
pub type R = crate::R<Txdata8PSpec>;
#[doc = "Register `TXDATA8_P` writer"]
pub type W = crate::W<Txdata8PSpec>;
#[doc = "Field `DATA8` reader - DATA8"]
pub type Data8R = crate::FieldReader;
#[doc = "Field `DATA8` writer - DATA8"]
pub type Data8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA8"]
    #[inline(always)]
    pub fn data8(&self) -> Data8R {
        Data8R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA8"]
    #[inline(always)]
    pub fn data8(&mut self) -> Data8W<'_, Txdata8PSpec> {
        Data8W::new(self, 0)
    }
}
#[doc = "Peli TX Data register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata8_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata8_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata8PSpec;
impl crate::RegisterSpec for Txdata8PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata8_p::R`](R) reader structure"]
impl crate::Readable for Txdata8PSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata8_p::W`](W) writer structure"]
impl crate::Writable for Txdata8PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA8_P to value 0"]
impl crate::Resettable for Txdata8PSpec {}
