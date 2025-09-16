#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `IMR` reader - Specific bit description shield RAWISR"]
pub type ImrR = crate::FieldReader<u16>;
#[doc = "Field `IMR` writer - Specific bit description shield RAWISR"]
pub type ImrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Specific bit description shield RAWISR"]
    #[inline(always)]
    pub fn imr(&self) -> ImrR {
        ImrR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Specific bit description shield RAWISR"]
    #[inline(always)]
    pub fn imr(&mut self) -> ImrW<'_, ImrSpec> {
        ImrW::new(self, 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR to value 0x08ff"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0x08ff;
}
