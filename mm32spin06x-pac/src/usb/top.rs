#[doc = "Register `TOP` reader"]
pub type R = crate::R<TopSpec>;
#[doc = "Register `TOP` writer"]
pub type W = crate::W<TopSpec>;
#[doc = "Field `SPEED` reader - SPEED"]
pub type SpeedR = crate::BitReader;
#[doc = "Field `SPEED` writer - SPEED"]
pub type SpeedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONNECT` reader - USB connection"]
pub type ConnectR = crate::BitReader;
#[doc = "Field `CONNECT` writer - USB connection"]
pub type ConnectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Reset EP and FIFO in USB controller"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Reset EP and FIFO in USB controller"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPEND` reader - USB suspend state"]
pub type SuspendR = crate::BitReader;
#[doc = "Field `SUSPEND` writer - USB suspend state"]
pub type SuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_DMSTATE` reader - Current USB DP/DM line state"]
pub type DpDmstateR = crate::FieldReader;
#[doc = "Field `DP_DMSTATE` writer - Current USB DP/DM line state"]
pub type DpDmstateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACTIVE` reader - USB bus is active"]
pub type ActiveR = crate::BitReader;
#[doc = "Field `ACTIVE` writer - USB bus is active"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPEED"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB connection"]
    #[inline(always)]
    pub fn connect(&self) -> ConnectR {
        ConnectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset EP and FIFO in USB controller"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB suspend state"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Current USB DP/DM line state"]
    #[inline(always)]
    pub fn dp_dmstate(&self) -> DpDmstateR {
        DpDmstateR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - USB bus is active"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPEED"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, TopSpec> {
        SpeedW::new(self, 0)
    }
    #[doc = "Bit 1 - USB connection"]
    #[inline(always)]
    pub fn connect(&mut self) -> ConnectW<'_, TopSpec> {
        ConnectW::new(self, 1)
    }
    #[doc = "Bit 3 - Reset EP and FIFO in USB controller"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, TopSpec> {
        ResetW::new(self, 3)
    }
    #[doc = "Bit 4 - USB suspend state"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SuspendW<'_, TopSpec> {
        SuspendW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Current USB DP/DM line state"]
    #[inline(always)]
    pub fn dp_dmstate(&mut self) -> DpDmstateW<'_, TopSpec> {
        DpDmstateW::new(self, 5)
    }
    #[doc = "Bit 7 - USB bus is active"]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<'_, TopSpec> {
        ActiveW::new(self, 7)
    }
}
#[doc = "USB_TOP\n\nYou can [`read`](crate::Reg::read) this register and get [`top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TopSpec;
impl crate::RegisterSpec for TopSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`top::R`](R) reader structure"]
impl crate::Readable for TopSpec {}
#[doc = "`write(|w| ..)` method takes [`top::W`](W) writer structure"]
impl crate::Writable for TopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOP to value 0x02"]
impl crate::Resettable for TopSpec {
    const RESET_VALUE: u16 = 0x02;
}
