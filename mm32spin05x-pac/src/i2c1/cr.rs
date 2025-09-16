#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `MASTER` reader - This bit controls whether the DW_apb_i2c master is enabled"]
pub type MasterR = crate::BitReader;
#[doc = "Field `MASTER` writer - This bit controls whether the DW_apb_i2c master is enabled"]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEED` reader - These bits control at which speed the DW_apb_i2c operates"]
pub type SpeedR = crate::FieldReader;
#[doc = "Field `SPEED` writer - These bits control at which speed the DW_apb_i2c operates"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLAVE10` reader - When acting as a alsve"]
pub type Slave10R = crate::BitReader;
#[doc = "Field `SLAVE10` writer - When acting as a alsve"]
pub type Slave10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER10` reader - Address mode when acting as a master"]
pub type Master10R = crate::BitReader;
#[doc = "Field `MASTER10` writer - Address mode when acting as a master"]
pub type Master10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPEN` reader - Determines whether RESTART comdtions may be sent when acting as a master"]
pub type RepenR = crate::BitReader;
#[doc = "Field `REPEN` writer - Determines whether RESTART comdtions may be sent when acting as a master"]
pub type RepenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISSLAVE` reader - This bit controls whether I2C has its slave diabled"]
pub type DisslaveR = crate::BitReader;
#[doc = "Field `DISSLAVE` writer - This bit controls whether I2C has its slave diabled"]
pub type DisslaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPINT` reader - STOP_DET_IFADDRESSED"]
pub type StopintR = crate::BitReader;
#[doc = "Field `STOPINT` writer - STOP_DET_IFADDRESSED"]
pub type StopintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPINT` reader - This bit controls the generation of the TX_EMPTY interrupt"]
pub type EmpintR = crate::BitReader;
#[doc = "Field `EMPINT` writer - This bit controls the generation of the TX_EMPTY interrupt"]
pub type EmpintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Whether to generate a STOP signal after sending or receiving"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Whether to generate a STOP signal after sending or receiving"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTART` reader - Whether to generate a RESTART signal after sending or receiving"]
pub type RestartR = crate::BitReader;
#[doc = "Field `RESTART` writer - Whether to generate a RESTART signal after sending or receiving"]
pub type RestartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit controls whether the DW_apb_i2c master is enabled"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the DW_apb_i2c operates"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - When acting as a alsve"]
    #[inline(always)]
    pub fn slave10(&self) -> Slave10R {
        Slave10R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Address mode when acting as a master"]
    #[inline(always)]
    pub fn master10(&self) -> Master10R {
        Master10R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Determines whether RESTART comdtions may be sent when acting as a master"]
    #[inline(always)]
    pub fn repen(&self) -> RepenR {
        RepenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave diabled"]
    #[inline(always)]
    pub fn disslave(&self) -> DisslaveR {
        DisslaveR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline(always)]
    pub fn stopint(&self) -> StopintR {
        StopintR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit controls the generation of the TX_EMPTY interrupt"]
    #[inline(always)]
    pub fn empint(&self) -> EmpintR {
        EmpintR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Whether to generate a STOP signal after sending or receiving"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Whether to generate a RESTART signal after sending or receiving"]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls whether the DW_apb_i2c master is enabled"]
    #[inline(always)]
    pub fn master(&mut self) -> MasterW<'_, CrSpec> {
        MasterW::new(self, 0)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the DW_apb_i2c operates"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, CrSpec> {
        SpeedW::new(self, 1)
    }
    #[doc = "Bit 3 - When acting as a alsve"]
    #[inline(always)]
    pub fn slave10(&mut self) -> Slave10W<'_, CrSpec> {
        Slave10W::new(self, 3)
    }
    #[doc = "Bit 4 - Address mode when acting as a master"]
    #[inline(always)]
    pub fn master10(&mut self) -> Master10W<'_, CrSpec> {
        Master10W::new(self, 4)
    }
    #[doc = "Bit 5 - Determines whether RESTART comdtions may be sent when acting as a master"]
    #[inline(always)]
    pub fn repen(&mut self) -> RepenW<'_, CrSpec> {
        RepenW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave diabled"]
    #[inline(always)]
    pub fn disslave(&mut self) -> DisslaveW<'_, CrSpec> {
        DisslaveW::new(self, 6)
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline(always)]
    pub fn stopint(&mut self) -> StopintW<'_, CrSpec> {
        StopintW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit controls the generation of the TX_EMPTY interrupt"]
    #[inline(always)]
    pub fn empint(&mut self) -> EmpintW<'_, CrSpec> {
        EmpintW::new(self, 8)
    }
    #[doc = "Bit 9 - Whether to generate a STOP signal after sending or receiving"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, CrSpec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - Whether to generate a RESTART signal after sending or receiving"]
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<'_, CrSpec> {
        RestartW::new(self, 10)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0x7f"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x7f;
}
