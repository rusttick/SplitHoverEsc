#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<IntEnSpec>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<IntEnSpec>;
#[doc = "Field `RSTIE` reader - BUS reset interrupt enable"]
pub type RstieR = crate::BitReader;
#[doc = "Field `RSTIE` writer - BUS reset interrupt enable"]
pub type RstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPENDIE` reader - BUS suspend interrupt enable"]
pub type SuspendieR = crate::BitReader;
#[doc = "Field `SUSPENDIE` writer - BUS suspend interrupt enable"]
pub type SuspendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUMIE` reader - BUS resume interrupt enable"]
pub type ResumieR = crate::BitReader;
#[doc = "Field `RESUMIE` writer - BUS resume interrupt enable"]
pub type ResumieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFIE` reader - SOF interrupt enable"]
pub type SofieR = crate::BitReader;
#[doc = "Field `SOFIE` writer - SOF interrupt enable"]
pub type SofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPIE` reader - EP interrupt enable"]
pub type EpieR = crate::BitReader;
#[doc = "Field `EPIE` writer - EP interrupt enable"]
pub type EpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMASK` reader - interrupt mask"]
pub type IntmaskR = crate::BitReader;
#[doc = "Field `INTMASK` writer - interrupt mask"]
pub type IntmaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BUS reset interrupt enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RstieR {
        RstieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BUS suspend interrupt enable"]
    #[inline(always)]
    pub fn suspendie(&self) -> SuspendieR {
        SuspendieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BUS resume interrupt enable"]
    #[inline(always)]
    pub fn resumie(&self) -> ResumieR {
        ResumieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOF interrupt enable"]
    #[inline(always)]
    pub fn sofie(&self) -> SofieR {
        SofieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP interrupt enable"]
    #[inline(always)]
    pub fn epie(&self) -> EpieR {
        EpieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt mask"]
    #[inline(always)]
    pub fn intmask(&self) -> IntmaskR {
        IntmaskR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BUS reset interrupt enable"]
    #[inline(always)]
    pub fn rstie(&mut self) -> RstieW<'_, IntEnSpec> {
        RstieW::new(self, 0)
    }
    #[doc = "Bit 1 - BUS suspend interrupt enable"]
    #[inline(always)]
    pub fn suspendie(&mut self) -> SuspendieW<'_, IntEnSpec> {
        SuspendieW::new(self, 1)
    }
    #[doc = "Bit 2 - BUS resume interrupt enable"]
    #[inline(always)]
    pub fn resumie(&mut self) -> ResumieW<'_, IntEnSpec> {
        ResumieW::new(self, 2)
    }
    #[doc = "Bit 3 - SOF interrupt enable"]
    #[inline(always)]
    pub fn sofie(&mut self) -> SofieW<'_, IntEnSpec> {
        SofieW::new(self, 3)
    }
    #[doc = "Bit 4 - EP interrupt enable"]
    #[inline(always)]
    pub fn epie(&mut self) -> EpieW<'_, IntEnSpec> {
        EpieW::new(self, 4)
    }
    #[doc = "Bit 7 - interrupt mask"]
    #[inline(always)]
    pub fn intmask(&mut self) -> IntmaskW<'_, IntEnSpec> {
        IntmaskW::new(self, 7)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnSpec;
impl crate::RegisterSpec for IntEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for IntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for IntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for IntEnSpec {}
