#[doc = "Register `PADHYS` reader"]
pub type R = crate::R<PadhysSpec>;
#[doc = "Register `PADHYS` writer"]
pub type W = crate::W<PadhysSpec>;
#[doc = "Field `I2C1_mode_sel` reader - I2C1 mode selection"]
pub type I2c1ModeSelR = crate::BitReader;
#[doc = "Field `I2C1_mode_sel` writer - I2C1 mode selection"]
pub type I2c1ModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - I2C1 mode selection"]
    #[inline(always)]
    pub fn i2c1_mode_sel(&self) -> I2c1ModeSelR {
        I2c1ModeSelR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - I2C1 mode selection"]
    #[inline(always)]
    pub fn i2c1_mode_sel(&mut self) -> I2c1ModeSelW<'_, PadhysSpec> {
        I2c1ModeSelW::new(self, 16)
    }
}
#[doc = "PAD configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`padhys::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padhys::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadhysSpec;
impl crate::RegisterSpec for PadhysSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padhys::R`](R) reader structure"]
impl crate::Readable for PadhysSpec {}
#[doc = "`write(|w| ..)` method takes [`padhys::W`](W) writer structure"]
impl crate::Writable for PadhysSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PADHYS to value 0"]
impl crate::Resettable for PadhysSpec {}
