#[doc = "Register `TXDATA4_P` reader"]
pub type R = crate::R<Txdata4PSpec>;
#[doc = "Register `TXDATA4_P` writer"]
pub type W = crate::W<Txdata4PSpec>;
#[doc = "Field `DATA4` reader - DATA4"]
pub type Data4R = crate::FieldReader;
#[doc = "Field `DATA4` writer - DATA4"]
pub type Data4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> Data4R {
        Data4R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&mut self) -> Data4W<'_, Txdata4PSpec> {
        Data4W::new(self, 0)
    }
}
#[doc = "Peli TX Data register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata4_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata4_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txdata4PSpec;
impl crate::RegisterSpec for Txdata4PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata4_p::R`](R) reader structure"]
impl crate::Readable for Txdata4PSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata4_p::W`](W) writer structure"]
impl crate::Writable for Txdata4PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA4_P to value 0"]
impl crate::Resettable for Txdata4PSpec {}
