#[doc = "Register `GCR` reader"]
pub type R = crate::R<GcrSpec>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GcrSpec>;
#[doc = "Field `UARTEN` reader - UART mode selection bit"]
pub type UartenR = crate::BitReader;
#[doc = "Field `UARTEN` writer - UART mode selection bit"]
pub type UartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMODE` reader - DMA mode selection bit"]
pub type DmamodeR = crate::BitReader;
#[doc = "Field `DMAMODE` writer - DMA mode selection bit"]
pub type DmamodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOFLOWEN` reader - Automatic flow control enable bit"]
pub type AutoflowenR = crate::BitReader;
#[doc = "Field `AUTOFLOWEN` writer - Automatic flow control enable bit"]
pub type AutoflowenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Enable receive"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Enable receive"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` reader - Enable transmit"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Enable transmit"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_B8` reader - Select bit8"]
pub type SelB8R = crate::BitReader;
#[doc = "Field `SEL_B8` writer - Select bit8"]
pub type SelB8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - change swap"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - change swap"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TOG` reader - Toggle RX"]
pub type RxTogR = crate::BitReader;
#[doc = "Field `RX_TOG` writer - Toggle RX"]
pub type RxTogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_TOG` reader - Toggle TX"]
pub type TxTogR = crate::BitReader;
#[doc = "Field `TX_TOG` writer - Toggle TX"]
pub type TxTogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART mode selection bit"]
    #[inline(always)]
    pub fn uarten(&self) -> UartenR {
        UartenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA mode selection bit"]
    #[inline(always)]
    pub fn dmamode(&self) -> DmamodeR {
        DmamodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic flow control enable bit"]
    #[inline(always)]
    pub fn autoflowen(&self) -> AutoflowenR {
        AutoflowenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable receive"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable transmit"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Select bit8"]
    #[inline(always)]
    pub fn sel_b8(&self) -> SelB8R {
        SelB8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - change swap"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Toggle RX"]
    #[inline(always)]
    pub fn rx_tog(&self) -> RxTogR {
        RxTogR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Toggle TX"]
    #[inline(always)]
    pub fn tx_tog(&self) -> TxTogR {
        TxTogR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART mode selection bit"]
    #[inline(always)]
    pub fn uarten(&mut self) -> UartenW<'_, GcrSpec> {
        UartenW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA mode selection bit"]
    #[inline(always)]
    pub fn dmamode(&mut self) -> DmamodeW<'_, GcrSpec> {
        DmamodeW::new(self, 1)
    }
    #[doc = "Bit 2 - Automatic flow control enable bit"]
    #[inline(always)]
    pub fn autoflowen(&mut self) -> AutoflowenW<'_, GcrSpec> {
        AutoflowenW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable receive"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, GcrSpec> {
        RxenW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable transmit"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, GcrSpec> {
        TxenW::new(self, 4)
    }
    #[doc = "Bit 7 - Select bit8"]
    #[inline(always)]
    pub fn sel_b8(&mut self) -> SelB8W<'_, GcrSpec> {
        SelB8W::new(self, 7)
    }
    #[doc = "Bit 8 - change swap"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<'_, GcrSpec> {
        SwapW::new(self, 8)
    }
    #[doc = "Bit 9 - Toggle RX"]
    #[inline(always)]
    pub fn rx_tog(&mut self) -> RxTogW<'_, GcrSpec> {
        RxTogW::new(self, 9)
    }
    #[doc = "Bit 10 - Toggle TX"]
    #[inline(always)]
    pub fn tx_tog(&mut self) -> TxTogW<'_, GcrSpec> {
        TxTogW::new(self, 10)
    }
}
#[doc = "Global control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcrSpec;
impl crate::RegisterSpec for GcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GcrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GcrSpec {}
