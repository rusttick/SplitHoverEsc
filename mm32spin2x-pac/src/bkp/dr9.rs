#[doc = "Register `DR9` reader"]
pub type R = crate::R<Dr9Spec>;
#[doc = "Register `DR9` writer"]
pub type W = crate::W<Dr9Spec>;
#[doc = "Field `D` reader - Backup data"]
pub type DR = crate::FieldReader<u16>;
#[doc = "Field `D` writer - Backup data"]
pub type DW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d(&mut self) -> DW<'_, Dr9Spec> {
        DW::new(self, 0)
    }
}
#[doc = "Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr9Spec;
impl crate::RegisterSpec for Dr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr9::R`](R) reader structure"]
impl crate::Readable for Dr9Spec {}
#[doc = "`write(|w| ..)` method takes [`dr9::W`](W) writer structure"]
impl crate::Writable for Dr9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR9 to value 0"]
impl crate::Resettable for Dr9Spec {}
