#[doc = "Register `TXDATA0_P` reader"]
pub type R = crate::R<Txdata0PSpec>;
#[doc = "Register `TXDATA0_P` writer"]
pub type W = crate::W<Txdata0PSpec>;
#[doc = "Field `DATA0` reader - DATA0"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA0` writer - DATA0"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&mut self) -> Data0W<'_, Txdata0PSpec> {
        Data0W::new(self, 0)
    }
}
#[doc = "Peli TX Data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata0_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata0_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata0PSpec;
impl crate::RegisterSpec for Txdata0PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata0_p::R`](R) reader structure"]
impl crate::Readable for Txdata0PSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata0_p::W`](W) writer structure"]
impl crate::Writable for Txdata0PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA0_P to value 0"]
impl crate::Resettable for Txdata0PSpec {}
