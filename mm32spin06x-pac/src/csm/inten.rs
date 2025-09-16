#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `TX_IEN` reader - Transmit FIFO available interrupt enable"]
pub type TxIenR = crate::BitReader;
#[doc = "Field `TX_IEN` writer - Transmit FIFO available interrupt enable"]
pub type TxIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IEN` reader - Receive data available interrupt enable"]
pub type RxIenR = crate::BitReader;
#[doc = "Field `RX_IEN` writer - Receive data available interrupt enable"]
pub type RxIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC_IEN` reader - Transmit complete interrupt enable"]
pub type TxcIenR = crate::BitReader;
#[doc = "Field `TXC_IEN` writer - Transmit complete interrupt enable"]
pub type TxcIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_IEN` reader - Start Reveive data interrupt enable"]
pub type StartIenR = crate::BitReader;
#[doc = "Field `START_IEN` writer - Start Reveive data interrupt enable"]
pub type StartIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_IEN` reader - Stop Reveive data interrupt enable"]
pub type StopIenR = crate::BitReader;
#[doc = "Field `STOP_IEN` writer - Stop Reveive data interrupt enable"]
pub type StopIenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO available interrupt enable"]
    #[inline(always)]
    pub fn tx_ien(&self) -> TxIenR {
        TxIenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive data available interrupt enable"]
    #[inline(always)]
    pub fn rx_ien(&self) -> RxIenR {
        RxIenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn txc_ien(&self) -> TxcIenR {
        TxcIenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start Reveive data interrupt enable"]
    #[inline(always)]
    pub fn start_ien(&self) -> StartIenR {
        StartIenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop Reveive data interrupt enable"]
    #[inline(always)]
    pub fn stop_ien(&self) -> StopIenR {
        StopIenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO available interrupt enable"]
    #[inline(always)]
    pub fn tx_ien(&mut self) -> TxIenW<'_, IntenSpec> {
        TxIenW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive data available interrupt enable"]
    #[inline(always)]
    pub fn rx_ien(&mut self) -> RxIenW<'_, IntenSpec> {
        RxIenW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn txc_ien(&mut self) -> TxcIenW<'_, IntenSpec> {
        TxcIenW::new(self, 2)
    }
    #[doc = "Bit 3 - Start Reveive data interrupt enable"]
    #[inline(always)]
    pub fn start_ien(&mut self) -> StartIenW<'_, IntenSpec> {
        StartIenW::new(self, 3)
    }
    #[doc = "Bit 4 - Stop Reveive data interrupt enable"]
    #[inline(always)]
    pub fn stop_ien(&mut self) -> StopIenW<'_, IntenSpec> {
        StopIenW::new(self, 4)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
