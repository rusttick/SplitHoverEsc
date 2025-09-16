#[doc = "Register `EP0_INT_EN` reader"]
pub type R = crate::R<Ep0IntEnSpec>;
#[doc = "Register `EP0_INT_EN` writer"]
pub type W = crate::W<Ep0IntEnSpec>;
#[doc = "Field `SETUPIE` reader - SETUP packet interrupt enable"]
pub type SetupieR = crate::BitReader;
#[doc = "Field `SETUPIE` writer - SETUP packet interrupt enable"]
pub type SetupieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDIE` reader - Status stage finished interrupt enable"]
pub type EndieR = crate::BitReader;
#[doc = "Field `ENDIE` writer - Status stage finished interrupt enable"]
pub type EndieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_NACKIE` reader - IN-NACK interrupt enable"]
pub type InNackieR = crate::BitReader;
#[doc = "Field `IN_NACKIE` writer - IN-NACK interrupt enable"]
pub type InNackieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ACKIE` reader - IN-ACK interrupt enable"]
pub type InAckieR = crate::BitReader;
#[doc = "Field `IN_ACKIE` writer - IN-ACK interrupt enable"]
pub type InAckieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_STALLIE` reader - IN-STALL interrupt enable"]
pub type InStallieR = crate::BitReader;
#[doc = "Field `IN_STALLIE` writer - IN-STALL interrupt enable"]
pub type InStallieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_NACKIE` reader - OUT-NACK interrupt enable"]
pub type OutNackieR = crate::BitReader;
#[doc = "Field `OUT_NACKIE` writer - OUT-NACK interrupt enable"]
pub type OutNackieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ACKIE` reader - OUT-ACK interrupt enable"]
pub type OutAckieR = crate::BitReader;
#[doc = "Field `OUT_ACKIE` writer - OUT-ACK interrupt enable"]
pub type OutAckieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_STALLIE` reader - OUT-STALL interrupt enable"]
pub type OutStallieR = crate::BitReader;
#[doc = "Field `OUT_STALLIE` writer - OUT-STALL interrupt enable"]
pub type OutStallieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SETUP packet interrupt enable"]
    #[inline(always)]
    pub fn setupie(&self) -> SetupieR {
        SetupieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status stage finished interrupt enable"]
    #[inline(always)]
    pub fn endie(&self) -> EndieR {
        EndieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IN-NACK interrupt enable"]
    #[inline(always)]
    pub fn in_nackie(&self) -> InNackieR {
        InNackieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IN-ACK interrupt enable"]
    #[inline(always)]
    pub fn in_ackie(&self) -> InAckieR {
        InAckieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN-STALL interrupt enable"]
    #[inline(always)]
    pub fn in_stallie(&self) -> InStallieR {
        InStallieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OUT-NACK interrupt enable"]
    #[inline(always)]
    pub fn out_nackie(&self) -> OutNackieR {
        OutNackieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OUT-ACK interrupt enable"]
    #[inline(always)]
    pub fn out_ackie(&self) -> OutAckieR {
        OutAckieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OUT-STALL interrupt enable"]
    #[inline(always)]
    pub fn out_stallie(&self) -> OutStallieR {
        OutStallieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SETUP packet interrupt enable"]
    #[inline(always)]
    pub fn setupie(&mut self) -> SetupieW<'_, Ep0IntEnSpec> {
        SetupieW::new(self, 0)
    }
    #[doc = "Bit 1 - Status stage finished interrupt enable"]
    #[inline(always)]
    pub fn endie(&mut self) -> EndieW<'_, Ep0IntEnSpec> {
        EndieW::new(self, 1)
    }
    #[doc = "Bit 2 - IN-NACK interrupt enable"]
    #[inline(always)]
    pub fn in_nackie(&mut self) -> InNackieW<'_, Ep0IntEnSpec> {
        InNackieW::new(self, 2)
    }
    #[doc = "Bit 3 - IN-ACK interrupt enable"]
    #[inline(always)]
    pub fn in_ackie(&mut self) -> InAckieW<'_, Ep0IntEnSpec> {
        InAckieW::new(self, 3)
    }
    #[doc = "Bit 4 - IN-STALL interrupt enable"]
    #[inline(always)]
    pub fn in_stallie(&mut self) -> InStallieW<'_, Ep0IntEnSpec> {
        InStallieW::new(self, 4)
    }
    #[doc = "Bit 5 - OUT-NACK interrupt enable"]
    #[inline(always)]
    pub fn out_nackie(&mut self) -> OutNackieW<'_, Ep0IntEnSpec> {
        OutNackieW::new(self, 5)
    }
    #[doc = "Bit 6 - OUT-ACK interrupt enable"]
    #[inline(always)]
    pub fn out_ackie(&mut self) -> OutAckieW<'_, Ep0IntEnSpec> {
        OutAckieW::new(self, 6)
    }
    #[doc = "Bit 7 - OUT-STALL interrupt enable"]
    #[inline(always)]
    pub fn out_stallie(&mut self) -> OutStallieW<'_, Ep0IntEnSpec> {
        OutStallieW::new(self, 7)
    }
}
#[doc = "EP0 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep0IntEnSpec;
impl crate::RegisterSpec for Ep0IntEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep0_int_en::R`](R) reader structure"]
impl crate::Readable for Ep0IntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`ep0_int_en::W`](W) writer structure"]
impl crate::Writable for Ep0IntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP0_INT_EN to value 0"]
impl crate::Resettable for Ep0IntEnSpec {}
