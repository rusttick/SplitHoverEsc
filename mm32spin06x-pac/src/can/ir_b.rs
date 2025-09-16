#[doc = "Register `IR_B` reader"]
pub type R = crate::R<IrBSpec>;
#[doc = "Field `RI` reader - Receive interrupt"]
pub type RiR = crate::BitReader;
#[doc = "Field `TI` reader - Transmit interrupt"]
pub type TiR = crate::BitReader;
#[doc = "Field `EI` reader - Error interrupt"]
pub type EiR = crate::BitReader;
#[doc = "Field `DOI` reader - Data overrun interrupt"]
pub type DoiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt"]
    #[inline(always)]
    pub fn ei(&self) -> EiR {
        EiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data overrun interrupt"]
    #[inline(always)]
    pub fn doi(&self) -> DoiR {
        DoiR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_b::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrBSpec;
impl crate::RegisterSpec for IrBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_b::R`](R) reader structure"]
impl crate::Readable for IrBSpec {}
#[doc = "`reset()` method sets IR_B to value 0xe0"]
impl crate::Resettable for IrBSpec {
    const RESET_VALUE: u32 = 0xe0;
}
