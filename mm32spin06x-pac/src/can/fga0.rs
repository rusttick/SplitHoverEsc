#[doc = "Register `FGA0` reader"]
pub type R = crate::R<Fga0Spec>;
#[doc = "Register `FGA0` writer"]
pub type W = crate::W<Fga0Spec>;
#[doc = "Field `FGA_7_0` reader - Filter group enable"]
pub type Fga7_0R = crate::FieldReader;
#[doc = "Field `FGA_7_0` writer - Filter group enable"]
pub type Fga7_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Filter group enable"]
    #[inline(always)]
    pub fn fga_7_0(&self) -> Fga7_0R {
        Fga7_0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter group enable"]
    #[inline(always)]
    pub fn fga_7_0(&mut self) -> Fga7_0W<'_, Fga0Spec> {
        Fga7_0W::new(self, 0)
    }
}
#[doc = "Filter Group Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fga0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fga0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fga0Spec;
impl crate::RegisterSpec for Fga0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fga0::R`](R) reader structure"]
impl crate::Readable for Fga0Spec {}
#[doc = "`write(|w| ..)` method takes [`fga0::W`](W) writer structure"]
impl crate::Writable for Fga0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FGA0 to value 0"]
impl crate::Resettable for Fga0Spec {}
