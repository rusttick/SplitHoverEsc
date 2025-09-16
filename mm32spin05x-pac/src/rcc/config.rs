#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `PAGESIZE` reader - Flash Page size"]
pub type PagesizeR = crate::BitReader;
#[doc = "Field `PAGESIZE` writer - Flash Page size"]
pub type PagesizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC_RTRIM` reader - Oscillator feedback resistance trimming"]
pub type OscRtrimR = crate::FieldReader;
#[doc = "Field `OSC_RTRIM` writer - Oscillator feedback resistance trimming"]
pub type OscRtrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OSC_ITRIM` reader - Oscillator drive current trimming"]
pub type OscItrimR = crate::FieldReader;
#[doc = "Field `OSC_ITRIM` writer - Oscillator drive current trimming"]
pub type OscItrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSC_LPFEN` reader - Oscillator low pass filtering enable"]
pub type OscLpfenR = crate::BitReader;
#[doc = "Field `OSC_LPFEN` writer - Oscillator low pass filtering enable"]
pub type OscLpfenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Flash Page size"]
    #[inline(always)]
    pub fn pagesize(&self) -> PagesizeR {
        PagesizeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator feedback resistance trimming"]
    #[inline(always)]
    pub fn osc_rtrim(&self) -> OscRtrimR {
        OscRtrimR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Oscillator drive current trimming"]
    #[inline(always)]
    pub fn osc_itrim(&self) -> OscItrimR {
        OscItrimR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - Oscillator low pass filtering enable"]
    #[inline(always)]
    pub fn osc_lpfen(&self) -> OscLpfenR {
        OscLpfenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Flash Page size"]
    #[inline(always)]
    pub fn pagesize(&mut self) -> PagesizeW<'_, ConfigSpec> {
        PagesizeW::new(self, 1)
    }
    #[doc = "Bits 8:10 - Oscillator feedback resistance trimming"]
    #[inline(always)]
    pub fn osc_rtrim(&mut self) -> OscRtrimW<'_, ConfigSpec> {
        OscRtrimW::new(self, 8)
    }
    #[doc = "Bits 11:12 - Oscillator drive current trimming"]
    #[inline(always)]
    pub fn osc_itrim(&mut self) -> OscItrimW<'_, ConfigSpec> {
        OscItrimW::new(self, 11)
    }
    #[doc = "Bit 14 - Oscillator low pass filtering enable"]
    #[inline(always)]
    pub fn osc_lpfen(&mut self) -> OscLpfenW<'_, ConfigSpec> {
        OscLpfenW::new(self, 14)
    }
}
#[doc = "System Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {}
