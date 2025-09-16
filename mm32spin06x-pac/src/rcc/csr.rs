#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `LSION` reader - Internal low-speed oscillator enable"]
pub type LsionR = crate::BitReader;
#[doc = "Field `LSION` writer - Internal low-speed oscillator enable"]
pub type LsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - Internal low-speed oscillator ready"]
pub type LsirdyR = crate::BitReader;
#[doc = "Field `LSIRDY` writer - Internal low-speed oscillator ready"]
pub type LsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDRSTEN` reader - PVD reset enable"]
pub type PvdrstenR = crate::BitReader;
#[doc = "Field `PVDRSTEN` writer - PVD reset enable"]
pub type PvdrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUPEN` reader - CPU lockup reset enable"]
pub type LockupenR = crate::BitReader;
#[doc = "Field `LOCKUPEN` writer - CPU lockup reset enable"]
pub type LockupenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDRSTF` reader - PVD reset flag"]
pub type PvdrstfR = crate::BitReader;
#[doc = "Field `PVDRSTF` writer - PVD reset flag"]
pub type PvdrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUPF` reader - CPU lockup reset flag"]
pub type LockupfR = crate::BitReader;
#[doc = "Field `LOCKUPF` writer - CPU lockup reset flag"]
pub type LockupfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RmvfR = crate::BitReader;
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RmvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINRSTF` reader - PIN reset flag"]
pub type PinrstfR = crate::BitReader;
#[doc = "Field `PINRSTF` writer - PIN reset flag"]
pub type PinrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub type PorrstfR = crate::BitReader;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub type PorrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SftrstfR = crate::BitReader;
#[doc = "Field `SFTRSTF` writer - Software reset flag"]
pub type SftrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDGRSTF` reader - Independent watchdog reset flag"]
pub type IwdgrstfR = crate::BitReader;
#[doc = "Field `IWDGRSTF` writer - Independent watchdog reset flag"]
pub type IwdgrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WwdgrstfR = crate::BitReader;
#[doc = "Field `WWDGRSTF` writer - Window watchdog reset flag"]
pub type WwdgrstfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LsionR {
        LsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal low-speed oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LsirdyR {
        LsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - PVD reset enable"]
    #[inline(always)]
    pub fn pvdrsten(&self) -> PvdrstenR {
        PvdrstenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU lockup reset enable"]
    #[inline(always)]
    pub fn lockupen(&self) -> LockupenR {
        LockupenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 22 - PVD reset flag"]
    #[inline(always)]
    pub fn pvdrstf(&self) -> PvdrstfR {
        PvdrstfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU lockup reset flag"]
    #[inline(always)]
    pub fn lockupf(&self) -> LockupfR {
        LockupfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RmvfR {
        RmvfR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PinrstfR {
        PinrstfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PorrstfR {
        PorrstfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SftrstfR {
        SftrstfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IwdgrstfR {
        IwdgrstfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WwdgrstfR {
        WwdgrstfR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&mut self) -> LsionW<'_, CsrSpec> {
        LsionW::new(self, 0)
    }
    #[doc = "Bit 1 - Internal low-speed oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&mut self) -> LsirdyW<'_, CsrSpec> {
        LsirdyW::new(self, 1)
    }
    #[doc = "Bit 6 - PVD reset enable"]
    #[inline(always)]
    pub fn pvdrsten(&mut self) -> PvdrstenW<'_, CsrSpec> {
        PvdrstenW::new(self, 6)
    }
    #[doc = "Bit 7 - CPU lockup reset enable"]
    #[inline(always)]
    pub fn lockupen(&mut self) -> LockupenW<'_, CsrSpec> {
        LockupenW::new(self, 7)
    }
    #[doc = "Bit 22 - PVD reset flag"]
    #[inline(always)]
    pub fn pvdrstf(&mut self) -> PvdrstfW<'_, CsrSpec> {
        PvdrstfW::new(self, 22)
    }
    #[doc = "Bit 23 - CPU lockup reset flag"]
    #[inline(always)]
    pub fn lockupf(&mut self) -> LockupfW<'_, CsrSpec> {
        LockupfW::new(self, 23)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RmvfW<'_, CsrSpec> {
        RmvfW::new(self, 24)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PinrstfW<'_, CsrSpec> {
        PinrstfW::new(self, 26)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PorrstfW<'_, CsrSpec> {
        PorrstfW::new(self, 27)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SftrstfW<'_, CsrSpec> {
        SftrstfW::new(self, 28)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&mut self) -> IwdgrstfW<'_, CsrSpec> {
        IwdgrstfW::new(self, 29)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WwdgrstfW<'_, CsrSpec> {
        WwdgrstfW::new(self, 30)
    }
}
#[doc = "Control Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0x0c00_0000"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
