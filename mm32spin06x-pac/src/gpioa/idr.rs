#[doc = "Register `IDR` reader"]
pub type R = crate::R<IdrSpec>;
#[doc = "Field `IDR` reader - Port input data"]
pub type IdrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Port input data"]
    #[inline(always)]
    pub fn idr(&self) -> IdrR {
        IdrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IdrSpec {}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
