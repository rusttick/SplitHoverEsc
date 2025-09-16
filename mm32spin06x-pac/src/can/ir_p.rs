#[doc = "Register `IR_P` reader"]
pub type R = crate::R<IrPSpec>;
#[doc = "Field `RI` reader - Receive interrupt"]
pub type RiR = crate::BitReader;
#[doc = "Field `TI` reader - Transmit interrupt"]
pub type TiR = crate::BitReader;
#[doc = "Field `EI` reader - Error interrupt"]
pub type EiR = crate::BitReader;
#[doc = "Field `DOI` reader - Data overrun interrupt"]
pub type DoiR = crate::BitReader;
#[doc = "Field `EPI` reader - Error passive interrupt"]
pub type EpiR = crate::BitReader;
#[doc = "Field `ALI` reader - Arbitration lost interrupt"]
pub type AliR = crate::BitReader;
#[doc = "Field `BEI` reader - Bus error interrupt"]
pub type BeiR = crate::BitReader;
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
    #[doc = "Bit 5 - Error passive interrupt"]
    #[inline(always)]
    pub fn epi(&self) -> EpiR {
        EpiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Arbitration lost interrupt"]
    #[inline(always)]
    pub fn ali(&self) -> AliR {
        AliR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus error interrupt"]
    #[inline(always)]
    pub fn bei(&self) -> BeiR {
        BeiR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_p::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrPSpec;
impl crate::RegisterSpec for IrPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_p::R`](R) reader structure"]
impl crate::Readable for IrPSpec {}
#[doc = "`reset()` method sets IR_P to value 0"]
impl crate::Resettable for IrPSpec {}
