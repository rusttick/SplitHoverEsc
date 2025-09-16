#[doc = "Register `EP2_INT_STATE` reader"]
pub type R = crate::R<Ep2IntStateSpec>;
#[doc = "Register `EP2_INT_STATE` writer"]
pub type W = crate::W<Ep2IntStateSpec>;
#[doc = "Field `END` reader - Status stage finished"]
pub type EndR = crate::BitReader;
#[doc = "Field `END` writer - Status stage finished"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_NACF` reader - IN-NACK received"]
pub type InNacfR = crate::BitReader;
#[doc = "Field `IN_NACF` writer - IN-NACK received"]
pub type InNacfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ACKF` reader - IN-ACK received"]
pub type InAckfR = crate::BitReader;
#[doc = "Field `IN_ACKF` writer - IN-ACK received"]
pub type InAckfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_STALLF` reader - IN-STALL received"]
pub type InStallfR = crate::BitReader;
#[doc = "Field `IN_STALLF` writer - IN-STALL received"]
pub type InStallfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_NACKF` reader - OUT-NACK received"]
pub type OutNackfR = crate::BitReader;
#[doc = "Field `OUT_NACKF` writer - OUT-NACK received"]
pub type OutNackfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ACKF` reader - OUT-ACK received"]
pub type OutAckfR = crate::BitReader;
#[doc = "Field `OUT_ACKF` writer - OUT-ACK received"]
pub type OutAckfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_STALLF` reader - OUT-STALL received"]
pub type OutStallfR = crate::BitReader;
#[doc = "Field `OUT_STALLF` writer - OUT-STALL received"]
pub type OutStallfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Status stage finished"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IN-NACK received"]
    #[inline(always)]
    pub fn in_nacf(&self) -> InNacfR {
        InNacfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IN-ACK received"]
    #[inline(always)]
    pub fn in_ackf(&self) -> InAckfR {
        InAckfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN-STALL received"]
    #[inline(always)]
    pub fn in_stallf(&self) -> InStallfR {
        InStallfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OUT-NACK received"]
    #[inline(always)]
    pub fn out_nackf(&self) -> OutNackfR {
        OutNackfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OUT-ACK received"]
    #[inline(always)]
    pub fn out_ackf(&self) -> OutAckfR {
        OutAckfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OUT-STALL received"]
    #[inline(always)]
    pub fn out_stallf(&self) -> OutStallfR {
        OutStallfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Status stage finished"]
    #[inline(always)]
    pub fn end(&mut self) -> EndW<'_, Ep2IntStateSpec> {
        EndW::new(self, 1)
    }
    #[doc = "Bit 2 - IN-NACK received"]
    #[inline(always)]
    pub fn in_nacf(&mut self) -> InNacfW<'_, Ep2IntStateSpec> {
        InNacfW::new(self, 2)
    }
    #[doc = "Bit 3 - IN-ACK received"]
    #[inline(always)]
    pub fn in_ackf(&mut self) -> InAckfW<'_, Ep2IntStateSpec> {
        InAckfW::new(self, 3)
    }
    #[doc = "Bit 4 - IN-STALL received"]
    #[inline(always)]
    pub fn in_stallf(&mut self) -> InStallfW<'_, Ep2IntStateSpec> {
        InStallfW::new(self, 4)
    }
    #[doc = "Bit 5 - OUT-NACK received"]
    #[inline(always)]
    pub fn out_nackf(&mut self) -> OutNackfW<'_, Ep2IntStateSpec> {
        OutNackfW::new(self, 5)
    }
    #[doc = "Bit 6 - OUT-ACK received"]
    #[inline(always)]
    pub fn out_ackf(&mut self) -> OutAckfW<'_, Ep2IntStateSpec> {
        OutAckfW::new(self, 6)
    }
    #[doc = "Bit 7 - OUT-STALL received"]
    #[inline(always)]
    pub fn out_stallf(&mut self) -> OutStallfW<'_, Ep2IntStateSpec> {
        OutStallfW::new(self, 7)
    }
}
#[doc = "EP2 interrupt state register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep2_int_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2_int_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep2IntStateSpec;
impl crate::RegisterSpec for Ep2IntStateSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep2_int_state::R`](R) reader structure"]
impl crate::Readable for Ep2IntStateSpec {}
#[doc = "`write(|w| ..)` method takes [`ep2_int_state::W`](W) writer structure"]
impl crate::Writable for Ep2IntStateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP2_INT_STATE to value 0"]
impl crate::Resettable for Ep2IntStateSpec {}
