#[doc = "Register `IDR` reader"]
pub type R = crate::R<IdrSpec>;
#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `IDR` reader - General-purpose 8-bit data register bits"]
pub type IdrR = crate::FieldReader;
#[doc = "Field `IDR` writer - General-purpose 8-bit data register bits"]
pub type IdrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - General-purpose 8-bit data register bits"]
    #[inline(always)]
    pub fn idr(&self) -> IdrR {
        IdrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - General-purpose 8-bit data register bits"]
    #[inline(always)]
    pub fn idr(&mut self) -> IdrW<'_, IdrSpec> {
        IdrW::new(self, 0)
    }
}
#[doc = "Independent data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IdrSpec {}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
