#[doc = "Register `SAR` reader"]
pub type R = crate::R<SarSpec>;
#[doc = "Register `SAR` writer"]
pub type W = crate::W<SarSpec>;
#[doc = "Field `ADDR` reader - The SAR holds the slave address when the i2c is operation as a slave"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - The SAR holds the slave address when the i2c is operation as a slave"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - The SAR holds the slave address when the i2c is operation as a slave"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The SAR holds the slave address when the i2c is operation as a slave"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SarSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SarSpec;
impl crate::RegisterSpec for SarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar::R`](R) reader structure"]
impl crate::Readable for SarSpec {}
#[doc = "`write(|w| ..)` method takes [`sar::W`](W) writer structure"]
impl crate::Writable for SarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR to value 0x55"]
impl crate::Resettable for SarSpec {
    const RESET_VALUE: u32 = 0x55;
}
