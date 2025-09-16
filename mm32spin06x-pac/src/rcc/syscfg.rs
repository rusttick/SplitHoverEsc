#[doc = "Register `SYSCFG` reader"]
pub type R = crate::R<SyscfgSpec>;
#[doc = "Register `SYSCFG` writer"]
pub type W = crate::W<SyscfgSpec>;
#[doc = "Field `PROG_CHECK_EN` reader - Whether to check if the data in Flash is 0xFF when writing Flash"]
pub type ProgCheckEnR = crate::BitReader;
#[doc = "Field `PROG_CHECK_EN` writer - Whether to check if the data in Flash is 0xFF when writing Flash"]
pub type ProgCheckEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECTOR_1K_CFG` reader - The size of flash page erase"]
pub type Sector1kCfgR = crate::BitReader;
#[doc = "Field `SECTOR_1K_CFG` writer - The size of flash page erase"]
pub type Sector1kCfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Whether to check if the data in Flash is 0xFF when writing Flash"]
    #[inline(always)]
    pub fn prog_check_en(&self) -> ProgCheckEnR {
        ProgCheckEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The size of flash page erase"]
    #[inline(always)]
    pub fn sector_1k_cfg(&self) -> Sector1kCfgR {
        Sector1kCfgR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Whether to check if the data in Flash is 0xFF when writing Flash"]
    #[inline(always)]
    pub fn prog_check_en(&mut self) -> ProgCheckEnW<'_, SyscfgSpec> {
        ProgCheckEnW::new(self, 0)
    }
    #[doc = "Bit 1 - The size of flash page erase"]
    #[inline(always)]
    pub fn sector_1k_cfg(&mut self) -> Sector1kCfgW<'_, SyscfgSpec> {
        Sector1kCfgW::new(self, 1)
    }
}
#[doc = "System Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgSpec;
impl crate::RegisterSpec for SyscfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg::R`](R) reader structure"]
impl crate::Readable for SyscfgSpec {}
#[doc = "`write(|w| ..)` method takes [`syscfg::W`](W) writer structure"]
impl crate::Writable for SyscfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG to value 0x0010_0c03"]
impl crate::Resettable for SyscfgSpec {
    const RESET_VALUE: u32 = 0x0010_0c03;
}
