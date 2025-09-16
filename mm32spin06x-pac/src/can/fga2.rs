#[doc = "Register `FGA2` reader"]
pub type R = crate::R<Fga2Spec>;
#[doc = "Register `FGA2` writer"]
pub type W = crate::W<Fga2Spec>;
#[doc = "Field `FGA_19_16` reader - Filter group enable"]
pub type Fga19_16R = crate::FieldReader;
#[doc = "Field `FGA_19_16` writer - Filter group enable"]
pub type Fga19_16W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Filter group enable"]
    #[inline(always)]
    pub fn fga_19_16(&self) -> Fga19_16R {
        Fga19_16R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Filter group enable"]
    #[inline(always)]
    pub fn fga_19_16(&mut self) -> Fga19_16W<'_, Fga2Spec> {
        Fga19_16W::new(self, 0)
    }
}
#[doc = "Filter Group Enable Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fga2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fga2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fga2Spec;
impl crate::RegisterSpec for Fga2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fga2::R`](R) reader structure"]
impl crate::Readable for Fga2Spec {}
#[doc = "`write(|w| ..)` method takes [`fga2::W`](W) writer structure"]
impl crate::Writable for Fga2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FGA2 to value 0"]
impl crate::Resettable for Fga2Spec {}
