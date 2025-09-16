#[doc = "Register `CH3DR` reader"]
pub type R = crate::R<Ch3drSpec>;
#[doc = "Field `DATA` reader - Transfer data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `OVERRUN` reader - Overrun flag"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `VALID` reader - Valid flag"]
pub type ValidR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Transfer data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 20 - Overrun flag"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Valid flag"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Channel 3 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3drSpec;
impl crate::RegisterSpec for Ch3drSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3dr::R`](R) reader structure"]
impl crate::Readable for Ch3drSpec {}
#[doc = "`reset()` method sets CH3DR to value 0"]
impl crate::Resettable for Ch3drSpec {}
