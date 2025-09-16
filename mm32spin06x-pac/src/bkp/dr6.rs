#[doc = "Register `DR6` reader"]
pub type R = crate::R<Dr6Spec>;
#[doc = "Register `DR6` writer"]
pub type W = crate::W<Dr6Spec>;
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
    pub fn d(&mut self) -> DW<'_, Dr6Spec> {
        DW::new(self, 0)
    }
}
#[doc = "Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr6Spec;
impl crate::RegisterSpec for Dr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr6::R`](R) reader structure"]
impl crate::Readable for Dr6Spec {}
#[doc = "`write(|w| ..)` method takes [`dr6::W`](W) writer structure"]
impl crate::Writable for Dr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR6 to value 0"]
impl crate::Resettable for Dr6Spec {}
