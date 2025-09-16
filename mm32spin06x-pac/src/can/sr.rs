#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `RBS` reader - Receive buffer status"]
pub type RbsR = crate::BitReader;
#[doc = "Field `RBS` writer - Receive buffer status"]
pub type RbsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOS` reader - Data overrun status"]
pub type DosR = crate::BitReader;
#[doc = "Field `DOS` writer - Data overrun status"]
pub type DosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBS` reader - Transmit buffer status"]
pub type TbsR = crate::BitReader;
#[doc = "Field `TBS` writer - Transmit buffer status"]
pub type TbsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCS` reader - Transmission complete status"]
pub type TcsR = crate::BitReader;
#[doc = "Field `TCS` writer - Transmission complete status"]
pub type TcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS` reader - Receive status"]
pub type RsR = crate::BitReader;
#[doc = "Field `RS` writer - Receive status"]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS` reader - Transmit status"]
pub type TsR = crate::BitReader;
#[doc = "Field `TS` writer - Transmit status"]
pub type TsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ES` reader - Error status"]
pub type EsR = crate::BitReader;
#[doc = "Field `ES` writer - Error status"]
pub type EsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS` reader - Bus status"]
pub type BsR = crate::BitReader;
#[doc = "Field `BS` writer - Bus status"]
pub type BsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive buffer status"]
    #[inline(always)]
    pub fn rbs(&self) -> RbsR {
        RbsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data overrun status"]
    #[inline(always)]
    pub fn dos(&self) -> DosR {
        DosR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer status"]
    #[inline(always)]
    pub fn tbs(&self) -> TbsR {
        TbsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission complete status"]
    #[inline(always)]
    pub fn tcs(&self) -> TcsR {
        TcsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive status"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit status"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error status"]
    #[inline(always)]
    pub fn es(&self) -> EsR {
        EsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus status"]
    #[inline(always)]
    pub fn bs(&self) -> BsR {
        BsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive buffer status"]
    #[inline(always)]
    pub fn rbs(&mut self) -> RbsW<'_, SrSpec> {
        RbsW::new(self, 0)
    }
    #[doc = "Bit 1 - Data overrun status"]
    #[inline(always)]
    pub fn dos(&mut self) -> DosW<'_, SrSpec> {
        DosW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit buffer status"]
    #[inline(always)]
    pub fn tbs(&mut self) -> TbsW<'_, SrSpec> {
        TbsW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmission complete status"]
    #[inline(always)]
    pub fn tcs(&mut self) -> TcsW<'_, SrSpec> {
        TcsW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive status"]
    #[inline(always)]
    pub fn rs(&mut self) -> RsW<'_, SrSpec> {
        RsW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit status"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<'_, SrSpec> {
        TsW::new(self, 5)
    }
    #[doc = "Bit 6 - Error status"]
    #[inline(always)]
    pub fn es(&mut self) -> EsW<'_, SrSpec> {
        EsW::new(self, 6)
    }
    #[doc = "Bit 7 - Bus status"]
    #[inline(always)]
    pub fn bs(&mut self) -> BsW<'_, SrSpec> {
        BsW::new(self, 7)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0x0c"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x0c;
}
