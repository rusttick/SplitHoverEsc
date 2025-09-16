#[doc = "Register `RAWISR` reader"]
pub type R = crate::R<RawisrSpec>;
#[doc = "Field `RX_UNDER` reader - Receive buffer under"]
pub type RxUnderR = crate::BitReader;
#[doc = "Field `RX_OVER` reader - Receive buffer over"]
pub type RxOverR = crate::BitReader;
#[doc = "Field `RX_FULL` reader - Receive buffer not empty"]
pub type RxFullR = crate::BitReader;
#[doc = "Field `TX_OVER` reader - Transmit buffer over"]
pub type TxOverR = crate::BitReader;
#[doc = "Field `TX_EMPTY` reader - Transmit buffer empty"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `RD_REQ` reader - Read request"]
pub type RdReqR = crate::BitReader;
#[doc = "Field `TX_ABRT` reader - Transmit abort"]
pub type TxAbrtR = crate::BitReader;
#[doc = "Field `RX_DONE` reader - Transmit done"]
pub type RxDoneR = crate::BitReader;
#[doc = "Field `ACTIV` reader - This bit captures DW_spb_i2c acticity and stays set until it is cleared"]
pub type ActivR = crate::BitReader;
#[doc = "Field `STOP` reader - Stop condition detection"]
pub type StopR = crate::BitReader;
#[doc = "Field `START` reader - Start condition detection"]
pub type StartR = crate::BitReader;
#[doc = "Field `GC` reader - General call"]
pub type GcR = crate::BitReader;
#[doc = "Field `RESTART` reader - RESTART_DET interrupt status"]
pub type RestartR = crate::BitReader;
#[doc = "Field `HOLD` reader - MST_ON_HOLD interrupt status"]
pub type HoldR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive buffer under"]
    #[inline(always)]
    pub fn rx_under(&self) -> RxUnderR {
        RxUnderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive buffer over"]
    #[inline(always)]
    pub fn rx_over(&self) -> RxOverR {
        RxOverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit buffer over"]
    #[inline(always)]
    pub fn tx_over(&self) -> TxOverR {
        TxOverR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit buffer empty"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read request"]
    #[inline(always)]
    pub fn rd_req(&self) -> RdReqR {
        RdReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit abort"]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TxAbrtR {
        TxAbrtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit done"]
    #[inline(always)]
    pub fn rx_done(&self) -> RxDoneR {
        RxDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit captures DW_spb_i2c acticity and stays set until it is cleared"]
    #[inline(always)]
    pub fn activ(&self) -> ActivR {
        ActivR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop condition detection"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Start condition detection"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - General call"]
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RESTART_DET interrupt status"]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MST_ON_HOLD interrupt status"]
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "RAW Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rawisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawisrSpec;
impl crate::RegisterSpec for RawisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawisr::R`](R) reader structure"]
impl crate::Readable for RawisrSpec {}
#[doc = "`reset()` method sets RAWISR to value 0"]
impl crate::Resettable for RawisrSpec {}
