#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "Field `TX_INTF` reader - Transmit FIFO available interrupt flag"]
pub type TxIntfR = crate::BitReader;
#[doc = "Field `TX_INTF` writer - Transmit FIFO available interrupt flag"]
pub type TxIntfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_INTF` reader - Receive data available interrupt flag"]
pub type RxIntfR = crate::BitReader;
#[doc = "Field `RX_INTF` writer - Receive data available interrupt flag"]
pub type RxIntfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC_INTF` reader - Transmit complete interrupt flag"]
pub type TxcIntfR = crate::BitReader;
#[doc = "Field `TXC_INTF` writer - Transmit complete interrupt flag"]
pub type TxcIntfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_INTF` reader - Start Reveive data interrupt flag"]
pub type StartIntfR = crate::BitReader;
#[doc = "Field `START_INTF` writer - Start Reveive data interrupt flag"]
pub type StartIntfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_INTF` reader - Stop Reveive data interrupt flag"]
pub type StopIntfR = crate::BitReader;
#[doc = "Field `STOP_INTF` writer - Stop Reveive data interrupt flag"]
pub type StopIntfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO available interrupt flag"]
    #[inline(always)]
    pub fn tx_intf(&self) -> TxIntfR {
        TxIntfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive data available interrupt flag"]
    #[inline(always)]
    pub fn rx_intf(&self) -> RxIntfR {
        RxIntfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit complete interrupt flag"]
    #[inline(always)]
    pub fn txc_intf(&self) -> TxcIntfR {
        TxcIntfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start Reveive data interrupt flag"]
    #[inline(always)]
    pub fn start_intf(&self) -> StartIntfR {
        StartIntfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop Reveive data interrupt flag"]
    #[inline(always)]
    pub fn stop_intf(&self) -> StopIntfR {
        StopIntfR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO available interrupt flag"]
    #[inline(always)]
    pub fn tx_intf(&mut self) -> TxIntfW<'_, IntstatSpec> {
        TxIntfW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive data available interrupt flag"]
    #[inline(always)]
    pub fn rx_intf(&mut self) -> RxIntfW<'_, IntstatSpec> {
        RxIntfW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit complete interrupt flag"]
    #[inline(always)]
    pub fn txc_intf(&mut self) -> TxcIntfW<'_, IntstatSpec> {
        TxcIntfW::new(self, 2)
    }
    #[doc = "Bit 3 - Start Reveive data interrupt flag"]
    #[inline(always)]
    pub fn start_intf(&mut self) -> StartIntfW<'_, IntstatSpec> {
        StartIntfW::new(self, 3)
    }
    #[doc = "Bit 4 - Stop Reveive data interrupt flag"]
    #[inline(always)]
    pub fn stop_intf(&mut self) -> StopIntfW<'_, IntstatSpec> {
        StopIntfW::new(self, 4)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {}
