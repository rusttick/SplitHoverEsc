#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `ACTIV` reader - I2C activity status"]
pub type ActivR = crate::BitReader;
#[doc = "Field `TFNF` reader - Transmit FIFO not full"]
pub type TfnfR = crate::BitReader;
#[doc = "Field `TFE` reader - Transmit FIFO completely empty"]
pub type TfeR = crate::BitReader;
#[doc = "Field `RFNE` reader - Receive FIFO not empty"]
pub type RfneR = crate::BitReader;
#[doc = "Field `RFF` reader - Receive FIFO completely full"]
pub type RffR = crate::BitReader;
#[doc = "Field `MST_ACTIV` reader - Master FSM activity status"]
pub type MstActivR = crate::BitReader;
#[doc = "Field `SLV_ACTIV` reader - Slave FSM activity status"]
pub type SlvActivR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I2C activity status"]
    #[inline(always)]
    pub fn activ(&self) -> ActivR {
        ActivR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full"]
    #[inline(always)]
    pub fn tfnf(&self) -> TfnfR {
        TfnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO completely empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO not empty"]
    #[inline(always)]
    pub fn rfne(&self) -> RfneR {
        RfneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO completely full"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master FSM activity status"]
    #[inline(always)]
    pub fn mst_activ(&self) -> MstActivR {
        MstActivR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slave FSM activity status"]
    #[inline(always)]
    pub fn slv_activ(&self) -> SlvActivR {
        SlvActivR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x06"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x06;
}
