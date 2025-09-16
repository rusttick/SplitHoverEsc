#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ADIE` reader - ADC interrupt enable"]
pub type AdieR = crate::BitReader;
#[doc = "Field `ADIE` writer - ADC interrupt enable"]
pub type AdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADWIE` reader - ADC window comparator interrupt enable"]
pub type AdwieR = crate::BitReader;
#[doc = "Field `ADWIE` writer - ADC window comparator interrupt enable"]
pub type AdwieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGEN` reader - External trigger enable"]
pub type TrgenR = crate::BitReader;
#[doc = "Field `TRGEN` writer - External trigger enable"]
pub type TrgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - Direct memory access enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - Direct memory access enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSELL` reader - External trigger selection low"]
pub type TrgsellR = crate::FieldReader;
#[doc = "Field `TRGSELL` writer - External trigger selection low"]
pub type TrgsellW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADST` reader - ADC start"]
pub type AdstR = crate::BitReader;
#[doc = "Field `ADST` writer - ADC start"]
pub type AdstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMD` reader - ADC mode"]
pub type AdmdR = crate::FieldReader;
#[doc = "Field `ADMD` writer - ADC mode"]
pub type AdmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ALIGN` reader - Data alignment"]
pub type AlignR = crate::BitReader;
#[doc = "Field `ALIGN` writer - Data alignment"]
pub type AlignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCH` reader - Window comparison channel selection"]
pub type CmpchR = crate::FieldReader;
#[doc = "Field `CMPCH` writer - Window comparison channel selection"]
pub type CmpchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCANDIR` reader - ADC scan direction"]
pub type ScandirR = crate::BitReader;
#[doc = "Field `SCANDIR` writer - ADC scan direction"]
pub type ScandirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSELH` reader - External trigger selection high"]
pub type TrgselhR = crate::FieldReader;
#[doc = "Field `TRGSELH` writer - External trigger selection high"]
pub type TrgselhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRGSHIFT` reader - External trigger shift sample"]
pub type TrgshiftR = crate::FieldReader;
#[doc = "Field `TRGSHIFT` writer - External trigger shift sample"]
pub type TrgshiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRG_EDGE` reader - Trigger edge selection"]
pub type TrgEdgeR = crate::FieldReader;
#[doc = "Field `TRG_EDGE` writer - Trigger edge selection"]
pub type TrgEdgeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - ADC interrupt enable"]
    #[inline(always)]
    pub fn adie(&self) -> AdieR {
        AdieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC window comparator interrupt enable"]
    #[inline(always)]
    pub fn adwie(&self) -> AdwieR {
        AdwieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TrgenR {
        TrgenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Direct memory access enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - External trigger selection low"]
    #[inline(always)]
    pub fn trgsell(&self) -> TrgsellR {
        TrgsellR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - ADC start"]
    #[inline(always)]
    pub fn adst(&self) -> AdstR {
        AdstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - ADC mode"]
    #[inline(always)]
    pub fn admd(&self) -> AdmdR {
        AdmdR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> AlignR {
        AlignR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Window comparison channel selection"]
    #[inline(always)]
    pub fn cmpch(&self) -> CmpchR {
        CmpchR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - ADC scan direction"]
    #[inline(always)]
    pub fn scandir(&self) -> ScandirR {
        ScandirR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - External trigger selection high"]
    #[inline(always)]
    pub fn trgselh(&self) -> TrgselhR {
        TrgselhR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - External trigger shift sample"]
    #[inline(always)]
    pub fn trgshift(&self) -> TrgshiftR {
        TrgshiftR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Trigger edge selection"]
    #[inline(always)]
    pub fn trg_edge(&self) -> TrgEdgeR {
        TrgEdgeR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC interrupt enable"]
    #[inline(always)]
    pub fn adie(&mut self) -> AdieW<'_, CrSpec> {
        AdieW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC window comparator interrupt enable"]
    #[inline(always)]
    pub fn adwie(&mut self) -> AdwieW<'_, CrSpec> {
        AdwieW::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger enable"]
    #[inline(always)]
    pub fn trgen(&mut self) -> TrgenW<'_, CrSpec> {
        TrgenW::new(self, 2)
    }
    #[doc = "Bit 3 - Direct memory access enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, CrSpec> {
        DmaenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - External trigger selection low"]
    #[inline(always)]
    pub fn trgsell(&mut self) -> TrgsellW<'_, CrSpec> {
        TrgsellW::new(self, 4)
    }
    #[doc = "Bit 8 - ADC start"]
    #[inline(always)]
    pub fn adst(&mut self) -> AdstW<'_, CrSpec> {
        AdstW::new(self, 8)
    }
    #[doc = "Bits 9:10 - ADC mode"]
    #[inline(always)]
    pub fn admd(&mut self) -> AdmdW<'_, CrSpec> {
        AdmdW::new(self, 9)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&mut self) -> AlignW<'_, CrSpec> {
        AlignW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Window comparison channel selection"]
    #[inline(always)]
    pub fn cmpch(&mut self) -> CmpchW<'_, CrSpec> {
        CmpchW::new(self, 12)
    }
    #[doc = "Bit 16 - ADC scan direction"]
    #[inline(always)]
    pub fn scandir(&mut self) -> ScandirW<'_, CrSpec> {
        ScandirW::new(self, 16)
    }
    #[doc = "Bits 17:18 - External trigger selection high"]
    #[inline(always)]
    pub fn trgselh(&mut self) -> TrgselhW<'_, CrSpec> {
        TrgselhW::new(self, 17)
    }
    #[doc = "Bits 19:21 - External trigger shift sample"]
    #[inline(always)]
    pub fn trgshift(&mut self) -> TrgshiftW<'_, CrSpec> {
        TrgshiftW::new(self, 19)
    }
    #[doc = "Bits 24:25 - Trigger edge selection"]
    #[inline(always)]
    pub fn trg_edge(&mut self) -> TrgEdgeW<'_, CrSpec> {
        TrgEdgeW::new(self, 24)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
