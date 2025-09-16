#[doc = "Register `CCTL` reader"]
pub type R = crate::R<CctlSpec>;
#[doc = "Register `CCTL` writer"]
pub type W = crate::W<CctlSpec>;
#[doc = "Field `CPHA` reader - Clock phase select bit"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase select bit"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity select bit"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity select bit"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBFE` reader - LSI first enable bit"]
pub type LsbfeR = crate::BitReader;
#[doc = "Field `LSBFE` writer - LSI first enable bit"]
pub type LsbfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPILEN` reader - SPI character length bit"]
pub type SpilenR = crate::BitReader;
#[doc = "Field `SPILEN` writer - SPI character length bit"]
pub type SpilenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEDGE` reader - Receive data edge select"]
pub type RxedgeR = crate::BitReader;
#[doc = "Field `RXEDGE` writer - Receive data edge select"]
pub type RxedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEDGE` reader - Transmit data edge select"]
pub type TxedgeR = crate::BitReader;
#[doc = "Field `TXEDGE` writer - Transmit data edge select"]
pub type TxedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHASEL` reader - CPHA polarity select"]
pub type CphaselR = crate::BitReader;
#[doc = "Field `CPHASEL` writer - CPHA polarity select"]
pub type CphaselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HISPD` reader - High speed slave mode"]
pub type HispdR = crate::BitReader;
#[doc = "Field `HISPD` writer - High speed slave mode"]
pub type HispdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock phase select bit"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity select bit"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSI first enable bit"]
    #[inline(always)]
    pub fn lsbfe(&self) -> LsbfeR {
        LsbfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI character length bit"]
    #[inline(always)]
    pub fn spilen(&self) -> SpilenR {
        SpilenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive data edge select"]
    #[inline(always)]
    pub fn rxedge(&self) -> RxedgeR {
        RxedgeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit data edge select"]
    #[inline(always)]
    pub fn txedge(&self) -> TxedgeR {
        TxedgeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPHA polarity select"]
    #[inline(always)]
    pub fn cphasel(&self) -> CphaselR {
        CphaselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - High speed slave mode"]
    #[inline(always)]
    pub fn hispd(&self) -> HispdR {
        HispdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase select bit"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, CctlSpec> {
        CphaW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity select bit"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, CctlSpec> {
        CpolW::new(self, 1)
    }
    #[doc = "Bit 2 - LSI first enable bit"]
    #[inline(always)]
    pub fn lsbfe(&mut self) -> LsbfeW<'_, CctlSpec> {
        LsbfeW::new(self, 2)
    }
    #[doc = "Bit 3 - SPI character length bit"]
    #[inline(always)]
    pub fn spilen(&mut self) -> SpilenW<'_, CctlSpec> {
        SpilenW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive data edge select"]
    #[inline(always)]
    pub fn rxedge(&mut self) -> RxedgeW<'_, CctlSpec> {
        RxedgeW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit data edge select"]
    #[inline(always)]
    pub fn txedge(&mut self) -> TxedgeW<'_, CctlSpec> {
        TxedgeW::new(self, 5)
    }
    #[doc = "Bit 6 - CPHA polarity select"]
    #[inline(always)]
    pub fn cphasel(&mut self) -> CphaselW<'_, CctlSpec> {
        CphaselW::new(self, 6)
    }
    #[doc = "Bit 7 - High speed slave mode"]
    #[inline(always)]
    pub fn hispd(&mut self) -> HispdW<'_, CctlSpec> {
        HispdW::new(self, 7)
    }
}
#[doc = "CCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`cctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CctlSpec;
impl crate::RegisterSpec for CctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cctl::R`](R) reader structure"]
impl crate::Readable for CctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cctl::W`](W) writer structure"]
impl crate::Writable for CctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCTL to value 0x08"]
impl crate::Resettable for CctlSpec {
    const RESET_VALUE: u32 = 0x08;
}
