#[doc = "Register `GCTL` reader"]
pub type R = crate::R<GctlSpec>;
#[doc = "Register `GCTL` writer"]
pub type W = crate::W<GctlSpec>;
#[doc = "Field `SPIEN` reader - SPI select bit"]
pub type SpienR = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPI select bit"]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_EN` reader - SPI interrupt enable bit"]
pub type IntEnR = crate::BitReader;
#[doc = "Field `INT_EN` writer - SPI interrupt enable bit"]
pub type IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Master mode bit"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - Master mode bit"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` reader - Transmit enable bit"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmit enable bit"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Receive enable bit"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receive enable bit"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTLF` reader - RX FIFO trigger level bit"]
pub type RxtlfR = crate::FieldReader;
#[doc = "Field `RXTLF` writer - RX FIFO trigger level bit"]
pub type RxtlfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXTLF` reader - TX FIFO trigger level bit"]
pub type TxtlfR = crate::FieldReader;
#[doc = "Field `TXTLF` writer - TX FIFO trigger level bit"]
pub type TxtlfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMAEN` reader - DMA access mode enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA access mode enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSS_SEL` reader - NSS select signal that from software and hardware"]
pub type NssSelR = crate::BitReader;
#[doc = "Field `NSS_SEL` writer - NSS select signal that from software and hardware"]
pub type NssSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DW8_32` reader - Valid byte or double-word data select signal"]
pub type Dw8_32R = crate::BitReader;
#[doc = "Field `DW8_32` writer - Valid byte or double-word data select signal"]
pub type Dw8_32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSTOG` reader - NSS selection signal is automatically flipped"]
pub type NsstogR = crate::BitReader;
#[doc = "Field `NSSTOG` writer - NSS selection signal is automatically flipped"]
pub type NsstogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI select bit"]
    #[inline(always)]
    pub fn spien(&self) -> SpienR {
        SpienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI interrupt enable bit"]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master mode bit"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit enable bit"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive enable bit"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - RX FIFO trigger level bit"]
    #[inline(always)]
    pub fn rxtlf(&self) -> RxtlfR {
        RxtlfR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - TX FIFO trigger level bit"]
    #[inline(always)]
    pub fn txtlf(&self) -> TxtlfR {
        TxtlfR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - DMA access mode enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NSS select signal that from software and hardware"]
    #[inline(always)]
    pub fn nss_sel(&self) -> NssSelR {
        NssSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Valid byte or double-word data select signal"]
    #[inline(always)]
    pub fn dw8_32(&self) -> Dw8_32R {
        Dw8_32R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NSS selection signal is automatically flipped"]
    #[inline(always)]
    pub fn nsstog(&self) -> NsstogR {
        NsstogR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI select bit"]
    #[inline(always)]
    pub fn spien(&mut self) -> SpienW<'_, GctlSpec> {
        SpienW::new(self, 0)
    }
    #[doc = "Bit 1 - SPI interrupt enable bit"]
    #[inline(always)]
    pub fn int_en(&mut self) -> IntEnW<'_, GctlSpec> {
        IntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Master mode bit"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, GctlSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit enable bit"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, GctlSpec> {
        TxenW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive enable bit"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, GctlSpec> {
        RxenW::new(self, 4)
    }
    #[doc = "Bits 5:6 - RX FIFO trigger level bit"]
    #[inline(always)]
    pub fn rxtlf(&mut self) -> RxtlfW<'_, GctlSpec> {
        RxtlfW::new(self, 5)
    }
    #[doc = "Bits 7:8 - TX FIFO trigger level bit"]
    #[inline(always)]
    pub fn txtlf(&mut self) -> TxtlfW<'_, GctlSpec> {
        TxtlfW::new(self, 7)
    }
    #[doc = "Bit 9 - DMA access mode enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, GctlSpec> {
        DmaenW::new(self, 9)
    }
    #[doc = "Bit 10 - NSS select signal that from software and hardware"]
    #[inline(always)]
    pub fn nss_sel(&mut self) -> NssSelW<'_, GctlSpec> {
        NssSelW::new(self, 10)
    }
    #[doc = "Bit 11 - Valid byte or double-word data select signal"]
    #[inline(always)]
    pub fn dw8_32(&mut self) -> Dw8_32W<'_, GctlSpec> {
        Dw8_32W::new(self, 11)
    }
    #[doc = "Bit 12 - NSS selection signal is automatically flipped"]
    #[inline(always)]
    pub fn nsstog(&mut self) -> NsstogW<'_, GctlSpec> {
        NsstogW::new(self, 12)
    }
}
#[doc = "GCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`gctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GctlSpec;
impl crate::RegisterSpec for GctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gctl::R`](R) reader structure"]
impl crate::Readable for GctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gctl::W`](W) writer structure"]
impl crate::Writable for GctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCTL to value 0x04"]
impl crate::Resettable for GctlSpec {
    const RESET_VALUE: u32 = 0x04;
}
