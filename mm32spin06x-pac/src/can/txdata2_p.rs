#[doc = "Register `TXDATA2_P` reader"]
pub type R = crate::R<Txdata2PSpec>;
#[doc = "Register `TXDATA2_P` writer"]
pub type W = crate::W<Txdata2PSpec>;
#[doc = "Field `DATA2` reader - DATA2"]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA2` writer - DATA2"]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA2"]
    #[inline(always)]
    pub fn data2(&mut self) -> Data2W<'_, Txdata2PSpec> {
        Data2W::new(self, 0)
    }
}
#[doc = "Peli TX Data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata2_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata2_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata2PSpec;
impl crate::RegisterSpec for Txdata2PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata2_p::R`](R) reader structure"]
impl crate::Readable for Txdata2PSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata2_p::W`](W) writer structure"]
impl crate::Writable for Txdata2PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA2_P to value 0"]
impl crate::Resettable for Txdata2PSpec {}
