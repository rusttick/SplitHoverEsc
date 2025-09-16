#[doc = "Register `SLVRCVADDR` reader"]
pub type R = crate::R<SlvrcvaddrSpec>;
#[doc = "Field `ADDR` reader - Slave Address"]
pub type AddrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Slave Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Receiver Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slvrcvaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlvrcvaddrSpec;
impl crate::RegisterSpec for SlvrcvaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvrcvaddr::R`](R) reader structure"]
impl crate::Readable for SlvrcvaddrSpec {}
#[doc = "`reset()` method sets SLVRCVADDR to value 0"]
impl crate::Resettable for SlvrcvaddrSpec {}
