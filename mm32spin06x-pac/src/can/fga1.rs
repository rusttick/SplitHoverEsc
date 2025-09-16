#[doc = "Register `FGA1` reader"]
pub type R = crate::R<Fga1Spec>;
#[doc = "Register `FGA1` writer"]
pub type W = crate::W<Fga1Spec>;
#[doc = "Field `FGA_15_8` reader - Filter group enable"]
pub type Fga15_8R = crate::FieldReader;
#[doc = "Field `FGA_15_8` writer - Filter group enable"]
pub type Fga15_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Filter group enable"]
    #[inline(always)]
    pub fn fga_15_8(&self) -> Fga15_8R {
        Fga15_8R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter group enable"]
    #[inline(always)]
    pub fn fga_15_8(&mut self) -> Fga15_8W<'_, Fga1Spec> {
        Fga15_8W::new(self, 0)
    }
}
#[doc = "Filter Group Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fga1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fga1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fga1Spec;
impl crate::RegisterSpec for Fga1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fga1::R`](R) reader structure"]
impl crate::Readable for Fga1Spec {}
#[doc = "`write(|w| ..)` method takes [`fga1::W`](W) writer structure"]
impl crate::Writable for Fga1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FGA1 to value 0"]
impl crate::Resettable for Fga1Spec {}
