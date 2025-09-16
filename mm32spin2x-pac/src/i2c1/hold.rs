#[doc = "Register `HOLD` reader"]
pub type R = crate::R<HoldSpec>;
#[doc = "Field `TX_HOLD` reader - Sets the required SDA hold time in units of ic_clk period"]
pub type TxHoldR = crate::FieldReader<u16>;
#[doc = "Field `RX_HOLD` reader - Sets the required SDA hold time in units of ic_clk period"]
pub type RxHoldR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    pub fn tx_hold(&self) -> TxHoldR {
        TxHoldR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    pub fn rx_hold(&self) -> RxHoldR {
        RxHoldR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "SDA Hold Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hold::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HoldSpec;
impl crate::RegisterSpec for HoldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hold::R`](R) reader structure"]
impl crate::Readable for HoldSpec {}
#[doc = "`reset()` method sets HOLD to value 0x01"]
impl crate::Resettable for HoldSpec {
    const RESET_VALUE: u32 = 0x01;
}
