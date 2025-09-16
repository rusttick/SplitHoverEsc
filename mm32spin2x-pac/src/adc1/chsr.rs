#[doc = "Register `CHSR` reader"]
pub type R = crate::R<ChsrSpec>;
#[doc = "Register `CHSR` writer"]
pub type W = crate::W<ChsrSpec>;
#[doc = "Field `CH0EN` reader - Analog input channel 0 enable"]
pub type Ch0enR = crate::BitReader;
#[doc = "Field `CH0EN` writer - Analog input channel 0 enable"]
pub type Ch0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1EN` reader - Analog input channel 1 enable"]
pub type Ch1enR = crate::BitReader;
#[doc = "Field `CH1EN` writer - Analog input channel 1 enable"]
pub type Ch1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2EN` reader - Analog input channel 2 enable"]
pub type Ch2enR = crate::BitReader;
#[doc = "Field `CH2EN` writer - Analog input channel 2 enable"]
pub type Ch2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3EN` reader - Analog input channel 3 enable"]
pub type Ch3enR = crate::BitReader;
#[doc = "Field `CH3EN` writer - Analog input channel 3 enable"]
pub type Ch3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4EN` reader - Analog input channel 4 enable"]
pub type Ch4enR = crate::BitReader;
#[doc = "Field `CH4EN` writer - Analog input channel 4 enable"]
pub type Ch4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5EN` reader - Analog input channel 5 enable"]
pub type Ch5enR = crate::BitReader;
#[doc = "Field `CH5EN` writer - Analog input channel 5 enable"]
pub type Ch5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6EN` reader - Analog input channel 6 enable"]
pub type Ch6enR = crate::BitReader;
#[doc = "Field `CH6EN` writer - Analog input channel 6 enable"]
pub type Ch6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7EN` reader - Analog input channel 7 enable"]
pub type Ch7enR = crate::BitReader;
#[doc = "Field `CH7EN` writer - Analog input channel 7 enable"]
pub type Ch7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8EN` reader - Analog input channel 8 enable"]
pub type Ch8enR = crate::BitReader;
#[doc = "Field `CH8EN` writer - Analog input channel 8 enable"]
pub type Ch8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9EN` reader - Analog input channel 9 enable"]
pub type Ch9enR = crate::BitReader;
#[doc = "Field `CH9EN` writer - Analog input channel 9 enable"]
pub type Ch9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10EN` reader - Analog input channel 10 enable"]
pub type Ch10enR = crate::BitReader;
#[doc = "Field `CH10EN` writer - Analog input channel 10 enable"]
pub type Ch10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11EN` reader - Analog input channel 11 enable"]
pub type Ch11enR = crate::BitReader;
#[doc = "Field `CH11EN` writer - Analog input channel 11 enable"]
pub type Ch11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHCALIB` reader - Enable ADC internal self-calibration channel"]
pub type ChcalibR = crate::BitReader;
#[doc = "Field `CHCALIB` writer - Enable ADC internal self-calibration channel"]
pub type ChcalibW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTSEN` reader - Temperature Sensor channel enable"]
pub type ChtsenR = crate::BitReader;
#[doc = "Field `CHTSEN` writer - Temperature Sensor channel enable"]
pub type ChtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHVSEN` reader - Internal reference voltage channel enable"]
pub type ChvsenR = crate::BitReader;
#[doc = "Field `CHVSEN` writer - Internal reference voltage channel enable"]
pub type ChvsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog input channel 0 enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> Ch0enR {
        Ch0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog input channel 1 enable"]
    #[inline(always)]
    pub fn ch1en(&self) -> Ch1enR {
        Ch1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog input channel 2 enable"]
    #[inline(always)]
    pub fn ch2en(&self) -> Ch2enR {
        Ch2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog input channel 3 enable"]
    #[inline(always)]
    pub fn ch3en(&self) -> Ch3enR {
        Ch3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog input channel 4 enable"]
    #[inline(always)]
    pub fn ch4en(&self) -> Ch4enR {
        Ch4enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog input channel 5 enable"]
    #[inline(always)]
    pub fn ch5en(&self) -> Ch5enR {
        Ch5enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog input channel 6 enable"]
    #[inline(always)]
    pub fn ch6en(&self) -> Ch6enR {
        Ch6enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog input channel 7 enable"]
    #[inline(always)]
    pub fn ch7en(&self) -> Ch7enR {
        Ch7enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog input channel 8 enable"]
    #[inline(always)]
    pub fn ch8en(&self) -> Ch8enR {
        Ch8enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog input channel 9 enable"]
    #[inline(always)]
    pub fn ch9en(&self) -> Ch9enR {
        Ch9enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Analog input channel 10 enable"]
    #[inline(always)]
    pub fn ch10en(&self) -> Ch10enR {
        Ch10enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Analog input channel 11 enable"]
    #[inline(always)]
    pub fn ch11en(&self) -> Ch11enR {
        Ch11enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable ADC internal self-calibration channel"]
    #[inline(always)]
    pub fn chcalib(&self) -> ChcalibR {
        ChcalibR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Temperature Sensor channel enable"]
    #[inline(always)]
    pub fn chtsen(&self) -> ChtsenR {
        ChtsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Internal reference voltage channel enable"]
    #[inline(always)]
    pub fn chvsen(&self) -> ChvsenR {
        ChvsenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog input channel 0 enable"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> Ch0enW<'_, ChsrSpec> {
        Ch0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Analog input channel 1 enable"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> Ch1enW<'_, ChsrSpec> {
        Ch1enW::new(self, 1)
    }
    #[doc = "Bit 2 - Analog input channel 2 enable"]
    #[inline(always)]
    pub fn ch2en(&mut self) -> Ch2enW<'_, ChsrSpec> {
        Ch2enW::new(self, 2)
    }
    #[doc = "Bit 3 - Analog input channel 3 enable"]
    #[inline(always)]
    pub fn ch3en(&mut self) -> Ch3enW<'_, ChsrSpec> {
        Ch3enW::new(self, 3)
    }
    #[doc = "Bit 4 - Analog input channel 4 enable"]
    #[inline(always)]
    pub fn ch4en(&mut self) -> Ch4enW<'_, ChsrSpec> {
        Ch4enW::new(self, 4)
    }
    #[doc = "Bit 5 - Analog input channel 5 enable"]
    #[inline(always)]
    pub fn ch5en(&mut self) -> Ch5enW<'_, ChsrSpec> {
        Ch5enW::new(self, 5)
    }
    #[doc = "Bit 6 - Analog input channel 6 enable"]
    #[inline(always)]
    pub fn ch6en(&mut self) -> Ch6enW<'_, ChsrSpec> {
        Ch6enW::new(self, 6)
    }
    #[doc = "Bit 7 - Analog input channel 7 enable"]
    #[inline(always)]
    pub fn ch7en(&mut self) -> Ch7enW<'_, ChsrSpec> {
        Ch7enW::new(self, 7)
    }
    #[doc = "Bit 8 - Analog input channel 8 enable"]
    #[inline(always)]
    pub fn ch8en(&mut self) -> Ch8enW<'_, ChsrSpec> {
        Ch8enW::new(self, 8)
    }
    #[doc = "Bit 9 - Analog input channel 9 enable"]
    #[inline(always)]
    pub fn ch9en(&mut self) -> Ch9enW<'_, ChsrSpec> {
        Ch9enW::new(self, 9)
    }
    #[doc = "Bit 10 - Analog input channel 10 enable"]
    #[inline(always)]
    pub fn ch10en(&mut self) -> Ch10enW<'_, ChsrSpec> {
        Ch10enW::new(self, 10)
    }
    #[doc = "Bit 11 - Analog input channel 11 enable"]
    #[inline(always)]
    pub fn ch11en(&mut self) -> Ch11enW<'_, ChsrSpec> {
        Ch11enW::new(self, 11)
    }
    #[doc = "Bit 13 - Enable ADC internal self-calibration channel"]
    #[inline(always)]
    pub fn chcalib(&mut self) -> ChcalibW<'_, ChsrSpec> {
        ChcalibW::new(self, 13)
    }
    #[doc = "Bit 14 - Temperature Sensor channel enable"]
    #[inline(always)]
    pub fn chtsen(&mut self) -> ChtsenW<'_, ChsrSpec> {
        ChtsenW::new(self, 14)
    }
    #[doc = "Bit 15 - Internal reference voltage channel enable"]
    #[inline(always)]
    pub fn chvsen(&mut self) -> ChvsenW<'_, ChsrSpec> {
        ChvsenW::new(self, 15)
    }
}
#[doc = "Channel select register\n\nYou can [`read`](crate::Reg::read) this register and get [`chsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChsrSpec;
impl crate::RegisterSpec for ChsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsr::R`](R) reader structure"]
impl crate::Readable for ChsrSpec {}
#[doc = "`write(|w| ..)` method takes [`chsr::W`](W) writer structure"]
impl crate::Writable for ChsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSR to value 0"]
impl crate::Resettable for ChsrSpec {}
